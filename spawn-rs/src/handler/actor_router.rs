use std::collections::HashMap;

use crate::actor::ActorDefinition;

#[derive()]
pub struct Handler {
    actors: HashMap<String, ActorDefinition>,
}

impl Default for Handler {
    fn default() -> Handler {
        Handler {
            actors: HashMap::new(),
        }
    }
}

impl Handler {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_actors(&mut self, actors: &mut Vec<ActorDefinition>) -> &mut Handler {
        for def in actors.iter() {
            let mut actor = def.to_owned();
            let settings = actor.get_settings();
            let name = settings.get_name();

            self.actors.insert(name.to_owned(), actor);
        }

        self
    }

    pub fn get_actors(&mut self) -> &mut HashMap<String, ActorDefinition> {
        &mut self.actors
    }
}
