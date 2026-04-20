use crate::{
    AspectLayoutReadPlanDecision, AspectLayoutReadRequest, AspectLayoutTarget, AspectProjectionSet,
    AspectReadRegime, AspectScopeClass, CdcTouchedAspectScope, ComplexityStatus,
    EntitySetUniformAspectScope, ForgeStoreBuilder, Milestone6LayoutSupportLane,
    Milestone6LayoutSupportPolicy, Milestone6ResolvedLayoutSupportLane, SingleEntityAspectScope,
};

use super::harness::corruption::local_file::{
    force_clear_milestone_6_materializations_and_derived_access_structures,
    force_milestone_6_chunk_membership_boundary_drift,
    force_milestone_6_commit_coupled_layout_seed_authority_digest_drift,
    force_milestone_6_commit_coupled_layout_seed_payload_drift,
    force_milestone_6_commit_coupled_layout_seed_payload_gap,
    force_milestone_6_commit_support_summary_seed_gap,
    force_milestone_6_layout_materialization_chunk_member_count_drift,
    force_milestone_6_layout_materialization_key_mismatch,
};
use super::harness::corruption::sqlite::simulate_legacy_milestone_6_commit_coupled_layout_seed_storage;
use super::harness::fixtures::{
    runtime::{create_entity, latest_envelope, runtime_with_demo_schema, update_entity_on_branch},
    stores::{build_store_for_lane, unique_test_sqlite_path, unique_test_store_path, StoreLane},
};

fn store_with_root_commit() -> (
    crate::ForgeStore,
    forge_relational::facade::history::BranchId,
    forge_relational::facade::history::CommitId,
) {
    store_with_root_commit_for_lane(StoreLane::InMemory)
}

fn store_with_root_commit_for_lane(
    lane: StoreLane,
) -> (
    crate::ForgeStore,
    forge_relational::facade::history::BranchId,
    forge_relational::facade::history::CommitId,
) {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let root = latest_envelope(&runtime);
    let branch_id = root.branch_context.clone();
    let commit_id = root.commit.commit_id;

    let mut store = match lane {
        StoreLane::InMemory => ForgeStoreBuilder::new().in_memory().build().unwrap(),
        _ => build_store_for_lane(lane, &format!("layout-{}", lane.label())),
    };
    store.append_canonical_commit(root).unwrap();
    (store, branch_id, commit_id)
}

fn admitted_plan(
    store: &crate::ForgeStore,
    request: AspectLayoutReadRequest,
) -> crate::AdmittedAspectLayoutReadPlan {
    match store.plan_aspect_layout_read(request).unwrap() {
        AspectLayoutReadPlanDecision::Admitted(plan) => plan,
        other => panic!("expected admitted plan, got {other:?}"),
    }
}

fn entity_set_request(
    branch_id: forge_relational::facade::history::BranchId,
    commit_id: forge_relational::facade::history::CommitId,
) -> AspectLayoutReadRequest {
    AspectLayoutReadRequest::new(
        AspectLayoutTarget::new(branch_id, commit_id),
        AspectScopeClass::EntitySetUniform(EntitySetUniformAspectScope::new(vec![
            "entity-a".to_string(),
            "entity-b".to_string(),
        ])),
        AspectProjectionSet::new(vec!["profile".to_string(), "status".to_string()]),
    )
}

#[path = "layout/admission_and_lanes.rs"]
mod admission_and_lanes;
#[path = "layout/corruption_and_migration.rs"]
mod corruption_and_migration;
#[path = "layout/scopes_and_reads.rs"]
mod scopes_and_reads;
#[path = "layout/execution.rs"]
mod execution;
#[path = "layout/export_and_truth.rs"]
mod export_and_truth;
#[path = "layout/parity.rs"]
mod parity;
