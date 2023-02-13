use crate::{actor::Actor, context::Context, value::Value, Message};

#[allow(unused_variables)]
pub trait Action
where
    Self: Actor,
{
    /// This method is called for every message received by this actor.
    fn handle(&mut self, msg: Message, ctx: &mut Context) -> Value;
}
