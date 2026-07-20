use worth_store_contracts::{
    BoundedCounterRecap, BufferPoolAuthorityRecap, DenialBehaviorRecap, DeniedBoundaryKind,
    IntegrityInspectionLifetimeLaw, NoMaterializationWitness, PhysicalAuthorityRecap,
    PhysicalIntegrityReadinessPayload, ProtectedIntegrityViewCapability,
    ScrubPlanningAllocationEnvelope, VerifierResidentEnvelope,
};

pub(crate) fn physical_integrity_model_payload() -> PhysicalIntegrityReadinessPayload {
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
