// Mongodb implementation for feeder
#[cfg(feature = "mongo")]
pub mod mongo;

/// Mongodb implementation imports
#[cfg(feature = "mongo")]
use {mongodb::db::DatabaseInner, std::sync::Arc};

/// Mongodb struct used for connections to the database
#[cfg(feature = "mongo")]
#[database("feeder")]
#[derive(Clone)]
pub struct DbConnection(Arc<DatabaseInner>);
