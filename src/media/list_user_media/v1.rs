//! [GET /_synapse/admin/v1/users/:user_id/media](https://github.com/element-hq/synapse/blob/master/docs/admin_api/user_admin_api.md#list-media-uploaded-by-a-user)

use ruma::{
    OwnedUserId, UInt,
    api::{Direction, auth_scheme::AccessToken, metadata, request, response},
    serde::StringEnum,
};
use serde::{Deserialize, Serialize};

metadata! {
    method: GET,
    rate_limited: false,
    authentication: AccessToken,
    path: "/_synapse/admin/v1/users/{user_id}/media",
}

#[request]
pub struct Request {
    /// The user whose media to list. Must be a local user.
    #[ruma_api(path)]
    pub user_id: OwnedUserId,

    /// Offset in the returned list. Defaults to 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ruma_api(query)]
    pub from: Option<UInt>,

    /// Maximum amount of media to return. Defaults to 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ruma_api(query)]
    pub limit: Option<UInt>,

    /// The field to sort the returned media by.
    ///
    /// When neither `order_by` nor `dir` is set, media is returned newest first for backwards
    /// compatibility.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ruma_api(query)]
    pub order_by: Option<MediaSortOrder>,

    /// The direction to sort the returned media in.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ruma_api(query)]
    pub dir: Option<Direction>,
}

#[response]
pub struct Response {
    /// A list of media uploaded by the user.
    pub media: Vec<UserMedia>,

    /// Token to receive the next batch of media.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<UInt>,

    /// The total number of media uploaded by the user.
    pub total: UInt,
}

impl Request {
    /// Creates a `Request` with the given user ID and all the other fields at their default value.
    pub fn new(user_id: OwnedUserId) -> Self {
        Self { user_id, from: None, limit: None, order_by: None, dir: None }
    }
}

impl Response {
    /// Creates a `Response` with the given media and total count.
    pub fn new(media: Vec<UserMedia>, total: UInt) -> Self {
        Self { media, next_token: None, total }
    }
}

/// The field to sort a user's media by.
#[derive(Clone, StringEnum)]
#[ruma_enum(rename_all = "snake_case")]
#[non_exhaustive]
pub enum MediaSortOrder {
    /// Sort by the media ID.
    MediaId,

    /// Sort by the upload name.
    UploadName,

    /// Sort by the creation time.
    CreatedTs,

    /// Sort by the last-access time.
    LastAccessTs,

    /// Sort by the media size.
    MediaLength,

    /// Sort by the content type.
    MediaType,

    /// Sort by the quarantining admin.
    QuarantinedBy,

    /// Sort by whether the media is protected from quarantine.
    SafeFromQuarantine,

    #[doc(hidden)]
    _Custom(crate::PrivOwnedStr),
}

/// A single piece of media uploaded by a user.
///
/// Every field is always present in the response; the nullable fields are populated with an
/// explicit `null` when they do not apply.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(not(ruma_unstable_exhaustive_types), non_exhaustive)]
pub struct UserMedia {
    /// The media ID.
    pub media_id: String,

    /// The content type of the media.
    pub media_type: String,

    /// The size of the media, in bytes.
    pub media_length: Option<UInt>,

    /// The name the media was uploaded with.
    pub upload_name: String,

    /// The time the media was created, in milliseconds since the unix epoch.
    pub created_ts: UInt,

    /// The URL the media was cached from as part of a URL preview.
    pub url_cache: Option<String>,

    /// The time the media was last accessed, in milliseconds since the unix epoch.
    pub last_access_ts: UInt,

    /// The user who quarantined the media, if it is quarantined.
    pub quarantined_by: Option<OwnedUserId>,

    /// Whether the media is protected from being quarantined.
    pub safe_from_quarantine: bool,

    /// The user who uploaded the media.
    pub user_id: Option<OwnedUserId>,

    /// Whether the media requires authentication to download.
    pub authenticated: Option<bool>,

    /// The SHA-256 hash of the media.
    pub sha256: Option<String>,
}

impl UserMedia {
    /// Creates a `UserMedia` with the given required fields and all the other fields set to their
    /// default value.
    pub fn new(
        media_id: String,
        media_type: String,
        upload_name: String,
        created_ts: UInt,
        last_access_ts: UInt,
    ) -> Self {
        Self {
            media_id,
            media_type,
            media_length: None,
            upload_name,
            created_ts,
            url_cache: None,
            last_access_ts,
            quarantined_by: None,
            safe_from_quarantine: false,
            user_id: None,
            authenticated: None,
            sha256: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MediaSortOrder;

    #[test]
    fn media_sort_order_serialization() {
        assert_eq!(MediaSortOrder::MediaId.as_ref(), "media_id");
        assert_eq!(MediaSortOrder::UploadName.as_ref(), "upload_name");
        assert_eq!(MediaSortOrder::CreatedTs.as_ref(), "created_ts");
        assert_eq!(MediaSortOrder::LastAccessTs.as_ref(), "last_access_ts");
        assert_eq!(MediaSortOrder::MediaLength.as_ref(), "media_length");
        assert_eq!(MediaSortOrder::MediaType.as_ref(), "media_type");
        assert_eq!(MediaSortOrder::QuarantinedBy.as_ref(), "quarantined_by");
        assert_eq!(MediaSortOrder::SafeFromQuarantine.as_ref(), "safe_from_quarantine");
    }

    #[test]
    fn media_sort_order_deserialization() {
        assert_eq!(MediaSortOrder::from("created_ts").as_ref(), "created_ts");
        assert_eq!(MediaSortOrder::from("safe_from_quarantine").as_ref(), "safe_from_quarantine");
    }
}
