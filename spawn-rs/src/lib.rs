#[macro_use]
extern crate rocket;
extern crate prost_types;

mod eigr;

pub mod action;
pub mod actor;
pub mod context;
pub mod handler;
pub mod serializer;
pub mod spawn;
pub mod value;

use prost_types::Any;

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
