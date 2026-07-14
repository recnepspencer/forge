use super::{
    blob_harness_closeout_sources_for_certification, evaluate_blob_closeout_request,
    BlobCloseoutCertificationInput, BlobCloseoutDenial, BlobCloseoutEvidencePolicy,
    BlobCloseoutRequest, BlobCloseoutShortcutAttempt, BlobCloseoutShortcutInput,
};
use worth_store_physical_certification::{BlobHarnessProfile, BlobHarnessScenarioSeed};

#[test]
fn blob_closeout_binds_executed_harness_evidence() {
    let sources = blob_harness_closeout_sources_for_certification(heavy_seed()).unwrap();
    let closeout = evaluate_blob_closeout_request(BlobCloseoutRequest::Canonical(
        BlobCloseoutCertificationInput::from_executed_sources(
            sources,
            BlobCloseoutEvidencePolicy::counter_backed_foundational(),
        ),
    ))
    .unwrap();

    assert!(closeout
        .materialized_evidence()
        .proof_summary()
        .checked_execution());
    assert!(!closeout.binding_tag().is_empty());
    assert!(closeout.declared_chunk_count() > 0);
}

#[test]
fn blob_closeout_rejects_copied_evidence() {
    let denial = evaluate_blob_closeout_request(BlobCloseoutRequest::Shortcut(
        BlobCloseoutShortcutInput::CopiedReceipt,
    ))
    .unwrap_err();
    assert!(matches!(
        denial,
        BlobCloseoutDenial::ShortcutRejected(report)
            if report.attempt() == BlobCloseoutShortcutAttempt::CopiedReceipt
    ));
}

fn heavy_seed() -> BlobHarnessScenarioSeed {
    BlobHarnessScenarioSeed::builder()
        .profile(BlobHarnessProfile::heavy_multi_gb())
        .placement_external()
        .security_scope_preserving()
        .read_only_access()
        .seed_actor_mix()
        .build()
        .unwrap()
}
