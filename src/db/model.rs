use std::option::Option;
use std::str;
use std::vec::Vec;

use crypto::digest::Digest;
use crypto::sha3::Sha3;
use log::{debug, error, warn};
use mongodb::Document;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Feed {
    uuid: Option<Uuid>,
    title: String,
    description: String,
    link: String,

    pub category: Option<String>,
    pub copyright: Option<String>,
    pub image: Option<FeedImage>,
    pub language: Option<String>,

    items_uuid: Vec<Uuid>,

    checksum: Option<String>,
}

impl Feed {
    /// Create a new feed.
    pub fn new(_title: &str, _description: &str, _link: &str) -> Option<Self> {
        debug!(
            "creating a new feed struct with args: {:?}, {:?}, {:?}",
            _title, _description, _link
        );

        let title = String::from(_title);
        let description = String::from(_description);
        let link = String::from(_link);

        let mut feed = Feed {
            uuid: Option::None,
            title,
            description,
            link,
            category: Option::None,
            copyright: Option::None,
            image: Option::None,
            language: Option::None,
            items_uuid: Vec::new(),
            checksum: Option::None,
        };

        // generate the uuid
        debug!("generating uuid");
        feed.uuid = Option::Some(Uuid::new_v4());
        debug!("generated {}", feed.uuid.unwrap());

        // compute the checksum
        if !feed.compute_checksum() {
            error!("checksum not computed, returning Option::None");
            return Option::None;
        }

        debug!("successfully created feed: {:?}", feed);
        return Option::Some(feed);
    }

    pub fn to_json(&self) -> Option<String> {
        match serde_json::to_string(self) {
            Ok(_result) => {
                Option::Some(_result)
            },
            Err(e) => {
                warn!("failed to convert feed {:?} to json: {:?}", self, e);
                Option::None
            }
        }
    }

    pub fn to_document(&self) -> Option<Document> {
        let mut document = Document::new();

        document.insert("uuid", self.uuid.clone());
        document.insert("title", self.title.clone());
        document.insert("description", self.description.clone());
        document.insert("link", self.link.clone());
        document.insert("category", self.category.clone());
        document.insert("copyright", self.copyright.clone());
        document.insert("image", self.image.clone());
        document.insert("language", self.language.clone());
        document.insert("items_uuid", self.items_uuid.clone());
        document.insert("checksum", self.checksum.clone());

        return Option::Some(document);
    }

    pub fn get_uuid(&self) -> Option<&Uuid> {
        return self.uuid.as_ref();
    }

    pub fn get_checksum(&self) -> Option<&String> {
        return self.checksum.as_ref();
    }

    /// Compute the checksum of this feed.
    /// The checksum is saved inside the object.
    fn compute_checksum(&mut self) -> bool {
        debug!("computing checksum for feed {:?}", self);
        let json_result = serde_json::to_string(self);

        match json_result {
            Ok(json) => {
                debug!("successfully converted feed to json: {}", json);
                let mut hasher = Sha3::sha3_256();
                hasher.input_str(json.as_str());
                self.checksum = Some(String::from(hasher.result_str()));
                debug!("successfully computed the checksum");
                return true;
            }
            Err(e) => {
                warn!("could not convert feed {:?} to json", self);
                return false;
            }
        }
    }

    /// Generate the RSS representation of this feed.
    pub fn generate_rss(&self) {
        unimplemented!();
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FeedImage {
    url: String,
    title: String,
    link: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FeedItem {
    uuid: Option<Uuid>,
    title: String,
    link: String,
    description: String,

    author: Option<String>,
    comments: Option<String>,
    enclosure: Option<FeedItemEnclosure>,

    checksum: Option<String>,
}

impl FeedItem {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FeedItemEnclosure {
    url: String,
    length: String,
    _type: String,
}

#[cfg(test)]
mod test {
    use super::Feed;

    #[test]
    fn feed_new_test() {
        *crate::LOG;

        let _feed = Feed::new(
            "My title",
            "My example description for my feed test",
            "https://example.com",
        );
        if _feed.is_none() {
            panic!("Failed to create feed");
        }

        let feed = _feed.unwrap();

        println!("uuid: {}", feed.uuid.unwrap());
        println!("checksum: {}", feed.checksum.unwrap());
    }
}
