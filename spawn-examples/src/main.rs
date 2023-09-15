extern crate env_logger;
extern crate prost_types;
extern crate rocket;
extern crate tokio;

mod joe;

use spawn_rs::spawn::Spawn;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    Spawn::new()
        .system("spawn-system".to_string())
        .start()
        .await?;

    Ok(())
}
