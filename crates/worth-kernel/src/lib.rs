//! `worth-kernel` no longer ships a public primitive-construction runtime or
//! authoring facade.
//!
//! The kernel does not export a public geometry runtime facade, a public
//! certification bucket, a public query-proof / realization-proof report
//! warehouse, public replay / branch-preview-runtime / hostility-suite proof
//! products, a public primitive-construction lane, a public `facade` namespace,
//! or a second local runtime.

#![forbid(unsafe_code)]

extern crate self as worth_kernel;

pub mod docs_closeout;
pub mod query_adoption;
pub mod workload_composition;

#[cfg(test)]
mod binding;
#[cfg(test)]
mod construction;
