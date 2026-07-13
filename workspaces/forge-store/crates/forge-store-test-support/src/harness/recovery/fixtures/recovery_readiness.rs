use forge_store_buffer_pool::{
    AdmittedBackgroundEnvelope, AllocationAdmission, AllocationByteBudget,
    AllocationEnvelopeDeclaration, BackgroundEnvelopeAdmission, BackgroundEnvelopeRequest,
    BackgroundWorkBudgetSnapshot, FixedMetadataReservation,
};
use forge_store_contracts::{
    AcceptedHandoffReadiness, BoundedCounterRecap, BufferPoolAuthorityRecap, DenialBehaviorRecap,
    DeniedBoundaryKind, HandoffEvidenceDigestSet, IntegrityInspectionLifetimeLaw,
    NoMaterializationWitness, PhysicalAuthorityRecap, PhysicalIntegrityReadinessPayload,
    ProtectedIntegrityViewCapability, ScrubPlanningAllocationEnvelope, StableDigest,
    VerifierResidentEnvelope, ROADMAP_2_S1_SCOPE,
};
use forge_store_readiness::{
    close_physical_substrate_readiness, prove_physical_substrate_readiness,
    PhysicalIntegrityReadiness, PhysicalSubstrateReadiness,
};
use forge_store_recovery_physics::RecoveryMemoryEnvelope;

pub(super) fn physical_integrity_readiness() -> PhysicalIntegrityReadiness {
    let s2 = physical_substrate_readiness();
    let facts = s2.facts();
    let payload = PhysicalIntegrityReadinessPayload::from_physical_substrate_closeout_evidence(
        ProtectedIntegrityViewCapability::protected_views(1).unwrap(),
        VerifierResidentEnvelope::bounded(128, 1).unwrap(),
        ScrubPlanningAllocationEnvelope::bounded(64).unwrap(),
        IntegrityInspectionLifetimeLaw::lease_scoped(),
        NoMaterializationWitness::observed_zero(0, 0).unwrap(),
        BoundedCounterRecap::exact(128, 1, 0, 64, 0, 0).unwrap(),
        DenialBehaviorRecap::from_named_boundaries(&DeniedBoundaryKind::ALL).unwrap(),
        PhysicalAuthorityRecap::from_physical_format_authority(
            facts.physical_reference_count(),
            facts.header_decode_witness_count(),
            facts.payload_admission_witness_count(),
        )
        .unwrap(),
        BufferPoolAuthorityRecap::physical_substrate_authority(true, true, true, true).unwrap(),
    );
    PhysicalIntegrityReadiness::from_physical_substrate_bounded_residency_closeout(s2, payload)
        .unwrap()
}

pub(super) fn physical_substrate_readiness() -> PhysicalSubstrateReadiness {
    prove_physical_substrate_readiness(
        close_physical_substrate_readiness(
            AcceptedHandoffReadiness::from_foundational_handoff_artifacts(
                ROADMAP_2_S1_SCOPE,
                HandoffEvidenceDigestSet::new(
                    digest("sha256:s7-replay-backend"),
                    digest("sha256:s7-replay-deferred"),
                    digest("sha256:s7-replay-harness"),
                    digest("sha256:s7-replay-terms"),
                    digest("sha256:s7-replay-audit"),
                    digest("sha256:s7-replay-complexity"),
                    digest("sha256:s7-replay-provenance"),
                ),
            )
            .unwrap(),
        )
        .unwrap(),
    )
    .unwrap()
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

fn digest(value: &str) -> StableDigest {
    StableDigest::new(value).unwrap()
}
