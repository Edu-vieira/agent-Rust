use std::collections::HashMap;

pub type ToolHandler = Box<dyn Fn(&serde_json::Value) -> Result<String, String> + Send + Sync>;

pub struct Tool {
    pub name: String,
    pub description: String,
    pub handler: ToolHandler,
}

pub struct ToolRegistry {
    tools: HashMap<String, Tool>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
        }
    }

    pub fn register(&mut self, tool: Tool) {
        self.tools.insert(tool.name.clone(), tool);
    }

    pub fn execute(&self, name: &str, arguments: &serde_json::Value) -> Result<String, String> {
        let tool = self
            .tools
            .get(name)
            .ok_or_else(|| format!("Ferramenta desconhecida: {}", name))?;

        (tool.handler)(arguments)
    }

    pub fn descriptions(&self) -> Vec<String> {
        self.tools
            .values()
            .map(|tool| format!("{}: {}", tool.name, tool.description))
            .collect()
    }
}
