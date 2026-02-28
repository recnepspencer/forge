//! Vertex-disk and umbrella invariant validators.
//!
//! DOMAIN: Vertex disk partition correctness, disk closure,
//! ordering determinism, cross-disk coedge detection, and
//! pinch-point consistency for NMT states.
//!
//! VALIDATORS (from validators.md §4):
//! - ValidateVertexDiskPartition
//! - ValidateDiskClosure
//! - ValidateDiskOrderingDeterminism
//! - ValidateNoCrossDiskCoedges
//! - ValidatePinchPointConsistency
//!
//! DEPENDENCIES: `arena`, `handles`
