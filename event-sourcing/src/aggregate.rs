use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::event::EventType;

/// An aggregate is a cluster of associated events that is treated as a unit for the purpose of data changes.
///
/// # Examples
///
/// ```
/// # use std::str::FromStr;
/// # use uuid::Uuid;
/// # use event_sourcing::Error;
/// # use serde::{Deserialize, Serialize};
/// # use event_sourcing::event::EventType;
/// # use crate::event_sourcing::aggregate::Aggregate;
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
/// #[derive(Debug, Clone, Serialize, Deserialize)]
/// struct TestAggregate {
///     id: Uuid,
///     total: i64,
/// }
///
/// impl Aggregate for TestAggregate {
///     type AggregateID = Uuid;
///     type Event = TestEvent;
///     type Error = Error;
///
///     fn aggregate_id(&self) -> &Self::AggregateID {
///         return &self.id;
///     }
///
///     fn apply(state: Option<Self>, event: Self::Event) -> Result<Self, Self::Error> {
///         match state {
///             None => Ok(Self {
///                 id: event.id,
///                 total: event.amount,
///             }),
///             Some(mut state) => {
///                 state.total = state.total + event.amount;
///                 Ok(state)
///             }
///         }
///     }
///
///     fn apply_all(events: Vec<Self::Event>) -> Result<Self, Self::Error> {
///         match events.into_iter().fold(Ok(None), |state, event| {
///             Self::apply(state?, event).map(|new_state| Some(new_state))
///         }) {
///             Ok(Some(state)) => Ok(state),
///             Ok(None) => Err(Error::try_from("Aggregate must not be None").unwrap()),
///             Err(error) => Err(error),
///         }
///     }
/// }
///
/// let test_event = TestEvent {
///     id: Uuid::from_str("2e996ba1-03a6-47af-8fd1-2039c6708dd4").expect("expected uuid"),
///     amount: 1,
/// };
/// let test_aggregate = TestAggregate::apply_all(vec![test_event]).expect("expected aggregate");
///
/// # assert_eq!(test_aggregate.id, Uuid::from_str("2e996ba1-03a6-47af-8fd1-2039c6708dd4").expect("expected uuid"));
/// # assert_eq!(test_aggregate.total, 1);
/// ```
pub trait Aggregate: Sized + Send + Sync + Clone + Serialize + DeserializeOwned {
    type AggregateID: Send + Sync + Clone;
    type Event: EventType;
    type Error: Send + Sync;

    fn aggregate_id(&self) -> &Self::AggregateID;
    fn apply(state: Option<Self>, event: Self::Event) -> Result<Self, Self::Error>;
    fn apply_all(events: Vec<Self::Event>) -> Result<Self, Self::Error>;
}
