//! DOMAIN: Modeling Operations
//!
//! All modeling operations (boolean, fillet, extrude, etc.) live here.
//! Each operation is a self-contained submodule following the Bento Box pattern.
//!
//! - `shared_operations`: Cross-cutting atomic operations (vertex placement,
//!   stitching, snapping) consumed by all feature domains.
//! - `shared_validators`: Cross-cutting invariant checks for traced operations.
//! - `pipeline`: Step-level infrastructure (Tier 2).

pub mod boolean;
pub mod facade;
pub mod pipeline;
pub mod primitives;
pub mod shared_operations;
pub mod shared_validators;
