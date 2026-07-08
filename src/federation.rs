//! Endpoints in the `/_synapse/admin/v<x>/federation/` scope.

pub mod destination;
pub mod destination_rooms;
pub mod list_destinations;
pub mod reset_connection;

use ruma::{OwnedServerName, UInt, uint};
use serde::{Deserialize, Serialize};

/// Details about a remote federation destination.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(not(ruma_unstable_exhaustive_types), non_exhaustive)]
pub struct Destination {
    /// The remote server name.
    pub destination: OwnedServerName,

    /// Timestamp in milliseconds of the last retry attempt.
    ///
    /// `0` when the destination is healthy.
    pub retry_last_ts: UInt,

    /// Interval in milliseconds before the next retry.
    ///
    /// `0` when no retry is scheduled.
    pub retry_interval: UInt,

    /// Timestamp in milliseconds of the first failure in the current streak.
    ///
    /// `null` when the destination is healthy.
    pub failure_ts: Option<UInt>,

    /// Stream ordering of the most recent PDU successfully sent to the destination.
    ///
    /// `null` when unknown.
    pub last_successful_stream_ordering: Option<UInt>,
}

impl Destination {
    /// Construct a `Destination` for the given server with all other fields defaulted.
    pub fn new(destination: OwnedServerName) -> Self {
        Self {
            destination,
            retry_last_ts: uint!(0),
            retry_interval: uint!(0),
            failure_ts: None,
            last_successful_stream_ordering: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use ruma::uint;
    use serde_json::{from_value, json, to_value};

    use super::Destination;

    #[test]
    fn destination_null_fields_round_trip() {
        let value = json!({
            "destination": "example.com",
            "retry_last_ts": 0,
            "retry_interval": 0,
            "failure_ts": null,
            "last_successful_stream_ordering": null,
        });

        let destination: Destination = from_value(value.clone()).unwrap();
        assert_eq!(destination.retry_last_ts, uint!(0));
        assert_eq!(destination.retry_interval, uint!(0));
        assert_eq!(destination.failure_ts, None);
        assert_eq!(destination.last_successful_stream_ordering, None);

        assert_eq!(to_value(&destination).unwrap(), value);
    }

    #[test]
    fn destination_populated_fields_round_trip() {
        let value = json!({
            "destination": "example.com",
            "retry_last_ts": 100,
            "retry_interval": 200,
            "failure_ts": 12345,
            "last_successful_stream_ordering": 42,
        });

        let destination: Destination = from_value(value.clone()).unwrap();
        assert_eq!(destination.retry_last_ts, uint!(100));
        assert_eq!(destination.retry_interval, uint!(200));
        assert_eq!(destination.failure_ts, Some(uint!(12345)));
        assert_eq!(destination.last_successful_stream_ordering, Some(uint!(42)));

        assert_eq!(to_value(&destination).unwrap(), value);
    }
}
