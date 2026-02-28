//! Region and cellular topology invariant validators.
//!
//! DOMAIN: Region adjacency graph correctness, outside region
//! connectivity, leak detection, boundary completeness,
//! internal wall consistency, and region count sanity.
//!
//! VALIDATORS (from validators.md §6):
//! - ValidateRegionAdjacencyGraph
//! - ValidateOutsideRegionConnectivity
//! - ValidateNoRegionLeaks
//! - ValidateRegionBoundaryCompleteness
//! - ValidateInternalWallConsistency
//! - ValidateRegionCountAgainstShellConfig
//!
//! DEPENDENCIES: `arena`, `handles`
