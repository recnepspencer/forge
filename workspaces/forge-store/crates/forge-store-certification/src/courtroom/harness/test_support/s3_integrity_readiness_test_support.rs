use super::bounded_memory_closeout_test_support::s2_readiness;
use forge_store_contracts::{
    BufferPoolAuthorityRecap, IntegrityInspectionLifetimeLaw, PhysicalAuthorityRecap,
    ProtectedIntegrityViewCapability, BoundedCounterRecap, DenialBehaviorRecap,
    DeniedBoundaryKind, NoMaterializationWitness, PhysicalIntegrityReadinessPayload,
    ScrubPlanningAllocationEnvelope, VerifierResidentEnvelope,
};
use forge_store_readiness::PhysicalIntegrityReadiness;

pub(crate) fn s3_integrity_readiness() -> PhysicalIntegrityReadiness {
    let s2 = s2_readiness();
    let facts = s2.facts();
    let payload = PhysicalIntegrityReadinessPayload::from_s2_closeout_evidence(
        ProtectedIntegrityViewCapability::protected_views(1).unwrap(),
        VerifierResidentEnvelope::bounded(128, 1).unwrap(),
        ScrubPlanningAllocationEnvelope::bounded(64).unwrap(),
        IntegrityInspectionLifetimeLaw::lease_scoped(),
        NoMaterializationWitness::observed_zero(0, 0).unwrap(),
        BoundedCounterRecap::exact(128, 1, 0, 64, 0, 0).unwrap(),
        DenialBehaviorRecap::from_named_boundaries(&DeniedBoundaryKind::ALL).unwrap(),
        PhysicalAuthorityRecap::from_s1_authority(
            facts.physical_reference_count(),
            facts.header_decode_witness_count(),
            facts.payload_admission_witness_count(),
        )
        .unwrap(),
        BufferPoolAuthorityRecap::s2_authority(true, true, true, true).unwrap(),
    );
    PhysicalIntegrityReadiness::from_s2_bounded_residency_closeout(s2, payload).unwrap()
}
