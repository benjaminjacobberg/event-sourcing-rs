pub mod aggregate;
pub mod command_handler;
pub mod event;
pub mod projection;
pub mod query_handler;
pub mod snapshot;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
