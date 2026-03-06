//! Cache framework data types.

pub mod dirty_state;
pub mod policy;

pub use dirty_state::{CacheDirtyState, DomainImpact};
pub use policy::{CacheCheckpoint, CacheDomain, CacheRefreshMode, CacheRefreshPolicy};
