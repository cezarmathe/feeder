use std::option::Option;
use std::str;
use std::vec::Vec;

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

    md5_checksum: Option<String>
}

impl Feed {
    /// Create a new feed.
    pub fn new(_title: &str, _description: &str, _link: &str) -> Option<Feed> {

        let title = String::from(_title);
        let description = String::from(_description);
        let link = String::from(_link);

        let mut feed = Feed{
            uuid: Option::None,
            title,
            description,
            link,
            category: Option::None,
            copyright: Option::None,
            image: Option::None,
            language: Option::None,
            items_uuid: Vec::new(),
            md5_checksum: Option::None,
        };

        // if the checksum fails to compute, do not proceed creating the feed
        if feed.compute_checksum() {
            return Option::None;
        }

        // extract the checksum and use it to generate an uuid
        let checksum = feed.md5_checksum.as_ref().unwrap();
        let _uuid = Uuid::new_v3(&Uuid::NAMESPACE_OID, checksum.as_bytes());

        // recompute the checksum because the object now has an uuid
        if feed.compute_checksum() {
            return Option::None;
        }

        return Option::Some(feed);
    }

    pub fn get_uuid(&self) -> Option<&Uuid> {
        return self.uuid.as_ref();
    }

    pub fn get_checksum(&self) -> Option<&String> {
        return self.md5_checksum.as_ref();
    }

    /// Compute the checksum of this feed.
    /// The checksum is saved inside the object.
    fn compute_checksum(&mut self) -> bool {
        let json_result = serde_json::to_string(self);

        match json_result {
            Ok(json) => {
                let digest = md5::compute(json.as_bytes());
                let checksum_result = str::from_utf8(digest.as_ref());
                if checksum_result.is_err() {
                    return false;
                }
                self.md5_checksum = Some(String::from(checksum_result.unwrap()));
                return true;
            }
            Err(_) => {
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
    link: String
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

    md5_checksum: Option<String>
}

impl FeedItem {

}

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
        let _feed =
            Feed::new("My title",
                      "My example description for my feed test",
                      "https://example.com");
        if _feed.is_none() {
            panic!("Failed to create feed");
        }

        let feed = _feed.unwrap();

        println!("uuid: {}", feed.uuid.unwrap());
        println!("checksum: {}", feed.md5_checksum.unwrap());
    }
}