#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RecoveryIntegrityIngressRejection {
    MissingBoundedArtifact,
    ScopeMismatch,
    SourceIncarnationMismatch,
}
