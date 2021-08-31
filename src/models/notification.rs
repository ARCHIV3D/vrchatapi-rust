/*
 * VRChat API Documentation
 *
 *
 * The version of the OpenAPI document: 1.1.2
 * Contact: me@ruby.js.org
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Notification {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "senderUserId")]
    pub sender_user_id: String,
    #[serde(rename = "senderUsername")]
    pub sender_username: String,
    #[serde(rename = "type")]
    pub _type: crate::models::NotificationType,
    #[serde(rename = "message")]
    pub message: String,
    /// **NOTICE:** This is not a JSON object, this is a json **encoded** object, meaning you have to json-de-encode to get the NotificationDetail object depending on the NotificationType.
    #[serde(rename = "details")]
    pub details: String,
    #[serde(rename = "seen")]
    pub seen: bool,
    #[serde(rename = "created_at")]
    pub created_at: String,
}

impl Notification {
    pub fn new(id: String, sender_user_id: String, sender_username: String, _type: crate::models::NotificationType, message: String, details: String, seen: bool, created_at: String) -> Notification {
        Notification {
            id,
            sender_user_id,
            sender_username,
            _type,
            message,
            details,
            seen,
            created_at,
        }
    }
}


