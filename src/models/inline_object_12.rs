/*
 * VRChat API Documentation
 *
 *
 * The version of the OpenAPI document: 1.1.2
 * Contact: me@ruby.js.org
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InlineObject12 {
    #[serde(rename = "moderated")]
    pub moderated: String,
    #[serde(rename = "type")]
    pub _type: crate::models::PlayerModerationType,
}

impl InlineObject12 {
    pub fn new(moderated: String, _type: crate::models::PlayerModerationType) -> InlineObject12 {
        InlineObject12 {
            moderated,
            _type,
        }
    }
}


