use crate::{WORTHStore, WORTHStoreBuilder, PolicyExpiredAuthorityRange, StoreErrorKind};
use worth_relational::facade::history::BranchId;
use sha2::{Digest, Sha256};

use super::harness::fixtures::runtime::{
    create_entity, latest_envelope, runtime_with_demo_schema, update_entity_on_branch,
};

#[test]
fn authoritative_reclaim_deletes_detached_branch_history() {
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
    let feature_head_digest = {
        let normalized = serde_json::to_value(feature_head).expect("feature head value");
        let bytes = serde_json::to_vec(&normalized).expect("feature head bytes");
        format!("{:x}", Sha256::digest(bytes))
    };
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

    assert_eq!(report.reclaim_unit().branch_id(), &feature);
    assert!(report.deleted_artifact_count() >= 2);
    assert!(detached_store
        .fetch_canonical_commit(feature_commit.commit.commit_id)
        .is_err());
    assert_eq!(
        detached_store
            .counters()
            .reclaimed_authoritative_artifact_count,
        report.deleted_artifact_count()
    );
}

#[test]
fn authoritative_reclaim_rejects_parent_of_surviving_commit() {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let first = latest_envelope(&runtime);
    create_entity(&mut runtime, "beta");
    let second = latest_envelope(&runtime);

    let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
    store.append_canonical_commit(first.clone()).unwrap();
    store.append_canonical_commit(second.clone()).unwrap();

    let error = store
        .execute_authoritative_reclaim(PolicyExpiredAuthorityRange::new(
            second.branch_context.clone(),
            Some(second.commit.commit_id),
            vec![first.commit.commit_id],
        ))
        .unwrap_err();

    assert_eq!(error.kind(), &StoreErrorKind::PolicyExpiredRangeIllegal);
    assert!(store.fetch_canonical_commit(first.commit.commit_id).is_ok());
}
