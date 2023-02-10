pub struct ActorSettings {}

/// Actor trait
#[allow(unused_variables)]
pub trait Actor {
    fn get_settings(&mut self) -> ActorSettings {}
}
