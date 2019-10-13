use crate::db::model::*;

use std::boxed::Box;

use thiserror::Error;
use uuid::Uuid;

/// Error struct used by feeder
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Error {
    timestamp: u64,
    scope: String,
    message: String,
}

impl Error {
    /// Create a new Error
    pub fn new(scope: String, error: Box<dyn std::error::Error>) -> Self {
        Error {
            timestamp: super::timestamp(),
            scope,
            message: format!("{}", error),
        }
    }
}

/// Errors that can be produced by the feed router
#[derive(Clone, Debug, Deserialize, Error, Serialize)]
pub enum FeedRouterError {
    #[error("model has no title: {model:?}")]
    ModelHasNoTitle { model: Feed },
    #[error("model has no description: {model:?}")]
    ModelHasNoDescription { model: Feed },
    #[error("model has no link: {model:?}")]
    ModelHasNoLink { model: Feed },
}

/// Errors that can be produced by the feed items router
#[derive(Clone, Debug, Deserialize, Error, Serialize)]
pub enum FeedItemsRouterError {
    #[error("feed has no items")]
    FeedHasNoItems,
    #[error("no feed item with the uuid {item_uuid} found in the feed with uuid {feed_uuid}")]
    NoFeedItemInFeed { item_uuid: Uuid, feed_uuid: Uuid },
    #[error("failed to retrieve items from the feed with uuid {feed_uuid}")]
    FailedToRetrieveItems { feed_uuid: Uuid },
    #[error("could not create feed item: {feed_item:?}")]
    CouldNotCreateFeedItem { feed_item: FeedItem },
    // #[error("failed to update feed {feed:?} after doing an operation on feed items")]
    // FailedToUpdateFeed { feed: Feed },
}

/// Errors that can be produced by uuid checking
#[derive(Clone, Debug, Deserialize, Error, Serialize)]
pub enum UuidError {
    #[error("invalid uuid: {err}")]
    UuidIsNotValid { err: String },
}

/// Errors that can be produced by db models
#[derive(Clone, Debug, Deserialize, Error, Serialize)]
pub enum ModelError {
    #[error("failed to compute checksum for model: {model}")]
    FailedToComputeChecksum { model: String },
}

/// Errors that can be produced by the feed db
#[derive(Clone, Debug, Deserialize, Error, Serialize)]
pub enum FeedDbError {
    #[error("failed to save the feed {feed:?} in the database")]
    FailedToSaveFeed { feed: Feed },
    #[error("failed to get feeds from the database")]
    FailedToGetFeeds,
    #[error("no feed found with the uuid {uuid}")]
    NoFeedFound { uuid: Uuid },
    #[error("feed {feed:?} has no checksum")]
    FeedHasNoChecksum { feed: Feed },
    #[error("failed to delete feed {feed:?}")]
    FailedToDeleteFeed { feed: Feed },
}

/// Errors that can be produced by Rocket catchers
#[derive(Clone, Debug, Deserialize, Error, Serialize)]
pub enum HttpError {
    #[error("bad request")]
    BadRequest,
    #[error("unauthorized")]
    Unauthorized,
    #[error("forbidden")]
    Forbidden,
    #[error("not found")]
    NotFound,
    #[error("not acceptable")]
    NotAcceptable,
    #[error("internal server error")]
    Ise,
    #[error("not implemented")]
    NotImplemented,
    #[error("service unavailable")]
    ServiceUnavailable,
}
