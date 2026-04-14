#[derive(Debug, Clone, Copy)]
pub(crate) struct ExternalArtifactIntakeCapabilityProof {
    _sealed: (),
}

impl ExternalArtifactIntakeCapabilityProof {
    pub(crate) fn issue() -> Self {
        Self { _sealed: () }
    }
}
