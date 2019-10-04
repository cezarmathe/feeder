pub mod feed;
pub mod model;

use std::sync::Arc;

use mongodb::db::DatabaseInner;

/// Struct used for connections to the database
#[database("feeder")]
pub struct FeederDbConn(Arc<DatabaseInner>);

/// Abreviation for a database connection struct
pub type DbConn = Arc<DatabaseInner>;
