use crate::Error;

#[async_trait::async_trait]
pub trait EventListener {
    async fn on_event(&self) -> Result<(), Error>;
}
