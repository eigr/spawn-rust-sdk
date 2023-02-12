use crate::actor::Actor;
use crate::handler::callback;

use std::io::Result;

use actix_web::{
    middleware,
    web::{self},
    App, HttpServer,
};

pub struct Spawn {
    system: String,
    actors: Vec<Box<dyn Actor>>,
    server_port: u16,
}

impl Default for Spawn {
    fn default() -> Spawn {
        Spawn {
            actors: Vec::new(),
            server_port: 8091,
            system: String::from(""),
        }
    }
}

impl Spawn {
    pub fn add_actor(&mut self, actor: Box<dyn Actor>) -> &mut Spawn {
        self.actors.push(actor);
        self
    }

    pub fn get_actors(&mut self) -> &mut Vec<Box<dyn Actor>> {
        &mut self.actors
    }

    pub fn get_port(&mut self) -> u16 {
        self.server_port
    }

    pub fn get_system(&mut self) -> &mut String {
        &mut self.system
    }

    pub fn new() -> Self {
        Default::default()
    }

    pub fn port(&mut self, server_port: u16) -> &mut Spawn {
        self.server_port = server_port;
        self
    }

    pub fn system(&mut self, system_name: String) -> &mut Spawn {
        self.system = system_name;
        self
    }

    pub async fn start(&mut self) -> Result<()> {
        let server = HttpServer::new(move || {
            App::new()
                //.app_data(self.get_actors())
                .wrap(middleware::Logger::default())
                .configure(Self::config)
        })
        .bind(("127.0.0.1", self.get_port()))?
        .run();

        //future::join(server, another_func()).await;
        server.await
    }

    fn config(conf: &mut web::ServiceConfig) {
        let scope = web::scope("/api/v1/actors").service(callback::v1::handle);

        conf.service(scope);
    }
}
