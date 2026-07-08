//! [DELETE /_synapse/admin/v1/media/:server_name/:media_id](https://github.com/element-hq/synapse/blob/master/docs/admin_api/media_admin_api.md#delete-a-specific-local-media)

use ruma::{
    OwnedServerName, UInt,
    api::{auth_scheme::AccessToken, metadata, request, response},
};

metadata! {
    method: DELETE,
    rate_limited: false,
    authentication: AccessToken,
    path: "/_synapse/admin/v1/media/{server_name}/{media_id}",
}

#[request]
pub struct Request {
    /// The server name of the media to delete. Must be the local server.
    #[ruma_api(path)]
    pub server_name: OwnedServerName,

    /// The media ID of the media to delete.
    #[ruma_api(path)]
    pub media_id: String,
}

#[response]
pub struct Response {
    /// The list of deleted media IDs.
    pub deleted_media: Vec<String>,

    /// The total number of deleted media.
    pub total: UInt,
}

impl Request {
    /// Creates a `Request` with the given server name and media ID.
    pub fn new(server_name: OwnedServerName, media_id: String) -> Self {
        Self { server_name, media_id }
    }
}

impl Response {
    /// Creates a `Response` with the given deleted media IDs and total count.
    pub fn new(deleted_media: Vec<String>, total: UInt) -> Self {
        Self { deleted_media, total }
    }
}
