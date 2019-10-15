use crate::{
    common::{
        errors::{FeedDbError, FeedItemDbError},
        report::Report,
        DbResult,
    },
    create_error, option_to_result,
};

use crate::db::*;

use log::*;
use mongodb::Document;
use uuid::Uuid;
use wither::prelude::*;

const SCOPE: &str = "database/mongo";

/// Implementation of the FeederWrapper for MongoDb
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
                    Result::Ok(feed)
                } else {
                    warn!("the database returned no feed");
                    Result::Err(create_error!(SCOPE, FeedDbError::FailedToGetFeeds))
                }
            }
            Err(e) => {
                warn!("failed to get the feed: {:?}", e);
                Result::Err(create_error!(SCOPE, FeedDbError::FailedToGetFeeds))
            }
        }
    }

    fn update_feed(self, uuid: Uuid, mut feed: model::Feed) -> DbResult<model::Feed> {
        if let Some(e) = feed.compute_checksum(Option::None) {
            return Result::Err(e);
        }

        let update: Document;
        let update_bson: mongodb::Bson;
        match mongodb::to_bson(&feed) {
            Ok(value) => update_bson = value,
            Err(e) => {
                warn!("failed to encode feed into bson: {:?}", e);
                return Result::Err(create_error!(SCOPE, FeedDbError::FailedToUpdateFeed));
            }
        }
        match update_bson.as_document() {
            Some(value) => update = doc! {"$set": value.clone()},
            None => {
                warn!("failed to get the bson-encoded feed as a document");
                return Result::Err(create_error!(SCOPE, FeedDbError::FailedToUpdateFeed));
            }
        }

        let mut find_and_update_options: mongodb::coll::options::FindOneAndUpdateOptions;
        find_and_update_options = mongodb::coll::options::FindOneAndUpdateOptions::new();
        find_and_update_options.return_document =
            Option::Some(mongodb::coll::options::ReturnDocument::After);

        let filter: Document = doc! {
            "uuid": format!("{}", uuid)
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
                if value.is_none() {
                    warn!("the database did not return the old feed after deleting");
                    Result::Err(create_error!(SCOPE, FeedDbError::FailedToDeleteFeed))
                } else {
                    Result::Ok(Report::new(SCOPE.to_string(), "deleted feed".to_string()))
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
        if let Some(value) = feed.get_uuid() {
            return Result::Ok(format!("{}", value));
        }
        warn!("the feed has no checksum");
        Result::Err(create_error!(SCOPE, FeedDbError::FeedHasNoChecksum))
    }
}

/// Implementation of FeedItemWrapper for MongoDb
impl FeedItemWrapper for DbConnection {
    fn create_feed_item(
        self,
        parent_feed: model::Feed,
        feed_item: model::FeedItem,
    ) -> DbResult<model::FeedItem> {
        let mut created_feed_item = model::FeedItem::new(
            feed_item.title.as_str(),
            feed_item.link.as_str(),
            feed_item.description.as_str(),
        )?;

        if let Err(e) = created_feed_item.save(self.clone(), Option::None) {
            warn!("failed to save feed item in the database: {:?}", e);
            return Result::Err(create_error!(SCOPE, FeedItemDbError::FailedToSaveItem));
        }

        if parent_feed.items.is_none() {
            info!("parent feed did not have any items, creating the items list now");
            parent_feed.items = Option::Some(model::ItemsVec::Uuid(Vec::new()));
        }

        let items_vec: Vec<Uuid>;
        match parent_feed.items.unwrap() {
            model::ItemsVec::Uuid(value) => items_vec = value,
            model::ItemsVec::Full(value) => {
                parent_feed.with_uuids();
                if let model::ItemsVec::Uuid(_value) = parent_feed.items.unwrap() {
                    info!("parent feed had the full items, changed to uuids only");
                    items_vec = _value;
                } else {
                    warn!("failed to change the parent feed to have uuids only");
                    return Result::Err(create_error!(SCOPE, FeedItemDbError::FailedToSaveItem));
                }
            }
        }
        items_vec.push(created_feed_item.get_uuid().unwrap());
        parent_feed.items = Option::Some(model::ItemsVec::Uuid(items_vec));
        self.update_feed(parent_feed.get_uuid().unwrap(), parent_feed)?;

        Result::Ok(created_feed_item)
    }

    fn get_feed_item(self, parent_feed: model::Feed, uuid: Uuid) -> DbResult<model::FeedItem> {
        if parent_feed.items.is_none() {
            return Result::Err(create_error!(SCOPE, FeedItemDbError::NoItemFound));
        }

        let items_vec: Vec<Uuid>;
        match parent_feed.items.unwrap() {
            model::ItemsVec::Uuid(value) => items_vec = value,
            model::ItemsVec::Full(value) => {
                parent_feed.with_uuids();
                if let model::ItemsVec::Uuid(_value) = parent_feed.items.unwrap() {
                    info!("parent feed had the full items, changed to uuids only");
                    items_vec = _value;
                } else {
                    warn!("failed to change the parent feed to have uuids only");
                    return Result::Err(create_error!(SCOPE, FeedItemDbError::NoItemFound));
                }
            }
        }

        for item_uuid in items_vec {
            if item_uuid == uuid {
                let filter: Document = doc! {
                    "uuid": format!("{}", uuid)
                };

                match model::FeedItem::find_one(self.clone(), Option::Some(filter), Option::None) {
                    Ok(value) => {
                        if let Some(feed_item) = value {
                            return Result::Ok(feed_item);
                        } else {
                            warn!("the database returned no feed item");
                            return Result::Err(create_error!(SCOPE, FeedItemDbError::NoItemFound));
                        }
                    }
                    Err(e) => {
                        warn!("failed to get the feed item: {:?}", e);
                        return Result::Err(create_error!(SCOPE, FeedItemDbError::NoItemFound));
                    }
                }
            }
        }

        warn!("parent feed has no matching feed item");
        Result::Err(create_error!(SCOPE, FeedItemDbError::NoItemFound))
    }

    fn get_feed_items(
        self,
        parent_feed: model::Feed,
        uuids: Option<Vec<Uuid>>,
    ) -> DbResult<Vec<model::FeedItem>> {
        // If parent feed has no items, error
        if parent_feed.items.is_none() {
            return Result::Err(create_error!(SCOPE, FeedItemDbError::NoItemFound));
        }

        // Get the item uuids of this feed
        let parent_item_uuids: Vec<Uuid>;
        match parent_feed.items.unwrap() {
            model::ItemsVec::Uuid(value) => parent_item_uuids = value,
            model::ItemsVec::Full(value) => {
                parent_feed.with_uuids();
                if let model::ItemsVec::Uuid(_value) = parent_feed.items.unwrap() {
                    info!("parent feed had the full items, changed to uuids only");
                    parent_item_uuids = _value;
                } else {
                    warn!("failed to change the parent feed to have uuids only");
                    return Result::Err(create_error!(SCOPE, FeedItemDbError::NoItemFound));
                }
            }
        }

        // Get the item uuids that should be retrieved
        // If no uuids are passed to the function, search for all feed items in this feed
        // Otherwise, trim the feed item uuid list to the ones specifiedj
        let item_uuids: Vec<Uuid>;
        if let Some(uuid_vec) = uuids {
            item_uuids = Vec::new();
            for item_uuid in uuid_vec {
                if parent_item_uuids.contains(&item_uuid) {
                    item_uuids.push(item_uuid);
                }
            }
        } else {
            item_uuids = parent_item_uuids;
        }

        let items_vec: Vec<model::FeedItem>;
        for item_uuid in item_uuids {
            let feed_item: model::FeedItem = self.get_feed_item(parent_feed.clone(), item_uuid)?;
            items_vec.push(feed_item);
        }

        Result::Ok(items_vec)
    }

    fn update_feed_item(
        self,
        parent_feed: model::Feed,
        uuid: Uuid,
        feed_item: model::FeedItem,
    ) -> DbResult<model::FeedItem> {
    }

    /// Delete a feed item
    fn delete_feed_item(self, parent_feed: model::Feed, uuid: Uuid) -> DbResult<Report<String>> {}

    /// Get the checksum of a feed item
    fn get_feed_item_checksum(self, parent_feed: model::Feed, uuid: Uuid) -> DbResult<String> {}
}
