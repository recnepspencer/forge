//! DOMAIN: Modeling Operations
//!
//! All modeling operations (boolean, fillet, extrude, etc.) live here.
//! Each operation is a self-contained submodule following the Bento Box pattern.
//! The `pipeline` submodule provides step-level infrastructure (Tier 2).

pub mod boolean;
pub mod facade;
pub mod pipeline;
pub mod primitives;
