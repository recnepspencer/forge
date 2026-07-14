//! Query replay audience facade (cert-only).
//!
//! Law: only cert-band consumers may depend on this crate. Ordinary bands use
//! `worth-query-decl` / `worth-query-host`. This crate re-exports engine types
//! only; it adds no replay executors or certification harnesses.
//!
//! Blessed import:
//!
//! ```
//! use worth_query_replay::facade::ReplayBasisCapability;
//! ```
//!
//! Type identity with the engine (no wrapper drift):
//!
//! ```
//! use worth_query_replay::facade::ReplayBasisCapability;
//! # fn _same_type(engine: worth_query::facade::ReplayBasisCapability) -> ReplayBasisCapability {
//! #     engine
//! # }
//! ```

pub mod facade;
