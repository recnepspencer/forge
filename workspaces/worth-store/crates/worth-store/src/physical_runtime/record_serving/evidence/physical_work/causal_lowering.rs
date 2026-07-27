use worth_store_physical_backend::{BackendTargetProfile, CapabilityEvidenceClass};

use crate::physical_runtime::{
    PhysicalSignalSettlementOutcome, PhysicalWorkCounterSnapshot, PhysicalWorkCounterStage,
    PhysicalWorkEffectFate, PhysicalWorkOperationFamily, PhysicalWorkPressureClass,
    PhysicalWorkRecoveryDisposition,
};

use super::{
    causal_validation::{
        CausalAttempt, CausalValidation, SignalAttemptLineage, SignalRequestLineage,
    },
    identity::PhysicalWorkCourtroomIdentity,
    validation::validate_identity,
    vocabulary::{PhysicalWorkCausalIdentity, PhysicalWorkCausalOutcome},
    PhysicalWorkBackendEvidenceClass, PhysicalWorkBackendProfileEvidence,
    PhysicalWorkCausalEvidence, PhysicalWorkCounterEvidence, PhysicalWorkCounterStageEvidence,
    PhysicalWorkCourtroomFinding, PhysicalWorkEffectFateEvidence, PhysicalWorkFamilyEvidence,
    PhysicalWorkPressureEvidence, PhysicalWorkRecoveryEvidence, PhysicalWorkSchedulerEvidence,
    PhysicalWorkSignalSettlementEvidence,
};

pub(super) fn lower_causal(
    expected: PhysicalWorkCourtroomIdentity,
    records: Box<[crate::physical_runtime::PhysicalWorkCausalRecord]>,
    findings: &mut Vec<PhysicalWorkCourtroomFinding>,
) -> (
    Box<[PhysicalWorkCausalEvidence]>,
    Option<PhysicalWorkBackendProfileEvidence>,
) {
    let mut causal_validation = CausalValidation::default();
    let mut backend_profile = None;
    let mut evidence = Vec::with_capacity(records.len());
    for record in records {
        let identity = record.identity();
        causal_validation.observe(causal_attempt(record), findings);
        let scheduler = lower_scheduler(record.scheduler_binding());
        match backend_profile {
            None => backend_profile = Some(scheduler.backend_profile()),
            Some(profile) if profile != scheduler.backend_profile() => {
                findings.push(PhysicalWorkCourtroomFinding::MixedBackendProfile);
            }
            Some(_) => {}
        }
        validate_identity(expected, identity, findings);
        evidence.push(lower_record(record, scheduler));
    }
    (evidence.into_boxed_slice(), backend_profile)
}

fn causal_attempt(record: crate::physical_runtime::PhysicalWorkCausalRecord) -> CausalAttempt {
    let identity = record.identity();
    let request = record.signal_request();
    let branch = request.branch_epoch();
    CausalAttempt {
        operation: identity.operation().get(),
        signal: SignalAttemptLineage {
            request: request.request_id().get(),
            generation: request.generation().get(),
            branch: branch.branch_id().0,
            restore_epoch: branch.restore_epoch(),
            attempt: record.signal_attempt().get(),
        },
        predecessor: record.signal_predecessor().map(|predecessor| {
            let branch = predecessor.branch_epoch();
            SignalRequestLineage {
                request: predecessor.request_id().get(),
                generation: predecessor.generation().get(),
                branch: branch.branch_id().0,
                restore_epoch: branch.restore_epoch(),
            }
        }),
        backend_operation: record
            .backend_operation()
            .map(|operation| operation.value()),
        fate: record.effect_fate(),
        recovery: record.recovery(),
    }
}

fn lower_record(
    record: crate::physical_runtime::PhysicalWorkCausalRecord,
    scheduler: PhysicalWorkSchedulerEvidence,
) -> PhysicalWorkCausalEvidence {
    let identity = record.identity();
    let request = record.signal_request();
    let predecessor = record.signal_predecessor();
    PhysicalWorkCausalEvidence::new(
        PhysicalWorkCausalIdentity {
            operation: identity.operation().get(),
            signal_request: request.request_id().get(),
            signal_generation: request.generation().get(),
            signal_predecessor_request: predecessor
                .map(|predecessor| predecessor.request_id().get()),
            signal_predecessor_generation: predecessor
                .map(|predecessor| predecessor.generation().get()),
            signal_attempt: record.signal_attempt().get(),
        },
        scheduler,
        PhysicalWorkCausalOutcome {
            backend_operation: record
                .backend_operation()
                .map(|operation| operation.value()),
            effect_fate: lower_fate(record.effect_fate()),
            recovery: lower_recovery(record.recovery()),
            signal_settlement: record.derived_completion().map(lower_signal_settlement),
        },
        lower_counters(record.counters()),
    )
}

fn lower_scheduler(
    binding: worth_store_physical_backend::BackendQueueExecutionPlanBinding,
) -> PhysicalWorkSchedulerEvidence {
    PhysicalWorkSchedulerEvidence::new(
        lower_backend_profile(binding.backend_profile()),
        lower_evidence_class(binding.backend_evidence_class()),
        binding.grouped_writes(),
        binding.primary().backend_requirement(),
        binding.secondary().is_some(),
    )
}

fn lower_counters(snapshot: PhysicalWorkCounterSnapshot) -> Box<[PhysicalWorkCounterEvidence]> {
    let mut rows = Vec::with_capacity(4 * 6 * 7);
    for (family, family_evidence) in FAMILIES {
        for (pressure, pressure_evidence) in PRESSURES {
            for (stage, stage_evidence) in STAGES {
                rows.push(PhysicalWorkCounterEvidence::new(
                    family_evidence,
                    pressure_evidence,
                    stage_evidence,
                    snapshot.count_under_pressure(family, pressure, stage),
                ));
            }
        }
    }
    rows.into_boxed_slice()
}

const FAMILIES: [(PhysicalWorkOperationFamily, PhysicalWorkFamilyEvidence); 4] = [
    (
        PhysicalWorkOperationFamily::ArtifactMetadataRead,
        PhysicalWorkFamilyEvidence::ArtifactMetadataRead,
    ),
    (
        PhysicalWorkOperationFamily::ArtifactRangeRead,
        PhysicalWorkFamilyEvidence::ArtifactRangeRead,
    ),
    (
        PhysicalWorkOperationFamily::ArtifactRangeWrite,
        PhysicalWorkFamilyEvidence::ArtifactRangeWrite,
    ),
    (
        PhysicalWorkOperationFamily::ArtifactPublication,
        PhysicalWorkFamilyEvidence::ArtifactPublication,
    ),
];

const PRESSURES: [(PhysicalWorkPressureClass, PhysicalWorkPressureEvidence); 6] = [
    (
        PhysicalWorkPressureClass::Unscheduled,
        PhysicalWorkPressureEvidence::Unscheduled,
    ),
    (
        PhysicalWorkPressureClass::ForegroundPointRead,
        PhysicalWorkPressureEvidence::ForegroundPointRead,
    ),
    (
        PhysicalWorkPressureClass::ForegroundRangeRead,
        PhysicalWorkPressureEvidence::ForegroundRangeRead,
    ),
    (
        PhysicalWorkPressureClass::ForegroundInteractiveRead,
        PhysicalWorkPressureEvidence::ForegroundInteractiveRead,
    ),
    (
        PhysicalWorkPressureClass::ForegroundInternalRead,
        PhysicalWorkPressureEvidence::ForegroundInternalRead,
    ),
    (
        PhysicalWorkPressureClass::ForegroundMutation,
        PhysicalWorkPressureEvidence::ForegroundMutation,
    ),
];

const STAGES: [(PhysicalWorkCounterStage, PhysicalWorkCounterStageEvidence); 7] = [
    (
        PhysicalWorkCounterStage::Declared,
        PhysicalWorkCounterStageEvidence::Declared,
    ),
    (
        PhysicalWorkCounterStage::Blocked,
        PhysicalWorkCounterStageEvidence::Blocked,
    ),
    (
        PhysicalWorkCounterStage::Ready,
        PhysicalWorkCounterStageEvidence::Ready,
    ),
    (
        PhysicalWorkCounterStage::Queued,
        PhysicalWorkCounterStageEvidence::Queued,
    ),
    (
        PhysicalWorkCounterStage::Dispatched,
        PhysicalWorkCounterStageEvidence::Dispatched,
    ),
    (
        PhysicalWorkCounterStage::Settling,
        PhysicalWorkCounterStageEvidence::Settling,
    ),
    (
        PhysicalWorkCounterStage::Terminal,
        PhysicalWorkCounterStageEvidence::Terminal,
    ),
];

const fn lower_backend_profile(
    profile: BackendTargetProfile,
) -> PhysicalWorkBackendProfileEvidence {
    match profile {
        BackendTargetProfile::SimulatedStrictDurable => {
            PhysicalWorkBackendProfileEvidence::SimulatedStrictDurable
        }
        BackendTargetProfile::PosixFileFsyncDirSync => {
            PhysicalWorkBackendProfileEvidence::PosixFileFsyncDirSync
        }
        BackendTargetProfile::WindowsFlushFileBuffers => {
            PhysicalWorkBackendProfileEvidence::WindowsFlushFileBuffers
        }
        BackendTargetProfile::MmapFlushNotDurabilityCertified => {
            PhysicalWorkBackendProfileEvidence::MmapFlushNotDurabilityCertified
        }
        BackendTargetProfile::AdversarialLostFlush => {
            PhysicalWorkBackendProfileEvidence::AdversarialLostFlush
        }
        BackendTargetProfile::AdversarialReorderedFlush => {
            PhysicalWorkBackendProfileEvidence::AdversarialReorderedFlush
        }
    }
}

const fn lower_evidence_class(class: CapabilityEvidenceClass) -> PhysicalWorkBackendEvidenceClass {
    match class {
        CapabilityEvidenceClass::DeclaredByConfig => {
            PhysicalWorkBackendEvidenceClass::DeclaredByConfig
        }
        CapabilityEvidenceClass::ObservedByProbe => {
            PhysicalWorkBackendEvidenceClass::ObservedByProbe
        }
        CapabilityEvidenceClass::EstablishedByFilesystemAdmission => {
            PhysicalWorkBackendEvidenceClass::EstablishedByFilesystemAdmission
        }
        CapabilityEvidenceClass::ExternallyGuaranteed => {
            PhysicalWorkBackendEvidenceClass::ExternallyGuaranteed
        }
        CapabilityEvidenceClass::UnverifiableAssumption => {
            PhysicalWorkBackendEvidenceClass::UnverifiableAssumption
        }
        CapabilityEvidenceClass::CertifiedBackendProfile => {
            PhysicalWorkBackendEvidenceClass::CertifiedBackendProfile
        }
    }
}

const fn lower_fate(fate: PhysicalWorkEffectFate) -> PhysicalWorkEffectFateEvidence {
    match fate {
        PhysicalWorkEffectFate::ProvenNoEffect => PhysicalWorkEffectFateEvidence::ProvenNoEffect,
        PhysicalWorkEffectFate::ReadCompleted => PhysicalWorkEffectFateEvidence::ReadCompleted,
        PhysicalWorkEffectFate::ReadIncomplete => PhysicalWorkEffectFateEvidence::ReadIncomplete,
        PhysicalWorkEffectFate::WriteCompleted => PhysicalWorkEffectFateEvidence::WriteCompleted,
        PhysicalWorkEffectFate::PublicationCompleted => {
            PhysicalWorkEffectFateEvidence::PublicationCompleted
        }
        PhysicalWorkEffectFate::WrittenButSchedulerRejected => {
            PhysicalWorkEffectFateEvidence::WrittenButSchedulerRejected
        }
        PhysicalWorkEffectFate::Indeterminate => PhysicalWorkEffectFateEvidence::Indeterminate,
        PhysicalWorkEffectFate::StaleOrForeignOutcome => {
            PhysicalWorkEffectFateEvidence::StaleOrForeignOutcome
        }
    }
}

const fn lower_recovery(recovery: PhysicalWorkRecoveryDisposition) -> PhysicalWorkRecoveryEvidence {
    match recovery {
        PhysicalWorkRecoveryDisposition::NoEffect => PhysicalWorkRecoveryEvidence::NoEffect,
        PhysicalWorkRecoveryDisposition::RetryExact => PhysicalWorkRecoveryEvidence::RetryExact,
        PhysicalWorkRecoveryDisposition::ContinueSettlement => {
            PhysicalWorkRecoveryEvidence::ContinueSettlement
        }
        PhysicalWorkRecoveryDisposition::InspectionRequired => {
            PhysicalWorkRecoveryEvidence::InspectionRequired
        }
    }
}

const fn lower_signal_settlement(
    settlement: PhysicalSignalSettlementOutcome,
) -> PhysicalWorkSignalSettlementEvidence {
    match settlement {
        PhysicalSignalSettlementOutcome::Committed => {
            PhysicalWorkSignalSettlementEvidence::Committed
        }
        PhysicalSignalSettlementOutcome::ReconciledFromPhysicalTruth => {
            PhysicalWorkSignalSettlementEvidence::ReconciledFromPhysicalTruth
        }
        PhysicalSignalSettlementOutcome::DerivedStateUnavailable => {
            PhysicalWorkSignalSettlementEvidence::DerivedStateUnavailable
        }
    }
}
