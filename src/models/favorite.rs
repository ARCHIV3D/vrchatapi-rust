/*
 * VRChat API Documentation
 *
 *
 * The version of the OpenAPI document: 1.1.2
 * Contact: me@ruby.js.org
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Favorite {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "type")]
    pub _type: crate::models::FavoriteType,
    /// MUST be either AvatarID, UserID or WorldID.
    #[serde(rename = "favoriteId")]
    pub favorite_id: String,
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
}

impl Favorite {
    pub fn new(id: String, _type: crate::models::FavoriteType, favorite_id: String, tags: Vec<String>) -> Favorite {
        Favorite {
            id,
            _type,
            favorite_id,
            tags,
        }
    }
}


