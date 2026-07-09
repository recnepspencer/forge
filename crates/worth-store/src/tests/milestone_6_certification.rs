use crate::{
    AspectLayoutReadRequest, AspectLayoutTarget, AspectProjectionSet, AspectScopeClass,
    EntitySetUniformAspectScope, WORTHStore, WORTHStoreBuilder, Milestone6LayoutSupportLane,
    Milestone6LayoutSupportPolicy, Milestone6ResolvedLayoutSupportLane, SingleEntityAspectScope,
    StoreErrorKind,
};
use worth_relational::facade::history::BranchId;
use worth_relational::facade::replay::CanonicalCommitEnvelope;

use super::harness::{
    certification::{
        assertions::{assert_all_equal, assert_any_not_equal, assert_rejection_payloads_present},
        core::{AssertionClass, CanonicalRow, CertificationSuite, LaneResult, RejectionRow},
        requirements::{evaluate_completeness, ASPECT_LAYOUT_PHYSICAL_CERTIFICATION_TEST},
    },
    corruption::local_file::force_clear_milestone_6_derived_access_structures,
    corruption::local_file::force_clear_milestone_6_materializations_and_derived_access_structures,
    corruption::local_file::force_milestone_6_chunk_membership_boundary_drift,
    corruption::local_file::force_milestone_6_commit_support_summary_seed_gap,
    corruption::local_file::force_milestone_6_layout_materialization_chunk_member_count_drift,
    corruption::sqlite::simulate_legacy_milestone_6_commit_coupled_layout_seed_storage,
    fixtures::{
        runtime::{
            create_entity, latest_envelope, runtime_with_demo_schema, update_entity_on_branch,
        },
        stores::{
            build_store_for_lane, unique_test_sqlite_path, unique_test_store_path, StoreLane,
        },
    },
};

fn store_for_lane_with_root(
    lane: StoreLane,
    suffix: &str,
) -> (WORTHStore, CanonicalCommitEnvelope) {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let root = latest_envelope(&runtime);
    let mut store = match lane {
        StoreLane::InMemory => WORTHStoreBuilder::new().in_memory().build().unwrap(),
        _ => build_store_for_lane(lane, &format!("milestone-6-{suffix}-{}", lane.label())),
    };
    store.append_canonical_commit(root.clone()).unwrap();
    (store, root)
}

fn request_for_scope(
    root: &CanonicalCommitEnvelope,
    scope_class: AspectScopeClass,
    projection_names: &[&str],
) -> AspectLayoutReadRequest {
    AspectLayoutReadRequest::new(
        AspectLayoutTarget::new(root.branch_context.clone(), root.commit.commit_id),
        scope_class,
        AspectProjectionSet::new(
            projection_names
                .iter()
                .map(|name| (*name).to_string())
                .collect(),
        ),
    )
}

fn admitted_request_for_lane(lane: StoreLane) -> (WORTHStore, AspectLayoutReadRequest) {
    let (store, root) = store_for_lane_with_root(lane, "admitted");
    (
        store,
        request_for_scope(
            &root,
            AspectScopeClass::EntitySetUniform(EntitySetUniformAspectScope::new(vec![
                "entity-a".to_string(),
                "entity-b".to_string(),
            ])),
            &["profile", "status"],
        ),
    )
}

fn single_entity_bundle_for_lane(lane: StoreLane) -> crate::Milestone6CertificationBundle {
    let (store, root) = store_for_lane_with_root(lane, "single-entity");
    store
        .milestone_6_certification_bundle(request_for_scope(
            &root,
            AspectScopeClass::SingleEntity(SingleEntityAspectScope::new("entity-a")),
            &["profile", "status"],
        ))
        .unwrap()
}

fn entity_set_bundle_for_lane(lane: StoreLane) -> crate::Milestone6CertificationBundle {
    let (store, request) = admitted_request_for_lane(lane);
    store.milestone_6_certification_bundle(request).unwrap()
}

#[path = "milestone_6_certification/bundle_basics.rs"]
mod bundle_basics;
#[path = "milestone_6_certification/persisted_materialization.rs"]
mod persisted_materialization;
#[path = "milestone_6_certification/rebuild_recovery.rs"]
mod rebuild_recovery;
#[path = "milestone_6_certification/suite.rs"]
mod suite;
#[path = "milestone_6_certification/suite_helpers.rs"]
mod suite_helpers;
#[path = "milestone_6_certification/suite_rows_foundation.rs"]
mod suite_rows_foundation;
#[path = "milestone_6_certification/suite_rows_overlap_and_corruption.rs"]
mod suite_rows_overlap_and_corruption;
#[path = "milestone_6_certification/suite_rows_rebuild.rs"]
mod suite_rows_rebuild;
#[path = "milestone_6_certification/support_lane_policy.rs"]
mod support_lane_policy;
