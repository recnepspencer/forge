use forge_proof::TransitionOutcome;
use forge_store_physical_backend::SimulatedStrictDurableProfile;
use forge_store_recovery_physics::{
    LogSequenceNumber, PartialPublicationClassification, PartialPublicationCrashEdge,
    PartialPublicationEvidence, PartialPublicationReplayedCrashEdge,
    RecoveryCheckpointRecordSecurityMetadataEnvelope,
    RecoveryCheckpointRecordSecurityMetadataIdentity, RecoveryEntryAdmission,
    RecoveryReplayEntryGate, RecoveryRootSecurityMetadataEnvelope,
    RecoverySecurityScopePropagation, RecoveryWalRecordSecurityMetadataEnvelope,
    RecoveryWalRecordSecurityMetadataIdentity, WalAppendPlan, WalLsnRange, WalSegmentGeneration,
    WalSegmentId,
};
use forge_store_security::{
    admit_store_security_scope, StoreAdmittedSecurityScope, StoreAuthenticityRequirement,
    StoreCustodyPosture, StoreKeyScope, StoreKeyVersionPosture, StoreLegacySecurityPosture,
    StoreSecurityScopeAdmissionExpectation, StoreSecurityScopeAdmissionRequest, StoreTenantScope,
};
use forge_store_wal::{DurablePublicationDeclaration, WalFrameDurablePublicationScope};

use crate::blob_generation_registry_test_support::{
    current_authority, lifecycle_receipt_for_publication_with_bytes, registry_authority,
    root_publication_with_bytes_and_chunk_size,
};
use crate::blob_publication_commit::evidence_identity::BlobPublicationRecoveryOperationDigest;
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

pub(crate) fn durable_wal_publication(frame_digest: &str) -> DurablePublicationDeclaration {
    let scope = WalFrameDurablePublicationScope::new(7, 1, 10, 11, frame_digest, 64)
        .expect("wal frame publication scope should admit");
    DurablePublicationDeclaration::wal_frame(scope)
}

pub(crate) fn replayable_wal_classification(
    frame_digest: &str,
) -> PartialPublicationClassification {
    let plan = WalAppendPlan::<SimulatedStrictDurableProfile>::new(
        WalSegmentId::new(7).unwrap(),
        WalSegmentGeneration::new(1).unwrap(),
        WalLsnRange::new(LogSequenceNumber::new(10), LogSequenceNumber::new(11)).unwrap(),
        frame_digest,
        64,
    )
    .expect("wal append plan should admit");
    let receipt = plan
        .record_written_bytes(64)
        .finish()
        .expect("wal append receipt should finish");
    PartialPublicationClassification::classify(
        PartialPublicationEvidence::from_persisted_crash_edge(
            PartialPublicationCrashEdge::after_durability_before_ack(receipt),
        ),
    )
}

fn pre_wal_replay_edge(
    operation_digest: &BlobPublicationRecoveryOperationDigest,
) -> PartialPublicationReplayedCrashEdge {
    let replay_entry = recovery_replay_entry(operation_digest.as_str());
    let artifact = replay_entry
        .read_partial_publication_before_wal_append()
        .expect("test recovery entry carries protected before-WAL replay bytes");
    PartialPublicationReplayedCrashEdge::from_replay_read_artifact(artifact)
        .expect("test pre-wal replay witness should admit through production readmission")
}

pub(crate) fn recovery_entry(operation_digest: &str) -> RecoveryEntryAdmission {
    forge_store_test_support::admitted_s4_partial_publication_recovery_entry(operation_digest)
}

pub(crate) fn recovery_replay_entry(operation_digest: &str) -> RecoveryReplayEntryGate {
    let recovery_entry = recovery_entry(operation_digest);
    replay_entry_from_recovery_entry(operation_digest, recovery_entry)
}

pub(crate) fn generic_recovery_replay_entry(operation_digest: &str) -> RecoveryReplayEntryGate {
    let recovery_entry = forge_store_test_support::admitted_s4_recovery_entry(operation_digest);
    replay_entry_from_recovery_entry(operation_digest, recovery_entry)
}

fn replay_entry_from_recovery_entry(
    operation_digest: &str,
    recovery_entry: RecoveryEntryAdmission,
) -> RecoveryReplayEntryGate {
    let admitted_scope = recovery_security_scope(operation_digest);
    let wal_record = RecoveryWalRecordSecurityMetadataEnvelope::from_admitted_scope(
        RecoveryWalRecordSecurityMetadataIdentity::new(7),
        &admitted_scope,
        StoreKeyVersionPosture::Current,
        StoreLegacySecurityPosture::NativeScoped,
    );
    let checkpoint_record = RecoveryCheckpointRecordSecurityMetadataEnvelope::from_admitted_scope(
        RecoveryCheckpointRecordSecurityMetadataIdentity::new(1),
        &admitted_scope,
        StoreKeyVersionPosture::Current,
        StoreLegacySecurityPosture::NativeScoped,
    );
    let recovery_root = RecoveryRootSecurityMetadataEnvelope::from_recovery_entry(
        &recovery_entry,
        &admitted_scope,
        StoreKeyVersionPosture::Current,
        StoreLegacySecurityPosture::NativeScoped,
    );
    let security_scope = match RecoverySecurityScopePropagation::admit_required(
        Some(&wal_record),
        Some(&checkpoint_record),
        Some(&recovery_root),
        &recovery_entry,
    ) {
        TransitionOutcome::Success(security_scope) => security_scope,
        outcome => panic!("recovery security scope should propagate: {outcome:?}"),
    };
    match RecoveryReplayEntryGate::before_source_precedence(recovery_entry, security_scope) {
        TransitionOutcome::Success(replay_entry) => replay_entry,
        outcome => panic!("recovery replay entry gate should admit: {outcome:?}"),
    }
}

fn recovery_security_scope(operation_digest: &str) -> StoreAdmittedSecurityScope {
    let authority = current_authority(
        &format!("{operation_digest}.recovery-replay-scope"),
        "recovery-replay",
    );
    let key_scope = StoreKeyScope::RepairScopeEnvelope;
    let tenant_scope = StoreTenantScope::RepairBlastRadius;
    let authenticity = StoreAuthenticityRequirement::required(
        forge_store_security::StoreAuthenticityRequirementClass::AuthenticatedRepairRead,
    );
    let custody = StoreCustodyPosture::InternalStoreCustody;
    let expectation =
        StoreSecurityScopeAdmissionExpectation::new(key_scope, tenant_scope, authenticity, custody);
    let request = StoreSecurityScopeAdmissionRequest::new(
        &authority,
        key_scope,
        StoreKeyVersionPosture::Current,
        tenant_scope,
        authenticity,
        custody,
        expectation,
    );
    match admit_store_security_scope(request) {
        TransitionOutcome::Success(admitted_scope) => admitted_scope,
        outcome => panic!("recovery security scope should admit: {outcome:?}"),
    }
}

pub(crate) fn chunk_write_replay_evidence(
    digest: &crate::LogicalContentDigest,
) -> BlobPublicationPreWalReplayEvidence {
    let replay = pre_wal_replay_edge(
        &BlobPublicationPreWalReplayEvidence::chunk_write_recovery_operation_digest(digest),
    );
    BlobPublicationPreWalReplayEvidence::from_chunk_write_replay(digest, &replay)
        .expect("chunk-write replay evidence should admit")
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
