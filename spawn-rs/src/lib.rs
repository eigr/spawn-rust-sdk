#[macro_use]
extern crate rocket;
extern crate log;
extern crate prost_types;

mod eigr;

pub mod actor;
pub mod handler;
pub mod serializer;
pub mod spawn;
pub mod value;

use std::collections::HashMap;

use prost::DecodeError;
use prost_types::Any;

use reqwest::Client;

fn from_any<T>(message: &Any) -> Result<T, DecodeError>
where
    T: prost::Message + Default,
{
    T::decode(message.value.as_slice())
}

#[derive(Debug, Clone)]
pub struct Message {
    body: Any,
}

impl Default for Message {
    fn default() -> Message {
        Message {
            body: Any::default(),
        }
    }
}

impl Message {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn body<T>(&self) -> Result<T, DecodeError>
    where
        T: prost::Message + Default,
    {
        from_any(&self.body)
    }

    pub fn set_body(&mut self, message: Any) {
        self.body = message
    }
}

#[derive(Debug, Clone)]
pub struct Context {
    state: Option<Any>,
    metadata: HashMap<String, String>,
    tags: HashMap<String, String>,
    spawn: SpawnClient,
}

impl Default for Context {
    fn default() -> Context {
        Context {
            state: None,
            metadata: HashMap::new(),
            tags: HashMap::new(),
            spawn: SpawnClient::new(),
        }
    }
}

impl Context {
    pub fn new(spawn: SpawnClient) -> Self {
        let mut default: Context = Default::default();
        default.spawn = spawn;
        default
    }

    pub fn spawn(&self) -> SpawnClient {
        self.spawn.clone()
    }

    /// Returns a reference to the state of this [`Context`].
    pub fn state<T>(&self) -> Option<Result<T, DecodeError>>
    where
        T: prost::Message + Default,
    {
        let state = if let Some(s) = &self.state.as_ref() {
            Some(from_any(s))
        } else {
            None
        };

        state
    }

    /// Sets the state of this [`Context`].
    pub fn set_state(&mut self, state: Any) {
        self.state = Some(state);
    }

    /// Sets the metadata of this [`Context`].
    pub fn set_metadata(&mut self, metadata: HashMap<String, String>) {
        self.metadata = metadata;
    }

    /// Sets the tags of this [`Context`].
    pub fn set_tags(&mut self, tags: HashMap<String, String>) {
        self.tags = tags;
    }
}

#[derive(Debug, Clone)]
pub struct SpawnClient {
    client: Client,
    proxyPort: u16,
    proxyHost: String,
}

impl Default for SpawnClient {
    fn default() -> SpawnClient {
        SpawnClient {
            client: Client::new(),
            proxyPort: 9001,
            proxyHost: "127.0.0.1".to_string(),
        }
    }
}

impl SpawnClient {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn set_proxy_port(&mut self, port: u16) -> &mut SpawnClient {
        self.proxyPort = port;
        self
    }

    pub fn set_proxy_host(&mut self, host: String) -> &mut SpawnClient {
        self.proxyHost = host;
        self
    }
}
