extern crate env_logger;
extern crate prost_types;
extern crate rocket;

mod joe;

use joe::set_language;
use spawn_rs::actor::{ActorDefinition, ActorSettings, Kind};
use spawn_rs::spawn::Spawn;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    Spawn::new()
        .create("spawn-system".to_string())
        .with_actor(
            ActorDefinition::new()
                .with_settings(
                    ActorSettings::new()
                        .name("joe".to_owned())
                        .kind(Kind::NAMED)
                        .stateful(true)
                        .deactivated_timeout(30000)
                        .snapshot_timeout(10000)
                        .to_owned(),
                )
                .with_action("setLanguage".to_owned(), set_language),
        )
        .start()
        .await?;

    Ok(())
}
