//! [GET /_synapse/admin/v1/statistics/users/media](https://github.com/element-hq/synapse/blob/master/docs/admin_api/statistics.md#users-media-usage-statistics)

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
    path: "/_synapse/admin/v1/statistics/users/media",
}

#[request]
#[derive(Default)]
pub struct Request {
    /// Offset in the returned list. Defaults to 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ruma_api(query)]
    pub from: Option<UInt>,

    /// Maximum amount of users to return. Defaults to 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ruma_api(query)]
    pub limit: Option<UInt>,

    /// The field to sort the returned users by. Defaults to `user_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ruma_api(query)]
    pub order_by: Option<UserMediaSortOrder>,

    /// Only count media created after this time, in milliseconds since the unix epoch.
    ///
    /// Defaults to 0, which applies no lower bound.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ruma_api(query)]
    pub from_ts: Option<UInt>,

    /// Only count media created before this time, in milliseconds since the unix epoch.
    ///
    /// Must be greater than `from_ts`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ruma_api(query)]
    pub until_ts: Option<UInt>,

    /// Filter to users whose user ID localpart or display name contains this value.
    ///
    /// Must not be the empty string.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ruma_api(query)]
    pub search_term: Option<String>,

    /// The direction to sort the returned users in.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ruma_api(query)]
    pub dir: Option<Direction>,
}

#[response]
pub struct Response {
    /// A list of users with their media usage.
    pub users: Vec<UserMediaStat>,

    /// Token to receive the next batch of users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<UInt>,

    /// The total number of users with media usage matching the query.
    pub total: UInt,
}

impl Request {
    /// Creates an empty `Request`.
    pub fn new() -> Self {
        Default::default()
    }
}

impl Response {
    /// Creates a `Response` with the given users and total count.
    pub fn new(users: Vec<UserMediaStat>, total: UInt) -> Self {
        Self { users, next_token: None, total }
    }
}

/// The field to sort users' media statistics by.
#[derive(Clone, StringEnum)]
#[ruma_enum(rename_all = "snake_case")]
#[non_exhaustive]
pub enum UserMediaSortOrder {
    /// Sort by the total size of uploaded media.
    MediaLength,

    /// Sort by the number of uploaded media.
    MediaCount,

    /// Sort alphabetically by user ID.
    UserId,

    /// Sort alphabetically by display name.
    Displayname,

    #[doc(hidden)]
    _Custom(crate::PrivOwnedStr),
}

/// A user's media usage statistics.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(not(ruma_unstable_exhaustive_types), non_exhaustive)]
pub struct UserMediaStat {
    /// The user ID.
    pub user_id: OwnedUserId,

    /// The user's display name, if set.
    pub displayname: Option<String>,

    /// The number of media the user has uploaded.
    pub media_count: UInt,

    /// The total size of the user's uploaded media, in bytes.
    pub media_length: UInt,
}

impl UserMediaStat {
    /// Creates a `UserMediaStat` with the given user ID, media count and total media size.
    pub fn new(user_id: OwnedUserId, media_count: UInt, media_length: UInt) -> Self {
        Self { user_id, displayname: None, media_count, media_length }
    }
}

#[cfg(test)]
mod tests {
    use super::UserMediaSortOrder;

    #[test]
    fn user_media_sort_order_serialization() {
        assert_eq!(UserMediaSortOrder::MediaLength.as_ref(), "media_length");
        assert_eq!(UserMediaSortOrder::MediaCount.as_ref(), "media_count");
        assert_eq!(UserMediaSortOrder::UserId.as_ref(), "user_id");
        assert_eq!(UserMediaSortOrder::Displayname.as_ref(), "displayname");
    }

    #[test]
    fn user_media_sort_order_deserialization() {
        assert_eq!(UserMediaSortOrder::from("media_count").as_ref(), "media_count");
        assert_eq!(UserMediaSortOrder::from("displayname").as_ref(), "displayname");
    }
}
