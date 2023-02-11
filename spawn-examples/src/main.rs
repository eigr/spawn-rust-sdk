extern crate prost_types;

mod joe;

use spawn_rs::spawn::Spawn;

fn main() {
    Spawn::new()
        .system("spawn-system".to_string())
        .port(8091)
        .add_actor(Box::new(joe::Joe {}))
        .start();
}
