use worth_store_buffer_pool::{
    AdmittedBackgroundEnvelope, AllocationAdmission, AllocationByteBudget,
    AllocationEnvelopeDeclaration, BackgroundEnvelopeAdmission, BackgroundEnvelopeRequest,
    BackgroundWorkBudgetSnapshot, FixedMetadataReservation,
};
use worth_store_contracts::{
    BoundedCounterRecap, BufferPoolAuthorityRecap, DenialBehaviorRecap, DeniedBoundaryKind,
    IntegrityInspectionLifetimeLaw, NoMaterializationWitness, PhysicalAuthorityRecap,
    PhysicalIntegrityReadinessPayload, ProtectedIntegrityViewCapability,
    ScrubPlanningAllocationEnvelope, VerifierResidentEnvelope,
};
use worth_store_recovery_physics::RecoveryMemoryEnvelope;

pub(super) fn physical_integrity_model_payload() -> PhysicalIntegrityReadinessPayload {
    PhysicalIntegrityReadinessPayload::from_physical_substrate_closeout_evidence(
        ProtectedIntegrityViewCapability::protected_views(1).unwrap(),
        VerifierResidentEnvelope::bounded(128, 1).unwrap(),
        ScrubPlanningAllocationEnvelope::bounded(64).unwrap(),
        IntegrityInspectionLifetimeLaw::lease_scoped(),
        NoMaterializationWitness::observed_zero(0, 0).unwrap(),
        BoundedCounterRecap::exact(128, 1, 0, 64, 0, 0).unwrap(),
        DenialBehaviorRecap::from_named_boundaries(&DeniedBoundaryKind::ALL).unwrap(),
        PhysicalAuthorityRecap::from_physical_format_authority(4, 2, 2).unwrap(),
        BufferPoolAuthorityRecap::physical_substrate_authority(true, true, true, true).unwrap(),
    )
}

pub(super) fn recovery_memory_envelope() -> RecoveryMemoryEnvelope {
    RecoveryMemoryEnvelope::from_admitted(admit_background()).unwrap()
}

pub(super) fn physical_authority() -> PhysicalAuthorityRecap {
    PhysicalAuthorityRecap::from_physical_format_authority(3, 2, 1).unwrap()
}

fn admit_background() -> AdmittedBackgroundEnvelope {
    let mut admission = BackgroundEnvelopeAdmission::new();
    let mut allocation = allocation_admission();
    admission
        .admit(
            BackgroundEnvelopeRequest::recovery_planning()
                .resident_frames(1)
                .resident_bytes(128)
                .pin_pages_for_bounded_step(1)
                .allocation_bytes(128)
                .finish(),
            BackgroundWorkBudgetSnapshot::foreground_reserved(16, 4, 0, 16),
            &mut allocation,
        )
        .unwrap()
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
