#[macro_use]
extern crate rocket;
extern crate log;
extern crate prost_types;

mod eigr;

pub mod actor;
pub mod context;
pub mod handler;
pub mod serializer;
pub mod spawn;
pub mod value;

use prost::DecodeError;
use prost_types::Any;

// fn to_any<T>(message: &T) -> Any
// where
//     T: prost::Message,
// {
//     Any {
//         type_url: T::type_url().to_string(),
//         value: message.encode_to_vec(),
//     }
// }

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
