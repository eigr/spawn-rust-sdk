extern crate env_logger;
extern crate prost_types;
extern crate rocket;

mod actors;

use actors::joe::{set_language, set_language_with_timer};
use spawn_rs::actor::{ActorDefinition, ActorSettings, Kind};
use spawn_rs::spawn::Spawn;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let mut spawn: Spawn = Spawn::new()
        .create("spawn-system".to_string())
        .with_service_name(env!("CARGO_PKG_NAME").to_string())
        .with_service_version(env!("CARGO_PKG_VERSION").to_string())
        .with_proxy_port(9003)
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
                .with_action("setLanguage".to_owned(), set_language)
                .with_timer_action(
                    "set_language_with_timer".to_owned(),
                    set_language_with_timer,
                    1000,
                ),
        )
        .clone();

    spawn.start().await?;

    Ok(())
}
