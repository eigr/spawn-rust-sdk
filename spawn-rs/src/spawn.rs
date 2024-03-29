use crate::actor::ActorDefinition;
use crate::eigr::spawn::{ActorInvocation, ActorInvocationResponse};
use crate::handler::actor_router::Handler;
use crate::SpawnClient;

use prost::Message;
use rocket::post;

use rocket::data::{Data, ToByteUnit};
use rocket::State;

use std::io::Cursor;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{io, thread};

#[post(
    "/api/v1/actors/actions",
    format = "application/octet-stream",
    data = "<data>"
)]
async fn handle(data: Data<'_>, handler: &State<Arc<Mutex<Handler>>>) -> io::Result<Vec<u8>> {
    let bytes = data.open(2048.megabytes()).into_bytes().await?;

    let request_handler = Arc::clone(&handler);

    let buffer = bytes.into_inner();
    let request: ActorInvocation = ActorInvocation::decode(&mut Cursor::new(buffer)).unwrap();

    let mut buf: Vec<u8> = Vec::new();
    let response: ActorInvocationResponse = request_handler.lock().unwrap().handle(request);
    response.encode(&mut buf).unwrap();
    return Ok(buf);
}

#[derive(Clone)]
pub struct Spawn {
    system: String,
    service_name: String,
    service_version: String,
    actors: Vec<ActorDefinition>,
    port: u16,
    host: String,
    client: SpawnClient,
}

impl Default for Spawn {
    fn default() -> Spawn {
        Spawn {
            system: String::from(""),
            service_name: String::from("spawn-rust-sdk"),
            service_version: String::from(""),
            actors: Vec::new(),
            port: 8093,
            host: "0.0.0.0".to_string(),
            client: SpawnClient::new(),
        }
    }
}

impl Spawn {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn create(&mut self, system_name: String) -> &mut Spawn {
        self.system = system_name;
        self
    }

    pub fn with_service_name(&mut self, service_name: String) -> &mut Spawn {
        self.service_name = service_name;
        self
    }

    pub fn with_service_version(&mut self, service_version: String) -> &mut Spawn {
        self.service_version = service_version;
        self
    }

    pub fn with_port(&mut self, port: u16) -> &mut Spawn {
        self.port = port;
        self
    }

    pub fn with_host(&mut self, host: String) -> &mut Spawn {
        self.host = host;
        self
    }

    pub fn with_proxy_port(&mut self, port: u16) -> &mut Spawn {
        self.client.set_proxy_port(port);
        self
    }

    pub fn with_proxy_host(&mut self, host: String) -> &mut Spawn {
        self.client.set_proxy_host(host);
        self
    }

    pub fn with_actor(&mut self, actor: &mut ActorDefinition) -> &mut Spawn {
        self.actors.push(actor.to_owned());
        self
    }

    pub async fn start(&mut self) -> Result<(), rocket::Error> {
        env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

        let _res = self
            .client
            .register(
                self.system.to_string(),
                self.service_name.to_string(),
                self.service_version.to_string(),
                self.actors.clone(),
            )
            .await;

        let figment = rocket::Config::figment()
            .merge(("port", self.port))
            .merge(("address", self.host.to_owned()));

        let mut handler: Handler = Handler::new(self.client.clone());
        handler.add_actors(self.actors.as_mut());

        let state = Arc::new(Mutex::new(handler));

        rocket::custom(figment)
            .mount("/", routes![handle])
            .manage(state)
            .ignite()
            .await?
            .launch()
            .await?;

        Ok(())
    }

    pub async fn start_with(
        &mut self,
        delay: u64,
        function: fn() -> (),
    ) -> Result<(), rocket::Error> {
        let function_thread = thread::spawn(move || {
            thread::sleep(Duration::from_millis(delay));
            (function)();
        });

        let result: Result<(), rocket::Error> = self.start().await;

        function_thread.join().unwrap();

        result
    }
}
