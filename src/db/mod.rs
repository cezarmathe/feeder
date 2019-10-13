pub mod feed;
pub mod feed_item;
pub mod model;

use std::sync::Arc;

use mongodb::db::DatabaseInner;

/// Struct used for connections to the database
#[database("feeder")]
pub struct FeederDbConn(Arc<DatabaseInner>);

/// Abreviation for a database connection struct
pub type DbConnection = Arc<DatabaseInner>;
