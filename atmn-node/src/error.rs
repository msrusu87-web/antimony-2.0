use std::fmt;

#[derive(Debug, Clone)]
pub struct NodeError {
    pub message: String,
}

impl NodeError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl fmt::Display for NodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for NodeError {}

impl From<sqlx::Error> for NodeError {
    fn from(err: sqlx::Error) -> Self {
        NodeError::new(format!("Database error: {}", err))
    }
}

impl From<std::io::Error> for NodeError {
    fn from(err: std::io::Error) -> Self {
        NodeError::new(format!("IO error: {}", err))
    }
}

impl From<chrono::ParseError> for NodeError {
    fn from(err: chrono::ParseError) -> Self {
        NodeError::new(format!("Parse error: {}", err))
    }
}

impl From<String> for NodeError {
    fn from(err: String) -> Self {
        NodeError::new(err)
    }
}

impl From<&str> for NodeError {
    fn from(err: &str) -> Self {
        NodeError::new(err.to_string())
    }
}
