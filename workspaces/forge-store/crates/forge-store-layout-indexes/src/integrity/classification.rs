#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutCorruptionClass {
    Clean,
    NotFound,
    Unsupported,
    StaleBinding,
    DerivedProjectionCorruption,
    AuthoritativeArtifactCorruption,
    QuarantineRequired,
    ReadmissionRequired,
    MigrationRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutReadmissionSource {
    QuarantineRecovery,
    OfflineRecoveryEvidence,
    TerminalImport,
}
