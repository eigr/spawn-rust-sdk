use spawn_rs::{
    action::{Action, Request},
    actor::{Actor, ActorSettings, Kind},
    context::Context,
    value::{self, Value},
};

struct Joe;

impl Actor for Joe {
    fn settings(&mut self) -> ActorSettings {
        ActorSettings::new()
            .name("joe".to_owned())
            .kind(Kind.SINGLETON)
            .stateful(true)
            .actions(["sum"])
            .deactivated_timeout(30000)
            .snapshot_timeout(10000)
    }
}

impl Action for Joe {
    fn handle(&mut self, req: Request, ctx: &mut Context) -> Value {
        match req.action() {
            "sum" => Value::new().state(ctx.state()).response(Any::default()),
            _ => Value::new().state(Any::default()).response(Any::default()),
        }
    }
}
