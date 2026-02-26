#[derive(Debug, Clone)]
pub struct Agent {
    pub id: String,
    pub name: String,
}

impl Agent {
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
        }
    }

    pub fn is_valid(&self) -> bool {
        !self.id.is_empty() && !self.name.is_empty()
    }
}
