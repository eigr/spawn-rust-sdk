use prost_types::Any;

#[derive(Debug, Clone)]
pub struct Value {
    state: Any,
    response: Any,
}

impl Default for Value {
    fn default() -> Request {
        Request {
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

    pub fn response(&mut self, response: Any) -> &mut Value {
        self.response = response;
        self
    }

    pub fn get_response(&self) -> &Any {
        &self.response
    }
}
