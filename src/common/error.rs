/// Error struct used by feeder
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Error {
    timestamp: u64,
    scope: String,
    message: String,
}

impl Error {
    /// Create a new Error
    pub fn new(scope: String, message: String) -> Error {
        Error {
            timestamp: super::timestamp(),
            scope,
            message,
        }
    }
}
