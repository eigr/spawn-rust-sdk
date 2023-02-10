use actor::Actor;

#[derive(Debug)]
pub struct Spawn {
    system: String,
    actor: Vec<dyn Actor>,
    server_port: u16,
}

impl Default for Spawn {
    fn default() -> Spawn {
        Spawn {
            system: String::from(""),
            actor: Actor::default(),
            server_port: 8091,
        }
    }
}

impl Spawn {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn system(&mut self, system_name: String) -> &mut Spawn {
        self.system = system_name;
        self
    }

    pub fn port(&mut self, server_port: u16) -> &mut Spawn {
        self.server_port = server_port;
        self
    }

    pub fn add_actor(&mut self, actor: dyn Actor) -> &mut Spawn {
        self.actor = actor;
        self
    }

    pub fn start(&mut self) {}
}
