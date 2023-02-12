extern crate actix_web;
extern crate prost_types;

mod eigr {
    #[path = "eigr.functions.protocol.actors.rs"]
    pub mod actors {}

    #[path = "eigr.functions.protocol.rs"]
    pub mod protocol {}
}

pub mod action;
pub mod actor;
pub mod context;
pub mod handler;
pub mod spawn;
pub mod value;
