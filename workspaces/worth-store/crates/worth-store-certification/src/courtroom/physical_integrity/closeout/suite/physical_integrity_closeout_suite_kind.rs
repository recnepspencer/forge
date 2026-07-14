#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PhysicalIntegrityAcceptanceSuite {
    CorruptionLocalization,
    BoundaryDenial,
    HarnessTranscript,
    SyntheticShortcutRejection,
    RecoveryIntegrityHandoff,
    LineCapComposition,
}

impl PhysicalIntegrityAcceptanceSuite {
    pub const ALL: [Self; 6] = [
        Self::CorruptionLocalization,
        Self::BoundaryDenial,
        Self::HarnessTranscript,
        Self::SyntheticShortcutRejection,
        Self::RecoveryIntegrityHandoff,
        Self::LineCapComposition,
    ];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum IntegrityCloseoutEvidenceFamily {
    CorruptionLocalization,
    BoundaryDenial,
    HarnessTranscript,
    SyntheticShortcutRejection,
    RecoveryIntegrityHandoff,
    LineCapComposition,
}

impl IntegrityCloseoutEvidenceFamily {
    pub const ALL: [Self; 6] = [
        Self::CorruptionLocalization,
        Self::BoundaryDenial,
        Self::HarnessTranscript,
        Self::SyntheticShortcutRejection,
        Self::RecoveryIntegrityHandoff,
        Self::LineCapComposition,
    ];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CorruptionLocalizationBoundary {
    ByteFlip,
    TornFrame,
    StaleGeneration,
    ManifestCorruption,
    IndexPageCorruption,
    WalFrameCorruption,
    ExtentDamage,
    ChunkDamage,
}

impl CorruptionLocalizationBoundary {
    pub const ALL: [Self; 8] = [
        Self::ByteFlip,
        Self::TornFrame,
        Self::StaleGeneration,
        Self::ManifestCorruption,
        Self::IndexPageCorruption,
        Self::WalFrameCorruption,
        Self::ExtentDamage,
        Self::ChunkDamage,
    ];
}

pub(super) const fn required_synthetic_attempts() -> [SyntheticCloseoutShortcutAttempt; 7] {
    [
        SyntheticCloseoutShortcutAttempt::LogsOnlyProof,
        SyntheticCloseoutShortcutAttempt::SameRunSelfComparison,
        SyntheticCloseoutShortcutAttempt::ExpectedErrorsOnly,
        SyntheticCloseoutShortcutAttempt::InMemoryOnlyBuffers,
        SyntheticCloseoutShortcutAttempt::SmallFixtureOnly,
        SyntheticCloseoutShortcutAttempt::FixtureLabelsOnly,
        SyntheticCloseoutShortcutAttempt::TestSupportOwnedOracleMeaning,
    ]
}
use crate::SyntheticCloseoutShortcutAttempt;
