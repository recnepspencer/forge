//! Cache and index consistency validators.
//!
//! DOMAIN: AABB tree coverage, BVH refit correctness, spatial cache
//! staleness flags, adjacency cache ground-truth matching, and
//! curvature/mass-property cache staleness.
//!
//! VALIDATORS (from validators.md §13):
//! - ValidateAABBTreeCoversAllEntities
//! - ValidateBVHRefitCorrectness
//! - ValidateSpatialCacheStalenessFlags
//! - ValidateAdjacencyCacheMatchesGroundTruth
//! - ValidateCurvatureCacheStaleness
//! - ValidateMassPropsCacheStaleness
//!
//! DEPENDENCIES: `arena`, `forge-geom` (spatial)
