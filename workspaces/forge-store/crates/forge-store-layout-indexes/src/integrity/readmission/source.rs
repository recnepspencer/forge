#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutReadmissionSource {
    QuarantineRecovery,
    OfflineRecoveryEvidence,
    TerminalImport,
}
