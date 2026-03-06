//! Graph-level CRUD and index operations on TopologyArena.
//!
//! DOMAIN: Accessor generation, insert/remove with hooks,
//! draft proxies, reverse indexes, and entity reassignment.

mod accessors;
mod adjacency_index;
pub mod draft_proxy;
mod insert_remove;
pub mod membership_tracker;
mod reassignment;
mod view_factory;
