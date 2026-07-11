use forge_store_budgets::CounterEvidenceStrength;
use forge_store_contracts::DurableArtifactFamilyId;
use forge_store_retention::RetentionDisposition;
use forge_store_security::StoreTenantScope;

use crate::corruption::test_support::quarantined_read_corruption;
use crate::phase24_layout_runtime_case;
use crate::test_support::{
    admitted_sequence_for_scope, blob_scope, candidate_for_bytes_and_scope, canonical_equivalence,
};
use crate::{
    phase25_compacted_rewritten_publication, phase25_compaction_intent, phase25_reclaim_fixture,
    phase25_verified_read_for_rewritten, publish_generation_with_bytes_and_chunk_size,
    BlobChunkDedupeAdmission, BlobChunkReachabilityRegistry, BlobCompactionAuthority,
    BlobCompactionEquivalence, BlobQuarantineRepairCapability, BlobReachabilityEdge,
    BlobRetentionHold, BlobRetentionReclaimRequest, BlobRetentionSafeReclaimPlanner,
    BlobVisibleGeneration,
};
use forge_proof::TransitionOutcome;

use super::{
    reject_chunk_tree_root_as_blob_object_layout_authority,
    reject_full_blob_buffer_as_streaming_layout_authority,
    reject_streaming_frontier_as_chunk_tree_layout_authority, BlobLayoutAccessDenialKind,
};

const CASE: &str = "phase10.streaming.read";
const BYTES: &[u8] = b"abcdefghijkl";

#[test]
fn phase24_layout_admission_uses_published_and_verified_runtime_path() {
    let (published, _, read_request, verified) = phase24_layout_runtime_case(CASE, BYTES, 4, 4);

    let blob = published.admit_blob_object_layout().unwrap();
    let chunk_tree = blob.admit_chunk_tree_layout(&verified).unwrap();
    let streaming = chunk_tree
        .admit_streaming_layout(&read_request, &verified)
        .unwrap();
    let publication = blob.admit_generation_publication_layout();
    let chunk_lookup = chunk_tree.admit_stored_chunk_lookup_layout();
    let resume = streaming.admit_resume_index_layout();

    assert_eq!(
        blob.family_id(),
        forge_store_contracts::DurableArtifactFamilyId::BlobManifest
    );
    assert_eq!(
        chunk_tree.family_id(),
        forge_store_contracts::DurableArtifactFamilyId::BlobChunk
    );
    assert_eq!(
        streaming.family_id(),
        forge_store_contracts::DurableArtifactFamilyId::BlobStream
    );
    assert_eq!(
        publication.family_id(),
        forge_store_contracts::DurableArtifactFamilyId::PublicationWalPublicationProgress
    );
    assert_eq!(
        resume.family_id(),
        forge_store_contracts::DurableArtifactFamilyId::SupportCursor
    );
    assert_eq!(blob.object_id(), verified.object_id());
    assert_eq!(blob.generation(), verified.generation());
    assert_eq!(chunk_tree.chunk_tree_root(), verified.chunk_tree_root());
    assert_eq!(
        chunk_lookup.lookup_chunks(),
        verified.counters().chunks_verified()
    );
    assert_eq!(streaming.bytes_read(), verified.counters().bytes_read());
    assert_eq!(
        streaming.counter_evidence().strength(),
        CounterEvidenceStrength::Exact
    );
    assert_eq!(
        resume.windows_observed(),
        verified.counters().windows_observed()
    );
    streaming.require_constant_memory_window().unwrap();
}

#[test]
fn phase25_layout_admission_uses_runtime_maintenance_authority() {
    let dedupe_scope = blob_scope("phase25-dedupe", StoreTenantScope::TenantPhysicalBoundary);
    let dedupe_existing = candidate_for_bytes_and_scope(BYTES, dedupe_scope);
    let dedupe_candidate = candidate_for_bytes_and_scope(
        BYTES,
        blob_scope(
            "phase25-dedupe-alt",
            StoreTenantScope::TenantPhysicalBoundary,
        ),
    );
    let equivalence = canonical_equivalence(&dedupe_existing, &dedupe_candidate);
    let claim =
        match BlobChunkDedupeAdmission::compare_candidates(dedupe_existing, dedupe_candidate)
            .with_foundational_canonical_equivalence(equivalence)
            .admit()
        {
            TransitionOutcome::Success(claim) => claim,
            other => panic!("dedupe share claim should admit: {other:?}"),
        };
    let dedupe = claim.admit_dedupe_layout().unwrap();

    let reachability_scope = blob_scope(
        "phase25-reachability",
        StoreTenantScope::TenantPhysicalBoundary,
    );
    let sequence = admitted_sequence_for_scope(reachability_scope, BYTES);
    let (published, _) =
        publish_generation_with_bytes_and_chunk_size("phase25-reachability", BYTES, 4);
    let leaf = sequence.proof_frontier().first_leaf();
    let edge = BlobReachabilityEdge::primary_blob_reference(&published, leaf).unwrap();
    let mut registry = BlobChunkReachabilityRegistry::new_store_owned();
    registry.admit_edge(edge).unwrap();
    registry
        .admit_retention_hold(&BlobRetentionHold::generation("phase25-hold"))
        .unwrap();
    let reachability = registry.prove_reachable_chunks().unwrap();
    let reachability_layout = reachability.admit_reachability_layout().unwrap();

    let admission = phase25_reclaim_fixture("phase25-reclaim", 11);
    let request = BlobRetentionReclaimRequest::for_admission(admission);
    let permit = BlobRetentionSafeReclaimPlanner::new_store_owned()
        .plan_reclaim(request)
        .into_permit();
    let retention = reachability.admit_retention_layout().unwrap();
    let reclaim = permit.admit_reclaim_layout().unwrap();
    let (quarantine_published, _) =
        publish_generation_with_bytes_and_chunk_size("phase25-quarantine", b"aaaabbbb", 4);
    let visible = BlobVisibleGeneration::from_published(&quarantine_published);
    let quarantine =
        quarantined_read_corruption("phase25-quarantine", &quarantine_published, visible);
    let quarantine_layout = quarantine.admit_quarantine_layout().unwrap();

    let plan = BlobCompactionAuthority::store_owned()
        .plan_compaction(phase25_compaction_intent("phase25-compaction"))
        .unwrap();
    let rewritten = phase25_compacted_rewritten_publication("phase25-compaction");
    let read = phase25_verified_read_for_rewritten(&plan, &rewritten);
    let equivalence =
        BlobCompactionEquivalence::from_rewritten_root_and_verified_read(&plan, &rewritten, &read)
            .unwrap();
    let compaction = plan.admit_compaction_layout(&equivalence).unwrap();

    assert_eq!(dedupe.family_id(), DurableArtifactFamilyId::DedupeIndex);
    assert_eq!(
        reachability_layout.family_id(),
        DurableArtifactFamilyId::ReachabilityEdge
    );
    assert_eq!(
        retention.family_id(),
        DurableArtifactFamilyId::RetentionHold
    );
    assert_eq!(reclaim.family_id(), DurableArtifactFamilyId::ReclaimReceipt);
    assert_eq!(
        quarantine_layout.family_id(),
        DurableArtifactFamilyId::QuarantineRecord
    );
    assert_eq!(
        compaction.family_id(),
        DurableArtifactFamilyId::MaintenanceCompaction
    );
    assert!(dedupe.requires_rebuild_parity());
    assert!(reachability_layout.requires_exact_counter_evidence());
    assert_eq!(reachability_layout.protected_holds(), 1);
    assert_eq!(retention.protected_holds(), 1);
    assert_eq!(
        reclaim.security_scope(),
        permit.reclaim_policy_evidence().security_scope()
    );
    assert_eq!(
        quarantine_layout.repair_capability(),
        BlobQuarantineRepairCapability::ClassifyGenerationPosture
    );
    reclaim
        .proves_scope_safe_absence_for_chunk(
            permit.chunk_identity(),
            permit.reclaim_policy_evidence().security_scope(),
        )
        .unwrap();
    let wrong_scope = blob_scope(
        "phase25-wrong-scope",
        StoreTenantScope::MultiTenantPhysicalBoundary,
    )
    .identity();
    let absence_scope_denial = reclaim
        .proves_scope_safe_absence_for_chunk(permit.chunk_identity(), wrong_scope)
        .expect_err("reclaim absence proof must stay bound to the released scope");
    assert_eq!(
        absence_scope_denial.kind(),
        BlobLayoutAccessDenialKind::ScopeSafeAbsenceRequiresReclaimReleaseMatch
    );
    assert_eq!(
        compaction.security_metadata(),
        equivalence.security_metadata()
    );
    assert_eq!(
        compaction.authority_classification(),
        equivalence.authority_classification()
    );

    let retention_shortcut_denial = permit
        .admit_retention_layout(RetentionDisposition::Retain)
        .expect_err("reclaim permit cannot stand in for retention layout authority");
    assert_eq!(
        retention_shortcut_denial.kind(),
        BlobLayoutAccessDenialKind::ReclaimReceiptCannotStandInForRetentionLayoutAuthority
    );
}

#[test]
fn phase24_layout_rejects_proxy_authority_inputs() {
    let (published, _, _, _) = phase24_layout_runtime_case(CASE, BYTES, 4, 4);

    let root_denial =
        reject_chunk_tree_root_as_blob_object_layout_authority(published.chunk_tree_root())
            .expect_err("chunk-tree root cannot stand in for blob-object layout");
    assert_eq!(
        root_denial.kind(),
        BlobLayoutAccessDenialKind::ChunkTreeRootCannotStandInForBlobObjectLayoutAuthority
    );

    let (_, _, read_request, _) = phase24_layout_runtime_case(CASE, BYTES, 4, 4);
    let frontier_denial =
        reject_streaming_frontier_as_chunk_tree_layout_authority(read_request.frontier())
            .expect_err("frontier cannot stand in for chunk-tree layout");
    assert_eq!(
        frontier_denial.kind(),
        BlobLayoutAccessDenialKind::StreamingFrontierCannotStandInForChunkTreeLayoutAuthority
    );

    let buffer_denial = reject_full_blob_buffer_as_streaming_layout_authority(BYTES)
        .expect_err("whole-object buffer cannot stand in for streaming layout");
    assert_eq!(
        buffer_denial.kind(),
        BlobLayoutAccessDenialKind::FullBlobBufferCannotStandInForStreamingLayoutAuthority
    );

    let blob = published.admit_blob_object_layout().unwrap();
    let (_, _, _, wrong_verified) =
        phase24_layout_runtime_case("phase10.streaming.unrelated", BYTES, 4, 4);
    let denial = blob
        .admit_chunk_tree_layout(&wrong_verified)
        .expect_err("unrelated verified read should not satisfy chunk-tree layout");
    assert_eq!(
        denial.kind(),
        BlobLayoutAccessDenialKind::PublishedGenerationDoesNotMatchVerifiedRead
    );
}

#[test]
fn phase24_streaming_layout_denies_whole_blob_residency() {
    let (published, _, read_request, verified) =
        phase24_layout_runtime_case(CASE, BYTES, BYTES.len() as u64, BYTES.len() as u64);

    let blob = published.admit_blob_object_layout().unwrap();
    let chunk_tree = blob.admit_chunk_tree_layout(&verified).unwrap();
    let denial = chunk_tree
        .admit_streaming_layout(&read_request, &verified)
        .expect_err("whole-object residency must deny streaming layout");
    assert_eq!(
        denial.kind(),
        BlobLayoutAccessDenialKind::StreamingLayoutRequiresConstantMemory
    );
}
