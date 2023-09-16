use crate::actor::ActorDefinition;
use crate::eigr::spawn::{ActorId, ActorInvocation, ActorInvocationResponse, Context};
use crate::handler::actor_router::Handler;

use prost::Message;
use rocket::post;

use rocket::data::{Data, ToByteUnit};
use rocket::State;

use std::io;
use std::io::Cursor;

#[post(
    "/api/v1/actors/actions",
    format = "application/octet-stream",
    data = "<data>"
)]
async fn handle(data: Data<'_>, _handler: &State<Handler>) -> io::Result<Vec<u8>> {
    let bytes = data.open(2048.megabytes()).into_bytes().await?;

    let buffer = bytes.into_inner();
    let request: ActorInvocation = ActorInvocation::decode(&mut Cursor::new(buffer)).unwrap();
    let actor: ActorId = request.actor.unwrap();
    let action: String = request.action_name;
    let ctx: Context = request.current_context.unwrap();

    let mut buf: Vec<u8> = Vec::new();
    let default_response = ActorInvocationResponse::default();
    default_response.encode(&mut buf).unwrap();
    Ok(buf)
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

        rocket::custom(figment)
            .mount("/", routes![handle])
            .manage(handler)
            .ignite()
            .await?
            .launch()
            .await?;

        Ok(())
    }
}
