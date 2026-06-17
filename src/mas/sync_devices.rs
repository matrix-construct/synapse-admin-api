//! [POST /_synapse/mas/sync_devices](https://github.com/element-hq/synapse/blob/develop/synapse/rest/synapse/mas/devices.py)
//!
//! Reconcile the user's device list to exactly `devices`: the handler adds the
//! missing devices and removes the extra ones.

use std::collections::BTreeSet;

use ruma::{
    OwnedDeviceId,
    api::{auth_scheme::NoAuthentication, metadata, request, response},
};

metadata! {
    method: POST,
    rate_limited: false,
    authentication: NoAuthentication,
    path: "/_synapse/mas/sync_devices",
}

#[request]
pub struct Request {
    /// Localpart of the devices' owner.
    pub localpart: String,

    /// The complete set of devices the user should have. Serialized as a JSON
    /// array.
    pub devices: BTreeSet<OwnedDeviceId>,
}

#[derive(Default)]
#[response]
pub struct Response {}

impl Request {
    /// Creates a `Request` for the given localpart and device set.
    pub fn new(localpart: String, devices: BTreeSet<OwnedDeviceId>) -> Self {
        Self { localpart, devices }
    }
}

impl Response {
    /// Creates an empty `Response`.
    pub fn new() -> Self {
        Self {}
    }
}
