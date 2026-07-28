use worth_proof::TransitionOutcome;
use worth_store_physical_backend::SimulatedStrictDurableProfile;
use worth_store_recovery_physics::{
    CrashBoundaryLayoutReport, LogSequenceNumber, PartialPublicationCrashEdge,
    PartialPublicationReplayedCrashEdge, RecoveryCheckpointRecordSecurityMetadataEnvelope,
    RecoveryCheckpointRecordSecurityMetadataIdentity, RecoveryEntryAdmission,
    RecoveryReplayEntryGate, RecoveryRootSecurityMetadataEnvelope,
    RecoverySecurityScopePropagation, RecoveryWalRecordSecurityMetadataEnvelope,
    RecoveryWalRecordSecurityMetadataIdentity, WalAppendPlan, WalLsnRange, WalSegmentGeneration,
    WalSegmentId,
};
use worth_store_security::{
    admit_store_security_scope, StoreAdmittedSecurityScope, StoreAuthenticityRequirement,
    StoreCustodyPosture, StoreKeyScope, StoreKeyVersionPosture, StoreLegacySecurityPosture,
    StoreSecurityScopeAdmissionExpectation, StoreSecurityScopeAdmissionRequest, StoreTenantScope,
};
use worth_store_wal::{DurablePublicationDeclaration, WalFrameDurablePublicationScope};

use crate::lifecycle::generation_registry_test_support::current_authority;
use crate::publication::evidence::identity::BlobPublicationRecoveryOperationDigest;
use crate::BlobPublicationPreWalReplayEvidence;

pub(crate) fn durable_wal_publication(frame_digest: &str) -> DurablePublicationDeclaration {
    let scope = WalFrameDurablePublicationScope::new(7, 1, 10, 11, frame_digest, 64)
        .expect("wal frame publication scope should admit");
    DurablePublicationDeclaration::wal_frame(scope)
}

pub(crate) fn replayable_wal_classification(frame_digest: &str) -> CrashBoundaryLayoutReport {
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
    CrashBoundaryLayoutReport::admit_crash_edge(
        PartialPublicationCrashEdge::after_durability_before_ack(receipt),
    )
    .expect("phase-22 crash report should admit replayable WAL evidence")
}

pub(crate) fn pre_wal_replay_edge(
    operation_digest: &BlobPublicationRecoveryOperationDigest,
) -> PartialPublicationReplayedCrashEdge {
    with_recovery_replay_entry(operation_digest.as_str(), |replay_entry| {
        let artifact = replay_entry
            .read_partial_publication_before_wal_append()
            .expect("test recovery entry carries protected before-WAL replay bytes");
        PartialPublicationReplayedCrashEdge::from_replay_read_artifact(artifact)
            .expect("test pre-wal replay witness should admit through production readmission")
    })
}

pub(crate) fn with_recovery_entry<R>(
    operation_digest: &str,
    run: impl FnOnce(RecoveryEntryAdmission<'_>) -> R,
) -> R {
    worth_store_test_support::with_admitted_recovery_partial_publication_entry(
        operation_digest,
        run,
    )
}

pub(crate) fn with_recovery_replay_entry<R>(
    operation_digest: &str,
    run: impl FnOnce(RecoveryReplayEntryGate<'_>) -> R,
) -> R {
    with_recovery_entry(operation_digest, |recovery_entry| {
        run(replay_entry_from_recovery_entry(
            operation_digest,
            recovery_entry,
        ))
    })
}

pub(crate) fn with_generic_recovery_replay_entry<R>(
    operation_digest: &str,
    run: impl FnOnce(RecoveryReplayEntryGate<'_>) -> R,
) -> R {
    worth_store_test_support::with_admitted_recovery_entry(operation_digest, |recovery_entry| {
        run(replay_entry_from_recovery_entry(
            operation_digest,
            recovery_entry,
        ))
    })
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

fn replay_entry_from_recovery_entry<'runtime>(
    operation_digest: &str,
    recovery_entry: RecoveryEntryAdmission<'runtime>,
) -> RecoveryReplayEntryGate<'runtime> {
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
        worth_store_security::StoreAuthenticityRequirementClass::AuthenticatedRepairRead,
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
