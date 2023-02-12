use prost_types::Any;

use crate::{actor::Actor, context::Context, value::Value};

#[derive(Debug, Clone)]
pub struct Message {
    action: String,
    body: Any,
}

impl Default for Message {
    fn default() -> Message {
        Message {
            action: String::from(""),
            body: Any::default(),
        }
    }
}

impl Message {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn action(&self) -> &str {
        &self.action
    }

    pub fn body(&self) -> &Any {
        &self.body
    }
}

#[allow(unused_variables)]
pub trait Action
where
    Self: Actor,
{
    /// This method is called for every message received by this actor.
    fn handle(&mut self, req: Message, ctx: &mut Context) -> Value;
}
