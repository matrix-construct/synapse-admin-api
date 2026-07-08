//! [DELETE /_synapse/admin/v1/users/:user_id/media](https://github.com/element-hq/synapse/blob/master/docs/admin_api/user_admin_api.md#delete-media-uploaded-by-a-user)

use ruma::{
    OwnedUserId, UInt,
    api::{Direction, auth_scheme::AccessToken, metadata, request, response},
};

pub use crate::media::list_user_media::v1::MediaSortOrder;

metadata! {
    method: DELETE,
    rate_limited: false,
    authentication: AccessToken,
    path: "/_synapse/admin/v1/users/{user_id}/media",
}

#[request]
pub struct Request {
    /// The user whose media to delete. Must be a local user.
    #[ruma_api(path)]
    pub user_id: OwnedUserId,

    /// Offset in the selection to delete. Defaults to 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ruma_api(query)]
    pub from: Option<UInt>,

    /// Maximum amount of media to delete in this call. Defaults to 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ruma_api(query)]
    pub limit: Option<UInt>,

    /// The field to sort the media by before selecting which to delete.
    ///
    /// When neither `order_by` nor `dir` is set, media is selected newest first for backwards
    /// compatibility.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ruma_api(query)]
    pub order_by: Option<MediaSortOrder>,

    /// The direction to sort the media in before selecting which to delete.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ruma_api(query)]
    pub dir: Option<Direction>,
}

#[response]
pub struct Response {
    /// The list of deleted media IDs.
    pub deleted_media: Vec<String>,

    /// The total number of deleted media.
    pub total: UInt,
}

impl Request {
    /// Creates a `Request` with the given user ID and all the other fields at their default value.
    pub fn new(user_id: OwnedUserId) -> Self {
        Self { user_id, from: None, limit: None, order_by: None, dir: None }
    }
}

impl Response {
    /// Creates a `Response` with the given deleted media IDs and total count.
    pub fn new(deleted_media: Vec<String>, total: UInt) -> Self {
        Self { deleted_media, total }
    }
}
