extern crate prost_types;
extern crate tokio;

mod joe;

use spawn_rs::spawn::Spawn;

#[tokio::main]
async fn main() {
    Spawn::new()
        .system("spawn-system".to_string())
        .port(8091)
        .add_actor(Box::new(joe::Joe {}))
        .start()
        .await;
}
