mod phase_eight;
mod phase_seven;
mod phase_six;

use super::super::read_repository_document;
use super::locked_surfaces::PHASE_TWO_PREPARATION_EXPORTS;

const PHASE_TWO_DURABILITY_ROOT_EXPORTS: [&str; 29] = [
    "AdmittedPhysicalDurabilityPolicy",
    "CheckpointMemoryLimit",
    "GroupCommitDelay",
    "GroupCommitLimit",
    "IdempotencyRetentionGenerations",
    "PendingUnresolvedMutationLimit",
    "PhysicalCheckpointPolicy",
    "PhysicalDurabilityDeclaration",
    "PhysicalDurabilityDeclarationBuilder",
    "PhysicalDurabilityObservation",
    "PhysicalDurabilityPolicyAdmissionOutcome",
    "PhysicalDurabilityPolicyDeferred",
    "PhysicalDurabilityPolicyDenial",
    "PhysicalDurabilityPolicyFailure",
    "PhysicalDurabilityPolicyIdentity",
    "PhysicalDurabilityPolicyRebindRequired",
    "PhysicalDurabilityPolicyStale",
    "PhysicalIdempotencyPolicy",
    "PhysicalMutationDeadline",
    "PhysicalMutationIdempotencyIssuanceDenial",
    "PhysicalMutationIdempotencyKey",
    "PhysicalMutationIdempotencyKeyIdentity",
    "PhysicalMutationIdempotencyLease",
    "PhysicalMutationIdempotencyMaterial",
    "PhysicalMutationIdentity",
    "PhysicalMutationRequest",
    "PhysicalMutationRequestFingerprint",
    "PhysicalNamespaceDurableCheckpointGeneration",
    "RetainedWalTailLimit",
];

const PHASE_THREE_RUNTIME_EXPORTS: [&str; 11] = [
    "CanonicalRedoRecords",
    "PhysicalWalAppendDeclaration",
    "PhysicalWalAppendFailureCause",
    "PhysicalWalAppendSettlement",
    "PhysicalWalMemberBasis",
    "PhysicalWalMemberIdentity",
    "PhysicalWalObservation",
    "PhysicalWalReservationDenial",
    "RedoRecord",
    "WalAppendedPhysicalMutation",
    "WalRangeReservedPhysicalMutation",
];

const PHASE_FOUR_RUNTIME_EXPORTS: [&str; 19] = [
    "CertifiedPriorPageBasis",
    "CertifiedPriorPageImage",
    "DataDispatchedPhysicalMutation",
    "DataSettledPhysicalMutation",
    "IndeterminatePhysicalDataDispatch",
    "PageWalBasis",
    "PhysicalDataDispatchFailureCause",
    "PhysicalDataDispatchOutcome",
    "PhysicalDataEffectSettlement",
    "PhysicalDataEffectSource",
    "PhysicalDataFrameIdentity",
    "PhysicalDataFrameKind",
    "PhysicalDataFrameSubject",
    "PhysicalDataSettlementFailureCause",
    "PhysicalDataSettlementOutcome",
    "PhysicalRedoLsn",
    "PhysicalRedoTargetClaim",
    "PhysicalWalBarrierSettlement",
    "WalDurablePhysicalMutation",
];

const PHASE_FIVE_RUNTIME_EXPORTS: [&str; 30] = [
    "AdmittedPhysicalDurabilityGroup",
    "AdmittedPhysicalDurabilityGroupMember",
    "IndeterminatePhysicalWalGroupAppend",
    "IndeterminatePhysicalWalGroupBarrier",
    "PhysicalDurabilityGroupAdmissionDenial",
    "PhysicalDurabilityGroupAdmissionOutcome",
    "PhysicalDurabilityGroupBasis",
    "PhysicalDurabilityGroupIdentity",
    "PhysicalDurabilityGroupMemberBinding",
    "PhysicalDurabilityGroupSealingDenial",
    "PhysicalGroupAppendAmplificationObservation",
    "PhysicalGroupBarrierAmplificationObservation",
    "PhysicalGroupMemberOrdinal",
    "PhysicalGroupQueueAdmissionTick",
    "PhysicalGroupRootPublicationPlan",
    "PhysicalWalGroupAppendContinuation",
    "PhysicalWalGroupAppendFailureCause",
    "PhysicalWalGroupAppendOutcome",
    "PhysicalWalGroupBarrierDeclaration",
    "PhysicalWalGroupBarrierDeclarationDenial",
    "PhysicalWalGroupBarrierFailureCause",
    "PhysicalWalGroupBarrierOutcome",
    "PhysicalWalGroupBarrierSettlement",
    "PhysicalMutationProvenNoEffectCause",
    "ProvenNoEffectPhysicalMutation",
    "RejectedPhysicalDurabilityGroup",
    "SealedPhysicalDurabilityGroupMembers",
    "SharedPhysicalRootPublicationPlan",
    "WalBarrierMember",
    "WalDurablePhysicalMutationMembers",
];

pub(super) fn assert_facade_reachability() {
    let runtime = read_repository_document(
        "workspaces/worth-store/crates/worth-store/src/physical_runtime/mod.rs",
    )
    .expect("read physical runtime facade");
    let durability_exports = export_block(&runtime, "pub use durability::{");
    assert_phase_two_reachability(&runtime, durability_exports);
    assert_phase_three_reachability(durability_exports);
    assert_phase_four_reachability(durability_exports);
    assert_phase_five_reachability(durability_exports);
    phase_six::assert_reachability(durability_exports);
    phase_seven::assert_reachability(durability_exports);
    phase_eight::assert_reachability(durability_exports);
}

fn assert_phase_two_reachability(runtime: &str, durability_exports: &str) {
    for surface in PHASE_TWO_DURABILITY_ROOT_EXPORTS {
        assert!(
            durability_exports.contains(surface),
            "Phase 2 durability surface `{surface}` is not exported by physical_runtime"
        );
    }
    assert!(runtime.contains("pub use record_serving::*;"));

    let serving = read_repository_document(
        "workspaces/worth-store/crates/worth-store/src/physical_runtime/record_serving/mod.rs",
    )
    .expect("read record-serving facade");
    let serving_exports = export_block(&serving, "pub use publication::{");
    assert!(serving_exports.contains("PhysicalRecordSubmission"));
    for surface in PHASE_TWO_PREPARATION_EXPORTS {
        assert!(
            serving_exports.contains(surface),
            "Phase 2 preparation surface `{surface}` is not exported by record_serving"
        );
    }

    let publication = read_repository_document(
        "workspaces/worth-store/crates/worth-store/src/physical_runtime/record_serving/publication/mod.rs",
    )
    .expect("read publication facade");
    assert!(publication.contains("pub use director::PhysicalRecordSubmission;"));
    let preparation_exports = export_block(&publication, "pub use durable_preparation::{");
    for surface in PHASE_TWO_PREPARATION_EXPORTS {
        assert!(
            preparation_exports.contains(surface),
            "Phase 2 preparation surface `{surface}` is not exported by publication"
        );
    }
}

fn assert_phase_three_reachability(durability_exports: &str) {
    for surface in PHASE_THREE_RUNTIME_EXPORTS {
        assert!(
            durability_exports.contains(surface),
            "Phase 3 Store surface `{surface}` is not exported by physical_runtime"
        );
    }

    let wal = read_repository_document("workspaces/worth-store/crates/worth-store-wal/src/lib.rs")
        .expect("read WAL facade");
    let append_exports = export_block(&wal, "pub use append::{");
    for surface in [
        "WalAppendFrontier",
        "PlannedWalFrameAppend",
        "WalFramePlanningDenial",
        "plan_wal_frame_append",
    ] {
        assert!(
            append_exports.contains(surface),
            "Phase 3 WAL surface `{surface}` is not exported by worth-store-wal"
        );
    }

    let submission = submission_facade();
    assert!(submission.contains("pub fn wal_observation("));
    assert!(!submission.contains("pub fn append_prepared_wal("));
}

fn assert_phase_four_reachability(durability_exports: &str) {
    for surface in PHASE_FOUR_RUNTIME_EXPORTS {
        assert!(
            durability_exports.contains(surface),
            "Phase 4 Store surface `{surface}` is not exported by physical_runtime"
        );
    }
    let submission = submission_facade();
    assert!(!submission.contains("pub fn dispatch_wal_durable_data("));
    assert!(!submission.contains("pub fn synchronize_appended_wal("));
}

fn assert_phase_five_reachability(durability_exports: &str) {
    for surface in PHASE_FIVE_RUNTIME_EXPORTS {
        assert!(
            durability_exports.contains(surface),
            "Phase 5 Store surface `{surface}` is not exported by physical_runtime"
        );
    }
    let submission = submission_facade();
    for entry in [
        "pub fn append_prepared_wal_group(",
        "pub fn cancel_prepared_before_group_seal(",
        "pub fn continue_prepared_wal_group(",
        "pub fn synchronize_appended_wal_group(",
    ] {
        assert!(
            !submission.contains(entry),
            "ordinary facade retains `{entry}`"
        );
    }

    let publication = read_repository_document(
        "workspaces/worth-store/crates/worth-store/src/physical_runtime/record_serving/publication/mod.rs",
    )
    .expect("read publication facade");
    let preparation_exports = export_block(&publication, "pub use durable_preparation::{");
    for surface in [
        "PhysicalMutationPreparationSuccess",
        "PhysicalPreSealCancellationDenial",
        "PhysicalPreSealCancellationOutcome",
    ] {
        assert!(
            preparation_exports.contains(surface),
            "Phase 5 preparation surface `{surface}` is not exported by publication"
        );
    }
}

fn submission_facade() -> String {
    read_repository_document(
        "workspaces/worth-store/crates/worth-store/src/physical_runtime/record_serving/publication/director/submission.rs",
    )
    .expect("read submission facade")
}

fn export_block<'a>(source: &'a str, start: &str) -> &'a str {
    source
        .split_once(start)
        .and_then(|(_, tail)| tail.split_once("};"))
        .map(|(body, _)| body)
        .unwrap_or_else(|| panic!("public facade lost export block `{start}`"))
}
