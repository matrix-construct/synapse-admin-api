//! [GET /_synapse/admin/v1/user/redact_status/:redact_id](https://github.com/element-hq/synapse/blob/master/docs/admin_api/user_admin_api.md#check-the-status-of-a-redaction-process)

use std::collections::BTreeMap;

use ruma::{
    OwnedEventId,
    api::{auth_scheme::AccessToken, metadata, request, response},
    serde::StringEnum,
};

metadata! {
    method: GET,
    rate_limited: false,
    authentication: AccessToken,
    path: "/_synapse/admin/v1/user/redact_status/{redact_id}",
}

#[request]
pub struct Request {
    /// The opaque ID of the redaction process to query.
    #[ruma_api(path)]
    pub redact_id: String,
}

#[response]
pub struct Response {
    /// The status of the redaction process.
    pub status: RedactStatus,

    /// A map of event ID to error for events that could not be redacted.
    ///
    /// Only present once the process is complete.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_redactions: Option<BTreeMap<OwnedEventId, String>>,

    /// An error describing why the redaction process failed.
    ///
    /// Only present when the process has failed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl Request {
    /// Creates a `Request` with the given redaction ID.
    pub fn new(redact_id: String) -> Self {
        Self { redact_id }
    }
}

impl Response {
    /// Creates a `Response` with the given status.
    pub fn new(status: RedactStatus) -> Self {
        Self { status, failed_redactions: None, error: None }
    }
}

/// The status of a user redaction process.
#[derive(Clone, StringEnum)]
#[ruma_enum(rename_all = "snake_case")]
#[non_exhaustive]
pub enum RedactStatus {
    /// The redaction process is scheduled but has not started.
    Scheduled,

    /// The redaction process is in progress.
    Active,

    /// The redaction process has completed.
    Complete,

    /// The redaction process has failed.
    Failed,

    #[doc(hidden)]
    _Custom(crate::PrivOwnedStr),
}
