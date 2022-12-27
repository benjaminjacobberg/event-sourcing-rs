use event_sourcing::event::envelope::EventEnvelope;
use event_sourcing::event::store::EventStore;
use event_sourcing::Error;
use serde::de::DeserializeOwned;
use serde::Serialize;
use event_sourcing::event::EventType;

#[derive(Clone)]
pub struct CassandraEventStoreConfiguration {}

#[derive(Clone)]
pub struct CassandraEventStore {
    pub configuration: CassandraEventStoreConfiguration,
}

#[async_trait::async_trait]
impl EventStore for CassandraEventStore {
    async fn read<Event: EventType + serde::ser::Serialize + DeserializeOwned>(
        &self,
        _aggregate_id: &String,
    ) -> Result<Vec<EventEnvelope<Event>>, Error> {
        todo!()
    }

    async fn read_from<Event: EventType + Serialize + DeserializeOwned>(
        &self,
        _aggregate_id: &String,
        _version: i64,
    ) -> Result<Vec<EventEnvelope<Event>>, Error> {
        todo!()
    }

    async fn persist<Event: EventType + serde::ser::Serialize + DeserializeOwned>(
        &self,
        _event_envelope: EventEnvelope<Event>,
    ) -> Result<(), Error> {
        todo!()
    }
}
