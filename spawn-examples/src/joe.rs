use prost_types::Any;

use spawn_rs::{
    action::Action,
    actor::{Actor, ActorSettings, Kind},
    context::Context,
    serializer::Serializer,
    value::Value,
    Message,
};

pub struct Joe;

impl Serializer for Joe {
    fn decode(&mut self, _msg: prost_types::Any) -> Box<dyn std::any::Any> {
        todo!()
    }

    fn encode(&mut self, _msg: Box<dyn std::any::Any>) -> prost_types::Any {
        todo!()
    }
}

impl Actor for Joe {
    fn settings(&mut self) -> ActorSettings {
        ActorSettings::new()
            .name("joe".to_owned())
            .kind(Kind::SINGLETON)
            .stateful(true)
            .actions(vec!["sum".to_string()])
            .deactivated_timeout(30000)
            .snapshot_timeout(10000)
            .to_owned()
    }
}

impl Action for Joe {
    fn handle(&mut self, msg: Message, ctx: &mut Context) -> Value {
        match msg.action() {
            "sum" => Value::new()
                .state(ctx.state().clone())
                .response(Any::default())
                .to_owned(),
            _ => Value::new()
                .state(Any::default())
                .response(Any::default())
                .to_owned(),
        }
    }
}
