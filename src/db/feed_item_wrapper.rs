use crate::common::{report::Report, DbResult};

use super::model::{Feed, FeedItem};

use uuid::Uuid;

/// A trait that defines the behaviour of a database connection used by feeder for feed items
pub trait FeedItemWrapper {}

// /// A trait that defines the behaviour of a database used by feeder
// /// T is type used to represent the database connection
// pub trait FeederDb {
//     // /// Create a feed item
//     // fn create_feed_item(&self, db_conn: &T) -> DbResult<model::FeedItem>;

//     // /// Create multiple feed items
//     // fn create_feed_items(&self, db_conn: &T) -> Vec<DbResult<model::FeedItem>>;

//     // /// Save a feed item in the database. Internally used by create and update operations
//     // fn save_feed_item(&self, db_conn: &T) -> DbResult<model::FeedItem>;

//     // /// Save multiple feed items in the database. Internally used by create and update operations
//     // fn save_feed_items(&self, db_conn: &T) -> Vec<DbResult<model::FeedItem>>;

//     // /// Get a feed item from the database
//     // fn get_feed_item(&self, db_conn: &T) -> DbResult<model::FeedItem>;

//     // /// Get multiple feed items from the database
//     // fn get_feed_items(&self, db_conn: &T) -> Vec<DbResult<model::FeedItem>>;

//     // /// Update a feed item
//     // fn update_feed_item(&self, db_conn: &T) -> DbResult<model::FeedItem>;

//     // /// Update multiple feed items
//     // fn update_feed_items(&self, db_conn: &T) -> Vec<DbResult<model::FeedItem>>;

//     // /// Delete a feed item
//     // fn delete_feed_item(&self, db_conn: &T) -> DbResult<String>;

//     // /// Delete multiple feed items
//     // fn delete_feed_items(&self, db_conn: &T) -> Vec<DbResult<String>>;

//     // /// Get the checksum of a feed item
//     // fn get_feed_item_checksum(&self, db_conn: &T) -> DbResult<String>;
//     // /// Get the checksums of multiple feed items
//     // fn get_feed_items_checksum(&self, db_conn: &T) -> Vec<DbResult<String>>;
// }
