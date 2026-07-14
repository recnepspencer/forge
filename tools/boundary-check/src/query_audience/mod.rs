//! Query audience matrix enforcement for the Query framework family.
//!
//! Authority separation:
//! - [`rules`] owns Cargo-metadata package-edge law (engine denial, wrong-audience bands)
//! - [`facades`] owns leaf-facade contract law (re-export-only topology under crates/)
//!
//! This module only aggregates those named steps; it implements no decision logic.

mod facades;
mod rules;

pub(crate) use facades::validate_query_audience_facades;
pub(crate) use rules::validate_query_audience_rules;
