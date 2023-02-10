use prost_types::Any;

use crate::{actor::Actor, context::Context, value::Value};

#[derive(Debug, Clone)]
pub struct Request {
    action: String,
    body: Any,
}

impl Default for Request {
    fn default() -> Request {
        Request {
            action: String::from(""),
            body: Any::default(),
        }
    }
}

impl Request {
    pub fn new() -> Self {
        Default::default()
    }
}

#[allow(unused_variables)]
pub trait Action<R>
where
    Self: Actor,
    R: Request,
{
    /// This method is called for every message received by this actor.
    fn handle(&mut self, req: R, ctx: &mut Context) -> Value;
}
