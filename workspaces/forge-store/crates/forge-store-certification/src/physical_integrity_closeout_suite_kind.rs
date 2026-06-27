#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum S3AcceptanceSuiteKind {
    CorruptionLocalization,
    BoundaryDenial,
    HarnessTranscript,
    SyntheticShortcutRejection,
    S4IntegrityHandoff,
    LineCapComposition,
}

impl S3AcceptanceSuiteKind {
    pub const ALL: [Self; 6] = [
        Self::CorruptionLocalization,
        Self::BoundaryDenial,
        Self::HarnessTranscript,
        Self::SyntheticShortcutRejection,
        Self::S4IntegrityHandoff,
        Self::LineCapComposition,
    ];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum S3CloseoutEvidenceFamily {
    CorruptionLocalization,
    BoundaryDenial,
    HarnessTranscript,
    SyntheticShortcutRejection,
    S4IntegrityHandoff,
    LineCapComposition,
}

impl S3CloseoutEvidenceFamily {
    pub const ALL: [Self; 6] = [
        Self::CorruptionLocalization,
        Self::BoundaryDenial,
        Self::HarnessTranscript,
        Self::SyntheticShortcutRejection,
        Self::S4IntegrityHandoff,
        Self::LineCapComposition,
    ];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum S3CorruptionLocalizationBoundary {
    ByteFlip,
    TornFrame,
    StaleGeneration,
    ManifestCorruption,
    IndexPageCorruption,
    WalFrameCorruption,
    ExtentDamage,
    ChunkDamage,
}

impl S3CorruptionLocalizationBoundary {
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
