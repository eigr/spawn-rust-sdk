use spawn_examples::domain::domain::{Reply, Request};
use spawn_rs::{context::Context, value::Value, Message};

use log::info;

pub fn set_language(msg: Message, ctx: Context) -> Value {
    info!("Actor msg: {:?}", msg);
    let value: Value = match msg.body::<Request>() {
        Ok(request) => {
            let lang = request.language;
            info!("Setlanguage To: {:?}", lang);
            let mut reply = Reply::default();
            reply.response = lang;

            Value::new()
                .state(ctx.state().clone())
                .response(&reply, "domain.Reply".to_string())
                .to_owned()
        }
        Err(_e) => Value::new().state(ctx.state().clone()).to_owned(),
    };

    return value;
}
