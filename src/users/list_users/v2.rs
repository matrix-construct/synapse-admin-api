//! [GET /_synapse/admin/v2/users](https://github.com/element-hq/synapse/blob/master/docs/admin_api/user_admin_api.md#list-accounts)

use ruma::{
    MilliSecondsSinceUnixEpoch, UInt,
    api::{Direction, auth_scheme::AccessToken, metadata, request, response},
    serde::StringEnum,
};
use serde::{Deserialize, Serialize};

metadata! {
    method: GET,
    rate_limited: false,
    authentication: AccessToken,
    path: "/_synapse/admin/v2/users",
}

#[request]
#[derive(Default)]
pub struct Request {
    /// Offset in the returned list.
    ///
    /// Defaults to 0.
    #[serde(default, skip_serializing_if = "ruma::serde::is_default")]
    #[ruma_api(query)]
    pub from: UInt,

    /// Maximum amount of users to return. Defaults to 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ruma_api(query)]
    pub limit: Option<UInt>,

    /// user_id is optional and filters to only return users with user IDs that contain this value.
    ///
    /// This parameter is ignored when using the name parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ruma_api(query)]
    pub user_id: Option<String>,

    /// name is optional and filters to only return users with user ID localparts or displaynames
    /// that contain this value.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ruma_api(query)]
    pub name: Option<String>,

    /// The parameter guests is optional and if false will exclude guest users.
    ///
    /// Defaults to true to include guest users.
    #[serde(default = "ruma::serde::default_true", skip_serializing_if = "ruma::serde::is_true")]
    #[ruma_api(query)]
    pub guests: bool,

    /// The parameter deactivated is optional and if true will include deactivated users.
    ///
    /// Defaults to false to exclude deactivated users.
    #[serde(default, skip_serializing_if = "ruma::serde::is_default")]
    #[ruma_api(query)]
    pub deactivated: bool,

    /// The parameter admins is optional and, if set, filters to only return admins (true) or
    /// non-admins (false). When omitted both are returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ruma_api(query)]
    pub admins: Option<bool>,

    /// Whether to include locked users in the response.
    ///
    /// Defaults to false to exclude locked users.
    #[serde(default, skip_serializing_if = "ruma::serde::is_default")]
    #[ruma_api(query)]
    pub locked: bool,

    /// The method by which to sort the returned list of users.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ruma_api(query)]
    pub order_by: Option<UserSortOrder>,

    /// Direction of the sort applied to the returned list of users.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ruma_api(query)]
    pub dir: Option<Direction>,

    /// Filters out users of the given Synapse user types. Can be specified more than once.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[ruma_api(query)]
    pub not_user_type: Vec<String>,

    /// The parameter approved is optional and, if set, filters by the account approval flag. Only
    /// parsed when MSC3866 support is enabled on the server.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ruma_api(query)]
    pub approved: Option<bool>,
}

#[response]
pub struct Response {
    /// List of users containing `UserMinorDetails`.
    pub users: Vec<UserMinorDetails>,

    /// Token to receive the next `UserMinorDetails` batch.
    ///
    /// To paginate, check for next_token and if present, call the endpoint again with from set
    /// to the value of next_token. This will return a new page. If the endpoint does not return
    /// a next_token then there are no more users to paginate through.
    pub next_token: Option<String>,

    /// Total amount of users.
    pub total: UInt,
}

impl Request {
    /// Creates an empty `Request`.
    pub fn new() -> Self {
        Default::default()
    }
}

impl Response {
    /// Creates a `Response` with the given `UserMinorDetails` and the total amount of users.
    pub fn new(users: Vec<UserMinorDetails>, total: UInt) -> Self {
        Self { users, next_token: None, total }
    }
}

/// The method by which to sort a list of user accounts.
#[derive(Clone, StringEnum)]
#[ruma_enum(rename_all = "snake_case")]
#[non_exhaustive]
pub enum UserSortOrder {
    /// Sort by user ID.
    Name,

    /// Sort by display name.
    Displayname,

    /// Sort by whether the account is a guest.
    IsGuest,

    /// Sort by whether the account is a server admin.
    Admin,

    /// Sort by whether the account is deactivated.
    Deactivated,

    /// Sort by Synapse user type.
    UserType,

    /// Sort by avatar URL.
    AvatarUrl,

    /// Sort by whether the account is shadow banned.
    ShadowBanned,

    /// Sort by account creation timestamp.
    CreationTs,

    /// Sort by the timestamp the account was last seen.
    LastSeenTs,

    /// Sort by whether the account is locked.
    Locked,

    #[doc(hidden)]
    _Custom(crate::PrivOwnedStr),
}

/// A minor set of user details.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(not(ruma_unstable_exhaustive_types), non_exhaustive)]
pub struct UserMinorDetails {
    /// The user's name.
    pub name: String,

    /// The Synapse user type of the account (e.g. `support`, `bot`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_type: Option<String>,

    /// Is the account a guest
    #[serde(deserialize_with = "crate::serde::bool_or_uint")]
    pub is_guest: bool,

    /// Is the user a server admin
    #[serde(deserialize_with = "crate::serde::bool_or_uint")]
    pub admin: bool,

    /// Is the account deactivated
    #[serde(deserialize_with = "crate::serde::bool_or_uint")]
    pub deactivated: bool,

    /// Is the account shadow banned
    #[serde(default, deserialize_with = "crate::serde::bool_or_uint")]
    pub shadow_banned: bool,

    /// The user's display name, if set.
    pub displayname: Option<String>,

    /// The user's avatar URL, if set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,

    /// Creation date for the account, in milliseconds.
    pub creation_ts: Option<MilliSecondsSinceUnixEpoch>,

    /// Whether the account has been erased following deactivation.
    #[serde(default, deserialize_with = "crate::serde::bool_or_uint")]
    pub erased: bool,

    /// The time the user was last seen, in milliseconds.
    pub last_seen_ts: Option<MilliSecondsSinceUnixEpoch>,

    /// Whether the account is locked.
    #[serde(default, deserialize_with = "crate::serde::bool_or_uint")]
    pub locked: bool,

    /// Whether the account has been approved, present only when MSC3866 is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved: Option<bool>,
}

impl UserMinorDetails {
    /// Construct a `UserMinorDetails` with the given user name and all the other fields set to
    /// their default value.
    pub fn new(name: String) -> Self {
        Self {
            name,
            user_type: None,
            is_guest: false,
            admin: false,
            deactivated: false,
            shadow_banned: false,
            displayname: None,
            avatar_url: None,
            creation_ts: None,
            erased: false,
            last_seen_ts: None,
            locked: false,
            approved: None,
        }
    }
}
