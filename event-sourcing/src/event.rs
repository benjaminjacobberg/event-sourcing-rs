pub mod envelope;
pub mod listener;
pub mod store;

/// Trait to determine the type of the event.
///
/// # Example
///
/// ```
/// # use uuid::Uuid;
/// # use event_sourcing::event::EventType;
///
/// #[derive(Debug, Clone, Copy)]
/// struct TestEvent;
///
/// impl EventType for TestEvent {
///     fn event_type(&self) -> String {
///         String::from("TestEvent")
///     }
/// }
///
/// # assert_eq!(TestEvent.event_type(), String::from("TestEvent"));
/// ```
pub trait EventType: Send + Sync + Clone + Copy {
    fn event_type(&self) -> String;
}