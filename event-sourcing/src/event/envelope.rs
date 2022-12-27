use chrono::{DateTime, Utc};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::Error;
use crate::event::EventType;

/// Event is a domain envelope describing a change that has happened to an aggregate.
///
/// As of now, I recommend that you mark your Event enum with `#[serde(tag = "internal_event_type")]`
/// so that it deserializes with the correct type when there are variants.  This approach is to be
/// deprecated in the near future.
#[derive(Debug, Clone, Serialize, Deserialize, derive_new::new)]
pub struct EventEnvelope<Event>
    where
        Event: EventType + Serialize,
{
    // Unique identifier of the envelope.
    #[new(value = "Uuid::new_v4()")]
    pub id: Uuid,
    // ID of the aggregate that the envelope belongs to.
    pub aggregate_id: String,
    // Type of the aggregate that the envelope can be applied to.
    pub aggregate_type: String,
    // Event attached to the envelope.
    pub data: Event,
    // Type of the envelope.
    pub event_type: String,
    // Version of the aggregate after the envelope has been applied.
    pub version: i64,
    // Timestamp of when the envelope was created.
    #[new(value = "Utc::now()")]
    pub timestamp: DateTime<Utc>,
}

/// Serialize the Event Envelope struct to a string.
///
/// # Example
///
/// ```
/// # use std::str::FromStr;
/// # use uuid::Uuid;
/// # use serde::{Deserialize, Serialize};
/// # use event_sourcing::event::envelope::{EventEnvelope, serialize};
/// # use event_sourcing::event::EventType;
///
/// # #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
/// # struct TestEvent {
/// #     id: Uuid,
/// #     amount: i64,
/// # }
///
/// # impl EventType for TestEvent {
/// #     fn event_type(&self) -> String {
/// #         String::from("TestEvent")
/// #     }
/// # }
///
/// # let test_event = TestEvent {
/// #     id: Uuid::from_str("2e996ba1-03a6-47af-8fd1-2039c6708dd4").expect("expected uuid"),
/// #     amount: 1,
/// # };
/// # let event_envelope: EventEnvelope<TestEvent> = EventEnvelope::new(
/// #     String::from("aggregate_id"),
/// #     String::from("TestAggregate"),
/// #     test_event,
/// #     String::from("TestEvent"),
/// #     0,
/// # );
/// let serialized_event_envelope: String = serialize(&event_envelope).expect("expected serialized struct");
///
/// # assert!(serialized_event_envelope.contains("aggregate_id"));
/// # assert!(serialized_event_envelope.contains("2e996ba1-03a6-47af-8fd1-2039c6708dd4"));
/// ```
pub fn serialize<Event: EventType + Serialize + DeserializeOwned>(
    event_envelope: &EventEnvelope<Event>,
) -> Result<String, Error> {
    serde_json::to_string(event_envelope).map_err(|error| error.into())
}

/// Deserialize a string Event Envelope to a struct.
///
/// # Example
///
/// ```
/// # use std::str::FromStr;
/// # use uuid::Uuid;
/// # use serde::{Deserialize, Serialize};
/// # use event_sourcing::event::envelope::{deserialize, EventEnvelope};
/// # use event_sourcing::event::EventType;
///
/// # #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
/// # struct TestEvent {
/// #     id: Uuid,
/// #     amount: i64,
/// # }
///
/// # impl EventType for TestEvent {
/// #     fn event_type(&self) -> String {
/// #         String::from("TestEvent")
/// #     }
/// # }
///
/// # let json_event_envelope: String = String::from("{\"id\":\"17401eba-ff5d-4c3c-9818-c603fe640cb5\",\"aggregate_id\":\"aggregate_id\",\"aggregate_type\":\"TestAggregate\",\"data\":{\"id\":\"2e996ba1-03a6-47af-8fd1-2039c6708dd4\",\"amount\":1},\"event_type\":\"TestEvent\",\"version\":0,\"timestamp\":\"2022-12-28T03:52:22.782613772Z\"}");
/// let event_envelope: EventEnvelope<TestEvent> = deserialize(json_event_envelope).expect("expected deserialized struct");
///
/// # assert_eq!(event_envelope.aggregate_id, String::from("aggregate_id"));
/// # assert_eq!(event_envelope.aggregate_type, String::from("TestAggregate"));
/// # assert_eq!(event_envelope.event_type, String::from("TestEvent"));
/// # assert_eq!(event_envelope.version, 0);
/// # assert_eq!(event_envelope.data, TestEvent {
/// #     id: Uuid::from_str("2e996ba1-03a6-47af-8fd1-2039c6708dd4").expect("expected uuid"),
/// #     amount: 1,
/// # });
/// ```
pub fn deserialize<Event: EventType + Serialize + DeserializeOwned>(
    event_envelope: String,
) -> Result<EventEnvelope<Event>, Error> {
    serde_json::from_str(event_envelope.as_str()).map_err(|error| error.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
    struct TestEvent {
        id: Uuid,
        amount: i64,
    }

    impl EventType for TestEvent {
        fn event_type(&self) -> String {
            String::from("TestEvent")
        }
    }

    #[test]
    fn it_serializes_and_deserializes() {
        let test_event = TestEvent {
            id: Uuid::from_str("2e996ba1-03a6-47af-8fd1-2039c6708dd4").expect("expected uuid"),
            amount: 1,
        };
        let event_envelope: EventEnvelope<TestEvent> = EventEnvelope::new(
            String::from("aggregate_id"),
            String::from("TestAggregate"),
            test_event,
            test_event.event_type(),
            0,
        );
        let serialized_event_envelope: String =
            serialize(&event_envelope).expect("expected serialized struct");
        let event_envelope: EventEnvelope<TestEvent> =
            deserialize(serialized_event_envelope).expect("expected deserialized struct");
        assert_eq!(event_envelope.aggregate_id, String::from("aggregate_id"));
        assert_eq!(event_envelope.aggregate_type, String::from("TestAggregate"));
        assert_eq!(
            event_envelope.event_type,
            String::from("TestEvent")
        );
        assert_eq!(event_envelope.version, 0);
        assert_eq!(
            event_envelope.data,
            TestEvent {
                id: Uuid::from_str("2e996ba1-03a6-47af-8fd1-2039c6708dd4").expect("expected uuid"),
                amount: 1,
            }
        );
    }
}
