//! Event subscriber traits and identity types for the lifecycle event bus.

mod id;
mod subscriber;

pub use id::SubscriberId;
pub use subscriber::EventSubscriber;
