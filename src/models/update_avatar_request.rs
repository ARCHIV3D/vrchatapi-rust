/*
 * VRChat API Documentation
 *
 *
 * Contact: me@ariesclark.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UpdateAvatarRequest {
    #[serde(rename = "assetUrl", skip_serializing_if = "Option::is_none")]
    pub asset_url: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///  
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(rename = "imageUrl", skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(rename = "releaseStatus", skip_serializing_if = "Option::is_none")]
    pub release_status: Option<crate::models::ReleaseStatus>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<f32>,
    #[serde(rename = "unityPackageUrl", skip_serializing_if = "Option::is_none")]
    pub unity_package_url: Option<String>,
}

impl UpdateAvatarRequest {
    pub fn new() -> UpdateAvatarRequest {
        UpdateAvatarRequest {
            asset_url: None,
            id: None,
            name: None,
            description: None,
            tags: None,
            image_url: None,
            release_status: None,
            version: None,
            unity_package_url: None,
        }
    }
}


