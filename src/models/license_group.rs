/*
 * VRChat API Documentation
 *
 *
 * Contact: me@ariesclark.com
 * Generated by: https://openapi-generator.tech
 */

/// LicenseGroup : 



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LicenseGroup {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "licenses")]
    pub licenses: Vec<crate::models::License>,
}

impl LicenseGroup {
    /// 
    pub fn new(id: String, name: String, description: String, licenses: Vec<crate::models::License>) -> LicenseGroup {
        LicenseGroup {
            id,
            name,
            description,
            licenses,
        }
    }
}


