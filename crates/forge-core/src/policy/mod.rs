//! Policy types for the Forge geometry kernel.
//!
//! DOMAIN: Three-state return type (`PolicyResult<T>`) and structured
//! policy queries for Doctrine D2 (Ambiguity Escalation). When a
//! geometry solver can't decide, it returns `PolicyResult::Ambiguous`
//! and the kernel layer applies the appropriate policy.
//!
//! DEPENDENCIES: serde

mod schema;

pub use schema::{PolicyKind, PolicyQuery, PolicyResult};
