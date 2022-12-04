/*
 * VRChat API Documentation
 *
 *
 * Contact: me@ariesclark.com
 * Generated by: https://openapi-generator.tech
 */

/// FileUploadUrl : 



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FileUploadUrl {
    /// 
    #[serde(rename = "url")]
    pub url: String,
}

impl FileUploadUrl {
    /// 
    pub fn new(url: String) -> FileUploadUrl {
        FileUploadUrl {
            url,
        }
    }
}


