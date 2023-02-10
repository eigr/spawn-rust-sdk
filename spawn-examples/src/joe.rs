use prost_types::Any;

use spawn_rs::{
    action::{Action, Request},
    actor::{Actor, ActorSettings, Kind},
    context::Context,
    value::Value,
};

struct Joe;

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
    fn handle(&mut self, req: Request, ctx: &mut Context) -> Value {
        match req.action() {
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
