//! Certification namespace for proof-oriented artifacts and boundary tests.

pub mod geometry_support_posture;
pub mod policy_support;
pub mod public_closeout_seed_support;
pub mod public_facade_contracts;
pub mod workload_evidence;

pub use public_closeout_seed_support::{
    current_spatial_milestone_fifteen_planner_seed_support,
    current_spatial_public_closeout_alignment_summary, SpatialMilestoneFifteenPlannerSeedSupport,
    SpatialPublicCloseoutAlignmentSummary, SpatialPublicCloseoutFreshnessRequirementPosture,
    SpatialPublicCloseoutRenderedOutputComparisonPosture, SpatialPublicCloseoutSeedSupportError,
};
pub use public_facade_contracts::{
    current_spatial_public_facade_compile_fail_closeout,
    spatial_public_facade_compile_fail_closeout_excluding_fence_class_for_tests,
    SpatialPublicFacadeCompileFailCloseout, SpatialPublicFacadeCompileFailCloseoutError,
    SpatialPublicFacadeCompileFailCloseoutErrorKind,
};
