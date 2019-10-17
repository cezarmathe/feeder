use serde::Serialize;

/// Struct that contains a report(for a successful action, unlike feeder::common::error::Error)
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Report<T: Serialize> {
    timestamp: u64,
    scope: String,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
}

impl<T: Serialize> Report<T> {
    /// Create a new Report without additional data
    pub fn new(scope: String, message: String) -> Report<T> {
        Report {
            timestamp: super::timestamp(),
            scope,
            message,
            data: Option::None,
        }
    }

    /// Create a new Report with some data
    pub fn _new_with_data(scope: String, message: String, data: T) -> Report<T> {
        Report {
            timestamp: super::timestamp(),
            scope,
            message,
            data: Option::Some(data),
        }
    }
}
