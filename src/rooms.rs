//! Endpoints in the `/_synapse/admin/v<x>/rooms/` scope.

pub mod admin_context;
pub mod admin_hierarchy;
pub mod admin_messages;
pub mod admin_state;
pub mod admin_timestamp_to_event;
pub mod block;
pub mod delete_room;
pub mod delete_status;
pub mod forward_extremities;
pub mod list_rooms;
pub mod make_room_admin;
pub mod room_details;
pub mod room_members;
