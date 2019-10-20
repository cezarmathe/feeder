use crate::common::{report::Report, DbResult};

use super::model::{Feed, FeedItem};

use uuid::Uuid;

/// A trait that defines the behaviour of a database connection used by feeder for feed items
pub trait FeedItemWrapper {
    /// Create a new feed item
    fn create_feed_item(self, parent_feed: Feed, feed_item: FeedItem) -> DbResult<FeedItem>;

    /// Get a feed item
    fn get_feed_item(self, parent_feed: Feed, uuid: Uuid) -> DbResult<FeedItem>;

    /// Get multiple feed items
    /// If no uuids are provided, all the feed items of the parent feed are returned
    fn get_feed_items(self, parent_feed: Feed, uuids: Option<Vec<Uuid>>)
        -> DbResult<Vec<FeedItem>>;

    /// Update a feed item
    fn update_feed_item(
        self,
        parent_feed: Feed,
        uuid: Uuid,
        feed_item: FeedItem,
    ) -> DbResult<FeedItem>;

    /// Delete a feed item
    fn delete_feed_item(self, parent_feed: Feed, uuid: Uuid) -> DbResult<Report<String>>;

    /// Get the checksum of a feed item
    fn get_feed_item_checksum(self, parent_feed: Feed, uuid: Uuid) -> DbResult<String>;
}
