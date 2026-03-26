//! Tool registry for external API integrations

use std::collections::HashMap;

pub struct ToolRegistry {
    tools: HashMap<String, Box<dyn Fn(String) -> Result<String, String> + Send>>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
        }
    }

    pub fn register<F>(&mut self, name: &str, handler: F)
    where
        F: Fn(String) -> Result<String, String> + Send + 'static,
    {
        self.tools.insert(name.to_string(), Box::new(handler));
    }

    pub fn call(&self, name: &str, arg: String) -> Option<Result<String, String>> {
        self.tools.get(name).map(|f| f(arg))
    }
}
