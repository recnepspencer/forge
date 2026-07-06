use super::*;
use forge_store_physical_isolation::{BlobOrphanReclaimBarrier, BlobPartialChunkOrphan};

#[test]
fn abandoned_partial_chunks_require_s7_orphan_reclaim_proof() {
    let lane = resume_lane("phase12-orphan", b"aaaabbbbcccc", 12, 12);
    let abandoned =
        BlobResumeSessionAbandoned::abandon(lane.checkpointed.export_checkpoint()).unwrap();
    let coverage = abandoned
        .reclaim_barrier()
        .clone()
        .admit_reclaim_coverage(reclaim_evidence_for_barrier(&abandoned))
        .unwrap();
    let proof = BlobOrphanReclaimProof::from_reclaim_coverage(coverage);
    let reclaimed = crate::BlobResumeSessionReclaimed::reclaim(abandoned, proof).unwrap();
    assert_eq!(reclaimed.counters().reclaims(), 1);
}

#[test]
fn orphan_reclaim_proof_from_another_abandoned_session_is_denied() {
    let abandoned = BlobResumeSessionAbandoned::abandon(
        resume_lane("phase12-orphan-a", b"aaaabbbbcccc", 12, 12)
            .checkpointed
            .export_checkpoint(),
    )
    .unwrap();
    let other = BlobResumeSessionAbandoned::abandon(
        resume_lane("phase12-orphan-b", b"ddddeeeeffff", 12, 12)
            .checkpointed
            .export_checkpoint(),
    )
    .unwrap();
    let unrelated_coverage = other
        .reclaim_barrier()
        .clone()
        .admit_reclaim_coverage(reclaim_evidence_for_barrier(&other))
        .unwrap();
    let unrelated_proof = BlobOrphanReclaimProof::from_reclaim_coverage(unrelated_coverage);

    let result = crate::BlobResumeSessionReclaimed::reclaim(abandoned, unrelated_proof);
    assert_eq!(result, Err(BlobResumeDenial::MissingS7ReclaimProof));
}

#[test]
fn orphan_reclaim_proof_is_minted_only_after_coverage_admission() {
    let lane = resume_lane("phase12-orphan-unbound", b"aaaabbbbcccc", 12, 12);
    let abandoned =
        BlobResumeSessionAbandoned::abandon(lane.checkpointed.export_checkpoint()).unwrap();

    let coverage = abandoned
        .reclaim_barrier()
        .clone()
        .admit_reclaim_coverage(reclaim_evidence_for_barrier(&abandoned))
        .unwrap();
    let proof = BlobOrphanReclaimProof::from_reclaim_coverage(coverage);

    assert_eq!(proof.counters().proofs(), 1);
}

#[test]
fn orphan_reclaim_coverage_denies_reclaim_evidence_for_wrong_physical_reference() {
    let lane = resume_lane("phase12-orphan-wrong-reference", b"aaaabbbbcccc", 12, 12);
    let abandoned =
        BlobResumeSessionAbandoned::abandon(lane.checkpointed.export_checkpoint()).unwrap();
    let wrong_evidence =
        ReclaimEligibilityProof::for_certification_reference(physical_reference(99));

    let result = abandoned
        .reclaim_barrier()
        .clone()
        .admit_reclaim_coverage(wrong_evidence);

    assert!(result.is_err());
}

#[test]
fn orphan_reclaim_denies_proof_with_copied_logical_identity_but_wrong_physical_reference() {
    let lane = resume_lane("phase12-orphan-copied-identity", b"aaaabbbbcccc", 12, 12);
    let abandoned =
        BlobResumeSessionAbandoned::abandon(lane.checkpointed.export_checkpoint()).unwrap();
    let copied = abandoned.reclaim_barrier().orphan();
    let wrong_reference = physical_reference(101);
    let forged_orphan = BlobPartialChunkOrphan::unreached(
        copied.session_digest(),
        copied.chunk_ordinal(),
        copied.chunk_digest(),
        copied.durable_bytes(),
        wrong_reference,
    )
    .unwrap();
    let forged_barrier =
        BlobOrphanReclaimBarrier::from_unreached_orphan(forged_orphan, false).unwrap();
    let forged_coverage = forged_barrier
        .admit_reclaim_coverage(ReclaimEligibilityProof::for_certification_reference(
            wrong_reference,
        ))
        .unwrap();
    let forged_proof = BlobOrphanReclaimProof::from_reclaim_coverage(forged_coverage);

    let result = crate::BlobResumeSessionReclaimed::reclaim(abandoned, forged_proof);

    assert_eq!(result, Err(BlobResumeDenial::MissingS7ReclaimProof));
}
