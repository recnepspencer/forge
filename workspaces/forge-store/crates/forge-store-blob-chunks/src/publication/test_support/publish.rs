use super::wal::{
    chunk_write_replay_evidence, durable_wal_publication, pre_wal_replay_edge,
    replayable_wal_classification,
};
use crate::lifecycle::generation_registry_test_support::{
    current_authority, lifecycle_receipt_for_publication_with_bytes, registry_authority,
    root_publication_with_bytes_and_chunk_size,
};
use crate::{
    BlobAuthorityClassification, BlobGenerationPublished, BlobGenerationRegistry,
    BlobGenerationRegistryAdmission, BlobObjectClassificationAdmission, BlobPublicationAuthority,
    BlobPublicationCrashPoint, BlobPublicationPreWalReplayEvidence, BlobPublicationRecoveredState,
    BlobPublicationRecoveryEvidence, BlobPublicationSessionCloseout, BlobPublicationWalCommit,
    BlobPublicationWalPayload, BlobPublicationWalRecord, BlobReachabilityStaging,
    BlobRootCandidateForPublication, BlobVisibleGeneration,
};

pub(crate) fn publish_generation(case: &str) -> (BlobGenerationPublished, BlobVisibleGeneration) {
    publish_generation_with_bytes_and_chunk_size(case, b"aaaabbbbcccc", 12)
}

pub(crate) fn publish_generation_with_bytes_and_chunk_size(
    case: &str,
    bytes: &[u8],
    chunk_size: u64,
) -> (BlobGenerationPublished, BlobVisibleGeneration) {
    let (candidate, reachability, resumability) =
        publication_inputs_with_bytes_and_chunk_size(case, bytes, chunk_size);
    let staged =
        BlobReachabilityStaging::stage(candidate, reachability).expect("reachability should stage");
    let payload = BlobPublicationWalPayload::from_staged_reachability(&staged);
    let wal_commit = BlobPublicationWalCommit::from_replayable_wal_record(
        staged,
        payload.clone(),
        durable_wal_publication(payload.frame_digest()),
        &replayable_wal_classification(payload.frame_digest()),
    )
    .expect("wal publication commit should admit");
    let wal_record = BlobPublicationWalRecord::append(wal_commit);
    let closeout = BlobPublicationSessionCloseout::close(wal_record, resumability)
        .expect("session closeout should admit");
    let published = BlobGenerationPublished::commit_visible(
        closeout,
        BlobPublicationAuthority::from_current_store_authority(current_authority(
            &format!("{case}.publication"),
            "publication",
        )),
    );
    let visible = BlobVisibleGeneration::from_published(&published);
    (published, visible)
}

pub(crate) fn recovery_cases() -> Vec<(
    BlobPublicationRecoveryEvidence,
    BlobPublicationCrashPoint,
    BlobPublicationRecoveredState,
)> {
    let (candidate, reachability, resumability) = publication_inputs("phase6-recovery");
    let checksum = candidate.intent().logical_content_digest().clone();
    let root = candidate.intent().chunk_tree_root().clone();
    let chunk_write_replay = chunk_write_replay_evidence(&checksum);
    let checksum_replay = checksum_admitted_replay_evidence(&checksum);
    let chunk_tree_replay = chunk_tree_node_durable_replay_evidence(&root);
    let staged =
        BlobReachabilityStaging::stage(candidate, reachability).expect("reachability should stage");
    let reachability_replay = reachability_staged_replay_evidence(&staged);
    let staged_evidence =
        BlobPublicationRecoveryEvidence::reachability_staged(&staged, reachability_replay)
            .expect("pre-wal replay evidence should admit staged reachability recovery");
    let payload = BlobPublicationWalPayload::from_staged_reachability(&staged);
    let wal_commit = BlobPublicationWalCommit::from_replayable_wal_record(
        staged,
        payload.clone(),
        durable_wal_publication(payload.frame_digest()),
        &replayable_wal_classification(payload.frame_digest()),
    )
    .expect("wal publication commit should admit");
    let closeout = BlobPublicationSessionCloseout::close(
        BlobPublicationWalRecord::append(wal_commit),
        resumability,
    )
    .expect("session closeout should admit");

    vec![
        recovery_case(
            BlobPublicationRecoveryEvidence::chunk_write_replayed(&checksum, chunk_write_replay)
                .expect("pre-wal replay evidence should admit chunk recovery"),
            BlobPublicationCrashPoint::AfterChunkWrite,
            BlobPublicationRecoveredState::DurableChunkNotVisible {
                counters: recovery_counters(),
            },
        ),
        recovery_case(
            BlobPublicationRecoveryEvidence::checksum_admitted(&checksum, checksum_replay)
                .expect("pre-wal replay evidence should admit checksum recovery"),
            BlobPublicationCrashPoint::AfterChecksumAdmission,
            BlobPublicationRecoveredState::ChecksumAdmittedNotVisible {
                counters: recovery_counters(),
            },
        ),
        recovery_case(
            BlobPublicationRecoveryEvidence::chunk_tree_node_durable(&root, chunk_tree_replay)
                .expect("pre-wal replay evidence should admit chunk-tree recovery"),
            BlobPublicationCrashPoint::AfterChunkTreeNodeDurability,
            BlobPublicationRecoveredState::ChunkTreeNodeDurableNotVisible {
                counters: recovery_counters(),
            },
        ),
        root_candidate_recovery_case(),
        recovery_case(
            staged_evidence,
            BlobPublicationCrashPoint::AfterReachabilityStaging,
            BlobPublicationRecoveredState::ReachabilityStagedNotVisible {
                counters: recovery_counters(),
            },
        ),
        recovery_case(
            BlobPublicationRecoveryEvidence::publication_record_replayable(
                &replayable_wal_classification("phase6-recovery-record"),
            )
            .expect("replayable wal should admit recovery evidence"),
            BlobPublicationCrashPoint::AfterPublicationRecordWrite,
            BlobPublicationRecoveredState::PublicationRecordReplayableNotVisible {
                counters: recovery_counters(),
            },
        ),
        recovery_case(
            BlobPublicationRecoveryEvidence::session_closed(&closeout),
            BlobPublicationCrashPoint::AfterSessionClose,
            BlobPublicationRecoveredState::SessionClosedAwaitingVisibilityCommit {
                counters: recovery_counters(),
            },
        ),
    ]
}

pub(crate) fn publication_inputs(
    case: &str,
) -> (
    BlobRootCandidateForPublication,
    crate::BlobChunkReachabilityProofSet,
    crate::BlobResumabilityReceipt,
) {
    publication_inputs_with_bytes_and_chunk_size(case, b"aaaabbbbcccc", 12)
}

pub(crate) fn publication_inputs_with_bytes_and_chunk_size(
    case: &str,
    bytes: &[u8],
    chunk_size: u64,
) -> (
    BlobRootCandidateForPublication,
    crate::BlobChunkReachabilityProofSet,
    crate::BlobResumabilityReceipt,
) {
    let (root, stored_digest) = root_publication_with_bytes_and_chunk_size(case, bytes, chunk_size);
    let receipt = lifecycle_receipt_for_publication_with_bytes(
        case,
        root.chunk_tree_root().clone(),
        root.logical_content_digest().clone(),
        stored_digest,
        BlobAuthorityClassification::StoreOwnedPhysicalBlob,
        bytes,
    );
    let object_id = receipt.declaration().object_id().clone();
    let generation = receipt.declaration().generation();
    let mut registry = BlobGenerationRegistry::new();
    let classification = BlobObjectClassificationAdmission::from_executed_lifecycle(&receipt);
    BlobGenerationRegistryAdmission::from_executed_lifecycle(root.clone(), receipt, classification)
        .publish(&mut registry, registry_authority(case))
        .expect("registry publication should admit");
    let observation = registry
        .observe_registered_generation(&object_id, generation)
        .expect("registered generation should observe");
    let reachability = observation.lifecycle_receipt().reachability().clone();
    let resumability = observation.lifecycle_receipt().resumability_receipt();
    let candidate = BlobRootCandidateForPublication::from_registry_observation(observation, root)
        .expect("root candidate should bind registry observation");
    (candidate, reachability, resumability)
}

fn checksum_admitted_replay_evidence(
    digest: &crate::LogicalContentDigest,
) -> BlobPublicationPreWalReplayEvidence {
    let replay = pre_wal_replay_edge(
        &BlobPublicationPreWalReplayEvidence::checksum_admitted_recovery_operation_digest(digest),
    );
    BlobPublicationPreWalReplayEvidence::from_checksum_admitted_replay(digest, &replay)
        .expect("checksum replay evidence should admit")
}

fn chunk_tree_node_durable_replay_evidence(
    root: &crate::ChunkTreeRoot,
) -> BlobPublicationPreWalReplayEvidence {
    let replay = pre_wal_replay_edge(
        &BlobPublicationPreWalReplayEvidence::chunk_tree_node_durable_recovery_operation_digest(
            root,
        ),
    );
    BlobPublicationPreWalReplayEvidence::from_chunk_tree_node_durable_replay(root, &replay)
        .expect("chunk-tree replay evidence should admit")
}

fn root_candidate_replay_evidence(
    candidate: &BlobRootCandidateForPublication,
) -> BlobPublicationPreWalReplayEvidence {
    let replay = pre_wal_replay_edge(
        &BlobPublicationPreWalReplayEvidence::root_candidate_recovery_operation_digest(candidate),
    );
    BlobPublicationPreWalReplayEvidence::from_root_candidate_replay(candidate, &replay)
        .expect("root-candidate replay evidence should admit")
}

fn reachability_staged_replay_evidence(
    staged: &BlobReachabilityStaging,
) -> BlobPublicationPreWalReplayEvidence {
    let replay = pre_wal_replay_edge(
        &BlobPublicationPreWalReplayEvidence::reachability_staged_recovery_operation_digest(staged),
    );
    BlobPublicationPreWalReplayEvidence::from_reachability_staged_replay(staged, &replay)
        .expect("pre-wal replay evidence should admit")
}

fn root_candidate_recovery_case() -> (
    BlobPublicationRecoveryEvidence,
    BlobPublicationCrashPoint,
    BlobPublicationRecoveredState,
) {
    let (candidate, _, _) = publication_inputs("phase6-recovery-root");
    let pre_wal = root_candidate_replay_evidence(&candidate);
    recovery_case(
        BlobPublicationRecoveryEvidence::root_candidate(&candidate, pre_wal)
            .expect("pre-wal replay evidence should admit root candidate recovery"),
        BlobPublicationCrashPoint::AfterRootCandidateFormation,
        BlobPublicationRecoveredState::RootCandidateNotVisible {
            counters: recovery_counters(),
        },
    )
}

fn recovery_case(
    evidence: BlobPublicationRecoveryEvidence,
    crash_point: BlobPublicationCrashPoint,
    state: BlobPublicationRecoveredState,
) -> (
    BlobPublicationRecoveryEvidence,
    BlobPublicationCrashPoint,
    BlobPublicationRecoveredState,
) {
    (evidence, crash_point, state)
}

fn recovery_counters() -> crate::BlobPublicationCounterSnapshot {
    crate::BlobPublicationCounterSnapshot::start().with_recovered_state()
}
