use crate::{
    AspectLayoutReadRequest, AspectLayoutTarget, AspectProjectionSet, AspectScopeClass,
    ComplexityStatus, ConservativeRetentionPolicy, DerivedFamilyRetentionPolicy, ForgeStore,
    ForgeStoreBuilder, RetentionPolicyClass, SingleEntityAspectScope,
};

use super::harness::fixtures::{
    runtime::{create_entity, latest_envelope, runtime_with_demo_schema},
    stores::{unique_test_sqlite_path, unique_test_store_path},
};

fn store_with_materialized_layout(
    builder: ForgeStoreBuilder,
) -> (ForgeStore, AspectLayoutReadRequest) {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);
    let branch_id = envelope.branch_context.clone();
    let commit_id = envelope.commit.commit_id;

    let mut store = builder.build().unwrap();
    store.append_canonical_commit(envelope).unwrap();
    let request = AspectLayoutReadRequest::new(
        AspectLayoutTarget::new(branch_id, commit_id),
        AspectScopeClass::SingleEntity(SingleEntityAspectScope::new("entity-alpha")),
        AspectProjectionSet::new(vec!["profile".to_string()]),
    );
    store
        .materialize_milestone_6_layout_support(request.clone())
        .unwrap();
    (store, request)
}

fn retention_policy() -> ConservativeRetentionPolicy {
    ConservativeRetentionPolicy::new(
        Vec::new(),
        Vec::new(),
        vec![DerivedFamilyRetentionPolicy::Milestone6LayoutMaterialization],
    )
}

#[test]
fn milestone_10_certification_bundle_reports_clean_verified_loop() {
    let (mut store, request) = store_with_materialized_layout(ForgeStoreBuilder::new().in_memory());
    let control = store.export_authoritative_records();

    let planning = store
        .plan_retention_candidates(RetentionPolicyClass::Conservative(retention_policy()))
        .unwrap();
    let compaction_plan = planning
        .compaction_plans()
        .iter()
        .find(|plan| {
            plan.family_labels()
                .iter()
                .any(|label| label == "milestone_6_layout_materialization")
        })
        .cloned()
        .expect("layout compaction plan");
    let publication = store.publish_compaction_product(compaction_plan).unwrap();
    store
        .verify_compaction_product(publication.product().clone())
        .unwrap();
    store
        .cutover_compaction_product(publication.product().clone())
        .unwrap();
    let reclaim = store
        .execute_derived_reclaim(
            planning
                .reclaim_candidates()
                .iter()
                .find(|witness| witness.artifact_family() == "milestone_6_layout_materialization")
                .cloned()
                .expect("layout reclaim witness"),
        )
        .unwrap();
    store
        .rebuild_reclaimed_derived_family(reclaim.rebuild_unit().clone())
        .unwrap();
    assert!(store.fetch_milestone_6_layout_support(request).is_ok());

    let bundle = store.milestone_10_certification_bundle(&control).unwrap();
    assert!(bundle.certification_summary.truth_matches_control_lane);
    assert!(bundle.certification_summary.restore_truth_parity);
    assert!(bundle.certification_summary.restore_matches_control_lane);
    assert!(
        bundle
            .certification_summary
            .no_unverified_compaction_products
    );
    assert!(bundle.certification_summary.no_uncleared_rebuild_debt);
    assert!(
        bundle
            .certification_summary
            .no_retention_truth_parity_failures
    );
    assert!(
        bundle
            .certification_summary
            .no_retention_restore_parity_failures
    );
    assert!(
        bundle
            .certification_summary
            .no_retention_artifact_rebuild_failures
    );
    assert_eq!(bundle.certification_summary.verified_path_count, 4);
    assert_eq!(bundle.certification_summary.debt_path_count, 0);
}

#[test]
fn milestone_10_certification_bundle_marks_uncut_compaction_after_restart() {
    let path = unique_test_store_path("forge-store-m10-uncut-compaction");
    let (mut store, _request) =
        store_with_materialized_layout(ForgeStoreBuilder::new().local_file(path.clone()));
    let control = store.export_authoritative_records();

    let planning = store
        .plan_retention_candidates(RetentionPolicyClass::Conservative(retention_policy()))
        .unwrap();
    let compaction_plan = planning
        .compaction_plans()
        .iter()
        .find(|plan| {
            plan.family_labels()
                .iter()
                .any(|label| label == "milestone_6_layout_materialization")
        })
        .cloned()
        .expect("layout compaction plan");
    store.publish_compaction_product(compaction_plan).unwrap();
    drop(store);

    let reopened = ForgeStoreBuilder::new().local_file(path).build().unwrap();
    let bundle = reopened
        .milestone_10_certification_bundle(&control)
        .unwrap();

    assert!(bundle.certification_summary.truth_matches_control_lane);
    assert!(bundle.certification_summary.restore_truth_parity);
    assert!(
        !bundle
            .certification_summary
            .no_unverified_compaction_products
    );
    assert_eq!(
        bundle.artifact_report.unverified_compaction_product_count,
        1
    );
    assert_eq!(
        bundle.complexity_surface.compaction_publication.status,
        ComplexityStatus::Debt
    );
    assert_eq!(bundle.certification_summary.debt_path_count, 1);
}

#[test]
fn milestone_10_certification_bundle_tracks_rebuild_debt_across_sqlite_restart() {
    let path = unique_test_sqlite_path("forge-store-m10-rebuild-debt");
    let (mut store, request) =
        store_with_materialized_layout(ForgeStoreBuilder::new().sqlite_file(path.clone()));
    let control = store.export_authoritative_records();

    let planning = store
        .plan_retention_candidates(RetentionPolicyClass::Conservative(retention_policy()))
        .unwrap();
    let compaction_plan = planning
        .compaction_plans()
        .iter()
        .find(|plan| {
            plan.family_labels()
                .iter()
                .any(|label| label == "milestone_6_layout_materialization")
        })
        .cloned()
        .expect("layout compaction plan");
    let publication = store.publish_compaction_product(compaction_plan).unwrap();
    store
        .verify_compaction_product(publication.product().clone())
        .unwrap();
    store
        .cutover_compaction_product(publication.product().clone())
        .unwrap();
    let reclaim = store
        .execute_derived_reclaim(
            planning
                .reclaim_candidates()
                .iter()
                .find(|witness| witness.artifact_family() == "milestone_6_layout_materialization")
                .cloned()
                .expect("layout reclaim witness"),
        )
        .unwrap();
    let rebuild_unit = reclaim.rebuild_unit().clone();
    drop(store);

    let mut reopened = ForgeStoreBuilder::new().sqlite_file(path).build().unwrap();
    let degraded_bundle = reopened
        .milestone_10_certification_bundle(&control)
        .unwrap();
    assert!(
        degraded_bundle
            .certification_summary
            .truth_matches_control_lane
    );
    assert!(degraded_bundle.certification_summary.restore_truth_parity);
    assert!(
        !degraded_bundle
            .certification_summary
            .no_uncleared_rebuild_debt
    );
    assert_eq!(
        degraded_bundle.artifact_report.uncleared_rebuild_debt_count,
        1
    );
    assert_eq!(
        degraded_bundle
            .complexity_surface
            .retained_range_rebuild
            .status,
        ComplexityStatus::Debt
    );

    reopened
        .rebuild_reclaimed_derived_family(rebuild_unit)
        .unwrap();
    assert!(reopened.fetch_milestone_6_layout_support(request).is_ok());

    let rebuilt_bundle = reopened
        .milestone_10_certification_bundle(&control)
        .unwrap();
    assert!(
        rebuilt_bundle
            .certification_summary
            .no_uncleared_rebuild_debt
    );
    assert_eq!(
        rebuilt_bundle.artifact_report.uncleared_rebuild_debt_count,
        0
    );
    assert_eq!(
        rebuilt_bundle
            .complexity_surface
            .retained_range_rebuild
            .status,
        ComplexityStatus::Verified
    );
}
