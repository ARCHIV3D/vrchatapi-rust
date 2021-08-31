/*
 * VRChat API Documentation
 *
 *
 * The version of the OpenAPI document: 1.1.2
 * Contact: me@ruby.js.org
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InlineObject13 {
    #[serde(rename = "moderated", skip_serializing_if = "Option::is_none")]
    pub moderated: Option<String>,
    #[serde(rename = "type")]
    pub _type: crate::models::PlayerModerationType,
}

impl InlineObject13 {
    pub fn new(_type: crate::models::PlayerModerationType) -> InlineObject13 {
        InlineObject13 {
            moderated: None,
            _type,
        }
    }
}


