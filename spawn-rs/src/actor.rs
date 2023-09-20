use crate::Message;
use crate::{value::Value, Context};

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Kind {
    NAMED,
    UNNAMED,
    POOLED,
    PROXY,
}

#[derive(Debug, Clone)]
pub struct ActorSettings {
    name: String,
    kind: Kind,
    stateful: bool,
    deactivated_timeout: i64,
    snapshot_timeout: i64,
    channel: String,
    min_pool_size: i32,
    max_pool_size: i32,
}

impl Default for ActorSettings {
    fn default() -> ActorSettings {
        ActorSettings {
            name: String::from(""),
            kind: Kind::NAMED,
            stateful: true,
            deactivated_timeout: 60000,
            snapshot_timeout: 50000,
            channel: String::new(),
            min_pool_size: 1,
            max_pool_size: 0,
        }
    }
}

impl ActorSettings {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn name(&mut self, name: String) -> &mut ActorSettings {
        self.name = name;
        self
    }

    pub fn kind(&mut self, kind: Kind) -> &mut ActorSettings {
        self.kind = kind;
        self
    }

    pub fn stateful(&mut self, stateful: bool) -> &mut ActorSettings {
        self.stateful = stateful;
        self
    }

    pub fn deactivated_timeout(&mut self, timeout: i64) -> &mut ActorSettings {
        self.deactivated_timeout = timeout;
        self
    }

    pub fn snapshot_timeout(&mut self, timeout: i64) -> &mut ActorSettings {
        self.snapshot_timeout = timeout;
        self
    }

    pub fn channel(&mut self, channel: String) -> &mut ActorSettings {
        self.channel = channel;
        self
    }

    pub fn min_pool_size(&mut self, size: i32) -> &mut ActorSettings {
        self.min_pool_size = size;
        self
    }

    pub fn max_pool_size(&mut self, size: i32) -> &mut ActorSettings {
        self.max_pool_size = size;
        self
    }

    pub fn get_name(&mut self) -> String {
        self.name.to_string()
    }

    pub fn get_kind(&mut self) -> &mut Kind {
        &mut self.kind
    }

    pub fn get_stateful(&mut self) -> bool {
        self.stateful
    }

    pub fn get_deactivated_timeout(&mut self) -> i64 {
        self.deactivated_timeout
    }

    pub fn get_snapshot_timeout(&mut self) -> i64 {
        self.snapshot_timeout
    }

    pub fn get_channel(&mut self) -> String {
        self.channel.to_string()
    }

    pub fn get_min_pool_size(&mut self) -> i32 {
        self.min_pool_size
    }

    pub fn get_max_pool_size(&mut self) -> i32 {
        self.max_pool_size
    }
}

#[derive(Clone)]
pub struct ActorDefinition {
    settings: ActorSettings,
    actions: HashMap<String, fn(Message, Context) -> Value>,
}

impl Default for ActorDefinition {
    fn default() -> ActorDefinition {
        ActorDefinition {
            actions: HashMap::new(),
            settings: ActorSettings::new(),
        }
    }
}

impl ActorDefinition {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_settings(&mut self, settings: ActorSettings) -> &mut ActorDefinition {
        self.settings = settings;
        self
    }

    pub fn with_action(
        &mut self,
        name: String,
        action: fn(Message, Context) -> Value,
    ) -> &mut ActorDefinition {
        self.actions.insert(name, action);
        self
    }

    pub fn get_settings(&mut self) -> &mut ActorSettings {
        &mut self.settings
    }

    pub fn get_actions(&mut self) -> &mut HashMap<String, fn(Message, Context) -> Value> {
        &mut self.actions
    }
}
