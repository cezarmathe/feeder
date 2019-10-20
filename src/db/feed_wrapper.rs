use crate::common::{report::Report, DbResult};

use super::model::Feed;

use uuid::Uuid;

/// A trait that defines the behaviour of a database connection used by feeder for feeds
pub trait FeedWrapper {
    /// Create a new feed
    fn create_feed(self, feed: Feed) -> DbResult<Feed>;

    /// Get a feed from the database
    fn get_feed(self, uuid: Uuid) -> DbResult<Feed>;

    /// Update a feed
    fn update_feed(self, uuid: Uuid, feed: Feed) -> DbResult<Feed>;

    /// Delete a feed
    fn delete_feed(self, _uuid: Uuid) -> DbResult<Report<String>>;

    /// Get the checksum of a feed
    fn get_feed_checksum(self, _uuid: Uuid) -> DbResult<String>;
}
