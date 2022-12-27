#[async_trait::async_trait]
pub trait CommandHandler<Command>
where
    Command: Send + Sync,
{
    type Error: Send + Sync;

    async fn handle(&self, command: Command) -> Result<(), Self::Error>;
}
