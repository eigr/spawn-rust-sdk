# Spawn Rust SDK

```protobuf
syntax = "proto3";

package domain;

message State {
  repeated string languages = 1;
}

message Request {
  string language = 1;
}

message Reply {
  string response = 1;
}
```

```rust
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
```

```rust
extern crate env_logger;
extern crate prost_types;
extern crate rocket;

mod joe;

use actors::joe::{set_language, set_language_with_timer};
use spawn_rs::actor::{ActorDefinition, ActorSettings, Kind};
use spawn_rs::spawn::Spawn;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let mut spawn: Spawn = Spawn::new()
        .create("spawn-system".to_string())
        .with_service_name(env!("CARGO_PKG_NAME").to_string()) // optional
        .with_service_version(env!("CARGO_PKG_VERSION").to_string()) // optional
        .with_proxy_port(9003)
        .with_actor(
            ActorDefinition::new()
                .with_settings(
                    ActorSettings::new()
                        .name("joe".to_owned())
                        .kind(Kind::NAMED)
                        .stateful(true)
                        .deactivated_timeout(30000) // optional
                        .snapshot_timeout(10000) // optional
                        .to_owned(),
                )
                .with_action("setLanguage".to_owned(), set_language)
                .with_timer_action(
                    "set_language_with_timer".to_owned(),
                    set_language_with_timer,
                    1000,
                ),
        )
        .clone();

    spawn.start().await?;

    Ok(())
}
```