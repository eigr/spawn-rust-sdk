use spawn_rs::{
    action::Action,
    actor::{Actor, ActorSettings},
    context::Context,
    value,
};

struct Joe;

impl Actor for Joe {
    fn get_settings(&mut self) -> ActorSettings {}
}

impl Action for Joe {
    fn handle(&mut self, req: R, ctx: &mut Context) -> Value {
        todo!()
    }
}
