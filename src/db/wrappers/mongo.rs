use crate::{
    common::{errors::FeedDbError, report::Report, DbResult},
    create_error, option_to_result,
};

use crate::db::*;

use log::warn;
use mongodb::Document;
use uuid::Uuid;
use wither::prelude::*;

const SCOPE: &str = "database/mongo";

/// Public implementation of a FeederDb for a Mongo database
impl FeedWrapper for DbConnection {
    fn create_feed(self, feed: model::Feed) -> DbResult<model::Feed> {
        let mut created_feed: model::Feed = model::Feed::new(
            feed.title.unwrap().as_str(),
            feed.description.unwrap().as_str(),
            feed.link.unwrap().as_str(),
        )?;

        match created_feed.save(self.clone(), Option::None) {
            Ok(_) => Result::Ok(created_feed),
            Err(e) => {
                warn!("error while saving feed: {:?}", e);
                Result::Err(create_error!(SCOPE, FeedDbError::FailedToSaveFeed))
            }
        }
    }

    fn get_feed(self, uuid: Uuid) -> DbResult<model::Feed> {
        let filter: Document = doc! {
            "uuid": format!("{}", uuid)
        };

        match model::Feed::find_one(self.clone(), Option::Some(filter), Option::None) {
            Ok(value) => {
                if let Some(feed) = value {
                    return Result::Ok(feed);
                } else {
                    return Result::Err(create_error!(SCOPE, FeedDbError::FailedToGetFeeds));
                }
            }
            Err(e) => {
                warn!("failed to get the feed: {:?}", e);
                Result::Err(create_error!(SCOPE, FeedDbError::FailedToGetFeeds))
            }
        }
    }

    fn update_feed(self, uuid: Uuid, feed: model::Feed) -> DbResult<model::Feed> {
        // FIXME: compute the checksum again
        let update: Document;
        update = doc! {
            "$set": mongodb::to_bson(&feed)
            .unwrap()
            .as_document()
            .unwrap()
            .clone()
        }; // FIXME: no unwraps

        let mut find_and_update_options: mongodb::coll::options::FindOneAndUpdateOptions;
        find_and_update_options = mongodb::coll::options::FindOneAndUpdateOptions::new();
        find_and_update_options.return_document =
            Option::Some(mongodb::coll::options::ReturnDocument::After);

        let filter: Document = doc! {
            "uuid": format!("{}", uuid) // FIXME: no unwraps
        };

        match model::Feed::find_one_and_update(
            self.clone(),
            filter,
            update,
            Option::Some(find_and_update_options),
        ) {
            Ok(value) => option_to_result!(value, SCOPE, FeedDbError::FailedToUpdateFeed),
            Err(e) => {
                warn!("error updating the feed: {:?}", e);
                Result::Err(create_error!(SCOPE, FeedDbError::FailedToUpdateFeed))
            }
        }
    }

    fn delete_feed(self, _uuid: Uuid) -> DbResult<Report<String>> {
        let filter: Document = doc! {
            "uuid": format!("{}", _uuid)
        };

        match model::Feed::find_one_and_delete(self.clone(), filter, Option::None) {
            Ok(value) => {
                if let None = value {
                    return Result::Err(create_error!(SCOPE, FeedDbError::FailedToDeleteFeed));
                } else {
                    return Result::Ok(Report::new(SCOPE.to_string(), "deleted feed".to_string()));
                }
            }
            Err(e) => {
                warn!("failed to delete the feed: {:?}", e);
                Result::Err(create_error!(SCOPE, FeedDbError::FailedToDeleteFeed))
            }
        }
    }

    fn get_feed_checksum(self, _uuid: Uuid) -> DbResult<String> {
        let feed: model::Feed = self.get_feed(_uuid)?;
        Result::Ok(format!("{}", feed.get_uuid().unwrap())) // FIXME: no unwraps
    }
}
