use prost_types::Any;

use spawn_rs::{context::Context, value::Value, Message};

pub fn sum(_msg: Message, ctx: Context) -> Value {
    return Value::new()
        .state(ctx.state().clone())
        .response(Any::default())
        .to_owned();
}
