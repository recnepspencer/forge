use crate::{
    PhysicalSubstrateLane, RuntimeVerifierParityTrace, RuntimeVerifierRelationship,
    ScenarioDenialBoundary, ShortcutRejectionTrace,
};
use worth_store_physical_format::{
    PhysicalStoreRuntimeCounterSnapshot, PhysicalStoreRuntimeEvidence,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalStoreRuntimeEvidenceRow {
    OperationSurface,
    RuntimeVerifierParity,
    ShortcutRejections,
}

impl PhysicalStoreRuntimeEvidenceRow {
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
pub struct PhysicalStoreRuntimeEvidenceReport {
    row: PhysicalStoreRuntimeEvidenceRow,
    lane: PhysicalSubstrateLane,
    observed_references: u32,
    counters: PhysicalStoreRuntimeCounterSnapshot,
    parity: RuntimeVerifierParityTrace,
    shortcut_rejections: ShortcutRejectionTrace,
}

impl PhysicalStoreRuntimeEvidenceReport {
    pub fn from_facade_evidence(
        row: PhysicalStoreRuntimeEvidenceRow,
        evidence: &PhysicalStoreRuntimeEvidence,
    ) -> Result<Self, PhysicalStoreRuntimeEvidenceDenial> {
        if row != PhysicalStoreRuntimeEvidenceRow::OperationSurface
            && row != PhysicalStoreRuntimeEvidenceRow::RuntimeVerifierParity
        {
            return Err(PhysicalStoreRuntimeEvidenceDenial::UnexpectedEvidenceRow(
                row,
            ));
        }
        if !evidence.proves_platform_boundary() {
            return Err(PhysicalStoreRuntimeEvidenceDenial::MissingFacadeEvidence);
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
        counters: PhysicalStoreRuntimeCounterSnapshot,
    ) -> Result<Self, PhysicalStoreRuntimeEvidenceDenial> {
        if counters.full_store_materialization_rejections() == 0
            || counters.backend_residue_guess_rejections() == 0
        {
            return Err(PhysicalStoreRuntimeEvidenceDenial::MissingShortcutRejection);
        }
        Ok(Self::new(
            PhysicalStoreRuntimeEvidenceRow::ShortcutRejections,
            0,
            counters,
            RuntimeVerifierParityTrace::new(RuntimeVerifierRelationship::NotApplicable),
            ShortcutRejectionTrace::new(vec![
                ScenarioDenialBoundary::WholeStoreMaterialization,
                ScenarioDenialBoundary::BackendResidueGuessing,
            ]),
        ))
    }

    pub const fn row(&self) -> PhysicalStoreRuntimeEvidenceRow {
        self.row
    }

    pub const fn lane(&self) -> PhysicalSubstrateLane {
        self.lane
    }

    pub const fn observed_references(&self) -> u32 {
        self.observed_references
    }

    pub const fn counters(&self) -> PhysicalStoreRuntimeCounterSnapshot {
        self.counters
    }

    pub const fn parity(&self) -> RuntimeVerifierParityTrace {
        self.parity
    }

    pub fn shortcut_rejections(&self) -> &[ScenarioDenialBoundary] {
        self.shortcut_rejections.forbidden_shortcuts()
    }

    const fn new(
        row: PhysicalStoreRuntimeEvidenceRow,
        observed_references: u32,
        counters: PhysicalStoreRuntimeCounterSnapshot,
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
pub enum PhysicalStoreRuntimeEvidenceDenial {
    UnexpectedEvidenceRow(PhysicalStoreRuntimeEvidenceRow),
    MissingFacadeEvidence,
    MissingShortcutRejection,
}
