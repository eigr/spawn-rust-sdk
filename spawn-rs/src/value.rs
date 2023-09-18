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

    pub fn state(&mut self, state: Any) -> &mut Value {
        self.state = state;
        self
    }

    pub fn get_state(&self) -> &Any {
        &self.state
    }

    pub fn response<T>(&mut self, message: &T) -> &mut Value
    where
        T: prost::Name,
    {
        self.response = Any::from_msg(message).unwrap();
        self
    }

    pub fn get_response(&self) -> &Any {
        &self.response
    }
}
