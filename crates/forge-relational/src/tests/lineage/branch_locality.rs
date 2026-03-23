use crate::facade::history::BranchId;
use crate::facade::lineage::{HistoricalResolutionRequest, LineageDivergenceRequest};
use crate::tests::support::*;

// CONTRACT: lineage_branch_locality
// LANES: success, determinism

#[test]
fn lineage_branch_divergence_is_queryable() {
    let mut runtime = runtime_with_test_schema();
    let _main = create_entity_outcome(&mut runtime, "main");
    runtime
        .history_authority()
        .create_branch(
            BranchId("feature".to_string()),
            &BranchId("main".to_string()),
        )
        .unwrap();
    let _feature =
        create_entity_outcome_on_branch(&mut runtime, "feature", BranchId("feature".to_string()));
    let divergence = runtime
        .lineage_access()
        .divergence_between_branches(LineageDivergenceRequest {
            left_branch: BranchId("main".to_string()),
            right_branch: BranchId("feature".to_string()),
        });

    assert!(!divergence.right_only_event_ids.is_empty());
    assert!(!divergence.shared_lineage_ids.is_empty());
    assert_eq!(divergence.metrics.right_event_count, 1);
    assert!(divergence.metrics.shared_lineage_count >= 1);
}

#[test]
fn historical_lineage_resolution_is_branch_local_under_divergent_replacements() {
    let mut runtime = runtime_with_test_schema();
    let created = create_entity_outcome(&mut runtime, "source");
    let main_target = create_entity_outcome(&mut runtime, "main-target");
    let entity = changed_entities(&created)[0];
    let start_lineage = runtime
        .lineage_access()
        .for_record(entity)
        .unwrap()
        .lineage_id;
    let main_target_lineage = runtime
        .lineage_access()
        .for_record(changed_entities(&main_target)[0])
        .unwrap()
        .lineage_id;
    runtime
        .history_authority()
        .create_branch(
            BranchId("feature".to_string()),
            &BranchId("main".to_string()),
        )
        .unwrap();
    let feature_target =
        create_entity_outcome_on_branch(&mut runtime, "feature-target", BranchId("feature".to_string()));
    let feature_target_lineage = runtime
        .lineage_access()
        .for_record(changed_entities(&feature_target)[0])
        .unwrap()
        .lineage_id;

    let main_candidate = runtime.lineage_authority().record_correspondence_candidate(
        BranchId("main".to_string()),
        vec![start_lineage],
        vec![main_target_lineage],
        "main-branch-resolution",
    );
    runtime
        .lineage_authority()
        .promote_correspondence(main_candidate.candidate_id, main_target.commit.clone())
        .unwrap();
    let feature_candidate = runtime.lineage_authority().record_correspondence_candidate(
        BranchId("feature".to_string()),
        vec![start_lineage],
        vec![feature_target_lineage],
        "feature-branch-resolution",
    );
    runtime
        .lineage_authority()
        .promote_correspondence(
            feature_candidate.candidate_id,
            feature_target.commit.clone(),
        )
        .unwrap();

    let main_resolution = runtime
        .lineage_access()
        .resolve_historical_lineage(HistoricalResolutionRequest {
            branch_id: BranchId("main".to_string()),
            lineage_id: start_lineage,
        });
    let feature_resolution = runtime
        .lineage_access()
        .resolve_historical_lineage(HistoricalResolutionRequest {
            branch_id: BranchId("feature".to_string()),
            lineage_id: start_lineage,
        });

    assert_ne!(main_resolution.resolved, feature_resolution.resolved);
    assert!(main_resolution.metrics.branch_event_scan_count >= 1);
    assert_eq!(
        main_resolution.metrics.traversed_event_count,
        main_resolution.traversed_event_ids.len()
    );
}
