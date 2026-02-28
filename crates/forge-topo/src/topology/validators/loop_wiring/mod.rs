//! Half-edge and loop wiring invariant validators.
//!
//! DOMAIN: Structural invariants for the half-edge data structure —
//! twin symmetry, next/prev symmetry, loop closure, and edge-endpoint
//! consistency with loop vertices.
//!
//! VALIDATORS (from validators.md §2):
//! - ValidateTwinSymmetry
//! - ValidateNextPrevSymmetry
//! - ValidateLoopClosure
//! - ValidateLoopIsSimpleTopologically
//! - ValidateEdgeEndpointsMatchCoedgeVertices
//! - ValidateConsistentEdgeSenseAcrossCoedges
//! - ValidateFaceLoopMembershipComplete
//!
//! DEPENDENCIES: `arena`, `handles`, `queries::traverse`
