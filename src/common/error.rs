use std::time::{SystemTime, UNIX_EPOCH};

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
        let time: u64;
        match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(_value) => time = _value.as_secs(),
            Err(e) => {
                // extremely bad if happens
                panic!(e);
            }
        }
        Error {
            timestamp: time,
            scope,
            message,
        }
    }
}