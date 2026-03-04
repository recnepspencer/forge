//! Graph-level CRUD and index operations on TopologyArena.
//!
//! DOMAIN: Accessor generation, insert/remove with hooks,
//! draft proxies, reverse indexes, and entity reassignment.

mod accessors;
mod insert_remove;
mod view_factory;
pub mod draft_proxy;
pub mod membership_tracker;
mod adjacency_index;
mod reassignment;
