//! Topology assembly shared operations.
//!
//! DOMAIN: Cross-arena copy, edge stitching, face rebuild, and
//! fragment utilities. Used by boolean assemble/postprocess and
//! future operations (fillet, shell).

pub mod copy;
pub mod fragment;
pub mod rebuild_face;
pub mod stitch;
