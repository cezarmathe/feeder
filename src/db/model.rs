use crate::{
    common::errors::{Error, ModelError},
    db::{FeedItemWrapper, FeedWrapper},
};

use std::sync::Arc;

use crypto::{digest::Digest, sha3::Sha3};
use log::*;
use mongodb::{coll::options::IndexModel, db::DatabaseInner, oid::ObjectId};
use serde::Serialize;
use uuid::Uuid;

const SCOPE: &str = "database/model";

/// Enum that specifies whether a feed contains only
/// the Uuids or the full items
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ItemsVec {
    #[serde(rename = "uuid")]
    Uuid(Vec<Uuid>),
    #[serde(rename = "full")]
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
    pub fn _new(_title: &str, _description: &str, _link: &str) -> Result<Self, Error> {
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

    /// Create a new feed fromma given model
    pub fn new_from_model(model: Feed) -> Result<Self, Error> {
        // Filter out bad models
        if model.title.is_none() {
            warn!("{}", ModelError::ModelHasNoTitle);
            return Result::Err(create_error!(SCOPE, ModelError::ModelHasNoTitle));
        }
        if model.description.is_none() {
            warn!("{}", ModelError::ModelHasNoDescription);
            return Result::Err(create_error!(SCOPE, ModelError::ModelHasNoDescription));
        }
        if model.link.is_none() {
            warn!("{}", ModelError::ModelHasNoLink);
            return Result::Err(create_error!(SCOPE, ModelError::ModelHasNoLink));
        }

        let mut feed = Feed {
            id: Option::None,
            uuid: Option::Some(Uuid::new_v4()),
            title: model.title,
            description: model.description,
            link: model.link,
            category: model.category,
            copyright: model.copyright,
            image: model.image,
            language: model.language,
            items: Option::None,
            checksum: Option::None,
        };

        // Compute the checksum
        if let Some(err) = feed.compute_checksum(Option::None) {
            error!("could not compute the checksum for this feed");
            return Result::Err(err);
        }

        Result::Ok(feed)
    }

    /// Get the UUID of this feed
    pub fn get_uuid(&self) -> Option<Uuid> {
        self.uuid.clone()
    }

    /// Get the checksum of this feed
    pub fn get_checksum(&self) -> Option<String> {
        self.checksum.clone()
    }

    /// Compute the checksum of this feed
    /// The checksum is saved inside the object
    pub fn compute_checksum(&mut self, db_conn: Option<Arc<DatabaseInner>>) -> Option<Error> {
        debug!("computing checksum for feed {:?}", self);

        let change_flag: bool = if let Some(value) = db_conn {
            self.with_items(value);
            true
        } else {
            false
        };
        let result: Option<Error>;

        match compute_checksum(self) {
            Ok(checksum) => {
                self.checksum = Option::Some(checksum);
                result = Option::None;
            }
            Err(e) => result = Option::Some(e),
        }

        if change_flag {
            self.with_uuids();
        }

        result
    }

    /// Return this feed along with its items
    pub fn with_items(&mut self, db_conn: Arc<DatabaseInner>) -> Option<Error> {
        if self.items.is_none() {
            self.items = Option::Some(ItemsVec::Uuid(Vec::new()));
            match db_conn
                .clone()
                .update_feed(self.get_uuid().unwrap(), self.clone())
            {
                Ok(_) => return Option::None,
                Err(e) => return Option::Some(e),
            }
        }

        match self.items.clone().unwrap() {
            ItemsVec::Full(_) => Option::None,
            ItemsVec::Uuid(items_uuid) => {
                let items_full: Vec<FeedItem>;
                match db_conn.get_feed_items(self.clone(), Option::Some(items_uuid)) {
                    Ok(value) => items_full = value,
                    Err(e) => return Option::Some(e),
                };
                self.items = Option::Some(ItemsVec::Full(items_full));
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
    pub fn _generate_rss(&self, _db_conn: Arc<DatabaseInner>) {
        unimplemented!();
    }
}

#[derive(Clone, Debug, Deserialize, Model, Serialize)]
pub struct FeedItem {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none", skip)]
    id: Option<mongodb::oid::ObjectId>,
    uuid: Option<Uuid>,
    pub title: Option<String>,
    pub link: Option<String>,
    pub description: Option<String>,

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
    pub fn _new(_title: &str, _link: &str, _description: &str) -> Result<Self, Error> {
        let title = String::from(_title);
        let link = String::from(_link);
        let description = String::from(_description);

        let mut feed_item = FeedItem {
            id: Option::None,
            uuid: Option::Some(Uuid::new_v4()),
            title: Option::Some(title),
            link: Option::Some(link),
            description: Option::Some(description),
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

    pub fn new_from_model(model: FeedItem) -> Result<Self, Error> {
        // Filter out bad models
        if model.title.is_none() {
            warn!("{}", ModelError::ModelHasNoTitle);
            return Result::Err(create_error!(SCOPE, ModelError::ModelHasNoTitle));
        }
        if model.description.is_none() {
            warn!("{}", ModelError::ModelHasNoDescription);
            return Result::Err(create_error!(SCOPE, ModelError::ModelHasNoDescription));
        }
        if model.link.is_none() {
            warn!("{}", ModelError::ModelHasNoLink);
            return Result::Err(create_error!(SCOPE, ModelError::ModelHasNoLink));
        }

        let mut feed_item: FeedItem = FeedItem {
            id: Option::None,
            uuid: Option::Some(Uuid::new_v4()),
            title: model.title,
            link: model.link,
            description: model.description,
            author: model.author,
            comments: model.comments,
            enclosure: model.enclosure,
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

        let _feed = Feed::_new(
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
