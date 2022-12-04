/*
 * VRChat API Documentation
 *
 *
 * Contact: me@ariesclark.com
 * Generated by: https://openapi-generator.tech
 */

/// InviteMessage : 



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InviteMessage {
    #[serde(rename = "canBeUpdated")]
    pub can_be_updated: bool,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "message")]
    pub message: String,
    #[serde(rename = "messageType")]
    pub message_type: crate::models::InviteMessageType,
    /// Changes to 60 when updated, although probably server-side configurable.
    #[serde(rename = "remainingCooldownMinutes")]
    pub remaining_cooldown_minutes: i32,
    #[serde(rename = "slot")]
    pub slot: i32,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

impl InviteMessage {
    /// 
    pub fn new(can_be_updated: bool, id: String, message: String, message_type: crate::models::InviteMessageType, remaining_cooldown_minutes: i32, slot: i32, updated_at: String) -> InviteMessage {
        InviteMessage {
            can_be_updated,
            id,
            message,
            message_type,
            remaining_cooldown_minutes,
            slot,
            updated_at,
        }
    }
}


