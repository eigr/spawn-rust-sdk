extern crate env_logger;
extern crate prost_types;
extern crate tokio;

mod joe;

use spawn_rs::spawn::Spawn;

#[tokio::main]
async fn main() {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    Spawn::new()
        .system("spawn-system".to_string())
        .port(8091)
        .add_actor(Box::new(joe::Joe {}))
        .start()
        .await;
}
