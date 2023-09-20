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
```

```rust
extern crate env_logger;
extern crate prost_types;
extern crate rocket;

mod joe;

use joe::set_language;
use spawn_rs::actor::{ActorDefinition, ActorSettings, Kind};
use spawn_rs::spawn::Spawn;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    Spawn::new()
        .create("spawn-system".to_string())
        .with_actor(
            ActorDefinition::new()
                .with_settings(
                    ActorSettings::new()
                        .name("joe".to_owned())
                        .kind(Kind::NAMED)
                        .stateful(true)
                        .deactivated_timeout(30000)
                        .snapshot_timeout(10000)
                        .to_owned(),
                )
                .with_action("setLanguage".to_owned(), set_language),
        )
        .start()
        .await?;

    Ok(())
}
```