use crate::actor::Actor;

#[allow(unused_variables)]
pub trait Serializer
where
    Self: Actor,
{
    /// This method is called for every message received by this actor.
    fn decode(&mut self, msg: prost_types::Any) -> Box<dyn std::any::Any>;

    fn encode(&mut self, msg: Box<dyn std::any::Any>) -> prost_types::Any;
}
