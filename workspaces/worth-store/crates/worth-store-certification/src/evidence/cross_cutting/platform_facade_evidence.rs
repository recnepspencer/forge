use crate::{
    PhysicalSubstrateLane, RuntimeVerifierParityTrace, RuntimeVerifierRelationship,
    ScenarioDenialBoundary, ShortcutRejectionTrace,
};
use worth_store_physical_format::{
    InMemoryPhysicalFormatModelCounterSnapshot, InMemoryPhysicalFormatModelEvidence,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InMemoryPhysicalFormatModelEvidenceRow {
    OperationSurface,
    RuntimeVerifierParity,
    ShortcutRejections,
}

impl InMemoryPhysicalFormatModelEvidenceRow {
    pub const fn physical_format_required() -> [Self; 3] {
        [
            Self::OperationSurface,
            Self::RuntimeVerifierParity,
            Self::ShortcutRejections,
        ]
    }

    pub const fn physical_substrate_lane(self) -> PhysicalSubstrateLane {
        PhysicalSubstrateLane::HappyAuthority
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InMemoryPhysicalFormatModelEvidenceReport {
    row: InMemoryPhysicalFormatModelEvidenceRow,
    lane: PhysicalSubstrateLane,
    observed_references: u32,
    counters: InMemoryPhysicalFormatModelCounterSnapshot,
    parity: RuntimeVerifierParityTrace,
    shortcut_rejections: ShortcutRejectionTrace,
}

impl InMemoryPhysicalFormatModelEvidenceReport {
    pub fn from_facade_evidence(
        row: InMemoryPhysicalFormatModelEvidenceRow,
        evidence: &InMemoryPhysicalFormatModelEvidence,
    ) -> Result<Self, InMemoryPhysicalFormatModelEvidenceDenial> {
        if row != InMemoryPhysicalFormatModelEvidenceRow::OperationSurface
            && row != InMemoryPhysicalFormatModelEvidenceRow::RuntimeVerifierParity
        {
            return Err(InMemoryPhysicalFormatModelEvidenceDenial::UnexpectedEvidenceRow(row));
        }
        if !evidence.satisfies_in_memory_observation_contract() {
            return Err(InMemoryPhysicalFormatModelEvidenceDenial::MissingFacadeEvidence);
        }
        Ok(Self::new(
            row,
            evidence.verified_references().len() as u32,
            evidence.counters(),
            RuntimeVerifierParityTrace::new(RuntimeVerifierRelationship::RuntimeMustMatchVerifier),
            ShortcutRejectionTrace::new(Vec::new()),
        ))
    }

    pub fn from_shortcut_counters(
        counters: InMemoryPhysicalFormatModelCounterSnapshot,
    ) -> Result<Self, InMemoryPhysicalFormatModelEvidenceDenial> {
        if counters.full_store_materialization_rejections() == 0
            || counters.backend_residue_guess_rejections() == 0
        {
            return Err(InMemoryPhysicalFormatModelEvidenceDenial::MissingShortcutRejection);
        }
        Ok(Self::new(
            InMemoryPhysicalFormatModelEvidenceRow::ShortcutRejections,
            0,
            counters,
            RuntimeVerifierParityTrace::new(RuntimeVerifierRelationship::NotApplicable),
            ShortcutRejectionTrace::new(vec![
                ScenarioDenialBoundary::WholeStoreMaterialization,
                ScenarioDenialBoundary::BackendResidueGuessing,
            ]),
        ))
    }

    pub const fn row(&self) -> InMemoryPhysicalFormatModelEvidenceRow {
        self.row
    }

    pub const fn lane(&self) -> PhysicalSubstrateLane {
        self.lane
    }

    pub const fn observed_references(&self) -> u32 {
        self.observed_references
    }

    pub const fn counters(&self) -> InMemoryPhysicalFormatModelCounterSnapshot {
        self.counters
    }

    pub const fn parity(&self) -> RuntimeVerifierParityTrace {
        self.parity
    }

    pub fn shortcut_rejections(&self) -> &[ScenarioDenialBoundary] {
        self.shortcut_rejections.forbidden_shortcuts()
    }

    const fn new(
        row: InMemoryPhysicalFormatModelEvidenceRow,
        observed_references: u32,
        counters: InMemoryPhysicalFormatModelCounterSnapshot,
        parity: RuntimeVerifierParityTrace,
        shortcut_rejections: ShortcutRejectionTrace,
    ) -> Self {
        Self {
            row,
            lane: row.physical_substrate_lane(),
            observed_references,
            counters,
            parity,
            shortcut_rejections,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InMemoryPhysicalFormatModelEvidenceDenial {
    UnexpectedEvidenceRow(InMemoryPhysicalFormatModelEvidenceRow),
    MissingFacadeEvidence,
    MissingShortcutRejection,
}
