//! [POST /_synapse/admin/v1/media/delete](https://github.com/element-hq/synapse/blob/master/docs/admin_api/media_admin_api.md#delete-local-media-by-date-or-size)

use ruma::{
    UInt,
    api::{auth_scheme::AccessToken, metadata, request, response},
};

metadata! {
    method: POST,
    rate_limited: false,
    authentication: AccessToken,
    path: "/_synapse/admin/v1/media/delete",
}

#[request]
pub struct Request {
    /// Only delete media that was last accessed before this time, in milliseconds since the unix
    /// epoch.
    ///
    /// Media that has never been accessed falls back to its creation time.
    #[ruma_api(query)]
    pub before_ts: UInt,

    /// Only delete media whose size in bytes is strictly greater than this value.
    ///
    /// Defaults to 0, so all media matching the other filters is deleted.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ruma_api(query)]
    pub size_gt: Option<UInt>,

    /// Whether to keep profile and room avatar media.
    ///
    /// Defaults to true.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ruma_api(query)]
    pub keep_profiles: Option<bool>,
}

#[response]
pub struct Response {
    /// The list of deleted media IDs.
    pub deleted_media: Vec<String>,

    /// The total number of deleted media.
    pub total: UInt,
}

impl Request {
    /// Creates a `Request` with the given last-access cutoff and all the other fields at their
    /// default value.
    pub fn new(before_ts: UInt) -> Self {
        Self { before_ts, size_gt: None, keep_profiles: None }
    }
}

impl Response {
    /// Creates a `Response` with the given deleted media IDs and total count.
    pub fn new(deleted_media: Vec<String>, total: UInt) -> Self {
        Self { deleted_media, total }
    }
}
