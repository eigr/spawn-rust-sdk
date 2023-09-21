#[macro_use]
extern crate rocket;
extern crate log;
extern crate prost_types;

mod eigr;

pub mod actor;
pub mod handler;
pub mod serializer;
pub mod spawn;
pub mod value;

use std::collections::HashMap;

use actor::{ActorDefinition, ActorSettings};
use eigr::spawn::{
    actor_deactivation_strategy::Strategy, Action, Actor, ActorDeactivationStrategy, ActorId,
    ActorSnapshotStrategy, ActorState, ActorSystem, FixedTimerAction, Metadata,
    RegistrationRequest, Registry, ServiceInfo, TimeoutStrategy,
};
use log::debug;
use prost::DecodeError;
use prost_types::Any;

use reqwest::{Client, Response};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware, Error};
use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};

const SUPPORTED_LIBRARY_NAME: &str = env!("CARGO_PKG_NAME");
const SUPPORTED_LIBRARY_VERSION: &str = env!("CARGO_PKG_VERSION");

fn from_any<T>(message: &Any) -> Result<T, DecodeError>
where
    T: prost::Message + Default,
{
    T::decode(message.value.as_slice())
}

#[derive(Debug, Clone)]
pub struct Message {
    body: Any,
}

impl Default for Message {
    fn default() -> Message {
        Message {
            body: Any::default(),
        }
    }
}

impl Message {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn body<T>(&self) -> Result<T, DecodeError>
    where
        T: prost::Message + Default,
    {
        from_any(&self.body)
    }

    pub fn set_body(&mut self, message: Any) {
        self.body = message
    }
}

#[derive(Debug, Clone)]
pub struct Context {
    state: Option<Any>,
    metadata: HashMap<String, String>,
    tags: HashMap<String, String>,
    spawn: SpawnClient,
}

impl Default for Context {
    fn default() -> Context {
        Context {
            state: None,
            metadata: HashMap::new(),
            tags: HashMap::new(),
            spawn: SpawnClient::new(),
        }
    }
}

impl Context {
    pub fn new(spawn: SpawnClient) -> Self {
        let mut default: Context = Default::default();
        default.spawn = spawn;
        default
    }

    pub fn spawn(&self) -> SpawnClient {
        self.spawn.clone()
    }

    /// Returns a reference to the state of this [`Context`].
    pub fn state<T>(&self) -> Option<Result<T, DecodeError>>
    where
        T: prost::Message + Default,
    {
        let state = if let Some(s) = &self.state.as_ref() {
            Some(from_any(s))
        } else {
            None
        };

        state
    }

    /// Sets the state of this [`Context`].
    pub fn set_state(&mut self, state: Any) {
        self.state = Some(state);
    }

    /// Sets the metadata of this [`Context`].
    pub fn set_metadata(&mut self, metadata: HashMap<String, String>) {
        self.metadata = metadata;
    }

    /// Sets the tags of this [`Context`].
    pub fn set_tags(&mut self, tags: HashMap<String, String>) {
        self.tags = tags;
    }
}

#[derive(Debug, Clone)]
pub struct SpawnClient {
    client: ClientWithMiddleware,
    proxy_port: u16,
    proxy_host: String,
}

impl Default for SpawnClient {
    fn default() -> SpawnClient {
        SpawnClient {
            client: ClientBuilder::new(Client::new())
                .with(RetryTransientMiddleware::new_with_policy(
                    ExponentialBackoff::builder().build_with_max_retries(50),
                ))
                .build(),
            proxy_port: 9001,
            proxy_host: "127.0.0.1".to_string(),
        }
    }
}

impl SpawnClient {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn set_proxy_port(&mut self, port: u16) -> &mut SpawnClient {
        self.proxy_port = port;
        self
    }

    pub fn set_proxy_host(&mut self, host: String) -> &mut SpawnClient {
        self.proxy_host = host;
        self
    }

    pub async fn register(
        &mut self,
        system: String,
        service_name: String,
        service_version: String,
        definitions: Vec<ActorDefinition>,
    ) -> Result<Response, Error> {
        debug!("Make registration request to Spawn proxy");
        let actors: Vec<Actor> = self.build_actors(system.to_string(), definitions);
        let request: RegistrationRequest =
            self.build_registration_request(system, service_name, service_version, actors);

        let mut request_buffer: Vec<u8> = Vec::new();
        prost::Message::encode(&request, &mut request_buffer).unwrap();

        let res = self
            .client
            .post(format!(
                "http://{}:{}/api/v1/system",
                self.proxy_host, self.proxy_port,
            ))
            .header("Content-Type", "application/octet-stream")
            .body(request_buffer)
            .send()
            .await;

        debug!("Actors register response {:?}", res.as_ref().unwrap());

        res
    }

    fn build_actors(&mut self, system: String, definitions: Vec<ActorDefinition>) -> Vec<Actor> {
        let actors: Vec<Actor> = definitions
            .iter()
            .map(|actor_def| {
                let mut settings = actor_def.to_owned().get_settings().to_owned();

                let mut ac: Actor = Actor::default();
                ac.id = self.build_actor_id(system.to_string(), &mut settings);
                ac.state = self.build_initial_actor_state(&mut settings);
                ac.settings = self.build_actor_settings(&mut settings);
                ac.metadata = self.build_actor_metadata(&mut settings);

                let actions = actor_def
                    .clone()
                    .get_actions()
                    .iter()
                    .map(|action| {
                        let mut ac = Action::default();
                        ac.name = action.0.to_string();
                        ac
                    })
                    .collect::<Vec<Action>>();

                let timer_actions: Vec<FixedTimerAction> = actor_def
                    .clone()
                    .get_timer_actions()
                    .iter()
                    .map(|action| {
                        let mut tac = FixedTimerAction::default();
                        tac.seconds = action.1.clone().get_seconds();

                        let mut ac = Action::default();
                        ac.name = action.0.to_string();

                        tac.action = Some(ac);
                        tac
                    })
                    .collect();

                ac.actions = actions;
                ac.timer_actions = timer_actions;

                return ac;
            })
            .collect::<Vec<Actor>>();

        actors
    }

    fn build_registration_request(
        &mut self,
        system: String,
        service_name: String,
        service_version: String,
        actors: Vec<Actor>,
    ) -> RegistrationRequest {
        let mut request: RegistrationRequest = RegistrationRequest::default();
        request.service_info = self.build_service_info(service_name, service_version);
        request.actor_system = self.build_actor_system(system, actors);

        request
    }

    fn build_actor_system(&mut self, system: String, actors: Vec<Actor>) -> Option<ActorSystem> {
        let mut actor_system = ActorSystem::default();
        actor_system.name = system;
        actor_system.registry = self.build_registry(actors);

        Some(actor_system)
    }

    fn build_actor_settings(
        &mut self,
        settings: &mut ActorSettings,
    ) -> Option<eigr::spawn::ActorSettings> {
        let mut definition_settings = settings.clone();
        let mut actor_settings = eigr::spawn::ActorSettings::default();

        actor_settings.deactivation_strategy = self.build_deactivate_timeout(settings);

        actor_settings.snapshot_strategy = self.build_snapshot_timeout(settings);
        actor_settings.stateful = definition_settings.get_stateful();

        match definition_settings.get_kind() {
            actor::Kind::NAMED => actor_settings.kind = eigr::spawn::Kind::Named.into(),
            actor::Kind::UNNAMED => actor_settings.kind = eigr::spawn::Kind::Unamed.into(),
            actor::Kind::POOLED => actor_settings.kind = eigr::spawn::Kind::Pooled.into(),
            actor::Kind::PROXY => actor_settings.kind = eigr::spawn::Kind::Proxy.into(),
        };

        Some(actor_settings)
    }

    fn build_actor_id(&mut self, system: String, settings: &mut ActorSettings) -> Option<ActorId> {
        let mut id = ActorId::default();
        id.system = system.to_string();
        id.name = settings.get_name();

        Some(id)
    }

    fn build_initial_actor_state(&mut self, _settings: &mut ActorSettings) -> Option<ActorState> {
        Some(ActorState::default())
    }

    fn build_actor_metadata(&mut self, settings: &mut ActorSettings) -> Option<Metadata> {
        let mut metadata = Metadata::default();
        metadata.channel_group = settings.get_channel();
        metadata.tags = HashMap::new();

        Some(metadata)
    }

    fn build_deactivate_timeout(
        &mut self,
        settings: &mut ActorSettings,
    ) -> Option<ActorDeactivationStrategy> {
        let mut deactivate_timeout_strategy = TimeoutStrategy::default();
        deactivate_timeout_strategy.timeout = settings.get_deactivated_timeout();

        let mut deactivate = ActorDeactivationStrategy::default();
        deactivate.strategy = Some(Strategy::Timeout(deactivate_timeout_strategy));

        Some(deactivate)
    }

    fn build_snapshot_timeout(
        &mut self,
        settings: &mut ActorSettings,
    ) -> Option<ActorSnapshotStrategy> {
        let mut snapshot_timeout_strategy = TimeoutStrategy::default();
        snapshot_timeout_strategy.timeout = settings.get_snapshot_timeout();

        let mut snapshot = ActorSnapshotStrategy::default();
        snapshot.strategy = Some(eigr::spawn::actor_snapshot_strategy::Strategy::Timeout(
            snapshot_timeout_strategy,
        ));

        Some(snapshot)
    }

    fn build_registry(&mut self, actors: Vec<Actor>) -> Option<Registry> {
        let mut registry: Registry = Registry::default();
        for item in actors.iter() {
            let name = item.id.to_owned().unwrap().name;
            registry.actors.insert(name, item.clone());
        }

        Some(registry)
    }

    fn build_service_info(
        &mut self,
        service_name: String,
        service_version: String,
    ) -> Option<ServiceInfo> {
        let mut service_info: ServiceInfo = ServiceInfo::default();
        service_info.service_name = service_name.to_string();
        service_info.service_version = service_version.to_string();
        service_info.support_library_name = SUPPORTED_LIBRARY_NAME.to_owned();
        service_info.support_library_version = SUPPORTED_LIBRARY_VERSION.to_owned();
        service_info.protocol_major_version = 1;
        service_info.protocol_minor_version = 1;

        Some(service_info)
    }
}
