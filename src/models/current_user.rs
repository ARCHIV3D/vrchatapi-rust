/*
 * VRChat API Documentation
 *
 *
 * Contact: me@ruby.js.org
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CurrentUser {
    #[serde(rename = "acceptedTOSVersion")]
    pub accepted_tos_version: i32,
    #[serde(rename = "accountDeletionDate", skip_serializing_if = "Option::is_none")]
    pub account_deletion_date: Option<String>,
    #[serde(rename = "activeFriends", skip_serializing_if = "Option::is_none")]
    pub active_friends: Option<Vec<String>>,
    #[serde(rename = "allowAvatarCopying")]
    pub allow_avatar_copying: bool,
    #[serde(rename = "bio")]
    pub bio: String,
    #[serde(rename = "bioLinks")]
    pub bio_links: Vec<String>,
    #[serde(rename = "currentAvatar")]
    pub current_avatar: String,
    #[serde(rename = "currentAvatarAssetUrl")]
    pub current_avatar_asset_url: String,
    /// When profilePicOverride is not empty, use it instead.
    #[serde(rename = "currentAvatarImageUrl")]
    pub current_avatar_image_url: String,
    /// When profilePicOverride is not empty, use it instead.
    #[serde(rename = "currentAvatarThumbnailImageUrl")]
    pub current_avatar_thumbnail_image_url: String,
    #[serde(rename = "date_joined")]
    pub date_joined: String,
    #[serde(rename = "developerType")]
    pub developer_type: crate::models::DeveloperType,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "emailVerified")]
    pub email_verified: bool,
    #[serde(rename = "fallbackAvatar", skip_serializing_if = "Option::is_none")]
    pub fallback_avatar: Option<String>,
    /// Always empty array.
    #[serde(rename = "friendGroupNames")]
    pub friend_group_names: Vec<String>,
    #[serde(rename = "friendKey")]
    pub friend_key: String,
    #[serde(rename = "friends")]
    pub friends: Vec<String>,
    #[serde(rename = "hasBirthday")]
    pub has_birthday: bool,
    #[serde(rename = "hasEmail")]
    pub has_email: bool,
    #[serde(rename = "hasLoggedInFromClient")]
    pub has_logged_in_from_client: bool,
    #[serde(rename = "hasPendingEmail")]
    pub has_pending_email: bool,
    /// WorldID be \"offline\" on User profiles if you are not friends with that user.
    #[serde(rename = "homeLocation")]
    pub home_location: String,
    /// A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed.
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "isFriend")]
    pub is_friend: bool,
    #[serde(rename = "last_login")]
    pub last_login: String,
    /// This can be `standalonewindows` or `android`, but can also pretty much be any random Unity verison such as `2019.2.4-801-Release` or `2019.2.2-772-Release` or even `unknownplatform`.
    #[serde(rename = "last_platform")]
    pub last_platform: String,
    #[serde(rename = "obfuscatedEmail")]
    pub obfuscated_email: String,
    #[serde(rename = "obfuscatedPendingEmail")]
    pub obfuscated_pending_email: String,
    #[serde(rename = "oculusId")]
    pub oculus_id: String,
    #[serde(rename = "offlineFriends", skip_serializing_if = "Option::is_none")]
    pub offline_friends: Option<Vec<String>>,
    #[serde(rename = "onlineFriends", skip_serializing_if = "Option::is_none")]
    pub online_friends: Option<Vec<String>>,
    #[serde(rename = "pastDisplayNames")]
    pub past_display_names: Vec<crate::models::PastDisplayName>,
    #[serde(rename = "profilePicOverride")]
    pub profile_pic_override: String,
    #[serde(rename = "state")]
    pub state: crate::models::UserState,
    #[serde(rename = "status")]
    pub status: crate::models::UserStatus,
    #[serde(rename = "statusDescription")]
    pub status_description: String,
    #[serde(rename = "statusFirstTime")]
    pub status_first_time: bool,
    #[serde(rename = "statusHistory")]
    pub status_history: Vec<String>,
    #[serde(rename = "steamDetails")]
    pub steam_details: serde_json::Value,
    #[serde(rename = "steamId")]
    pub steam_id: String,
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
    #[serde(rename = "twoFactorAuthEnabled")]
    pub two_factor_auth_enabled: bool,
    #[serde(rename = "twoFactorAuthEnabledDate")]
    pub two_factor_auth_enabled_date: String,
    #[serde(rename = "unsubscribe")]
    pub unsubscribe: bool,
    #[serde(rename = "userIcon")]
    pub user_icon: String,
    #[serde(rename = "username")]
    pub username: String,
}

impl CurrentUser {
    pub fn new(accepted_tos_version: i32, allow_avatar_copying: bool, bio: String, bio_links: Vec<String>, current_avatar: String, current_avatar_asset_url: String, current_avatar_image_url: String, current_avatar_thumbnail_image_url: String, date_joined: String, developer_type: crate::models::DeveloperType, display_name: String, email_verified: bool, friend_group_names: Vec<String>, friend_key: String, friends: Vec<String>, has_birthday: bool, has_email: bool, has_logged_in_from_client: bool, has_pending_email: bool, home_location: String, id: String, is_friend: bool, last_login: String, last_platform: String, obfuscated_email: String, obfuscated_pending_email: String, oculus_id: String, past_display_names: Vec<crate::models::PastDisplayName>, profile_pic_override: String, state: crate::models::UserState, status: crate::models::UserStatus, status_description: String, status_first_time: bool, status_history: Vec<String>, steam_details: serde_json::Value, steam_id: String, tags: Vec<String>, two_factor_auth_enabled: bool, two_factor_auth_enabled_date: String, unsubscribe: bool, user_icon: String, username: String) -> CurrentUser {
        CurrentUser {
            accepted_tos_version,
            account_deletion_date: None,
            active_friends: None,
            allow_avatar_copying,
            bio,
            bio_links,
            current_avatar,
            current_avatar_asset_url,
            current_avatar_image_url,
            current_avatar_thumbnail_image_url,
            date_joined,
            developer_type,
            display_name,
            email_verified,
            fallback_avatar: None,
            friend_group_names,
            friend_key,
            friends,
            has_birthday,
            has_email,
            has_logged_in_from_client,
            has_pending_email,
            home_location,
            id,
            is_friend,
            last_login,
            last_platform,
            obfuscated_email,
            obfuscated_pending_email,
            oculus_id,
            offline_friends: None,
            online_friends: None,
            past_display_names,
            profile_pic_override,
            state,
            status,
            status_description,
            status_first_time,
            status_history,
            steam_details,
            steam_id,
            tags,
            two_factor_auth_enabled,
            two_factor_auth_enabled_date,
            unsubscribe,
            user_icon,
            username,
        }
    }
}


