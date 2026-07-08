#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8LayoutCorruptionClass {
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
pub enum S8LayoutReadmissionSource {
    QuarantineRecovery,
    OfflineRecoveryEvidence,
    TerminalImport,
}
