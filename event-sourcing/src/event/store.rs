use crate::Error;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::event::envelope::EventEnvelope;
use crate::event::EventType;

#[async_trait::async_trait]
pub trait EventStore: Sized + Send + Sync + Clone {
    // Fetch all events for the aggregate.
    async fn read<Event: EventType + Serialize + DeserializeOwned>(
        &self,
        aggregate_id: &String,
    ) -> Result<Vec<EventEnvelope<Event>>, Error>;
    // Fetch all events on and after the specified version for the aggregate.
    async fn read_from<Event: EventType + Serialize + DeserializeOwned>(
        &self,
        aggregate_id: &String,
        version: i64,
    ) -> Result<Vec<EventEnvelope<Event>>, Error>;
    // Persist the event for the aggregate.
    async fn persist<Event: EventType + Serialize + DeserializeOwned>(
        &self,
        event_envelope: EventEnvelope<Event>,
    ) -> Result<(), Error>;
}
