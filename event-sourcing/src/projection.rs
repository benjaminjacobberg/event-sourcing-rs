pub trait Projection: Sized + Send + Sync + Clone {
    type Event: Send + Sync + Clone;
    type Error: Send + Sync;

    // Apply event to projection.
    fn apply(state: Option<Self>, event: Self::Event) -> Result<Self, Self::Error>;
    // Rebuild the projection by clearing it's state and then replaying all the events from the beginning.
    fn replay() -> Result<Self, Self::Error>;
}
