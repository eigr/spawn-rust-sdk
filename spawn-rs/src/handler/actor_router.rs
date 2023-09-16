use crate::actor::ActorDefinition;

#[derive()]
pub struct Handler {
    actors: Vec<ActorDefinition>,
}

impl Default for Handler {
    fn default() -> Handler {
        Handler { actors: Vec::new() }
    }
}

impl Handler {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_actors(&mut self, actors: &mut Vec<ActorDefinition>) -> &mut Handler {
        for def in actors.iter() {
            self.actors.push(def.to_owned());
        }

        self
    }
}
