//! Domain-owned topology authoring support.
//!
//! These helpers provide the primitive corpus, seed, and authoring surfaces
//! used by certification, fixtures, and topology authoring support.

pub use crate::data::seed::{
    build_milestone_one_primitive_intent, created_ref,
    milestone_one_admitted_range_sweep_out_of_class_scenarios,
    milestone_one_admitted_range_sweep_scenarios, milestone_one_default_primitive_corpus,
    milestone_one_heavy_branch_local_sweep_scenarios, seed_milestone_one_primitive,
    seed_milestone_one_primitive_on_branch, seed_minimal_topology,
    MilestoneOnePrimitiveAuthoringError, MilestoneOnePrimitiveCase,
    MilestoneOnePrimitiveExpectedOutcome, MilestoneOnePrimitiveRole,
    MilestoneOnePrimitiveScenario, MinimalTopologySeed, TopologyCreateBatchBuilder,
};
