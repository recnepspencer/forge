use crate::{
    AspectLayoutReadRequest, AspectLayoutTarget, AspectProjectionSet, AspectScopeClass,
    ConservativeRetentionPolicy, DerivedFamilyRetentionPolicy, WORTHStore, WORTHStoreBuilder,
    PolicyExpiredAuthorityRange, RetentionPolicyClass, SingleEntityAspectScope,
};
use worth_relational::facade::history::BranchId;
use sha2::{Digest, Sha256};

use super::harness::fixtures::runtime::{
    create_entity, latest_envelope, runtime_with_demo_schema, update_entity_on_branch,
};

fn store_with_materialized_layout() -> (WORTHStore, String) {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);

    let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
    store.append_canonical_commit(envelope.clone()).unwrap();
    let request = AspectLayoutReadRequest::new(
        AspectLayoutTarget::new(envelope.branch_context.clone(), envelope.commit.commit_id),
        AspectScopeClass::SingleEntity(SingleEntityAspectScope::new("entity-alpha")),
        AspectProjectionSet::new(vec!["profile".to_string()]),
    );
    let materialization = store
        .materialize_milestone_6_layout_support(request)
        .unwrap();
    (store, materialization.artifact_id().to_string())
}

fn digest_json<T: serde::Serialize>(value: &T) -> String {
    let normalized = serde_json::to_value(value).expect("json value");
    let bytes = serde_json::to_vec(&normalized).expect("json bytes");
    format!("{:x}", Sha256::digest(bytes))
}

#[test]
fn derived_reclaim_and_rebuild_reports_publish_verification_verdicts() {
    let (mut store, artifact_id) = store_with_materialized_layout();
    let planning = store
        .plan_retention_candidates(RetentionPolicyClass::Conservative(
            ConservativeRetentionPolicy::new(
                Vec::new(),
                Vec::new(),
                vec![DerivedFamilyRetentionPolicy::Milestone6LayoutMaterialization],
            ),
        ))
        .unwrap();
    let witness = planning
        .reclaim_candidates()
        .iter()
        .find(|witness| witness.artifact_family() == "milestone_6_layout_materialization")
        .cloned()
        .expect("layout reclaim witness");

    let reclaim = store.execute_derived_reclaim(witness).unwrap();
    assert!(reclaim.verification().restore_truth_parity());
    let reclaim_target = reclaim
        .verification()
        .target_state()
        .expect("derived reclaim verification target");
    assert_eq!(
        reclaim_target.family_label(),
        "milestone_6_layout_materialization"
    );
    assert_eq!(reclaim_target.target_id(), artifact_id);
    assert!(!reclaim_target.expected_present());
    assert!(!reclaim_target.observed_present());
    assert!(reclaim_target.matches_expectation());

    let rebuild = store
        .rebuild_reclaimed_derived_family(reclaim.rebuild_unit().clone())
        .unwrap();
    assert!(rebuild.verification().restore_truth_parity());
    let rebuild_target = rebuild
        .verification()
        .target_state()
        .expect("rebuild verification target");
    assert_eq!(
        rebuild_target.family_label(),
        "milestone_6_layout_materialization"
    );
    assert_eq!(rebuild_target.target_id(), artifact_id);
    assert!(rebuild_target.expected_present());
    assert!(rebuild_target.observed_present());
    assert!(rebuild_target.matches_expectation());
}

#[test]
fn authoritative_reclaim_report_publishes_restore_parity_verdict() {
    let mut runtime = runtime_with_demo_schema();
    let entity_id = create_entity(&mut runtime, "alpha");
    let initial = latest_envelope(&runtime);

    let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
    store.append_canonical_commit(initial.clone()).unwrap();
    let feature = BranchId("feature".to_string());
    runtime
        .history_authority()
        .create_branch(feature.clone(), &initial.branch_context)
        .unwrap();
    store
        .create_branch(feature.clone(), Some(&initial.branch_context))
        .unwrap();

    update_entity_on_branch(&mut runtime, entity_id, "beta", Some(feature.clone()));
    let feature_commit = latest_envelope(&runtime);
    store
        .append_canonical_commit(feature_commit.clone())
        .unwrap();

    let mut export = store.export_authoritative_records();
    let feature_head = export
        .branch_head_records
        .iter_mut()
        .find(|record| record.branch_id == feature)
        .expect("feature head");
    feature_head.head_commit_id = None;
    feature_head.head_commit_digest = None;
    let feature_head_digest = digest_json(feature_head);
    let digest_record = export
        .authoritative_artifact_digests
        .iter_mut()
        .find(|record| {
            record.artifact_family
                == crate::backend::records::AuthoritativeArtifactFamily::BranchHeadRecord
                && record.artifact_id == feature.0
        })
        .expect("feature head digest record");
    digest_record.artifact_digest = feature_head_digest;

    let mut detached_store =
        WORTHStore::restore_from_authoritative_export(export.admit_restore()).unwrap();
    let report = detached_store
        .execute_authoritative_reclaim(PolicyExpiredAuthorityRange::new(
            feature.clone(),
            None,
            vec![feature_commit.commit.commit_id],
        ))
        .unwrap();

    assert!(report.verification().restore_truth_parity());
    assert!(report.verification().target_state().is_none());
    assert_eq!(
        report.verification().operation_label(),
        "execute_authoritative_reclaim"
    );
}
