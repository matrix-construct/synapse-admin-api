//! [GET /_synapse/admin/v1/scheduled_tasks](https://github.com/element-hq/synapse/blob/master/docs/admin_api/scheduled_tasks.md)
use ruma::{
    MilliSecondsSinceUnixEpoch, UInt,
    api::{auth_scheme::AccessToken, metadata, request, response},
    serde::{JsonObject, Raw, StringEnum},
};
use serde::{Deserialize, Serialize};

metadata! {
    method: GET,
    rate_limited: false,
    authentication: AccessToken,
    path: "/_synapse/admin/v1/scheduled_tasks",
}

#[request]
#[derive(Default)]
pub struct Request {
    /// Filter by the action name of the task.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ruma_api(query)]
    pub action_name: Option<String>,

    /// Filter by the resource ID the task acts on.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ruma_api(query)]
    pub resource_id: Option<String>,

    /// Filter by the status of the task.
    ///
    /// The `job_status` query name is accepted as a legacy alias.
    #[serde(alias = "job_status", skip_serializing_if = "Option::is_none")]
    #[ruma_api(query)]
    pub status: Option<TaskStatus>,

    /// Only return tasks scheduled to run at or before this timestamp, in milliseconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ruma_api(query)]
    pub max_timestamp: Option<UInt>,
}

#[response]
pub struct Response {
    /// The list of scheduled tasks, ordered by increasing timestamp.
    pub scheduled_tasks: Vec<ScheduledTask>,
}

impl Request {
    /// Creates an empty `Request`.
    pub fn new() -> Self {
        Default::default()
    }
}

impl Response {
    /// Creates a `Response` with the given scheduled tasks.
    pub fn new(scheduled_tasks: Vec<ScheduledTask>) -> Self {
        Self { scheduled_tasks }
    }
}

/// The status of a scheduled task.
#[derive(Clone, StringEnum)]
#[ruma_enum(rename_all = "snake_case")]
#[non_exhaustive]
pub enum TaskStatus {
    /// The task is scheduled but has not started yet.
    Scheduled,

    /// The task is currently running.
    Active,

    /// The task has completed successfully.
    Complete,

    /// The task has been cancelled.
    Cancelled,

    /// The task has failed.
    Failed,

    #[doc(hidden)]
    _Custom(crate::PrivOwnedStr),
}

/// A scheduled task and its status.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(not(ruma_unstable_exhaustive_types), non_exhaustive)]
pub struct ScheduledTask {
    /// The unique ID of the task.
    pub id: String,

    /// The action the task performs.
    pub action: String,

    /// The status of the task.
    pub status: TaskStatus,

    /// The timestamp the task is scheduled to run at, in milliseconds.
    pub timestamp_ms: MilliSecondsSinceUnixEpoch,

    /// The resource ID the task acts on, if any.
    pub resource_id: Option<String>,

    /// The result of the task, if any.
    pub result: Option<Raw<JsonObject>>,

    /// The error message of the task, if it failed.
    pub error: Option<String>,
}

impl ScheduledTask {
    /// Construct a `ScheduledTask` with the given required fields and all others defaulted.
    pub fn new(
        id: String,
        action: String,
        status: TaskStatus,
        timestamp_ms: MilliSecondsSinceUnixEpoch,
    ) -> Self {
        Self { id, action, status, timestamp_ms, resource_id: None, result: None, error: None }
    }
}
