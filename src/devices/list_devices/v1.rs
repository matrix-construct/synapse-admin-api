//! [GET /_synapse/admin/v2/users/:user_id/devices](https://github.com/element-hq/synapse/blob/master/docs/admin_api/user_admin_api.md#list-all-devices)

use ruma::{
    OwnedUserId, UInt,
    api::{auth_scheme::AccessToken, metadata, request, response},
};

pub use crate::devices::Device;

metadata! {
    method: GET,
    rate_limited: false,
    authentication: AccessToken,
    path: "/_synapse/admin/v2/users/{user_id}/devices",
}

#[request]
pub struct Request {
    /// The user whose devices to list.
    #[ruma_api(path)]
    pub user_id: OwnedUserId,
}

#[response]
pub struct Response {
    /// A list of all the user's devices.
    pub devices: Vec<Device>,

    /// The total number of devices.
    pub total: UInt,
}

impl Request {
    /// Creates a `Request` with the given user ID.
    pub fn new(user_id: OwnedUserId) -> Self {
        Self { user_id }
    }
}

impl Response {
    /// Creates a `Response` with the given devices and total count.
    pub fn new(devices: Vec<Device>, total: UInt) -> Self {
        Self { devices, total }
    }
}
