use crate::actor::Actor;
use crate::handler::actor_router;

use rocket::post;

use rocket::data::{Data, ToByteUnit};

#[post(
    "/api/v1/actors/actions",
    format = "application/octet-stream",
    data = "<data>"
)]
fn handle(data: Data<'_>) -> &'static str {
    "Hello, world!"
}

pub struct Spawn {
    system: String,
}

impl Default for Spawn {
    fn default() -> Spawn {
        Spawn {
            system: String::from(""),
        }
    }
}

impl Spawn {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn system(&mut self, system_name: String) -> &mut Spawn {
        self.system = system_name;
        self
    }

    pub async fn start(&mut self) -> Result<(), rocket::Error> {
        let figment = rocket::Config::figment().merge(("port", 8093));

        rocket::custom(figment)
            .mount("/", routes![handle])
            .ignite()
            .await?
            .launch()
            .await?;

        Ok(())
    }
}
