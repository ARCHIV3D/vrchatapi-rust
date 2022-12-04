/*
 * VRChat API Documentation
 *
 *
 * Contact: me@ariesclark.com
 * Generated by: https://openapi-generator.tech
 */

/// Permission : 



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Permission {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    /// A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed.
    #[serde(rename = "ownerId")]
    pub owner_id: String,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

impl Permission {
    /// 
    pub fn new(id: String, name: String, owner_id: String) -> Permission {
        Permission {
            id,
            name,
            owner_id,
            data: None,
        }
    }
}


