use crate::common::{report::Report, DbResult};

use super::model::Feed;

use uuid::Uuid;

/// A trait that defines the behaviour of a database connection used by feeder for feeds
pub trait FeedWrapper {
    /// Create a new feed
    fn create_feed(self, feed: Feed) -> DbResult<Feed>;

    /// Create multiple feeds
    fn create_feeds(self, feeds: Vec<Feed>) -> Vec<DbResult<Feed>>
    where
        Self: Clone,
    {
        let mut results: Vec<DbResult<Feed>> = Vec::new();

        for feed in feeds {
            results.push(self.clone().create_feed(feed));
        }

        results
    }

    // /// Save a feed in the database. Internally used by create and update operations
    // fn save_feed(&self, db_conn: &DbConnection, feed: model::Feed) -> DbResult<model::Feed>;

    // /// Save multiple feeds in the database. Internally used by create and update operations
    // fn save_feeds(
    //     &self,
    //     db_conn: &DbConnection,
    //     feeds: Vec<model::Feed>,
    // ) -> Vec<DbResult<model::Feed>> {
    //     let results: Vec<DbResult<model::Feed>> = Vec::new();

    //     for feed in feeds {
    //         results.push(self.save_feed(db_conn, feed));
    //     }

    //     results
    // }

    /// Get a feed from the database
    fn get_feed(self, uuid: Uuid) -> DbResult<Feed>;

    /// Get multiple feeds from the database
    fn get_feeds(self, uuids: Vec<Uuid>) -> Vec<DbResult<Feed>>
    where
        Self: Clone,
    {
        let mut results: Vec<DbResult<Feed>> = Vec::new();

        for _uuid in uuids {
            results.push(self.clone().get_feed(_uuid));
        }

        results
    }

    /// Update a feed
    fn update_feed(self, uuid: Uuid, feed: Feed) -> DbResult<Feed>;

    /// Update multiple feeds
    fn update_feeds(self, feeds: Vec<Feed>) -> Vec<DbResult<Feed>>
    where
        Self: Clone,
    {
        let mut results: Vec<DbResult<Feed>> = Vec::new();

        for feed in feeds {
            results.push(self.clone().update_feed(Uuid::new_v4(), feed)); // FIXME: dangerous
        }

        results
    }

    /// Delete a feed
    fn delete_feed(self, _uuid: Uuid) -> DbResult<Report<String>>;

    /// Delete multiple feed
    fn delete_feeds(self, uuids: Vec<Uuid>) -> Vec<DbResult<Report<String>>>
    where
        Self: Clone,
    {
        let mut results: Vec<DbResult<Report<String>>> = Vec::new();

        for _uuid in uuids {
            results.push(self.clone().delete_feed(_uuid));
        }

        results
    }

    /// Get the checksum of a feed
    fn get_feed_checksum(self, _uuid: Uuid) -> DbResult<String>;

    /// Get the checksum of multiple feeds
    fn get_feeds_checksum(self: Self, uuids: Vec<Uuid>) -> Vec<DbResult<String>>
    where
        Self: Clone,
    {
        let mut results: Vec<DbResult<String>> = Vec::new();

        for _uuid in uuids {
            results.push(self.clone().get_feed_checksum(_uuid));
        }

        results
    }
}
