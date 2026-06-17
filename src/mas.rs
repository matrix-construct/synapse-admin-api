//! Endpoints in the `/_synapse/mas/` scope.
//!
//! Private provisioning API the Matrix Authentication Service (MAS) calls on
//! its homeserver to create and manage users and devices. MAS is the client,
//! the homeserver the server. The surface is unversioned, so each endpoint is
//! a flat module with no `vN` submodule. See
//! <https://github.com/element-hq/synapse/tree/develop/synapse/rest/synapse/mas>.

pub mod allow_cross_signing_reset;
pub mod delete_device;
pub mod delete_user;
pub mod is_localpart_available;
pub mod provision_user;
pub mod query_user;
pub mod reactivate_user;
pub mod set_displayname;
pub mod sync_devices;
pub mod unset_displayname;
pub mod update_device_display_name;
pub mod upsert_device;
