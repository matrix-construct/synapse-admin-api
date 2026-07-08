//! [GET /_synapse/admin/v3/users](https://github.com/element-hq/synapse/blob/master/docs/admin_api/user_admin_api.md#list-accounts)

use ruma::{
    UInt,
    api::{Direction, auth_scheme::AccessToken, metadata, request, response},
};

pub use super::v2::{UserMinorDetails, UserSortOrder};

metadata! {
    method: GET,
    rate_limited: false,
    authentication: AccessToken,
    path: "/_synapse/admin/v3/users",
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

    /// The parameter deactivated is a tri-state filter.
    ///
    /// When omitted no filter is applied. When true only deactivated users are returned. When
    /// false deactivated users are excluded.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ruma_api(query)]
    pub deactivated: Option<bool>,

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
