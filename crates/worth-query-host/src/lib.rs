//! Query host audience facade.
//!
//! Law: admission, lowering, and execution consumers depend on this crate,
//! never on `worth-query` directly. This crate re-exports engine types only;
//! it adds no behavior.
//!
//! Blessed import:
//!
//! ```
//! use worth_query_host::facade::WorthQueryApplicationFacade;
//! ```
//!
//! Type identity with the engine (no wrapper drift):
//!
//! ```
//! use worth_query_host::facade::WorthQueryApplicationFacade;
//! # fn _same_type(
//! #     engine: worth_query::facade::WorthQueryApplicationFacade,
//! # ) -> WorthQueryApplicationFacade {
//! #     engine
//! # }
//! ```

pub mod facade;
