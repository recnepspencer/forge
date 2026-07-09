use worth_proof::TransitionOutcome;
use worth_store_budgets::CounterEvidenceStrength;
use worth_store_operations::{
    BackupExportCustodyDeclaration, BackupExportCustodyMode, BackupExportCustodyReadiness,
};
use worth_store_physical_isolation::stable_physical_read_plan_for_certification_test;
use worth_store_security::StoreKeyVersionPosture;

use crate::lifecycle::generation_registry_test_support::current_authority;
use crate::publication::test_support::publish_generation_with_bytes_and_chunk_size;
use crate::reachability::hold_test_support::root_candidate_resume_checkpoint;
use crate::test_support::{
    admitted_multichunk_sequence_for_scope, blob_scope, candidate_for_bytes_and_scope,
    canonical_equivalence,
};
use crate::{
    reject_copied_refcount_row_as_reachability, BlobChunkDedupeAdmission,
    BlobChunkDedupeReferenceRegistry, BlobChunkOrdinal, BlobChunkQuarantine,
    BlobChunkReachabilityRegistry, BlobChunkSize, BlobChunkingRuleAdmission,
    BlobCorruptedChunkLocalization, BlobCorruptionDetectionSource, BlobCorruptionGuard,
    BlobCorruptionPlacementClass, BlobCorruptionReferenceEdges, BlobQuarantineAuthority,
    BlobReachabilityDenial, BlobReachabilityEdge, BlobReachabilityEdgeKind,
    BlobReachabilityProtectedHold, BlobReachabilityReclaimDecision, BlobResumeSessionAdmitted,
    BlobResumeSessionDeclaration, BlobResumeStoreAuthority, BlobStreamingContentFrontier,
};
use worth_store_security::StoreTenantScope;
use worth_store_wal::{
    BlobWalRecordIdentity, BlobWalRecordKind, DurablePublicationDeclaration,
    WalFrameDurablePublicationScope,
};

#[test]
fn equivalent_reference_churn_converges_to_same_reachability_snapshot() {
    let (published, sequence) = published_with_sequence("phase14-converge");
    let leaf = sequence.proof_frontier().first_leaf();
    let edge = BlobReachabilityEdge::primary_blob_reference(&published, leaf)
        .expect("primary edge should admit");

    let mut first = BlobChunkReachabilityRegistry::new_store_owned();
    first.admit_edge(edge.clone()).expect("edge should admit");
    first
        .admit_edge(edge.clone())
        .expect("duplicate edge is idempotent");

    let mut second = BlobChunkReachabilityRegistry::new_store_owned();
    second.admit_edge(edge).expect("edge should admit");

    let first_snapshot = first
        .canonical_snapshot()
        .expect("first proof should be nonempty");
    let second_snapshot = second
        .canonical_snapshot()
        .expect("second proof should be nonempty");
    assert_eq!(first_snapshot, second_snapshot);
    assert_eq!(
        first_snapshot.counters().strength(),
        CounterEvidenceStrength::Exact
    );
    assert_eq!(first_snapshot.counters().reference_edges(), 1);
    assert_eq!(first_snapshot.counters().reachable_chunks(), 1);
}

#[test]
fn copied_rows_and_empty_reference_proofs_do_not_mint_reachability() {
    assert!(matches!(
        reject_copied_refcount_row_as_reachability(&3_u64),
        BlobReachabilityDenial::CopiedRefcountRowRejected { .. }
    ));

    let registry = BlobChunkReachabilityRegistry::new_store_owned();
    assert!(matches!(
        registry.prove_reachable_chunks(),
        Err(BlobReachabilityDenial::EmptyReferenceProofRejected { .. })
    ));
}

#[test]
fn wrong_publication_authority_and_stale_generation_edges_are_denied() {
    let (published_a, sequence_a) = published_with_sequence("phase14-authority-a");
    let (published_b, sequence_b) = published_with_sequence("phase14-authority-b");
    let edge_a = BlobReachabilityEdge::primary_blob_reference(
        &published_a,
        sequence_a.proof_frontier().first_leaf(),
    )
    .expect("first edge should admit");
    let edge_b = BlobReachabilityEdge::primary_blob_reference(
        &published_b,
        sequence_b.proof_frontier().first_leaf(),
    )
    .expect("second edge should admit");

    let mut registry = BlobChunkReachabilityRegistry::new_store_owned();
    registry
        .admit_edge(edge_a)
        .expect("first authority sets registry");
    assert!(matches!(
        registry.admit_edge(edge_b),
        Err(BlobReachabilityDenial::WrongBlobAuthority { .. })
    ));
}

#[test]
fn protection_holds_are_visible_and_block_reclaim() {
    let (published, sequence) =
        published_with_sequence_and_bytes("phase14-protection", b"aaaabbbb");
    let leaf = sequence.proof_frontier().first_leaf();
    let edge = BlobReachabilityEdge::primary_blob_reference(&published, leaf)
        .expect("primary edge should admit");
    let mut registry = BlobChunkReachabilityRegistry::new_store_owned();
    registry
        .admit_edge(edge.clone())
        .expect("edge should admit");
    let read_plan = stable_physical_read_plan_for_certification_test(64);
    let guard = corruption_guard("phase14-protection", &published);
    let quarantine = BlobReachabilityProtectedHold::from_corruption_guard(&guard, &published)
        .expect("matching quarantine guard should hold reachability");
    let checkpoint = root_candidate_resume_checkpoint("phase14-protection");
    let resume =
        BlobReachabilityProtectedHold::from_unfinished_resume_checkpoint(&checkpoint, &published)
            .expect("unfinished resume checkpoint should hold reachability");
    assert_eq!(
        quarantine.kind(),
        BlobReachabilityEdgeKind::QuarantineHoldReference
    );
    assert_eq!(
        resume.kind(),
        BlobReachabilityEdgeKind::ResumeSessionReference
    );
    registry
        .admit_stable_read_plan_hold(&read_plan)
        .expect("read hold should admit through registry authority");
    registry
        .admit_hold(quarantine)
        .expect("quarantine hold should admit");
    registry
        .admit_hold(resume)
        .expect("resume hold should admit");

    let proof = registry
        .prove_reachable_chunks()
        .expect("protected reachability should prove");
    assert_eq!(proof.protected_holds().len(), 3);
    assert!(matches!(
        registry.reclaim_decision_for(leaf.identity()),
        BlobReachabilityReclaimDecision::ReclaimDenied(_)
    ));
}

#[test]
fn registry_bound_read_plan_holds_use_existing_reachability_authority() {
    let (published_a, sequence_a) = published_with_sequence("phase14-hold-authority-a");
    let edge = BlobReachabilityEdge::primary_blob_reference(
        &published_a,
        sequence_a.proof_frontier().first_leaf(),
    )
    .expect("primary edge should admit");
    let read_plan = stable_physical_read_plan_for_certification_test(64);
    let mut registry = BlobChunkReachabilityRegistry::new_store_owned();
    registry.admit_edge(edge).expect("edge should admit");
    registry
        .admit_stable_read_plan_hold(&read_plan)
        .expect("registry authority should admit read plan hold");
    let proof = registry
        .prove_reachable_chunks()
        .expect("read-plan hold should remain visible");
    assert_eq!(proof.protected_holds().len(), 1);
}

#[test]
fn unbound_read_plan_holds_cannot_seed_reachability_authority() {
    let read_plan = stable_physical_read_plan_for_certification_test(64);
    let mut registry = BlobChunkReachabilityRegistry::new_store_owned();

    assert!(matches!(
        registry.admit_stable_read_plan_hold(&read_plan),
        Err(BlobReachabilityDenial::InvalidProtectedHold { .. })
    ));
}

#[test]
fn backup_requires_s10_handoff_and_export_uses_s7_readiness() {
    let (published, _) = published_with_sequence("phase14-backup-export-holds");
    let backup = backup_export_readiness("phase14-backup-hold", BackupExportCustodyMode::Backup);
    let export = backup_export_readiness("phase14-export-hold", BackupExportCustodyMode::Export);

    assert!(matches!(
        BlobReachabilityProtectedHold::from_export_readiness(
            &backup,
            crate::reachability::edges::BlobReachabilityAuthorityKey::from_published(&published)
        ),
        Err(BlobReachabilityDenial::InvalidProtectedHold { .. })
    ));
    let export_hold = BlobReachabilityProtectedHold::from_export_readiness(
        &export,
        crate::reachability::edges::BlobReachabilityAuthorityKey::from_published(&published),
    )
    .expect("export readiness should create export hold");

    assert_eq!(
        export_hold.kind(),
        BlobReachabilityEdgeKind::ExportHoldReference
    );
}

#[test]
fn quarantine_and_resume_hold_evidence_cannot_be_laundered_to_another_publication() {
    let (published_a, sequence_a) =
        published_with_sequence_and_bytes("phase14-hold-evidence-a", b"aaaabbbb");
    let (published_b, _) =
        published_with_sequence_and_bytes("phase14-hold-evidence-b", b"ccccdddd");
    let guard_a = corruption_guard("phase14-hold-evidence-a", &published_a);
    assert!(matches!(
        BlobReachabilityProtectedHold::from_corruption_guard(&guard_a, &published_b),
        Err(BlobReachabilityDenial::WrongBlobAuthority { .. })
    ));

    let rootless_checkpoint_a = unfinished_resume_checkpoint(
        "phase14-hold-evidence-a",
        sequence_a.proof_frontier().first_leaf().security_metadata(),
    );
    assert!(matches!(
        BlobReachabilityProtectedHold::from_unfinished_resume_checkpoint(
            &rootless_checkpoint_a,
            &published_a
        ),
        Err(BlobReachabilityDenial::InvalidProtectedHold { .. })
    ));
    let checkpoint_a = root_candidate_resume_checkpoint("phase14-hold-evidence-a");
    assert!(matches!(
        BlobReachabilityProtectedHold::from_unfinished_resume_checkpoint(
            &checkpoint_a,
            &published_b
        ),
        Err(BlobReachabilityDenial::WrongBlobAuthority { .. })
    ));
}

#[test]
fn registered_dedupe_reference_creates_reachability_edge_and_blocks_reclaim() {
    let existing_scope = blob_scope(
        "phase14-dedupe",
        worth_store_security::StoreTenantScope::TenantPhysicalBoundary,
    );
    let candidate_scope = blob_scope(
        "phase14-dedupe",
        worth_store_security::StoreTenantScope::TenantPhysicalBoundary,
    );
    let existing = candidate_for_bytes_and_scope(b"shared bytes", existing_scope);
    let candidate = candidate_for_bytes_and_scope(b"shared bytes", candidate_scope);
    let equivalence = canonical_equivalence(&existing, &candidate);
    let receipt = BlobChunkDedupeAdmission::compare_candidates(existing, candidate)
        .with_foundational_canonical_equivalence(equivalence)
        .admit();
    let receipt = match receipt {
        TransitionOutcome::Success(receipt) => receipt,
        outcome => panic!("dedupe should admit: {outcome:?}"),
    };
    let mut dedupe_registry = BlobChunkDedupeReferenceRegistry::new_store_owned();
    let registered = receipt
        .admit_into_reference_registry(&mut dedupe_registry)
        .expect("registered dedupe reference should mint");
    let (published, sequence) =
        published_with_sequence_and_bytes("phase14-dedupe", b"shared bytes");
    let leaf = sequence.proof_frontier().first_leaf();
    let edge = BlobReachabilityEdge::dedupe_shared_reference(&registered, &published, leaf)
        .expect("registered dedupe reference should create reachability edge");

    let mut reachability = BlobChunkReachabilityRegistry::new_store_owned();
    reachability
        .admit_edge(edge)
        .expect("dedupe edge should admit");
    let proof = reachability
        .prove_reachable_chunks()
        .expect("dedupe edge should prove reachability");
    assert_eq!(proof.counters().dedupe_reference_edges(), 1);
    assert!(matches!(
        reachability.reclaim_decision_for(leaf.identity()),
        BlobReachabilityReclaimDecision::ReclaimDenied(_)
    ));
}

fn published_with_sequence(
    case: &str,
) -> (
    crate::BlobGenerationPublished,
    crate::AdmittedBlobChunkSequence,
) {
    published_with_sequence_and_bytes(case, b"phase14 reachability")
}

fn published_with_sequence_and_bytes(
    case: &str,
    bytes: &[u8],
) -> (
    crate::BlobGenerationPublished,
    crate::AdmittedBlobChunkSequence,
) {
    let scope = blob_scope(
        case,
        worth_store_security::StoreTenantScope::TenantPhysicalBoundary,
    );
    let sequence = crate::test_support::admitted_sequence_for_scope(scope, bytes);
    let (published, _) =
        publish_generation_with_bytes_and_chunk_size(case, bytes, bytes.len() as u64);
    (published, sequence)
}

fn corruption_guard(case: &str, published: &crate::BlobGenerationPublished) -> BlobCorruptionGuard {
    let (_same_published, visible) =
        publish_generation_with_bytes_and_chunk_size(case, b"aaaabbbb", 8);
    let frontier = frontier_for(case, b"aaaabbbb", 8);
    let edges = BlobCorruptionReferenceEdges::from_reachability_staging_identity(
        published.staging_identity(),
    )
    .expect("published reachability staging identity should bind");
    let localized = BlobCorruptedChunkLocalization::from_detected_source(
        BlobCorruptionDetectionSource::VerifiedRead,
        visible,
        frontier,
        BlobChunkOrdinal::first(),
        BlobCorruptionPlacementClass::LocalPhysical,
        edges,
    )
    .expect("published frontier ordinal should localize");
    let quarantine = BlobChunkQuarantine::seal(
        localized,
        BlobQuarantineAuthority::from_current_store_authority(current_authority(
            &format!("{case}.quarantine"),
            "quarantine",
        )),
    );
    BlobCorruptionGuard::from_quarantine(quarantine)
}

fn unfinished_resume_checkpoint(
    case: &str,
    security_metadata: crate::BlobChunkSecurityMetadataWitness,
) -> crate::BlobResumeCheckpoint {
    let rule = BlobChunkingRuleAdmission::fixed_size(BlobChunkSize::from_bytes(4).unwrap())
        .expect("chunking rule should admit");
    let declaration = BlobResumeSessionDeclaration::new(security_metadata, rule, 8)
        .expect("resume declaration should admit");
    let authority = BlobResumeStoreAuthority::from_current_store_authority(current_authority(
        &format!("{case}.resume-authority"),
        "resume",
    ));
    let admitted = BlobResumeSessionAdmitted::admit(declaration, authority);
    admitted
        .export_checkpoint(wal_record(BlobWalRecordKind::SessionCheckpoint, 1, case))
        .expect("unfinished resume checkpoint should export")
}

fn backup_export_readiness(
    case: &str,
    mode: BackupExportCustodyMode,
) -> BackupExportCustodyReadiness {
    let authority = current_authority(case, "backup-export");
    let admission =
        BackupExportCustodyDeclaration::native(&authority, mode, StoreKeyVersionPosture::Current)
            .expect("backup/export declaration should build")
            .admit_with_current_authority(&authority)
            .expect("backup/export declaration should admit");
    BackupExportCustodyReadiness::from_admitted_custody(admission)
        .expect("backup/export readiness should admit")
}

fn frontier_for(case: &str, bytes: &[u8], chunk_size: u64) -> BlobStreamingContentFrontier {
    let sequence = admitted_multichunk_sequence_for_scope(
        blob_scope(case, StoreTenantScope::TenantPhysicalBoundary),
        bytes,
        chunk_size,
    );
    BlobStreamingContentFrontier::from_sequence(&sequence)
}

fn wal_record(
    kind: BlobWalRecordKind,
    sequence: u64,
    case: &str,
) -> worth_store_wal::BlobWalRecordEnvelope {
    let payload = format!("phase14:{case}:{kind:?}:{sequence}");
    let scope = WalFrameDurablePublicationScope::new(9, 1, sequence, sequence + 1, &payload, 64)
        .expect("wal scope should admit");
    worth_store_wal::BlobWalRecordEnvelope::new(
        BlobWalRecordIdentity::new(sequence, kind).expect("wal identity should admit"),
        DurablePublicationDeclaration::wal_frame(scope),
        payload,
    )
    .expect("wal record should admit")
}
