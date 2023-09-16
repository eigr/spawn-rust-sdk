use crate::actor::ActorDefinition;
use crate::handler::actor_router::Handler;

use rocket::post;

use rocket::data::{Data, ToByteUnit};
use rocket::State;

#[post(
    "/api/v1/actors/actions",
    format = "application/octet-stream",
    data = "<data>"
)]
fn handle(data: Data<'_>, _handler: &State<Handler>) -> &'static str {
    "Hello, world!"
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
