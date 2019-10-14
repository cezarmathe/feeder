pub mod feed_item_wrapper;
pub mod feed_wrapper;
pub mod model;

// Mongodb implementation for feeder
#[cfg(feature = "mongo")]
pub mod mongo;

use crate::common::DbResult;

/// Mongodb implementation imports
#[cfg(feature = "mongo")]
use {mongodb::db::DatabaseInner, std::sync::Arc};

pub use feed_item_wrapper::FeedItemWrapper;
pub use feed_wrapper::FeedWrapper;

/// Mongodb struct used for connections to the database
#[cfg(feature = "mongo")]
#[database("feeder")]
#[derive(Clone)]
pub struct DbConnection(Arc<DatabaseInner>);
