use forge_store_physical_isolation::{
    compaction_cutover_evidence_for_certification_plan,
    compaction_cutover_evidence_for_certification_rewrite_manifest,
    compaction_read_interlock_plan_for_certification_test,
    stable_physical_read_receipt_for_compaction_plan_test,
    stable_physical_read_receipt_for_mismatched_compaction_test,
};

use crate::test_support::{
    admitted_sequence_for_scope, blob_scope, candidate_for_bytes_and_scope, canonical_equivalence,
};
use crate::lifecycle::generation_registry_test_support::{
    current_authority, lifecycle_receipt_for_publication_with_bytes, root_publication_with_bytes,
};
use crate::placement::admission::test_support::admit_inline_placement;
use crate::publication::test_support::publish_generation_with_bytes_and_chunk_size;
use crate::{
    BlobAuthorityClassification, BlobChunkDedupeAdmission, BlobChunkDedupeReferenceRegistry,
    BlobChunkOrdinal, BlobChunkRootPublication, BlobCompactionColdReadiness, BlobCompactionIntent,
    BlobCompactionReadHold, BlobCompactionS6Pacing, BlobCorruptedChunkLocalization,
    BlobCorruptionDetectionSource, BlobCorruptionGuard, BlobCorruptionPlacementClass,
    BlobCorruptionReferenceEdges, BlobQuarantineAuthority, BlobStreamingContentFrontier,
    BlobStreamingVerifiedRead, LifecycleReceipt,
};
use forge_proof::TransitionOutcome;
use forge_store_physical_isolation::{
    execute_read_during_compaction_cutover, CompactionReadInterlockDenial,
    ReadDuringCompactionVerdict,
};
use forge_store_security::StoreTenantScope;

use super::rewrite_binding::physical_rewrite_manifest_epoch_for_root;

pub(crate) const BYTES: &[u8] = b"phase18-compaction-bytes";

pub(crate) fn lifecycle_with_publication(
    case: &str,
) -> (LifecycleReceipt, BlobChunkRootPublication) {
    let (publication, stored) = root_publication_with_bytes(case, BYTES);
    let lifecycle = lifecycle_receipt_for_publication_with_bytes(
        case,
        publication.chunk_tree_root().clone(),
        publication.logical_content_digest().clone(),
        stored,
        BlobAuthorityClassification::StoreOwnedPhysicalBlob,
        BYTES,
    );
    (lifecycle, publication)
}

pub(crate) fn compacted_rewritten_publication(case: &str) -> BlobChunkRootPublication {
    crate::lifecycle::generation_registry_test_support::root_publication_with_bytes_and_chunk_size(
        case, BYTES, 4,
    )
    .0
}

pub(crate) fn rewritten_publication_with_bytes(
    case: &str,
    bytes: &[u8],
) -> BlobChunkRootPublication {
    root_publication_with_bytes(case, bytes).0
}

pub(crate) fn intent(case: &str) -> BlobCompactionIntent {
    let (lifecycle, uncompacted_publication) = lifecycle_with_publication(case);
    let reachability = lifecycle.reachability().clone();
    let placement = admit_inline_placement(&reachability);
    let physical = compaction_read_interlock_plan_for_certification_test();
    let read_hold = read_hold_for_plan(&physical);
    BlobCompactionIntent::for_published_generation(
        lifecycle,
        uncompacted_publication,
        reachability,
        placement,
        read_hold,
        physical,
    )
}

pub(crate) fn intent_without_reachability(case: &str) -> BlobCompactionIntent {
    let (lifecycle, uncompacted_publication) = lifecycle_with_publication(case);
    let placement = admit_inline_placement(lifecycle.reachability());
    let physical = compaction_read_interlock_plan_for_certification_test();
    let read_hold = read_hold_for_plan(&physical);
    BlobCompactionIntent::without_reachability(
        lifecycle,
        uncompacted_publication,
        placement,
        read_hold,
        physical,
    )
}

fn read_hold_for_plan(
    plan: &forge_store_physical_isolation::CompactionReadInterlockPlan,
) -> BlobCompactionReadHold {
    BlobCompactionReadHold::released(stable_physical_read_receipt_for_compaction_plan_test(
        plan, 64,
    ))
}

pub(crate) fn mismatched_read_hold() -> BlobCompactionReadHold {
    BlobCompactionReadHold::released(stable_physical_read_receipt_for_mismatched_compaction_test(
        64,
    ))
}

pub(crate) fn active_read_hold() -> BlobCompactionReadHold {
    BlobCompactionReadHold::active(stable_physical_read_receipt_for_mismatched_compaction_test(
        64,
    ))
}

pub(crate) fn unavailable_cold() -> BlobCompactionColdReadiness {
    BlobCompactionColdReadiness::from_state(
        forge_store_tiering::S7ColdPlacementState::ColdUnavailable,
    )
}

pub(crate) fn physical_interlock_denial() -> CompactionReadInterlockDenial {
    CompactionReadInterlockDenial::QuarantinedCandidateRange
}

pub(crate) fn stale_dedupe_reference(case: &str) -> crate::BlobChunkRegisteredDedupeReference {
    let existing = candidate_for_bytes_and_scope(
        b"other-bytes",
        blob_scope(
            &format!("{case}-existing"),
            StoreTenantScope::TenantPhysicalBoundary,
        ),
    );
    let candidate = candidate_for_bytes_and_scope(
        b"other-bytes",
        blob_scope(
            &format!("{case}-candidate"),
            StoreTenantScope::TenantPhysicalBoundary,
        ),
    );
    let equivalence = canonical_equivalence(&existing, &candidate);
    let receipt = match BlobChunkDedupeAdmission::compare_candidates(existing, candidate)
        .with_foundational_canonical_equivalence(equivalence)
        .admit()
    {
        TransitionOutcome::Success(receipt) => receipt,
        outcome => panic!("dedupe should admit for stale reference fixture: {outcome:?}"),
    };
    let mut registry = BlobChunkDedupeReferenceRegistry::new_store_owned();
    receipt
        .admit_into_reference_registry(&mut registry)
        .expect("registered dedupe reference should mint")
}

pub(crate) fn quarantine_guard(case: &str) -> BlobCorruptionGuard {
    let (published, visible) =
        publish_generation_with_bytes_and_chunk_size(case, BYTES, BYTES.len() as u64);
    let sequence = admitted_sequence_for_scope(
        blob_scope(case, StoreTenantScope::TenantPhysicalBoundary),
        BYTES,
    );
    let frontier = BlobStreamingContentFrontier::from_sequence(&sequence);
    let edges = BlobCorruptionReferenceEdges::from_reachability_staging_identity(
        published.staging_identity(),
    )
    .expect("published reachability staging identity should bind");
    let localization = BlobCorruptedChunkLocalization::from_detected_source(
        BlobCorruptionDetectionSource::VerifiedRead,
        visible,
        frontier,
        BlobChunkOrdinal::first(),
        BlobCorruptionPlacementClass::LocalPhysical,
        edges,
    )
    .expect("published frontier should localize corruption");
    let quarantine = crate::BlobChunkQuarantine::seal(
        localization,
        BlobQuarantineAuthority::from_current_store_authority(current_authority(
            case,
            "quarantine",
        )),
    );
    BlobCorruptionGuard::from_quarantine(quarantine)
}

pub(crate) fn pacing() -> BlobCompactionS6Pacing {
    BlobCompactionS6Pacing::admitted_compaction(2)
}

pub(crate) fn verified_read_for_rewritten(
    plan: &crate::BlobCompactionRewritePlan,
    rewritten: &BlobChunkRootPublication,
) -> BlobStreamingVerifiedRead {
    BlobStreamingVerifiedRead::for_movement_certification_test(
        plan.basis().object_id().clone(),
        plan.basis().generation(),
        rewritten.chunk_tree_root().clone(),
        plan.basis().logical_digest().clone(),
        rewritten.canonical_basis().total_bytes(),
    )
}

pub(crate) fn verdict_for_plan(
    plan: &crate::BlobCompactionRewritePlan,
) -> ReadDuringCompactionVerdict {
    let evidence = compaction_cutover_evidence_for_certification_plan(plan.physical());
    admit_verdict_from_evidence(evidence)
}

pub(crate) fn verdict_for_rewrite(
    plan: &crate::BlobCompactionRewritePlan,
    rewritten: &BlobChunkRootPublication,
) -> ReadDuringCompactionVerdict {
    let evidence = compaction_cutover_evidence_for_certification_rewrite_manifest(
        plan.physical(),
        physical_rewrite_manifest_epoch_for_root(
            rewritten.chunk_tree_root(),
            plan.physical().protected().root().manifest_epoch().get(),
        ),
    );
    admit_verdict_from_evidence(evidence)
}

pub(crate) fn mismatched_verdict_for_rewrite(
    plan: &crate::BlobCompactionRewritePlan,
    rewritten: &BlobChunkRootPublication,
) -> ReadDuringCompactionVerdict {
    let evidence = compaction_cutover_evidence_for_certification_rewrite_manifest(
        plan.physical(),
        physical_rewrite_manifest_epoch_for_root(
            rewritten.chunk_tree_root(),
            plan.physical().protected().root().manifest_epoch().get(),
        )
        .wrapping_add(1),
    );
    admit_verdict_from_evidence(evidence)
}

fn admit_verdict_from_evidence(
    evidence: forge_store_physical_isolation::CompactionCutoverEvidenceForCertification,
) -> ReadDuringCompactionVerdict {
    let (publication, recovery, pre_cutover_read, post_cutover_read) = evidence.into_parts();
    execute_read_during_compaction_cutover(
        publication,
        recovery,
        pre_cutover_read,
        post_cutover_read,
    )
    .expect("read-during-compaction verdict should admit")
}
