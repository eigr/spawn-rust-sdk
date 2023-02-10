use spawn_rs::Spawn;

mod joe;

fn main() {
    Spawn::new().system("spawn-system")
}
