use forge_store_buffer_pool::{
    AdmittedBackgroundEnvelope, AllocationAdmission, AllocationByteBudget,
    AllocationEnvelopeDeclaration, BackgroundEnvelopeAdmission, BackgroundEnvelopeRequest,
    BackgroundWorkBudgetSnapshot, FixedMetadataReservation,
};
use forge_store_recovery_physics::RecoveryMemoryEnvelope;

pub(crate) fn recovery_memory_envelope() -> RecoveryMemoryEnvelope {
    RecoveryMemoryEnvelope::from_admitted(admit_background(recovery_request()))
        .expect("recovery budget uses S.2 recovery-planning envelope")
}

fn admit_background(request: BackgroundEnvelopeRequest) -> AdmittedBackgroundEnvelope {
    let mut admission = BackgroundEnvelopeAdmission::new();
    let mut allocation = allocation_admission();
    admission
        .admit(request, permissive_budget(), &mut allocation)
        .expect("background envelope admits")
}

fn recovery_request() -> BackgroundEnvelopeRequest {
    BackgroundEnvelopeRequest::recovery_planning()
        .resident_frames(1)
        .resident_bytes(128)
        .pin_pages_for_bounded_step(1)
        .allocation_bytes(128)
        .finish()
}

fn permissive_budget() -> BackgroundWorkBudgetSnapshot {
    BackgroundWorkBudgetSnapshot::foreground_reserved(16, 4, 0, 16)
}

fn allocation_admission() -> AllocationAdmission {
    AllocationAdmission::from_declaration(
        AllocationEnvelopeDeclaration::declare()
            .foreground(bytes(512))
            .maintenance(bytes(512))
            .recovery(bytes(512))
            .scrub(bytes(512))
            .import_export(bytes(512))
            .streaming(bytes(512))
            .fixed_metadata(FixedMetadataReservation::constant_bytes(64).unwrap())
            .seal()
            .unwrap(),
    )
}

fn bytes(bytes: u64) -> AllocationByteBudget {
    AllocationByteBudget::bytes(bytes).unwrap()
}
