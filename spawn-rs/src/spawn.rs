use crate::actor::ActorDefinition;
use crate::context::Context as ActorContext;
use crate::eigr::spawn::{ActorId, ActorInvocation, ActorInvocationResponse, Context};
use crate::handler::actor_router::Handler;
use crate::value::Value;
use crate::Message as ActorMessage;

use prost::Message;
use rocket::post;

use rocket::data::{Data, ToByteUnit};
use rocket::State;

use std::io;
use std::io::Cursor;
use std::sync::{Arc, Mutex};

#[post(
    "/api/v1/actors/actions",
    format = "application/octet-stream",
    data = "<data>"
)]
async fn handle(data: Data<'_>, handler: &State<Arc<Mutex<Handler>>>) -> io::Result<Vec<u8>> {
    let bytes = data.open(2048.megabytes()).into_bytes().await?;

    let request_handler = Arc::clone(&handler);
    let actors = request_handler.lock().unwrap().get_actors().clone();

    let buffer = bytes.into_inner();
    let request: ActorInvocation = ActorInvocation::decode(&mut Cursor::new(buffer)).unwrap();
    let actor_id: ActorId = request.actor.unwrap();
    let action: String = request.action_name;
    let context: Context = request.current_context.unwrap();

    let mut buf: Vec<u8> = Vec::new();
    let response = ActorInvocationResponse::default();

    if actors.contains_key(actor_id.name.as_str()) {
        // handle response
        let mut actor_def = actors.get(actor_id.name.as_str()).unwrap().clone();

        if actor_def.get_actions().contains_key(action.as_str()) {
            let function: &fn(ActorMessage, ActorContext) -> Value =
                actor_def.get_actions().get(action.as_str()).unwrap();

            let msg: ActorMessage = ActorMessage::new();
            let ctx: ActorContext = ActorContext::new();

            let result: Value = (function)(msg, ctx);
        }

        response.encode(&mut buf).unwrap();
        return Ok(buf);
    }

    response.encode(&mut buf).unwrap();
    return Ok(buf);
}

#[derive()]
pub struct Spawn {
    system: String,
    actors: Vec<ActorDefinition>,
}

impl Default for Spawn {
    fn default() -> Spawn {
        Spawn {
            system: String::from(""),
            actors: Vec::new(),
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

    pub fn with_actor(&mut self, actor: &mut ActorDefinition) -> &mut Spawn {
        self.actors.push(actor.to_owned());
        self
    }

    pub async fn start(&mut self) -> Result<(), rocket::Error> {
        let figment = rocket::Config::figment().merge(("port", 8093));
        let mut handler: Handler = Handler::new();
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
}
