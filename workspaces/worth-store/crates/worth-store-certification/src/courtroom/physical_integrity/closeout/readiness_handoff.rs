use crate::{
    BoundedMemoryCloseoutReport, BoundedMemoryOperationKind, BufferPoolCertificationBundle,
    MemoryBoundaryDenialKind,
};
use worth_store_buffer_pool::BackgroundWorkClass;
use worth_store_contracts::{
    BoundedCounterRecap, BufferPoolAuthorityRecap, DenialBehaviorRecap, DeniedBoundaryKind,
    IntegrityInspectionLifetimeLaw, NoMaterializationWitness, PhysicalAuthorityRecap,
    PhysicalIntegrityReadinessDenial, PhysicalIntegrityReadinessPayload,
    ProtectedIntegrityViewCapability, ScrubPlanningAllocationEnvelope, VerifierResidentEnvelope,
};
use worth_store_readiness::{PhysicalIntegrityReadiness, PhysicalSubstrateReadiness};

impl BoundedMemoryCloseoutReport {
    pub fn publish_physical_integrity_readiness(
        self,
        physical_substrate_readiness: PhysicalSubstrateReadiness,
    ) -> Result<PhysicalIntegrityReadiness, PhysicalIntegrityReadinessDenial> {
        let payload = payload_from_closeout(self.bundle(), physical_substrate_readiness)?;
        PhysicalIntegrityReadiness::from_physical_substrate_bounded_residency_closeout(
            physical_substrate_readiness,
            payload,
        )
    }
}

fn payload_from_closeout(
    bundle: &BufferPoolCertificationBundle,
    physical_substrate_readiness: PhysicalSubstrateReadiness,
) -> Result<PhysicalIntegrityReadinessPayload, PhysicalIntegrityReadinessDenial> {
    let suite = bundle.suite();
    let aggregate = aggregate_operation_counters(suite.reports());
    let protected_view_capability = protected_view_capability_from_closeout(bundle)?;
    let scrub_allocation = bundle
        .background()
        .envelope_for(BackgroundWorkClass::ScrubPlanning)
        .map(|evidence| evidence.counters().allocation_bytes_admitted())
        .unwrap_or(0);
    let facts = physical_substrate_readiness.facts();
    Ok(
        PhysicalIntegrityReadinessPayload::from_physical_substrate_closeout_evidence(
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
            PhysicalAuthorityRecap::from_physical_format_authority(
                facts.physical_reference_count(),
                facts.header_decode_witness_count(),
                facts.payload_admission_witness_count(),
            )?,
            BufferPoolAuthorityRecap::physical_substrate_authority(
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
        .contains(&MemoryBoundaryDenialKind::ForgedViewAccess)
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

fn readiness_denials(denials: &[MemoryBoundaryDenialKind]) -> Vec<DeniedBoundaryKind> {
    denials
        .iter()
        .map(|denial| match denial {
            MemoryBoundaryDenialKind::OverBudgetResidency => {
                DeniedBoundaryKind::OverBudgetResidency
            }
            MemoryBoundaryDenialKind::PinLeak => DeniedBoundaryKind::PinLeak,
            MemoryBoundaryDenialKind::DirtyOverflow => DeniedBoundaryKind::DirtyOverflow,
            MemoryBoundaryDenialKind::WholeStoreMaterialization => {
                DeniedBoundaryKind::WholeStoreMaterialization
            }
            MemoryBoundaryDenialKind::WholeObjectStreaming => {
                DeniedBoundaryKind::WholeObjectStreaming
            }
            MemoryBoundaryDenialKind::ForgedViewAccess => DeniedBoundaryKind::ForgedViewAccess,
        })
        .collect()
}
