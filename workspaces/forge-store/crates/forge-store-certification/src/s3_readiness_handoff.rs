use crate::{
    BoundedMemoryCloseoutReport, BoundedMemoryOperationKind, BufferPoolCertificationBundle,
    S2BoundaryDenialKind,
};
use forge_store_buffer_pool::BackgroundWorkClass;
use forge_store_readiness::{
    BufferPoolAuthorityRecap, IntegrityInspectionLifetimeLaw, PhysicalAuthorityRecap,
    ProtectedIntegrityViewCapability, S2BoundedCounterRecap, S2DenialBehaviorRecap,
    S2DeniedBoundaryKind, S2NoMaterializationWitness, S2PhysicalSubstrateReadiness,
    S3PhysicalIntegrityReadinessPayload, S3ReadinessDenial, S3ReadinessDenialKind,
    ScrubPlanningAllocationEnvelope, VerifierResidentEnvelope,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S3PhysicalIntegrityReadiness {
    s2_readiness: S2PhysicalSubstrateReadiness,
    payload: S3PhysicalIntegrityReadinessPayload,
}

impl S3PhysicalIntegrityReadiness {
    fn from_s2_bounded_residency_closeout(
        s2_readiness: S2PhysicalSubstrateReadiness,
        payload: S3PhysicalIntegrityReadinessPayload,
    ) -> Result<Self, S3ReadinessDenial> {
        if !s2_readiness.is_sealed() {
            return Err(S3ReadinessDenial::new(
                S3ReadinessDenialKind::S2ReadinessNotSealed,
            ));
        }
        require_physical_recap_matches_s2_facts(s2_readiness, payload)?;
        Ok(Self {
            s2_readiness,
            payload: payload.require_complete()?,
        })
    }

    pub const fn s2_readiness(self) -> S2PhysicalSubstrateReadiness {
        self.s2_readiness
    }

    pub const fn payload(self) -> S3PhysicalIntegrityReadinessPayload {
        self.payload
    }

    pub const fn protected_view_capability(self) -> ProtectedIntegrityViewCapability {
        self.payload.protected_view_capability()
    }

    pub const fn verifier_resident_envelope(self) -> VerifierResidentEnvelope {
        self.payload.verifier_resident_envelope()
    }

    pub const fn scrub_allocation_envelope(self) -> ScrubPlanningAllocationEnvelope {
        self.payload.scrub_allocation_envelope()
    }

    pub const fn inspection_lifetime_law(self) -> IntegrityInspectionLifetimeLaw {
        self.payload.inspection_lifetime_law()
    }

    pub const fn no_materialization_witness(self) -> S2NoMaterializationWitness {
        self.payload.no_materialization_witness()
    }
}

impl BoundedMemoryCloseoutReport {
    pub fn publish_s3_physical_integrity_readiness(
        self,
        s2_readiness: S2PhysicalSubstrateReadiness,
    ) -> Result<S3PhysicalIntegrityReadiness, S3ReadinessDenial> {
        let payload = payload_from_closeout(self.bundle(), s2_readiness)?;
        S3PhysicalIntegrityReadiness::from_s2_bounded_residency_closeout(s2_readiness, payload)
    }
}

fn require_physical_recap_matches_s2_facts(
    s2_readiness: S2PhysicalSubstrateReadiness,
    payload: S3PhysicalIntegrityReadinessPayload,
) -> Result<(), S3ReadinessDenial> {
    let facts = s2_readiness.facts();
    let recap = payload.physical_authority_recap();
    if recap.physical_reference_count() == facts.physical_reference_count()
        && recap.header_decode_witness_count() == facts.header_decode_witness_count()
        && recap.payload_admission_witness_count() == facts.payload_admission_witness_count()
    {
        Ok(())
    } else {
        Err(S3ReadinessDenial::new(
            S3ReadinessDenialKind::PhysicalAuthorityRecapMismatch,
        ))
    }
}

fn payload_from_closeout(
    bundle: &BufferPoolCertificationBundle,
    s2_readiness: S2PhysicalSubstrateReadiness,
) -> Result<S3PhysicalIntegrityReadinessPayload, S3ReadinessDenial> {
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
        S3PhysicalIntegrityReadinessPayload::from_s2_closeout_evidence(
            protected_view_capability,
            VerifierResidentEnvelope::bounded(
                aggregate.max_resident_bytes,
                aggregate.max_pinned_pages as u32,
            )?,
            ScrubPlanningAllocationEnvelope::bounded(scrub_allocation)?,
            IntegrityInspectionLifetimeLaw::lease_scoped(),
            S2NoMaterializationWitness::observed_zero(0, aggregate.materialized_bytes)?,
            S2BoundedCounterRecap::exact(
                aggregate.max_resident_bytes,
                aggregate.max_pinned_pages,
                aggregate.max_dirty_pages,
                aggregate.allocation_bytes,
                aggregate.copied_bytes,
                aggregate.materialized_bytes,
            )?,
            S2DenialBehaviorRecap::from_named_boundaries(&readiness_denials(
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
) -> Result<ProtectedIntegrityViewCapability, S3ReadinessDenial> {
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

fn readiness_denials(denials: &[S2BoundaryDenialKind]) -> Vec<S2DeniedBoundaryKind> {
    denials
        .iter()
        .map(|denial| match denial {
            S2BoundaryDenialKind::OverBudgetResidency => S2DeniedBoundaryKind::OverBudgetResidency,
            S2BoundaryDenialKind::PinLeak => S2DeniedBoundaryKind::PinLeak,
            S2BoundaryDenialKind::DirtyOverflow => S2DeniedBoundaryKind::DirtyOverflow,
            S2BoundaryDenialKind::WholeStoreMaterialization => {
                S2DeniedBoundaryKind::WholeStoreMaterialization
            }
            S2BoundaryDenialKind::WholeObjectStreaming => {
                S2DeniedBoundaryKind::WholeObjectStreaming
            }
            S2BoundaryDenialKind::ForgedViewAccess => S2DeniedBoundaryKind::ForgedViewAccess,
        })
        .collect()
}
