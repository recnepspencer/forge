use crate::{
    PhysicalSubstrateLane, RuntimeVerifierParityTrace, RuntimeVerifierRelationship,
    ScenarioDenialBoundary, ShortcutRejectionTrace,
};
use worth_store_physical_format::{
    PlatformPhysicalFacadeCounterSnapshot, PlatformPhysicalFacadeEvidence,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlatformPhysicalFacadeEvidenceRow {
    OperationSurface,
    RuntimeVerifierParity,
    ShortcutRejections,
}

impl PlatformPhysicalFacadeEvidenceRow {
    pub const fn s1_required() -> [Self; 3] {
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
pub struct PlatformPhysicalFacadeEvidenceReport {
    row: PlatformPhysicalFacadeEvidenceRow,
    lane: PhysicalSubstrateLane,
    observed_references: u32,
    counters: PlatformPhysicalFacadeCounterSnapshot,
    parity: RuntimeVerifierParityTrace,
    shortcut_rejections: ShortcutRejectionTrace,
}

impl PlatformPhysicalFacadeEvidenceReport {
    pub fn from_facade_evidence(
        row: PlatformPhysicalFacadeEvidenceRow,
        evidence: &PlatformPhysicalFacadeEvidence,
    ) -> Result<Self, PlatformPhysicalFacadeEvidenceDenial> {
        if row != PlatformPhysicalFacadeEvidenceRow::OperationSurface
            && row != PlatformPhysicalFacadeEvidenceRow::RuntimeVerifierParity
        {
            return Err(PlatformPhysicalFacadeEvidenceDenial::UnexpectedEvidenceRow(
                row,
            ));
        }
        if !evidence.proves_platform_boundary() {
            return Err(PlatformPhysicalFacadeEvidenceDenial::MissingFacadeEvidence);
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
        counters: PlatformPhysicalFacadeCounterSnapshot,
    ) -> Result<Self, PlatformPhysicalFacadeEvidenceDenial> {
        if counters.full_store_materialization_rejections() == 0
            || counters.backend_residue_guess_rejections() == 0
        {
            return Err(PlatformPhysicalFacadeEvidenceDenial::MissingShortcutRejection);
        }
        Ok(Self::new(
            PlatformPhysicalFacadeEvidenceRow::ShortcutRejections,
            0,
            counters,
            RuntimeVerifierParityTrace::new(RuntimeVerifierRelationship::NotApplicable),
            ShortcutRejectionTrace::new(vec![
                ScenarioDenialBoundary::WholeStoreMaterialization,
                ScenarioDenialBoundary::BackendResidueGuessing,
            ]),
        ))
    }

    pub const fn row(&self) -> PlatformPhysicalFacadeEvidenceRow {
        self.row
    }

    pub const fn lane(&self) -> PhysicalSubstrateLane {
        self.lane
    }

    pub const fn observed_references(&self) -> u32 {
        self.observed_references
    }

    pub const fn counters(&self) -> PlatformPhysicalFacadeCounterSnapshot {
        self.counters
    }

    pub const fn parity(&self) -> RuntimeVerifierParityTrace {
        self.parity
    }

    pub fn shortcut_rejections(&self) -> &[ScenarioDenialBoundary] {
        self.shortcut_rejections.forbidden_shortcuts()
    }

    const fn new(
        row: PlatformPhysicalFacadeEvidenceRow,
        observed_references: u32,
        counters: PlatformPhysicalFacadeCounterSnapshot,
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
pub enum PlatformPhysicalFacadeEvidenceDenial {
    UnexpectedEvidenceRow(PlatformPhysicalFacadeEvidenceRow),
    MissingFacadeEvidence,
    MissingShortcutRejection,
}
