use spawn_examples::domain::domain::{Reply, Request};
use spawn_rs::{context::Context, value::Value, Message};

use log::info;

pub fn set_language(msg: Message, ctx: Context) -> Value {
    info!("Actor msg: {:?}", msg);
    let value: Value = match msg.body::<Request>() {
        Ok(request) => {
            let lang = request.language;
            info!("Setlanguage To: {:?}", lang);
            let reply = Reply::default();

            Value::new()
                .state(ctx.state().clone())
                .response(&Reply::default())
                .to_owned()
        }
        Err(e) => Value::new()
            .state(ctx.state().clone())
            //.response(Any::default())
            .to_owned(),
    };

    return value;
}
