use crate::{
    common::errors::{Error, ModelError},
    db::{feed_item, DbConnection},
};

use crypto::{digest::Digest, sha3::Sha3};
use log::*;
use mongodb::{coll::options::IndexModel, oid::ObjectId};
use serde::Serialize;
use uuid::Uuid;

const SCOPE: &str = "database/model";

/// Enum that specifies whether a feed contains only
/// the Uuids or the full items
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ItemsVec {
    Uuid(Vec<Uuid>),
    Full(Vec<FeedItem>),
}

#[derive(Clone, Debug, Deserialize, Model, Serialize)]
pub struct Feed {
    #[serde(
        rename = "_id",
        skip_serializing_if = "Option::is_none",
        skip_serializing
    )]
    id: Option<ObjectId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[model(index(index = "asc", unique = "true"))]
    uuid: Option<Uuid>,

    pub title: Option<String>,
    pub description: Option<String>,
    pub link: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copyright: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<aux::FeedImage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<ItemsVec>,

    #[serde(skip_serializing_if = "Option::is_none")]
    checksum: Option<String>,
}

impl Feed {
    /// Create a new feed.
    pub fn new(_title: &str, _description: &str, _link: &str) -> Result<Self, Error> {
        debug!(
            "creating a new feed struct with args: {:?}, {:?}, {:?}",
            _title, _description, _link
        );

        let title = String::from(_title);
        let description = String::from(_description);
        let link = String::from(_link);

        let mut feed = Feed {
            id: Option::None,
            uuid: Option::Some(Uuid::new_v4()),
            title: Option::Some(title),
            description: Option::Some(description),
            link: Option::Some(link),
            category: Option::None,
            copyright: Option::None,
            image: Option::None,
            language: Option::None,
            items: Option::None,
            checksum: Option::None,
        };

        // compute the checksum
        if let Some(err) = feed.compute_checksum(Option::None) {
            error!("checksum could not computed");
            return Result::Err(err);
        }

        debug!("successfully created feed: {:?}", feed);
        Result::Ok(feed)
    }

    /// Get the UUID of this feed
    pub fn get_uuid(&self) -> Option<Uuid> {
        self.uuid
    }

    /// Get the checksum of this feed
    pub fn get_checksum(&self) -> Option<String> {
        self.checksum.clone()
    }

    /// Compute the checksum of this feed
    /// The checksum is saved inside the object
    pub fn compute_checksum(&mut self, db_conn: Option<DbConnection>) -> Option<Error> {
        debug!("computing checksum for feed {:?}", self);

        if let Some(value) = db_conn {
            return self.with_items(&value);
        }

        match compute_checksum(self) {
            Ok(checksum) => {
                self.checksum = Option::Some(checksum);
                Option::None
            }
            Err(e) => Option::Some(e),
        }
    }

    /// Return this feed along with its items
    pub fn with_items(&mut self, db_conn: &DbConnection) -> Option<Error> {
        if self.items.is_none() {
            return Option::None;
        }

        match self.items.as_ref().unwrap() {
            ItemsVec::Full(_) => Option::None,
            ItemsVec::Uuid(items_uuid) => {
                let mut items_full: Vec<FeedItem> = Vec::new();

                for item_uuid in items_uuid {
                    match feed_item::get_feed_item(db_conn, self, &item_uuid) {
                        Ok(item) => items_full.push(item),
                        Err(e) => return Option::Some(e),
                    }
                }

                Option::None
            }
        }
    }

    /// Return this feed with the items having only uuids
    pub fn with_uuids(&mut self) {
        // If there are no items to be converted, return
        if self.items.is_none() {
            return;
        }

        let items_vec: ItemsVec = self.clone().items.unwrap();
        match items_vec {
            ItemsVec::Uuid(_) => {}
            ItemsVec::Full(items) => {
                let mut items_uuid: Vec<Uuid> = Vec::new();

                for item in items {
                    if item.get_uuid().is_none() {
                        continue;
                    }
                    items_uuid.push(item.get_uuid().unwrap());
                }

                self.items = Option::Some(ItemsVec::Uuid(items_uuid));
            }
        }
    }

    /// Generate the RSS representation of this feed.
    pub fn generate_rss(&self, _db_conn: DbConnection) {
        unimplemented!();
    }
}

#[derive(Clone, Debug, Deserialize, Model, Serialize)]
pub struct FeedItem {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<mongodb::oid::ObjectId>,
    uuid: Option<Uuid>,
    pub title: String,
    pub link: String,
    pub description: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enclosure: Option<aux::FeedItemEnclosure>,

    #[serde(skip_serializing_if = "Option::is_none")]
    checksum: Option<String>,
}

impl FeedItem {
    /// Create a new feed item
    pub fn new(_title: &str, _link: &str, _description: &str) -> Result<Self, Error> {
        let title = String::from(_title);
        let link = String::from(_link);
        let description = String::from(_description);

        let mut feed_item = FeedItem {
            id: Option::None,
            uuid: Option::Some(Uuid::new_v4()),
            title,
            link,
            description,
            author: Option::None,
            comments: Option::None,
            enclosure: Option::None,
            checksum: Option::None,
        };

        if let Some(e) = feed_item.compute_checksum() {
            return Result::Err(e);
        }

        Result::Ok(feed_item)
    }

    /// Compute the checksum for this feed item
    pub fn compute_checksum(&mut self) -> Option<Error> {
        match compute_checksum(self) {
            Ok(value) => {
                self.checksum = Option::Some(value);
                Option::None
            }
            Err(e) => Option::Some(e),
        }
    }

    /// Get the uuid of this feed item
    pub fn get_uuid(&self) -> Option<Uuid> {
        self.uuid
    }

    /// Get the checksum of this feed item
    pub fn get_checksum(&self) -> Option<String> {
        self.checksum.clone()
    }
}

/// Compute the checksum for a given model
fn compute_checksum<T>(model: &mut T) -> Result<String, Error>
where
    T: Serialize + std::fmt::Debug,
{
    if let Ok(json) = serde_json::to_string(&model) {
        let mut hasher = Sha3::sha3_256();
        hasher.input_str(json.as_str());

        Result::Ok(hasher.result_str())
    } else {
        Result::Err(create_error!(SCOPE, ModelError::FailedToComputeChecksum))
    }
}

/// Module that contains auxiliary models
pub mod aux {
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct FeedItemEnclosure {
        url: String,
        length: String,
        _type: String,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct FeedImage {
        url: String,
        title: String,
        link: String,
    }
}

#[cfg(test)]
mod test {
    use super::Feed;

    #[test]
    fn feed_new_test() {
        #![allow(warnings)]
        *crate::_LOG;

        let _feed = Feed::new(
            "My title",
            "My example description for my feed test",
            "https://example.com",
        );
        if _feed.is_err() {
            panic!("Failed to create feed");
        }

        let feed = _feed.unwrap();

        println!("uuid: {}", feed.uuid.unwrap());
        println!("checksum: {}", feed.checksum.unwrap());
    }
}
