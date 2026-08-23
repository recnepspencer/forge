use crate::ForbiddenShortcutKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SyntheticHarnessShortcutDenialReceipt {
    shortcut: ForbiddenShortcutKind,
    boundary: ShortcutRejectionBoundary,
}

impl SyntheticHarnessShortcutDenialReceipt {
    pub(crate) const fn from_store_denial(
        shortcut: ForbiddenShortcutKind,
        boundary: ShortcutRejectionBoundary,
    ) -> Self {
        Self { shortcut, boundary }
    }

    pub const fn shortcut(&self) -> ForbiddenShortcutKind {
        self.shortcut
    }

    pub const fn boundary(&self) -> ShortcutRejectionBoundary {
        self.boundary
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ShortcutRejectionBoundary {
    EvidenceLooseLog,
    ScenarioJsonAuthority,
    EvidenceTerminalProjection,
    EvidenceSameRunSelfComparison,
    FaultDeliveryPrivateMutation,
    OracleFixtureLabel,
    TranscriptCopiedFields,
    PlanProofProgressionSkipped,
    OracleTestSupportVerdict,
    ScenarioTerminalProjection,
    ScenarioRawStringAuthority,
    ScenarioCopiedDigest,
    ScenarioFixtureLabel,
    ScenarioProofProgressionSkipped,
    EvidenceSameRunTranscript,
    EvidenceLooseLogTranscript,
    EvidenceTerminalJsonTranscript,
    HarnessBoundaryCopiedRecoveryReport,
    HarnessBoundaryLogOutput,
    HarnessBoundarySameRunSelfComparison,
    HarnessBoundaryTerminalProjection,
    HarnessBoundaryTestSupportMeaning,
    HarnessBoundaryProofProgressionSkipped,
}
