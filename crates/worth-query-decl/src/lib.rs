//! Query declaration audience facade.
//!
//! Law: ordinary declaration consumers depend on this crate, never on
//! `worth-query` directly. This crate re-exports engine types only; it adds no
//! behavior.
//!
//! Blessed import:
//!
//! ```
//! use worth_query_decl::facade::CanonicalQueryArtifact;
//! ```
//!
//! Type identity with the engine (no wrapper drift):
//!
//! ```
//! use worth_query_decl::facade::CanonicalQueryArtifact;
//! # fn _same_type(engine: worth_query::facade::CanonicalQueryArtifact) -> CanonicalQueryArtifact {
//! #     engine
//! # }
//! ```

pub mod facade;
