//! [GET /_synapse/admin/v1/media/:server_name/:media_id](https://github.com/element-hq/synapse/blob/master/docs/admin_api/media_admin_api.md#query-a-piece-of-media-by-id)

use ruma::{
    OwnedServerName, OwnedUserId, UInt,
    api::{auth_scheme::AccessToken, metadata, request, response},
};
use serde::{Deserialize, Serialize};

metadata! {
    method: GET,
    rate_limited: false,
    authentication: AccessToken,
    path: "/_synapse/admin/v1/media/{server_name}/{media_id}",
}

#[request]
pub struct Request {
    /// The server name of the media to query.
    #[ruma_api(path)]
    pub server_name: OwnedServerName,

    /// The media ID of the media to query.
    #[ruma_api(path)]
    pub media_id: String,
}

#[response]
pub struct Response {
    /// Information about the queried piece of media.
    pub media_info: MediaInfo,
}

impl Request {
    /// Creates a `Request` with the given server name and media ID.
    pub fn new(server_name: OwnedServerName, media_id: String) -> Self {
        Self { server_name, media_id }
    }
}

impl Response {
    /// Creates a `Response` with the given media information.
    pub fn new(media_info: MediaInfo) -> Self {
        Self { media_info }
    }
}

/// Information about a piece of local or cached remote media.
///
/// Every field is always present in the response; the fields that do not apply to the queried
/// media are populated with an explicit `null`. Remote media sets `filesystem_id` and leaves
/// `user_id`, `url_cache` and `safe_from_quarantine` null; local media leaves `media_origin` and
/// `filesystem_id` null.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(not(ruma_unstable_exhaustive_types), non_exhaustive)]
pub struct MediaInfo {
    /// The origin server of the media, for remote media.
    pub media_origin: Option<OwnedServerName>,

    /// The user who uploaded the media, for local media.
    pub user_id: Option<OwnedUserId>,

    /// The media ID.
    pub media_id: String,

    /// The content type of the media.
    pub media_type: String,

    /// The size of the media, in bytes.
    pub media_length: Option<UInt>,

    /// The name the media was uploaded with.
    pub upload_name: Option<String>,

    /// The time the media was created, in milliseconds since the unix epoch.
    pub created_ts: UInt,

    /// The filesystem ID of the media, for remote media.
    pub filesystem_id: Option<String>,

    /// The URL the media was cached from as part of a URL preview, for local media.
    pub url_cache: Option<String>,

    /// The time the media was last accessed, in milliseconds since the unix epoch.
    pub last_access_ts: Option<UInt>,

    /// The user who quarantined the media, if it is quarantined.
    pub quarantined_by: Option<OwnedUserId>,

    /// Whether the media requires authentication to download.
    pub authenticated: Option<bool>,

    /// Whether the media is protected from being quarantined, for local media.
    pub safe_from_quarantine: Option<bool>,

    /// The SHA-256 hash of the media.
    pub sha256: Option<String>,
}

impl MediaInfo {
    /// Creates a `MediaInfo` with the given media ID, content type and creation time and all the
    /// other fields set to `None`.
    pub fn new(media_id: String, media_type: String, created_ts: UInt) -> Self {
        Self {
            media_origin: None,
            user_id: None,
            media_id,
            media_type,
            media_length: None,
            upload_name: None,
            created_ts,
            filesystem_id: None,
            url_cache: None,
            last_access_ts: None,
            quarantined_by: None,
            authenticated: None,
            safe_from_quarantine: None,
            sha256: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{from_value as from_json_value, json, to_value as to_json_value};

    use super::MediaInfo;

    #[test]
    fn deserialize_remote_media_info() {
        let json = json!({
            "media_origin": "remote.example.com",
            "user_id": null,
            "media_id": "abcdefg12345",
            "media_type": "image/png",
            "media_length": 1024,
            "upload_name": "picture.png",
            "created_ts": 1_500_000_000_000_u64,
            "filesystem_id": "fs-id-123",
            "url_cache": null,
            "last_access_ts": 1_500_000_001_000_u64,
            "quarantined_by": null,
            "authenticated": false,
            "safe_from_quarantine": null,
            "sha256": "0123456789abcdef",
        });

        let info: MediaInfo = from_json_value(json.clone()).unwrap();
        assert_eq!(info.media_origin.as_ref().map(|s| s.as_str()), Some("remote.example.com"));
        assert_eq!(info.user_id, None);
        assert_eq!(info.media_id, "abcdefg12345");
        assert_eq!(info.filesystem_id.as_deref(), Some("fs-id-123"));
        assert_eq!(info.url_cache, None);
        assert_eq!(info.safe_from_quarantine, None);
        assert_eq!(info.authenticated, Some(false));

        // The explicit nulls must round-trip back out as keys, not be dropped.
        assert_eq!(to_json_value(&info).unwrap(), json);
    }

    #[test]
    fn deserialize_local_media_info() {
        let json = json!({
            "media_origin": null,
            "user_id": "@alice:example.com",
            "media_id": "localmedia123",
            "media_type": "text/plain",
            "media_length": null,
            "upload_name": null,
            "created_ts": 1_600_000_000_000_u64,
            "filesystem_id": null,
            "url_cache": "https://example.com/preview",
            "last_access_ts": null,
            "quarantined_by": "@admin:example.com",
            "authenticated": null,
            "safe_from_quarantine": true,
            "sha256": null,
        });

        let info: MediaInfo = from_json_value(json.clone()).unwrap();
        assert_eq!(info.media_origin, None);
        assert_eq!(info.user_id.as_ref().map(|u| u.as_str()), Some("@alice:example.com"));
        assert_eq!(info.media_length, None);
        assert_eq!(info.url_cache.as_deref(), Some("https://example.com/preview"));
        assert_eq!(info.safe_from_quarantine, Some(true));

        assert_eq!(to_json_value(&info).unwrap(), json);
    }
}
