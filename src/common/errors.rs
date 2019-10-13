use std::boxed::Box;

use thiserror::Error;

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
    #[error("model has no title")]
    ModelHasNoTitle,
    #[error("model has no description")]
    ModelHasNoDescription,
    #[error("model has no link")]
    ModelHasNoLink,
    #[error("model has no uuid")]
    ModelHasNoUuid,
}

/// Errors that can be produced by the feed items router
#[derive(Clone, Debug, Deserialize, Error, Serialize)]
pub enum FeedItemsRouterError {
    #[error("feed has no items")]
    FeedHasNoItems,
    #[error("no feed item found")]
    NoFeedItemInFeed,
    #[error("failed to retrieve the items from the feed")]
    FailedToRetrieveItems,
    #[error("could not create the feed item")]
    CouldNotCreateFeedItem,
    // #[error("failed to update feed {feed:?} after doing an operation on feed items")]
    // FailedToUpdateFeed { feed: Feed },
}

/// Errors that can be produced by uuid checking
#[derive(Clone, Debug, Deserialize, Error, Serialize)]
pub enum UuidError {
    #[error("invalid uuid: {err}")]
    UuidNotValid { err: String },
}

/// Errors that can be produced by db models
#[derive(Clone, Debug, Deserialize, Error, Serialize)]
pub enum ModelError {
    #[error("failed to compute the checksum")]
    FailedToComputeChecksum,
}

/// Errors that can be produced by the feed db
#[derive(Clone, Debug, Deserialize, Error, Serialize)]
pub enum FeedDbError {
    #[error("failed to save the feed in the database")]
    FailedToSaveFeed,
    #[error("failed to get the feeds from the database")]
    FailedToGetFeeds,
    #[error("no feed found")]
    NoFeedFound,
    #[error("feed has no checksum")]
    FeedHasNoChecksum,
    #[error("failed to delete the feed")]
    FailedToDeleteFeed,
    #[error("failed to update feed")]
    FailedToUpdateFeed,
}

/// Errors that can be produced by the feed item db
pub enum FeedItemDbError {}

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
