use spawn_examples::domain::domain::{Reply, Request, State};
use spawn_rs::{value::Value, Context, Message};

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
                .state::<State>(&ctx.state::<State>().unwrap(), "domain.State".to_string())
                .response(&reply, "domain.Reply".to_string())
                .to_owned()
        }
        Err(_e) => Value::new()
            .state::<State>(&ctx.state::<State>().unwrap(), "domain.State".to_string())
            .to_owned(),
    };

    return value;
}
