pub enum Kind {
    ABSTRACT,
    SINGLETON,
    POOLED,
    PROXY,
}

#[derive(Debug, Clone)]
pub struct ActorSettings {
    name: String,
    actions: Vec<String>,
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
            kind: Kind.SINGLETON,
            actions: Vec::new(),
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

    pub fn actions(&mut self, actions: Vec<String>) -> &mut ActorSettings {
        self.actions = actions;
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
}

/// Actor trait
#[allow(unused_variables)]
pub trait Actor {
    fn settings(&mut self) -> ActorSettings {}
}
