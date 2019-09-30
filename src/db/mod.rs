pub mod feed;
pub mod model;

use std::sync::Arc;

use mongodb::db::DatabaseInner;

#[database("feeder")]
pub struct FeederDbConn(Arc<DatabaseInner>);
