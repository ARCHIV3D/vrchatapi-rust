/*
 * VRChat API Documentation
 *
 *
 * The version of the OpenAPI document: 1.1.2
 * Contact: me@ruby.js.org
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    /// VRChat's office address
    #[serde(rename = "address")]
    pub address: String,
    /// Public Announcements
    #[serde(rename = "announcements")]
    pub announcements: Vec<crate::models::ConfigAnnouncements>,
    /// apiKey to be used for all other requests
    #[serde(rename = "apiKey")]
    pub api_key: String,
    /// Game name
    #[serde(rename = "appName")]
    pub app_name: String,
    /// Build tag of the API server
    #[serde(rename = "buildVersionTag")]
    pub build_version_tag: String,
    /// apiKey to be used for all other requests
    #[serde(rename = "clientApiKey")]
    pub client_api_key: String,
    /// Unknown
    #[serde(rename = "clientBPSCeiling", skip_serializing_if = "Option::is_none")]
    pub client_bps_ceiling: Option<f32>,
    /// Unknown
    #[serde(rename = "clientDisconnectTimeout", skip_serializing_if = "Option::is_none")]
    pub client_disconnect_timeout: Option<f32>,
    /// Unknown
    #[serde(rename = "clientReservedPlayerBPS", skip_serializing_if = "Option::is_none")]
    pub client_reserved_player_bps: Option<f32>,
    /// Unknown
    #[serde(rename = "clientSentCountAllowance", skip_serializing_if = "Option::is_none")]
    pub client_sent_count_allowance: Option<f32>,
    /// VRChat's contact email
    #[serde(rename = "contactEmail")]
    pub contact_email: String,
    /// VRChat's copyright-issues-related email
    #[serde(rename = "copyrightEmail")]
    pub copyright_email: String,
    /// Current version number of the Terms of Service
    #[serde(rename = "currentTOSVersion")]
    pub current_tos_version: f32,
    #[serde(rename = "defaultAvatar")]
    pub default_avatar: String,
    #[serde(rename = "deploymentGroup")]
    pub deployment_group: crate::models::DeploymentGroup,
    /// Version number for game development build
    #[serde(rename = "devAppVersionStandalone")]
    pub dev_app_version_standalone: String,
    /// Developer Download link
    #[serde(rename = "devDownloadLinkWindows")]
    pub dev_download_link_windows: String,
    /// Link to download the development SDK, use downloadUrls instead
    #[serde(rename = "devSdkUrl")]
    pub dev_sdk_url: String,
    /// Version of the development SDK
    #[serde(rename = "devSdkVersion")]
    pub dev_sdk_version: String,
    /// Version number for server development build
    #[serde(rename = "devServerVersionStandalone")]
    pub dev_server_version_standalone: String,
    /// Toggles if copying avatars should be disabled
    #[serde(rename = "disableAvatarCopying")]
    pub disable_avatar_copying: bool,
    /// Toggles if avatar gating should be disabled. Avatar gating restricts uploading of avatars to people with the `system_avatar_access` Tag or `admin_avatar_access` Tag
    #[serde(rename = "disableAvatarGating")]
    pub disable_avatar_gating: bool,
    /// Toggles if the Community Labs should be disabled
    #[serde(rename = "disableCommunityLabs")]
    pub disable_community_labs: bool,
    /// Toggles if promotion out of Community Labs should be disabled
    #[serde(rename = "disableCommunityLabsPromotion")]
    pub disable_community_labs_promotion: bool,
    /// Unknown
    #[serde(rename = "disableEmail", skip_serializing_if = "Option::is_none")]
    pub disable_email: Option<bool>,
    /// Toggles if Analytics should be disabled (this sreportedly not used in the Client)
    #[serde(rename = "disableEventStream")]
    pub disable_event_stream: bool,
    /// Toggles if feedback gating should be disabled. Feedback gating restricts submission of feedback (reporting a World or User) to people with the `system_feedback_access` Tag.
    #[serde(rename = "disableFeedbackGating")]
    pub disable_feedback_gating: bool,
    /// Unknown
    #[serde(rename = "disableHello", skip_serializing_if = "Option::is_none")]
    pub disable_hello: Option<bool>,
    /// Toggles if new user account registration should be disabled
    #[serde(rename = "disableRegistration")]
    pub disable_registration: bool,
    /// Toggles if Steam Networking should be disabled. VRChat these days uses Photon Unity Networking (PUN) instead.
    #[serde(rename = "disableSteamNetworking")]
    pub disable_steam_networking: bool,
    /// Toggles if 2FA should be disabled.
    #[serde(rename = "disableTwoFactorAuth")]
    pub disable_two_factor_auth: bool,
    /// Toggles if Udon should be universally disabled in-game.
    #[serde(rename = "disableUdon")]
    pub disable_udon: bool,
    /// Toggles if account upgrading \"linking with Steam/Oculus\" should be disabled.
    #[serde(rename = "disableUpgradeAccount")]
    pub disable_upgrade_account: bool,
    /// Download link for game on the Oculus Rift website.
    #[serde(rename = "downloadLinkWindows")]
    pub download_link_windows: String,
    #[serde(rename = "downloadUrls")]
    pub download_urls: Box<crate::models::ConfigDownloadUrls>,
    /// Array of DynamicWorldRow objects, used by the game to display the list of world rows
    #[serde(rename = "dynamicWorldRows")]
    pub dynamic_world_rows: Vec<crate::models::ConfigDynamicWorldRows>,
    #[serde(rename = "events")]
    pub events: Box<crate::models::ConfigEvents>,
    /// Unknown
    #[serde(rename = "gearDemoRoomId")]
    pub gear_demo_room_id: String,
    /// Redirect target if you try to open the base API domain in your browser
    #[serde(rename = "homepageRedirectTarget")]
    pub homepage_redirect_target: String,
    #[serde(rename = "homeWorldId")]
    pub home_world_id: String,
    #[serde(rename = "hubWorldId")]
    pub hub_world_id: String,
    /// VRChat's job application email
    #[serde(rename = "jobsEmail")]
    pub jobs_email: String,
    /// MOTD
    #[serde(rename = "messageOfTheDay")]
    pub message_of_the_day: String,
    /// VRChat's moderation related email
    #[serde(rename = "moderationEmail")]
    pub moderation_email: String,
    /// Unknown
    #[serde(rename = "moderationQueryPeriod")]
    pub moderation_query_period: f32,
    /// Used in-game to notify a user they aren't allowed to select avatars in private worlds
    #[serde(rename = "notAllowedToSelectAvatarInPrivateWorldMessage")]
    pub not_allowed_to_select_avatar_in_private_world_message: String,
    /// Extra [plugin](https://doc.photonengine.com/en-us/server/current/plugins/manual) to run in each instance
    #[serde(rename = "plugin")]
    pub plugin: String,
    /// Version number for game release build
    #[serde(rename = "releaseAppVersionStandalone")]
    pub release_app_version_standalone: String,
    /// Link to download the release SDK
    #[serde(rename = "releaseSdkUrl")]
    pub release_sdk_url: String,
    /// Version of the release SDK
    #[serde(rename = "releaseSdkVersion")]
    pub release_sdk_version: String,
    /// Version number for server release build
    #[serde(rename = "releaseServerVersionStandalone")]
    pub release_server_version_standalone: String,
    /// Link to the developer FAQ
    #[serde(rename = "sdkDeveloperFaqUrl")]
    pub sdk_developer_faq_url: String,
    /// Link to the official VRChat Discord
    #[serde(rename = "sdkDiscordUrl")]
    pub sdk_discord_url: String,
    /// Used in the SDK to notify a user they aren't allowed to upload avatars/worlds yet
    #[serde(rename = "sdkNotAllowedToPublishMessage")]
    pub sdk_not_allowed_to_publish_message: String,
    /// Unity version supported by the SDK
    #[serde(rename = "sdkUnityVersion")]
    pub sdk_unity_version: String,
    /// Server name of the API server currently responding
    #[serde(rename = "serverName")]
    pub server_name: String,
    /// VRChat's support email
    #[serde(rename = "supportEmail")]
    pub support_email: String,
    #[serde(rename = "timeOutWorldId")]
    pub time_out_world_id: String,
    #[serde(rename = "tutorialWorldId")]
    pub tutorial_world_id: String,
    /// Unknown
    #[serde(rename = "updateRateMsMaximum")]
    pub update_rate_ms_maximum: f32,
    /// Unknown
    #[serde(rename = "updateRateMsMinimum")]
    pub update_rate_ms_minimum: f32,
    /// Unknown
    #[serde(rename = "updateRateMsNormal")]
    pub update_rate_ms_normal: f32,
    /// Unknown
    #[serde(rename = "updateRateMsUdonManual")]
    pub update_rate_ms_udon_manual: f32,
    /// Unknown
    #[serde(rename = "uploadAnalysisPercent")]
    pub upload_analysis_percent: f32,
    /// List of allowed URLs that bypass the \"Allow untrusted URL's\" setting in-game
    #[serde(rename = "urlList")]
    pub url_list: Vec<String>,
    /// Unknown
    #[serde(rename = "useReliableUdpForVoice")]
    pub use_reliable_udp_for_voice: bool,
    /// Unknown
    #[serde(rename = "userUpdatePeriod")]
    pub user_update_period: f32,
    /// Unknown
    #[serde(rename = "userVerificationDelay")]
    pub user_verification_delay: f32,
    /// Unknown
    #[serde(rename = "userVerificationRetry")]
    pub user_verification_retry: f32,
    /// Unknown
    #[serde(rename = "userVerificationTimeout")]
    pub user_verification_timeout: f32,
    /// Download link for game on the Steam website.
    #[serde(rename = "viveWindowsUrl")]
    pub vive_windows_url: String,
    /// List of allowed URLs that are allowed to host avatar assets
    #[serde(rename = "whiteListedAssetUrls")]
    pub white_listed_asset_urls: Vec<String>,
    /// Unknown
    #[serde(rename = "worldUpdatePeriod")]
    pub world_update_period: f32,
    /// Currently used youtube-dl.exe hash in SHA-256-delimited format
    #[serde(rename = "youtubedl-hash")]
    pub youtubedl_hash: String,
    /// Currently used youtube-dl.exe version
    #[serde(rename = "youtubedl-version")]
    pub youtubedl_version: String,
}

impl Config {
    pub fn new(address: String, announcements: Vec<crate::models::ConfigAnnouncements>, api_key: String, app_name: String, build_version_tag: String, client_api_key: String, contact_email: String, copyright_email: String, current_tos_version: f32, default_avatar: String, deployment_group: crate::models::DeploymentGroup, dev_app_version_standalone: String, dev_download_link_windows: String, dev_sdk_url: String, dev_sdk_version: String, dev_server_version_standalone: String, disable_avatar_copying: bool, disable_avatar_gating: bool, disable_community_labs: bool, disable_community_labs_promotion: bool, disable_event_stream: bool, disable_feedback_gating: bool, disable_registration: bool, disable_steam_networking: bool, disable_two_factor_auth: bool, disable_udon: bool, disable_upgrade_account: bool, download_link_windows: String, download_urls: crate::models::ConfigDownloadUrls, dynamic_world_rows: Vec<crate::models::ConfigDynamicWorldRows>, events: crate::models::ConfigEvents, gear_demo_room_id: String, homepage_redirect_target: String, home_world_id: String, hub_world_id: String, jobs_email: String, message_of_the_day: String, moderation_email: String, moderation_query_period: f32, not_allowed_to_select_avatar_in_private_world_message: String, plugin: String, release_app_version_standalone: String, release_sdk_url: String, release_sdk_version: String, release_server_version_standalone: String, sdk_developer_faq_url: String, sdk_discord_url: String, sdk_not_allowed_to_publish_message: String, sdk_unity_version: String, server_name: String, support_email: String, time_out_world_id: String, tutorial_world_id: String, update_rate_ms_maximum: f32, update_rate_ms_minimum: f32, update_rate_ms_normal: f32, update_rate_ms_udon_manual: f32, upload_analysis_percent: f32, url_list: Vec<String>, use_reliable_udp_for_voice: bool, user_update_period: f32, user_verification_delay: f32, user_verification_retry: f32, user_verification_timeout: f32, vive_windows_url: String, white_listed_asset_urls: Vec<String>, world_update_period: f32, youtubedl_hash: String, youtubedl_version: String) -> Config {
        Config {
            address,
            announcements,
            api_key,
            app_name,
            build_version_tag,
            client_api_key,
            client_bps_ceiling: None,
            client_disconnect_timeout: None,
            client_reserved_player_bps: None,
            client_sent_count_allowance: None,
            contact_email,
            copyright_email,
            current_tos_version,
            default_avatar,
            deployment_group,
            dev_app_version_standalone,
            dev_download_link_windows,
            dev_sdk_url,
            dev_sdk_version,
            dev_server_version_standalone,
            disable_avatar_copying,
            disable_avatar_gating,
            disable_community_labs,
            disable_community_labs_promotion,
            disable_email: None,
            disable_event_stream,
            disable_feedback_gating,
            disable_hello: None,
            disable_registration,
            disable_steam_networking,
            disable_two_factor_auth,
            disable_udon,
            disable_upgrade_account,
            download_link_windows,
            download_urls: Box::new(download_urls),
            dynamic_world_rows,
            events: Box::new(events),
            gear_demo_room_id,
            homepage_redirect_target,
            home_world_id,
            hub_world_id,
            jobs_email,
            message_of_the_day,
            moderation_email,
            moderation_query_period,
            not_allowed_to_select_avatar_in_private_world_message,
            plugin,
            release_app_version_standalone,
            release_sdk_url,
            release_sdk_version,
            release_server_version_standalone,
            sdk_developer_faq_url,
            sdk_discord_url,
            sdk_not_allowed_to_publish_message,
            sdk_unity_version,
            server_name,
            support_email,
            time_out_world_id,
            tutorial_world_id,
            update_rate_ms_maximum,
            update_rate_ms_minimum,
            update_rate_ms_normal,
            update_rate_ms_udon_manual,
            upload_analysis_percent,
            url_list,
            use_reliable_udp_for_voice,
            user_update_period,
            user_verification_delay,
            user_verification_retry,
            user_verification_timeout,
            vive_windows_url,
            white_listed_asset_urls,
            world_update_period,
            youtubedl_hash,
            youtubedl_version,
        }
    }
}


