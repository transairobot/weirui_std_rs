#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HostErrorCode {
    Success = 0,
    InvalidParameter = 1,
    MemoryError = 2,
    ServoError = 3,
    NetworkError = 4,
    McpError = 5,
    SerializationError = 6,
    InternalError = 7,
}

#[derive(Debug, Clone)]
pub struct HostError {
    pub code: HostErrorCode,
    pub message: String,
}

impl HostError {
    pub fn new(code: HostErrorCode, message: String) -> Self {
        Self { code, message }
    }
}

impl std::fmt::Display for HostError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.code as i32, self.message)
    }
}

impl std::error::Error for HostError {}
