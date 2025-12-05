//! Rendering system
//!
//! Layer-based flexible rendering pipeline supporting
//! user-defined composition of 2D and 3D layers.

pub struct Engine {
    name: String,
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            name: String::from("VulpID-tech"),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_creation() {
        let engine = Engine::new();
        assert_eq!(engine.name(), "VulpID-tech");
    }
}
