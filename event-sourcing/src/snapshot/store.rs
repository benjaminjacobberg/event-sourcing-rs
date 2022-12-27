use crate::event::envelope::EventEnvelope;
use crate::Error;
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::event::EventType;

#[async_trait::async_trait]
pub trait SnapshotStore: Sized + Send + Sync + Clone {
    // Fetch the latest snapshot version for the aggregate.
    async fn read<Aggregate: EventType + Serialize + DeserializeOwned>(
        &self,
        aggregate_id: &String,
    ) -> Result<EventEnvelope<Aggregate>, Error>;
    // Persist a snapshot for the aggregate.
    async fn persist<Aggregate: EventType + Serialize + DeserializeOwned>(
        &self,
        snapshot_envelope: EventEnvelope<Aggregate>,
    ) -> Result<(), Error>;
}
