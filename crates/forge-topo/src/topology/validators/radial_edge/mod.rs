//! Radial-edge invariant validators (NMT core).
//!
//! DOMAIN: Radial cycle closure, uniqueness, neighbor consistency,
//! ordering determinism, and edge use-count vs state agreement.
//!
//! VALIDATORS (from validators.md §3):
//! - ValidateRadialCycleClosure
//! - ValidateRadialCycleUniqueness
//! - ValidateRadialNeighborConsistency
//! - ValidateRadialOrderingDeterminism
//! - ValidateEdgeUseCountMatchesEdgeState
//! - ValidateNoBrokenRadialSplices
//!
//! DEPENDENCIES: `arena`, `handles`, `queries::radial`
