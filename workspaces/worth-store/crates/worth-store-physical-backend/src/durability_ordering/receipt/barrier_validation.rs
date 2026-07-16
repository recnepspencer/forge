use crate::{CapabilityEvidenceClass, WalDurabilityBarrier, WalDurabilityBarrierSet};

use super::super::{
    StoreDurabilityDenial, StoreDurabilityDenialKind, StoreDurabilityFileSyncKind,
    StoreDurabilityOperation, StoreDurabilityPublicationKind, StoreDurabilityRequirement,
    StoreDurabilityState,
};
use super::StoreDurabilityReceiptCore;

pub(super) const fn operation_for(
    requirement: StoreDurabilityRequirement,
) -> StoreDurabilityOperation {
    match requirement.publication() {
        StoreDurabilityPublicationKind::WalFrame => StoreDurabilityOperation::WalPublication,
        StoreDurabilityPublicationKind::Checkpoint => {
            StoreDurabilityOperation::CheckpointPublication
        }
        StoreDurabilityPublicationKind::Manifest => StoreDurabilityOperation::ManifestPublication,
    }
}

pub(super) const fn file_sync_satisfies(
    actual: StoreDurabilityFileSyncKind,
    required: StoreDurabilityFileSyncKind,
) -> bool {
    matches!(
        (actual, required),
        (
            StoreDurabilityFileSyncKind::Fsync,
            StoreDurabilityFileSyncKind::Fsync
        ) | (
            StoreDurabilityFileSyncKind::Fsync,
            StoreDurabilityFileSyncKind::Fdatasync
        ) | (
            StoreDurabilityFileSyncKind::Fdatasync,
            StoreDurabilityFileSyncKind::Fdatasync
        )
    )
}

pub(super) fn missing_completed_step_denial<S>(
    kind: StoreDurabilityDenialKind,
    operation: StoreDurabilityOperation,
    core: &StoreDurabilityReceiptCore<S>,
) -> StoreDurabilityDenial {
    StoreDurabilityDenial::new(
        kind,
        StoreDurabilityState::Denied,
        operation,
        core.profile,
        CapabilityEvidenceClass::CertifiedBackendProfile,
        core.evidence_class,
        core.counters.with_denied_claim(),
    )
}

pub(super) fn require_all_barriers<S>(
    core: &StoreDurabilityReceiptCore<S>,
) -> Result<(), StoreDurabilityDenial> {
    require_barrier_class(
        core,
        core.completed_barriers,
        core.requirement.required_barriers(),
        operation_for(core.requirement),
    )
}

pub(super) fn require_barrier_class<S>(
    core: &StoreDurabilityReceiptCore<S>,
    completed: WalDurabilityBarrierSet,
    candidates: WalDurabilityBarrierSet,
    operation: StoreDurabilityOperation,
) -> Result<(), StoreDurabilityDenial> {
    let required = WalDurabilityBarrierSet::from_bits(
        core.requirement.required_barriers().bits() & candidates.bits(),
    );
    if completed.satisfies(required) {
        return Ok(());
    }
    let missing = completed
        .first_missing_from(required)
        .expect("required barrier class is not satisfied");
    Err(StoreDurabilityDenial::new(
        StoreDurabilityDenialKind::MissingRequiredBarrier,
        StoreDurabilityState::Denied,
        operation,
        core.profile,
        CapabilityEvidenceClass::CertifiedBackendProfile,
        core.evidence_class,
        core.counters.with_denied_claim(),
    )
    .with_missing_barrier(missing))
}

pub(super) const fn file_barriers() -> WalDurabilityBarrierSet {
    WalDurabilityBarrierSet::of(WalDurabilityBarrier::SimulatedDurableCommit)
        .insert(WalDurabilityBarrier::WalFileFsync)
        .insert(WalDurabilityBarrier::WindowsFlushFileBuffers)
}

pub(super) const fn directory_barriers() -> WalDurabilityBarrierSet {
    WalDurabilityBarrierSet::of(WalDurabilityBarrier::WalDirectoryFsync)
        .insert(WalDurabilityBarrier::WindowsDirectorySync)
}
