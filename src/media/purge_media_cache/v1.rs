//! [POST /_synapse/admin/v1/purge_media_cache](https://github.com/element-hq/synapse/blob/master/docs/admin_api/media_admin_api.md#purge-remote-media-api)

use ruma::{
    UInt,
    api::{auth_scheme::AccessToken, metadata, request, response},
};

metadata! {
    method: POST,
    rate_limited: false,
    authentication: AccessToken,
    path: "/_synapse/admin/v1/purge_media_cache",
}

#[request]
pub struct Request {
    /// Only purge cached remote media that was last accessed before this time, in milliseconds
    /// since the unix epoch.
    #[ruma_api(query)]
    pub before_ts: UInt,
}

#[response]
pub struct Response {
    /// The number of purged media.
    pub deleted: UInt,
}

impl Request {
    /// Creates a `Request` with the given last-access cutoff.
    pub fn new(before_ts: UInt) -> Self {
        Self { before_ts }
    }
}

impl Response {
    /// Creates a `Response` with the given number of purged media.
    pub fn new(deleted: UInt) -> Self {
        Self { deleted }
    }
}
