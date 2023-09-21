use spawn_examples::domain::domain::{Reply, Request, State};
use spawn_rs::{value::Value, Context, Message};

use log::info;

pub fn set_language(msg: Message, ctx: Context) -> Value {
    info!("Actor msg: {:?}", msg);
    return match msg.body::<Request>() {
        Ok(request) => {
            let lang = request.language;
            info!("Setlanguage To: {:?}", lang);
            let mut reply = Reply::default();
            reply.response = lang;

            match &ctx.state::<State>() {
                Some(state) => Value::new()
                    .state::<State>(&state.as_ref().unwrap(), "domain.State".to_string())
                    .response(&reply, "domain.Reply".to_string())
                    .to_owned(),
                _ => Value::new()
                    .state::<State>(&State::default(), "domain.State".to_string())
                    .response(&reply, "domain.Reply".to_string())
                    .to_owned(),
            }
        }
        Err(_e) => Value::new()
            .state::<State>(&State::default(), "domain.State".to_string())
            .to_owned(),
    };
}

pub fn set_language_with_timer(msg: Message, ctx: Context) -> Value {
    info!("Actor msg: {:?}", msg);
    return match msg.body::<Request>() {
        Ok(request) => {
            let lang = request.language;
            info!("Setlanguage To: {:?}", lang);
            let mut reply = Reply::default();
            reply.response = lang;

            match &ctx.state::<State>() {
                Some(state) => Value::new()
                    .state::<State>(&state.as_ref().unwrap(), "domain.State".to_string())
                    .response(&reply, "domain.Reply".to_string())
                    .to_owned(),
                _ => Value::new()
                    .state::<State>(&State::default(), "domain.State".to_string())
                    .response(&reply, "domain.Reply".to_string())
                    .to_owned(),
            }
        }
        Err(_e) => Value::new()
            .state::<State>(&State::default(), "domain.State".to_string())
            .to_owned(),
    };
}
