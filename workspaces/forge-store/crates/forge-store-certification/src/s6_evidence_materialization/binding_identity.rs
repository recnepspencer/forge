use forge_store_physical_backend::{
    AccessPolicyViolationKind, BackendTargetProfile, CapabilityEvidenceClass,
};

use crate::{S6PostAdmissionViolationCause, S6PostAdmissionViolationEvidenceRow};

pub(super) fn post_admission_violation_tag(row: S6PostAdmissionViolationEvidenceRow) -> u64 {
    let family = match row.family() {
        crate::S6PostAdmissionViolationFamily::QueueExecution => 1,
        crate::S6PostAdmissionViolationFamily::BackgroundPacing => 2,
        crate::S6PostAdmissionViolationFamily::AccessPolicy => 3,
    };
    mix(family, cause_tag(row.cause()))
}

pub(super) const fn profile_tag(profile: BackendTargetProfile) -> u64 {
    match profile {
        BackendTargetProfile::SimulatedStrictDurable => 1,
        BackendTargetProfile::PosixFileFsyncDirSync => 2,
        BackendTargetProfile::WindowsFlushFileBuffers => 3,
        BackendTargetProfile::MmapFlushNotDurabilityCertified => 4,
        BackendTargetProfile::AdversarialLostFlush => 5,
        BackendTargetProfile::AdversarialReorderedFlush => 6,
    }
}

pub(super) const fn evidence_class_tag(evidence_class: CapabilityEvidenceClass) -> u64 {
    match evidence_class {
        CapabilityEvidenceClass::DeclaredByConfig => 1,
        CapabilityEvidenceClass::ObservedByProbe => 2,
        CapabilityEvidenceClass::ExternallyGuaranteed => 3,
        CapabilityEvidenceClass::UnverifiableAssumption => 4,
        CapabilityEvidenceClass::CertifiedBackendProfile => 5,
    }
}

pub(super) const fn mix(state: u64, value: u64) -> u64 {
    state.wrapping_mul(1_099_511_628_211).wrapping_add(value)
}

fn cause_tag(cause: S6PostAdmissionViolationCause) -> u64 {
    match cause {
        S6PostAdmissionViolationCause::QueueExecution(cause) => match cause {
            forge_store_io_scheduler::queue_execution::QueueExecutionViolationCause::BackendContradictedWitness => 11,
            forge_store_io_scheduler::queue_execution::QueueExecutionViolationCause::ExecutionReclassifiedWork => 12,
        },
        S6PostAdmissionViolationCause::BackgroundPacing(kind) => match kind {
            forge_store_io_scheduler::BackgroundDebtKind::CompactionDebt => 21,
            forge_store_io_scheduler::BackgroundDebtKind::CheckpointFlushDebt => 22,
            forge_store_io_scheduler::BackgroundDebtKind::ScrubPressure => 23,
            forge_store_io_scheduler::BackgroundDebtKind::ReplicationPrepPressure => 24,
            forge_store_io_scheduler::BackgroundDebtKind::BlobContention => 25,
            forge_store_io_scheduler::BackgroundDebtKind::BackupPressure => 26,
            forge_store_io_scheduler::BackgroundDebtKind::RepairPressure => 27,
        },
        S6PostAdmissionViolationCause::AccessPolicy(kind) => match kind {
            AccessPolicyViolationKind::None => 31,
            AccessPolicyViolationKind::MmapLazyFault => 32,
            AccessPolicyViolationKind::MixedModeInvalidationMissed => 33,
            AccessPolicyViolationKind::PageCacheVisibilityLost => 34,
            AccessPolicyViolationKind::DirectIoAlignmentContradicted => 36,
            AccessPolicyViolationKind::BackendContradictedWitness => 37,
        },
    }
}
