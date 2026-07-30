use worth_store::physical_runtime::{
    PhysicalSignalSettlementOutcome, PhysicalWorkCausalRecord, PhysicalWorkSignalFamily,
};
use worth_store_physical_backend::{BackendTargetProfile, CapabilityEvidenceClass};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::bounded_residency) struct PhysicalWorkSignalLineageEvidence {
    pub(in crate::bounded_residency) request: u64,
    pub(in crate::bounded_residency) generation: u64,
    pub(in crate::bounded_residency) branch: u64,
    pub(in crate::bounded_residency) restore_epoch: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::bounded_residency) struct PhysicalWorkCausalRouteEvidence {
    pub(in crate::bounded_residency) signal: PhysicalWorkSignalLineageEvidence,
    pub(in crate::bounded_residency) predecessor: Option<PhysicalWorkSignalLineageEvidence>,
    pub(in crate::bounded_residency) signal_attempt: u64,
    pub(in crate::bounded_residency) signal_family: PhysicalWorkSignalFamily,
    pub(in crate::bounded_residency) signal_binding: [u8; 32],
    pub(in crate::bounded_residency) scheduler_profile: BackendTargetProfile,
    pub(in crate::bounded_residency) scheduler_evidence_class: CapabilityEvidenceClass,
    pub(in crate::bounded_residency) scheduler_grouped_writes: u32,
    pub(in crate::bounded_residency) scheduler_primary_requirement: u8,
    pub(in crate::bounded_residency) scheduler_secondary_present: bool,
    pub(in crate::bounded_residency) signal_settlement: PhysicalSignalSettlementOutcome,
}

impl PhysicalWorkCausalRouteEvidence {
    pub(super) fn from_record(record: PhysicalWorkCausalRecord) -> Result<Self, String> {
        let scheduler = record.scheduler_binding();
        let signal_binding = *record.signal_binding().as_bytes();
        if signal_binding.iter().all(|byte| *byte == 0) {
            return Err("physical work route retained an empty Signal binding".to_owned());
        }
        Ok(Self {
            signal: lineage(record.signal_request()),
            predecessor: record.signal_predecessor().map(lineage),
            signal_attempt: record.signal_attempt().get(),
            signal_family: record.signal_family(),
            signal_binding,
            scheduler_profile: scheduler.backend_profile(),
            scheduler_evidence_class: scheduler.backend_evidence_class(),
            scheduler_grouped_writes: scheduler.grouped_writes(),
            scheduler_primary_requirement: scheduler.primary().backend_requirement(),
            scheduler_secondary_present: scheduler.secondary().is_some(),
            signal_settlement: record.derived_completion().ok_or_else(|| {
                "physical work route omitted derived Signal settlement".to_owned()
            })?,
        })
    }
}

fn lineage(
    request: worth_signal::facade::ResourceRequestHandle,
) -> PhysicalWorkSignalLineageEvidence {
    let branch = request.branch_epoch();
    PhysicalWorkSignalLineageEvidence {
        request: request.request_id().get(),
        generation: request.generation().get(),
        branch: branch.branch_id().0,
        restore_epoch: branch.restore_epoch(),
    }
}
