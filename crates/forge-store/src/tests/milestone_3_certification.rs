use crate::{
    evidence::{Milestone3CertificationBundle, ObservedRecoveryFailure},
    modes::SimulatedCrashPoint,
    DurableMutationRequest, ForgeStore, ForgeStoreBuilder, StoreErrorKind,
};

use super::support::{
    corrupt_first_sqlite_wal_record_digest, create_entity_commit, runtime_with_demo_schema,
    unique_test_sqlite_path,
};

fn create_alpha_commit(
    runtime: &mut forge_relational::facade::runtime::RelationalRuntime,
) -> Result<forge_relational::facade::history::CommitId, crate::StoreError> {
    Ok(create_entity_commit(runtime, "alpha"))
}

fn create_beta_commit(
    runtime: &mut forge_relational::facade::runtime::RelationalRuntime,
) -> Result<forge_relational::facade::history::CommitId, crate::StoreError> {
    Ok(create_entity_commit(runtime, "beta"))
}

#[test]
fn milestone_3_certification_bundle_proves_recovery_and_rebuild_equivalence() {
    let path = unique_test_sqlite_path("forge-store-m3-certification");
    let durable_runtime = runtime_with_demo_schema();
    let mut durable = ForgeStoreBuilder::new()
        .sqlite_file(path.clone())
        .durable_mode(durable_runtime)
        .build()
        .expect("durable store should build");

    durable
        .execute_mutation(DurableMutationRequest::new(
            "create-alpha",
            create_alpha_commit,
        ))
        .expect("first durable commit should acknowledge");
    durable
        .execute_mutation_until_crash(
            DurableMutationRequest::new("create-beta", create_beta_commit),
            SimulatedCrashPoint::AfterCanonicalResultRecorded,
        )
        .expect("second durable commit should crash after recording canonical result");
    drop(durable);

    let recovered = ForgeStoreBuilder::new()
        .sqlite_file(path)
        .durable_mode(runtime_with_demo_schema())
        .build_pending()
        .expect("recovery handle should build")
        .recover()
        .expect("recovery should complete");

    let recovered_export = recovered.store().export_authoritative_records();
    let rebuilt = ForgeStore::rebuild_from_authoritative_export(recovered_export.clone())
        .expect("rebuild should succeed");
    let rebuilt_export = rebuilt.export_authoritative_records();

    let bundle = Milestone3CertificationBundle::new(
        &recovered_export,
        &rebuilt_export,
        recovered.store().counters(),
        &[],
    );

    assert_eq!(bundle.truth_digest, bundle.restore_digest);
    assert_eq!(bundle.failure_digest, stable_failure_digest(&[]));
    assert_eq!(bundle.counter_snapshot.durable_commit_recovered_count, 1);
    assert_eq!(
        bundle
            .counter_snapshot
            .durable_commit_duplicate_suppression_count,
        1
    );
    assert!(bundle.canonical_json().contains(&bundle.truth_digest));
}

#[test]
fn milestone_3_certification_bundle_captures_typed_recovery_failure() {
    let path = unique_test_sqlite_path("forge-store-m3-failure-certification");
    let durable_runtime = runtime_with_demo_schema();
    let mut durable = ForgeStoreBuilder::new()
        .sqlite_file(path.clone())
        .durable_mode(durable_runtime)
        .build()
        .expect("durable store should build");

    durable
        .execute_mutation(DurableMutationRequest::new(
            "create-alpha",
            create_alpha_commit,
        ))
        .expect("durable mutation should acknowledge");
    drop(durable);

    corrupt_first_sqlite_wal_record_digest(&path);

    let failure = ForgeStoreBuilder::new()
        .sqlite_file(path)
        .durable_mode(runtime_with_demo_schema())
        .build_pending()
        .expect_err("corrupted wal should fail before recovery starts");
    assert_eq!(failure.kind(), &StoreErrorKind::WalDigestMismatch);

    let observed_failure = ObservedRecoveryFailure::from_error(&failure);
    let empty_store = ForgeStoreBuilder::new()
        .in_memory()
        .build()
        .expect("empty store should build");
    let export = empty_store.export_authoritative_records();
    let bundle = Milestone3CertificationBundle::new(
        &export,
        &export,
        empty_store.counters(),
        std::slice::from_ref(&observed_failure),
    );

    assert_eq!(
        bundle.failure_digest,
        stable_failure_digest(std::slice::from_ref(&observed_failure))
    );
    assert_ne!(bundle.failure_digest, stable_failure_digest(&[]));
    assert!(bundle.canonical_json().contains(&bundle.failure_digest));
}

fn stable_failure_digest(failures: &[ObservedRecoveryFailure]) -> String {
    use sha2::{Digest, Sha256};

    let bytes = serde_json::to_vec(failures).expect("failure digest serialization");
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}
