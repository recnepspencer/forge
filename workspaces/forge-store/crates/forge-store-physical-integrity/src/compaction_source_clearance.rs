use crate::{
    IntegrityEvidenceLocality, IntegrityEvidenceOutcome, PhysicalIntegrityEvidenceBundle,
    QuarantineHandoffPosture, QuarantineRecord, ScrubExecutionReceipt, ScrubIntegrityFinding,
};
use forge_store_physical_format::PhysicalGenerationOwner;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompactionSourceClearanceKind {
    IntactIntegrityEvidence,
    IntactScrubExecution,
    Quarantined,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompactionSourceClearanceDenial {
    NonIntactIntegrityEvidence,
    InterruptedScrubExecution,
    EmptyScrubExecution,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompactionSourceIntegrityClearance {
    kind: CompactionSourceClearanceKind,
    inspected_bytes: u64,
    locality: Option<IntegrityEvidenceLocality>,
    handoff_posture: Option<QuarantineHandoffPosture>,
}

impl CompactionSourceIntegrityClearance {
    pub fn from_integrity_evidence(
        evidence: &PhysicalIntegrityEvidenceBundle,
    ) -> Result<Self, CompactionSourceClearanceDenial> {
        if *evidence.integrity_outcome() != IntegrityEvidenceOutcome::IntactPhysicalBoundary {
            return Err(CompactionSourceClearanceDenial::NonIntactIntegrityEvidence);
        }
        Ok(Self {
            kind: CompactionSourceClearanceKind::IntactIntegrityEvidence,
            inspected_bytes: 1,
            locality: Some(evidence.locality()),
            handoff_posture: None,
        })
    }

    pub fn from_scrub_execution(
        receipt: &ScrubExecutionReceipt,
    ) -> Result<Self, CompactionSourceClearanceDenial> {
        if receipt.progress().interrupted() || receipt.resume_token().is_some() {
            return Err(CompactionSourceClearanceDenial::InterruptedScrubExecution);
        }
        let counters = receipt.counters();
        if receipt.finding() != ScrubIntegrityFinding::Intact || counters.checked_byte_count() == 0
        {
            return Err(CompactionSourceClearanceDenial::EmptyScrubExecution);
        }
        Ok(Self {
            kind: CompactionSourceClearanceKind::IntactScrubExecution,
            inspected_bytes: counters.checked_byte_count(),
            locality: None,
            handoff_posture: None,
        })
    }

    pub fn from_quarantine_record(record: &QuarantineRecord) -> Self {
        Self {
            kind: CompactionSourceClearanceKind::Quarantined,
            inspected_bytes: 0,
            locality: Some(IntegrityEvidenceLocality::Quarantine(record.locality())),
            handoff_posture: Some(record.handoff_posture()),
        }
    }

    pub const fn kind(self) -> CompactionSourceClearanceKind {
        self.kind
    }

    pub const fn inspected_bytes(self) -> u64 {
        self.inspected_bytes
    }

    pub const fn handoff_posture(self) -> Option<QuarantineHandoffPosture> {
        self.handoff_posture
    }

    pub const fn locality(self) -> Option<IntegrityEvidenceLocality> {
        self.locality
    }

    pub const fn locality_owner(self) -> Option<PhysicalGenerationOwner> {
        match self.locality {
            Some(IntegrityEvidenceLocality::PhysicalScope(scope)) => Some(scope.owner()),
            Some(IntegrityEvidenceLocality::Quarantine(report)) => Some(report.owner()),
            Some(IntegrityEvidenceLocality::SupportReport) | None => None,
        }
    }

    pub const fn permits_compaction_movement(self) -> bool {
        matches!(
            self.kind,
            CompactionSourceClearanceKind::IntactIntegrityEvidence
        ) && self.locality_owner().is_some()
    }
}
