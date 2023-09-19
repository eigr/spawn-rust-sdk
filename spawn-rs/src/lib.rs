#[macro_use]
extern crate rocket;
extern crate log;
extern crate prost_types;

mod eigr;

pub mod actor;
pub mod handler;
pub mod serializer;
pub mod spawn;
pub mod value;

use prost::DecodeError;
use prost_types::Any;

fn from_any<T>(message: &Any) -> Result<T, DecodeError>
where
    T: prost::Message + Default,
{
    T::decode(message.value.as_slice())
}

#[derive(Debug, Clone)]
pub struct Message {
    body: Any,
}

impl Default for Message {
    fn default() -> Message {
        Message {
            body: Any::default(),
        }
    }
}

impl Message {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn body<T>(&self) -> Result<T, DecodeError>
    where
        T: prost::Message + Default,
    {
        from_any(&self.body)
    }

    pub fn set_body(&mut self, message: Any) {
        self.body = message
    }
}

#[derive(Debug, Clone)]
pub struct Context {
    state: Any,
}

impl Default for Context {
    fn default() -> Context {
        Context {
            state: Any::default(),
        }
    }
}

impl Context {
    pub fn new() -> Self {
        Default::default()
    }

    /// Returns a reference to the state of this [`Context`].
    pub fn state<T>(&self) -> Result<T, DecodeError>
    where
        T: prost::Message + Default,
    {
        from_any(&self.state)
    }

    /// Sets the state of this [`Context`].
    pub fn set_state(&mut self, state: Any) {
        self.state = state;
    }
}
