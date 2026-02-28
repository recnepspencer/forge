//! Intersection and imprint graph invariant validators.
//!
//! DOMAIN: Graph connectivity, dangling spur detection,
//! consistent vertex merges, tangent event encoding,
//! and coplanar overlap loop extraction consistency.
//!
//! VALIDATORS (from validators.md §10):
//! - ValidateIntersectionGraphConnectivity
//! - ValidateNoDanglingIntersectionSpurs
//! - ValidateConsistentVertexMergesInGraph
//! - ValidateTangentEventEncoding
//! - ValidateCoplanarOverlapLoopExtraction
//!
//! DEPENDENCIES: `arena`, `handles`
