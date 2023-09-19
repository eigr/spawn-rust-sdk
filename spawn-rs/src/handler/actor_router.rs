use std::collections::HashMap;

use crate::actor::ActorDefinition;

use crate::eigr::spawn::actor_invocation::Payload;
use crate::eigr::spawn::actor_invocation_response::Payload as ResponsePayload;
use crate::eigr::spawn::{
    ActorId, ActorInvocation, ActorInvocationResponse, Context, Noop, Workflow,
};
use crate::value::Value;
use crate::Message as ActorMessage;
use crate::{Context as ActorContext, SpawnClient};

use log::{debug, info};
use prost_types::Any;

#[derive()]
pub struct Handler {
    spawn: SpawnClient,
    actors: HashMap<String, ActorDefinition>,
}

impl Default for Handler {
    fn default() -> Handler {
        Handler {
            spawn: SpawnClient::new(),
            actors: HashMap::new(),
        }
    }
}

impl Handler {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_actors(&mut self, actors: &mut Vec<ActorDefinition>) -> &mut Handler {
        for def in actors.iter() {
            let mut actor = def.to_owned();
            let settings = actor.get_settings();
            let name = settings.get_name();

            self.actors.insert(name.to_owned(), actor);
        }

        self
    }

    pub fn get_actors(&mut self) -> &mut HashMap<String, ActorDefinition> {
        &mut self.actors
    }

    pub fn handle(&mut self, request: ActorInvocation) -> ActorInvocationResponse {
        info!("Received ActorInvocation request.");
        debug!(
            "Handle ActorInvocation with incoming request: {:?}",
            request
        );

        let actor_id: ActorId = request.actor.unwrap();
        let action: String = request.action_name;

        let mut response = ActorInvocationResponse::default();

        if self.actors.contains_key(actor_id.name.as_str()) {
            debug!(
                "Forward ActorInvocation to Actor: {:?}",
                actor_id.name.as_str()
            );
            // handle response
            let mut actor_def = self.actors.get(actor_id.name.as_str()).unwrap().clone();

            if actor_def.get_actions().contains_key(action.as_str()) {
                let function: &fn(ActorMessage, ActorContext) -> Value =
                    actor_def.get_actions().get(action.as_str()).unwrap();

                let payload = match request.payload {
                    Some(Payload::Value(value)) => value,
                    Some(Payload::Noop(_)) => Any::default(),
                    None => Any::default(),
                };

                let ctx: ActorContext = if let Some(current_context) = request.current_context {
                    let mut ctx: ActorContext = ActorContext::new(self.spawn.clone());
                    ctx.set_metadata(current_context.metadata);
                    ctx.set_tags(current_context.tags);

                    if let Some(current_state) = current_context.state {
                        ctx.set_state(current_state)
                    }

                    ctx.clone()
                } else {
                    ActorContext::new(self.spawn.clone())
                };

                let mut msg: ActorMessage = ActorMessage::new();
                msg.set_body(payload);

                let result: Value = (function)(msg, ctx);
                response.actor_name = actor_id.name;
                response.actor_system = actor_id.system;
                response.payload = Some(ResponsePayload::Value(result.get_response().to_owned()));
                response.updated_context = Some(Context::default());
                response.workflow = Some(Workflow::default());
                response.checkpoint = false;

                // TODO: build correct response
                return response;
            }
        }

        return response;
    }
}
