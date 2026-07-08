//! [GET /_synapse/admin/v1/purge_history_status/:purge_id](https://github.com/element-hq/synapse/blob/master/docs/admin_api/purge_history_api.md#purge-status-query)

use ruma::{
    api::{auth_scheme::AccessToken, metadata, request, response},
    serde::StringEnum,
};

metadata! {
    method: GET,
    rate_limited: false,
    authentication: AccessToken,
    path: "/_synapse/admin/v1/purge_history_status/{purge_id}",
}

#[request]
pub struct Request {
    /// The identifier of the purge task to query.
    #[ruma_api(path)]
    pub purge_id: String,
}

#[response]
pub struct Response {
    /// The stage the purge task has reached.
    pub status: PurgeStatus,

    /// The error message, present only when the task set an error.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl Request {
    /// Creates a `Request` with the given purge ID.
    pub fn new(purge_id: String) -> Self {
        Self { purge_id }
    }
}

impl Response {
    /// Creates a `Response` with the given purge status.
    pub fn new(status: PurgeStatus) -> Self {
        Self { status, error: None }
    }
}

/// The stage reached by a history purge task.
#[derive(Clone, StringEnum)]
#[ruma_enum(rename_all = "snake_case")]
#[non_exhaustive]
pub enum PurgeStatus {
    /// The purge is in progress.
    ///
    /// A scheduled purge that has not yet started is also reported as active.
    Active,

    /// The purge has completed successfully.
    Complete,

    /// The purge has failed.
    Failed,

    #[doc(hidden)]
    _Custom(crate::PrivOwnedStr),
}
