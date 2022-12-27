use chrono::{DateTime, Utc};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::Error;

/// Event is a domain envelope describing a change that has happened to an aggregate.
#[derive(Debug, Clone, Serialize, Deserialize, derive_new::new)]
pub struct SnapshotEnvelope<Aggregate>
    where
        Aggregate: Send + Sync + Clone + Serialize,
{
    // Unique identifier of the envelope.
    #[new(value = "Uuid::new_v4()")]
    pub id: Uuid,
    // ID of the aggregate that the envelope belongs to.
    pub aggregate_id: String,
    // Type of the aggregate that the envelope can be applied to.
    pub aggregate_type: String,
    // Aggregate attached to the envelope.
    pub data: Aggregate,
    // Version of the aggregate after the envelope has been applied.
    pub version: i64,
    // Timestamp of when the envelope was created.
    #[new(value = "Utc::now()")]
    pub timestamp: DateTime<Utc>,
}

/// Serialize the Snapshot Envelope struct to a string.
///
/// # Example
///
/// ```
/// # use std::str::FromStr;
/// # use uuid::Uuid;
/// # use serde::{Deserialize, Serialize};
/// # use event_sourcing::aggregate::Aggregate;
/// # use event_sourcing::Error;
/// # use event_sourcing::snapshot::envelope::{SnapshotEnvelope, serialize};
///
///
/// # #[derive(Debug, Clone, Serialize, Deserialize)]
/// # struct TestAggregate {
/// #     id: Uuid,
/// #     total: i64,
/// # }
///
/// # let test_aggregate = TestAggregate {
/// #     id: Uuid::from_str("2e996ba1-03a6-47af-8fd1-2039c6708dd4").expect("expected uuid"),
/// #     total: 1,
/// # };
/// # let snapshot_envelope: SnapshotEnvelope<TestAggregate> = SnapshotEnvelope::new(
/// #     String::from("aggregate_id"),
/// #     String::from("TestAggregate"),
/// #     test_aggregate,
/// #     0,
/// # );
/// let serialized_snapshot_envelope: String = serialize(&snapshot_envelope).expect("expected serialized struct");
///
/// # assert!(serialized_snapshot_envelope.contains("aggregate_id"));
/// # assert!(serialized_snapshot_envelope.contains("2e996ba1-03a6-47af-8fd1-2039c6708dd4"));
/// ```
pub fn serialize<Aggregate: Send + Sync + Clone + Serialize + DeserializeOwned>(
    snapshot_envelope: &SnapshotEnvelope<Aggregate>,
) -> Result<String, Error> {
    serde_json::to_string(snapshot_envelope).map_err(|error| error.into())
}

/// Deserialize a string Snapshot Envelope to a struct.
///
/// # Examples
///
/// ```
/// # use std::str::FromStr;
/// # use uuid::Uuid;
/// # use serde::{Deserialize, Serialize};
/// # use event_sourcing::snapshot::envelope::{deserialize, SnapshotEnvelope};
///
/// # #[derive(Debug, Clone, Serialize, Deserialize)]
/// # struct TestAggregate {
/// #     id: Uuid,
/// #     total: i64,
/// # }
///
/// # let json_snapshot_envelope:String = String::from("{\"id\":\"352c182d-9002-4a0c-b9f8-96c8cb2ff90f\",\"aggregate_id\":\"aggregate_id\",\"aggregate_type\":\"TestAggregate\",\"data\":{\"id\":\"2e996ba1-03a6-47af-8fd1-2039c6708dd4\",\"total\":1},\"version\":0,\"timestamp\":\"2022-12-28T00:16:53.162038985Z\"}");
/// let snapshot_envelope: SnapshotEnvelope<TestAggregate> = deserialize(json_snapshot_envelope).expect("expected deserialized struct");
///
/// # assert_eq!(snapshot_envelope.aggregate_id, String::from("aggregate_id"));
/// # assert_eq!(snapshot_envelope.aggregate_type, String::from("TestAggregate"));
/// # assert_eq!(snapshot_envelope.version, 0);
/// # assert_eq!(snapshot_envelope.data.id, Uuid::from_str("2e996ba1-03a6-47af-8fd1-2039c6708dd4").expect("expected uuid"));
/// # assert_eq!(snapshot_envelope.data.total, 1);
/// ```
pub fn deserialize<Aggregate: Send + Sync + Clone + Serialize + DeserializeOwned>(
    snapshot_envelope: String,
) -> Result<SnapshotEnvelope<Aggregate>, Error> {
    serde_json::from_str(snapshot_envelope.as_str()).map_err(|error| error.into())
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::aggregate::Aggregate;
    use crate::event::EventType;

    use super::*;

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

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct TestAggregate {
        id: Uuid,
        total: i64,
    }

    impl Aggregate for TestAggregate {
        type AggregateID = Uuid;
        type Event = TestEvent;
        type Error = Error;

        fn aggregate_id(&self) -> &Self::AggregateID {
            return &self.id;
        }

        fn apply(state: Option<Self>, event: Self::Event) -> Result<Self, Self::Error> {
            match state {
                None => Ok(Self {
                    id: event.id,
                    total: event.amount,
                }),
                Some(mut state) => {
                    state.total = state.total + event.amount;
                    Ok(state)
                }
            }
        }

        fn apply_all(events: Vec<Self::Event>) -> Result<Self, Self::Error> {
            match events.into_iter().fold(Ok(None), |state, event| {
                Self::apply(state?, event).map(|new_state| Some(new_state))
            }) {
                Ok(Some(state)) => Ok(state),
                Ok(None) => Err(Error::try_from("Aggregate must not be None").unwrap()),
                Err(error) => Err(error),
            }
        }
    }

    #[test]
    fn it_serializes_and_deserializes() {
        let test_event = TestEvent {
            id: Uuid::from_str("2e996ba1-03a6-47af-8fd1-2039c6708dd4").expect("expected uuid"),
            amount: 1,
        };
        let test_aggregate =
            TestAggregate::apply_all(vec![test_event]).expect("expected aggregate");
        let event_envelope: SnapshotEnvelope<TestAggregate> = SnapshotEnvelope::new(
            String::from("aggregate_id"),
            String::from("TestAggregate"),
            test_aggregate,
            0,
        );
        let serialized_event_envelope: String =
            serialize(&event_envelope).expect("expected serialized struct");
        let event_envelope: SnapshotEnvelope<TestAggregate> =
            deserialize(serialized_event_envelope).expect("expected deserialized struct");
        assert_eq!(event_envelope.aggregate_id, String::from("aggregate_id"));
        assert_eq!(event_envelope.aggregate_type, String::from("TestAggregate"));
        assert_eq!(event_envelope.version, 0);
        assert_eq!(
            event_envelope.data.id,
            Uuid::from_str("2e996ba1-03a6-47af-8fd1-2039c6708dd4").expect("expected uuid")
        );
        assert_eq!(event_envelope.data.total, 1);
    }
}
