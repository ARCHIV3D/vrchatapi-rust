/*
 * VRChat API Documentation
 *
 *
 * The version of the OpenAPI document: 1.1.2
 * Contact: me@ruby.js.org
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Instance {
    #[serde(rename = "active")]
    pub active: bool,
    #[serde(rename = "canRequestInvite")]
    pub can_request_invite: bool,
    #[serde(rename = "capacity")]
    pub capacity: f32,
    #[serde(rename = "clientNumber")]
    pub client_number: String,
    #[serde(rename = "full")]
    pub full: bool,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "instanceId")]
    pub instance_id: String,
    #[serde(rename = "location")]
    pub location: String,
    #[serde(rename = "n_users")]
    pub n_users: f32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "nonce", skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,
    #[serde(rename = "ownerId")]
    pub owner_id: String,
    #[serde(rename = "permanent")]
    pub permanent: bool,
    #[serde(rename = "photonRegion")]
    pub photon_region: String,
    #[serde(rename = "platforms")]
    pub platforms: Box<crate::models::InstancePlatforms>,
    #[serde(rename = "region")]
    pub region: String,
    #[serde(rename = "shortName")]
    pub short_name: String,
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
    #[serde(rename = "type")]
    pub _type: String,
    /// Always empty on non-existing instances, and non-present on existing instances.
    #[serde(rename = "users", skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<serde_json::Value>>,
    /// Only present on non-existing instances, and only contains a very small subject of World object. Use World API instead.
    #[serde(rename = "world", skip_serializing_if = "Option::is_none")]
    pub world: Option<serde_json::Value>,
    #[serde(rename = "worldId")]
    pub world_id: String,
}

impl Instance {
    pub fn new(active: bool, can_request_invite: bool, capacity: f32, client_number: String, full: bool, id: String, instance_id: String, location: String, n_users: f32, name: String, owner_id: String, permanent: bool, photon_region: String, platforms: crate::models::InstancePlatforms, region: String, short_name: String, tags: Vec<String>, _type: String, world_id: String) -> Instance {
        Instance {
            active,
            can_request_invite,
            capacity,
            client_number,
            full,
            id,
            instance_id,
            location,
            n_users,
            name,
            nonce: None,
            owner_id,
            permanent,
            photon_region,
            platforms: Box::new(platforms),
            region,
            short_name,
            tags,
            _type,
            users: None,
            world: None,
            world_id,
        }
    }
}


