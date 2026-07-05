use forge_store_physical_backend::{
    BackendCapabilityAdmissionRequest, BackendCapabilityEvidenceBasis, BackendCapabilitySupportSet,
    BackendMediaAssumptionSet, BackendRebindTriggers, BackendTargetProfile,
    PhysicalBackendCapabilityAdmissionAuthority,
};
use forge_store_physical_isolation::publish_s6_io_qos_isolation_readiness_for_foreground_reservation_test;
use forge_store_readiness::{
    accept_s5_1_admitted_security_scope_readiness, S51SecurityScopeReadinessReservation,
};
use forge_store_security::admitted_store_internal_security_scope_for_s6_test;

use crate::{
    admit_backend_capability_for_scheduler_claim, admit_s5_1_security_scope_for_s6_io_qos,
    admit_secure_frame_backend_capability_for_scheduler_claim, admit_secure_io_scope_for_scheduler,
    admit_store_published_s6_io_qos_isolation_readiness, BackgroundIoDebt,
    BackgroundIoPressureClass, BackgroundPacingCounterSnapshot, BackgroundPacingOutcome,
    BackgroundPacingViolation, BackgroundResourceBudget, IoSchedulerBackendCapabilityAdmission,
    IoSchedulerBackendCapabilityRequirement, IoSchedulerS6ReadinessAdmission,
    IoSchedulerS6SecurityScopeAdmission, QueueSlot, S6IoQosSecurityScopeHandoff, SecureIoOperation,
    SecureIoPostureRequirement, SecureIoPreservationReceipt, SecureIoPreservationRequest,
};

pub(super) fn scheduler_readiness() -> IoSchedulerS6ReadinessAdmission {
    let readiness = publish_s6_io_qos_isolation_readiness_for_foreground_reservation_test(2, 1)
        .expect("S.5 closeout should publish S.6 readiness");
    admit_store_published_s6_io_qos_isolation_readiness(&readiness)
        .expect("scheduler should admit Store-published S.6 readiness")
}

pub(super) fn background_pacing_outcome(
    class: BackgroundIoPressureClass,
) -> BackgroundPacingOutcome {
    let requested = BackgroundResourceBudget::new().with_queue_slots(QueueSlot::new(1).unwrap());
    BackgroundPacingOutcome::Violation(BackgroundPacingViolation::new(
        BackgroundIoDebt::new(class, requested),
        BackgroundPacingCounterSnapshot::violation(
            requested,
            BackgroundResourceBudget::new(),
            BackgroundResourceBudget::new(),
            requested,
            class.debt_kind(),
            1,
        ),
    ))
}

pub(super) fn scheduler_security_scope() -> IoSchedulerS6SecurityScopeAdmission {
    let readiness = accept_s5_1_admitted_security_scope_readiness(
        S51SecurityScopeReadinessReservation::io_qos(),
        admitted_store_internal_security_scope_for_s6_test(),
    );
    let handoff = S6IoQosSecurityScopeHandoff::from_s5_1_readiness(readiness)
        .expect("S.5.1 handoff should admit");
    admit_s5_1_security_scope_for_s6_io_qos(handoff)
}

pub(super) fn secure_io_receipt(
    security: &IoSchedulerS6SecurityScopeAdmission,
    backend: &IoSchedulerBackendCapabilityAdmission,
    operation: SecureIoOperation,
) -> SecureIoPreservationReceipt {
    secure_io_receipt_with_posture(
        security,
        backend,
        operation,
        SecureIoPostureRequirement::ScopePreserving,
    )
}

pub(super) fn secure_io_receipt_with_posture(
    security: &IoSchedulerS6SecurityScopeAdmission,
    backend: &IoSchedulerBackendCapabilityAdmission,
    operation: SecureIoOperation,
    posture: SecureIoPostureRequirement,
) -> SecureIoPreservationReceipt {
    admit_secure_io_scope_for_scheduler(
        SecureIoPreservationRequest::new(operation, security, backend).require_posture(posture),
    )
    .expect("secure I/O scope should admit")
}

pub(super) fn secure_frame_backend_admission(
    security: &IoSchedulerS6SecurityScopeAdmission,
) -> IoSchedulerBackendCapabilityAdmission {
    let witness = backend_witness(BackendCapabilitySupportSet::all_supported());
    admit_secure_frame_backend_capability_for_scheduler_claim(&witness, security)
        .expect("scheduler backend should admit")
}

pub(super) fn non_secure_backend_admission() -> IoSchedulerBackendCapabilityAdmission {
    let witness = backend_witness(BackendCapabilitySupportSet::all_supported());
    admit_backend_capability_for_scheduler_claim(
        &witness,
        IoSchedulerBackendCapabilityRequirement::Fsync,
    )
    .expect("ordinary fsync backend should admit")
}

fn backend_witness(
    support: BackendCapabilitySupportSet,
) -> forge_store_physical_backend::AdmittedBackendCapabilityWitness {
    let request = BackendCapabilityAdmissionRequest::new(
        BackendTargetProfile::PosixFileFsyncDirSync,
        BackendCapabilityEvidenceBasis::externally_guaranteed(1),
        support,
        BackendMediaAssumptionSet::platform_file_defaults()
            .with_direct_io_alignment()
            .with_sector_atomicity()
            .with_page_cache_policy()
            .with_mmap_coherence()
            .with_async_ordering()
            .with_secure_frame_io()
            .with_flush_ordering(),
        BackendRebindTriggers::kernel_filesystem_mount_firmware_and_backend(),
    );
    PhysicalBackendCapabilityAdmissionAuthority::store_owned()
        .admit_backend_capability(request)
        .expect("backend should admit")
}
