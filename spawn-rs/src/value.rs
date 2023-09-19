use prost::Message;
use prost_types::Any;

#[derive(Debug, Clone)]
pub struct Value {
    state: Any,
    response: Any,
}

impl Default for Value {
    fn default() -> Value {
        Value {
            state: Any::default(),
            response: Any::default(),
        }
    }
}

impl Value {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn state<T>(&mut self, state: &T, import: String) -> &mut Value
    where
        T: prost::Message,
    {
        let mut value = Vec::new();
        Message::encode(state, &mut value).unwrap();

        let type_url = format!("type.googleapis.com/{}", import);

        self.state = Any { type_url, value };
        self
    }

    pub fn get_state(&self) -> &Any {
        &self.state
    }

    pub fn response<T>(&mut self, message: &T, import: String) -> &mut Value
    where
        T: prost::Message,
    {
        let mut value = Vec::new();
        Message::encode(message, &mut value).unwrap();

        let type_url = format!("type.googleapis.com/{}", import);

        let any = Any { type_url, value };
        self.response = any;
        //self.response = Any::from_msg(message).unwrap();
        self
    }

    pub fn get_response(&self) -> &Any {
        &self.response
    }
}
