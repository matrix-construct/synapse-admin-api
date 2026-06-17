//! [POST /_synapse/mas/update_device_display_name](https://github.com/element-hq/synapse/blob/develop/synapse/rest/synapse/mas/devices.py)

use ruma::{
    OwnedDeviceId,
    api::{auth_scheme::NoAuthentication, metadata, request, response},
};

metadata! {
    method: POST,
    rate_limited: false,
    authentication: NoAuthentication,
    path: "/_synapse/mas/update_device_display_name",
}

#[request]
pub struct Request {
    /// Localpart of the device's owner.
    pub localpart: String,

    /// Device whose display name to update.
    pub device_id: OwnedDeviceId,

    /// New device display name.
    pub display_name: String,
}

#[derive(Default)]
#[response]
pub struct Response {}

impl Request {
    /// Creates a `Request` for the given localpart, device, and display name.
    pub fn new(localpart: String, device_id: OwnedDeviceId, display_name: String) -> Self {
        Self { localpart, device_id, display_name }
    }
}

impl Response {
    /// Creates an empty `Response`.
    pub fn new() -> Self {
        Self {}
    }
}
