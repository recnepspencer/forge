use crate::{
    BoundedMemoryCloseoutReport, BoundedMemoryOperationKind, BufferPoolCertificationBundle,
    S2BoundaryDenialKind,
};
use forge_store_buffer_pool::BackgroundWorkClass;
use forge_store_contracts::{
    BufferPoolAuthorityRecap, IntegrityInspectionLifetimeLaw, PhysicalAuthorityRecap,
    ProtectedIntegrityViewCapability, BoundedCounterRecap, DenialBehaviorRecap,
    DeniedBoundaryKind, NoMaterializationWitness, PhysicalIntegrityReadinessPayload,
    PhysicalIntegrityReadinessDenial, ScrubPlanningAllocationEnvelope, VerifierResidentEnvelope,
};
use forge_store_readiness::{PhysicalSubstrateReadiness, PhysicalIntegrityReadiness};

impl BoundedMemoryCloseoutReport {
    pub fn publish_s3_physical_integrity_readiness(
        self,
        s2_readiness: PhysicalSubstrateReadiness,
    ) -> Result<PhysicalIntegrityReadiness, PhysicalIntegrityReadinessDenial> {
        let payload = payload_from_closeout(self.bundle(), s2_readiness)?;
        PhysicalIntegrityReadiness::from_s2_bounded_residency_closeout(s2_readiness, payload)
    }
}

fn payload_from_closeout(
    bundle: &BufferPoolCertificationBundle,
    s2_readiness: PhysicalSubstrateReadiness,
) -> Result<PhysicalIntegrityReadinessPayload, PhysicalIntegrityReadinessDenial> {
    let suite = bundle.suite();
    let aggregate = aggregate_operation_counters(suite.reports());
    let protected_view_capability = protected_view_capability_from_closeout(bundle)?;
    let scrub_allocation = bundle
        .background()
        .envelope_for(BackgroundWorkClass::ScrubPlanning)
        .map(|evidence| evidence.counters().allocation_bytes_admitted())
        .unwrap_or(0);
    let facts = s2_readiness.facts();
    Ok(
        PhysicalIntegrityReadinessPayload::from_s2_closeout_evidence(
            protected_view_capability,
            VerifierResidentEnvelope::bounded(
                aggregate.max_resident_bytes,
                aggregate.max_pinned_pages as u32,
            )?,
            ScrubPlanningAllocationEnvelope::bounded(scrub_allocation)?,
            IntegrityInspectionLifetimeLaw::lease_scoped(),
            NoMaterializationWitness::observed_zero(0, aggregate.materialized_bytes)?,
            BoundedCounterRecap::exact(
                aggregate.max_resident_bytes,
                aggregate.max_pinned_pages,
                aggregate.max_dirty_pages,
                aggregate.allocation_bytes,
                aggregate.copied_bytes,
                aggregate.materialized_bytes,
            )?,
            DenialBehaviorRecap::from_named_boundaries(&readiness_denials(
                bundle.suite().denials(),
            ))?,
            PhysicalAuthorityRecap::from_s1_authority(
                facts.physical_reference_count(),
                facts.header_decode_witness_count(),
                facts.payload_admission_witness_count(),
            )?,
            BufferPoolAuthorityRecap::s2_authority(
                has_pinning_evidence(suite),
                aggregate.max_resident_bytes > 0,
                aggregate.allocation_bytes > 0,
                protected_view_capability.is_concrete(),
            )?,
        ),
    )
}

fn protected_view_capability_from_closeout(
    bundle: &BufferPoolCertificationBundle,
) -> Result<ProtectedIntegrityViewCapability, PhysicalIntegrityReadinessDenial> {
    if !bundle
        .suite()
        .denials()
        .contains(&S2BoundaryDenialKind::ForgedViewAccess)
    {
        return ProtectedIntegrityViewCapability::protected_views(0);
    }
    ProtectedIntegrityViewCapability::protected_views(
        bundle.protected_view().protected_view_count(),
    )
}

struct AggregateOperationCounters {
    max_resident_bytes: u64,
    max_pinned_pages: u64,
    max_dirty_pages: u32,
    allocation_bytes: u64,
    copied_bytes: u64,
    materialized_bytes: u64,
}

fn aggregate_operation_counters(
    reports: &[crate::BoundedOperationEnvelopeReport],
) -> AggregateOperationCounters {
    reports.iter().fold(
        AggregateOperationCounters {
            max_resident_bytes: 0,
            max_pinned_pages: 0,
            max_dirty_pages: 0,
            allocation_bytes: 0,
            copied_bytes: 0,
            materialized_bytes: 0,
        },
        |mut aggregate, report| {
            let counters = report.counters();
            aggregate.max_resident_bytes =
                aggregate.max_resident_bytes.max(counters.resident_bytes());
            aggregate.max_pinned_pages = aggregate.max_pinned_pages.max(counters.pinned_pages());
            aggregate.max_dirty_pages = aggregate.max_dirty_pages.max(counters.dirty_pages());
            aggregate.allocation_bytes += counters.allocation_bytes();
            aggregate.copied_bytes += counters.copied_bytes();
            aggregate.materialized_bytes += counters.materialized_bytes();
            aggregate
        },
    )
}

fn has_pinning_evidence(suite: &crate::BoundedMemoryResidencySuite) -> bool {
    suite
        .report_for(BoundedMemoryOperationKind::AdmittedRead)
        .map(|report| report.counters().pinned_pages() > 0)
        .unwrap_or(false)
}

fn readiness_denials(denials: &[S2BoundaryDenialKind]) -> Vec<DeniedBoundaryKind> {
    denials
        .iter()
        .map(|denial| match denial {
            S2BoundaryDenialKind::OverBudgetResidency => DeniedBoundaryKind::OverBudgetResidency,
            S2BoundaryDenialKind::PinLeak => DeniedBoundaryKind::PinLeak,
            S2BoundaryDenialKind::DirtyOverflow => DeniedBoundaryKind::DirtyOverflow,
            S2BoundaryDenialKind::WholeStoreMaterialization => {
                DeniedBoundaryKind::WholeStoreMaterialization
            }
            S2BoundaryDenialKind::WholeObjectStreaming => {
                DeniedBoundaryKind::WholeObjectStreaming
            }
            S2BoundaryDenialKind::ForgedViewAccess => DeniedBoundaryKind::ForgedViewAccess,
        })
        .collect()
}
