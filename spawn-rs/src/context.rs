use prost_types::Any;

#[derive(Debug, Clone)]
pub struct Context {
    state: Any,
}

impl Context {
    /// Returns a reference to the state of this [`Context`].
    pub fn state(&self) -> &Any {
        &self.state
    }

    /// Sets the state of this [`Context`].
    pub fn set_state(&mut self, state: Any) {
        self.state = state;
    }
}
