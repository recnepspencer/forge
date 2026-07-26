#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryArtifactDenialKind {
    ArtifactContractNotInstalled,
    ProducerRoleNotInstalled,
    ConsumerRoleNotInstalled,
    ProviderFamilyMismatch,
    EmptySemanticProjection,
    InvalidProductionEvidence,
    ForeignRuntime,
    StaleInstallationGeneration,
    OperationMismatch,
    RunMismatch,
    StageMismatch,
    BasisMismatch,
    ArtifactContractMismatch,
    PayloadOwnerMismatch,
    StageExecutionMismatch,
    ActiveWorkflowRun,
    ProductionClosed,
    MovementForbidden,
    BorrowForbidden,
    LeaseForbidden,
    StaleLifecycleGeneration,
    ActiveBorrow,
    ActiveLease,
    AlreadyDisposed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryArtifactDenial {
    kind: WorthQueryArtifactDenialKind,
    artifact_family: Option<String>,
    detail: &'static str,
    rejected_resource_release: Option<super::WorthQueryArtifactProviderReleasePosture>,
}

impl WorthQueryArtifactDenial {
    pub fn new(
        kind: WorthQueryArtifactDenialKind,
        artifact_family: Option<&str>,
        detail: &'static str,
    ) -> Self {
        Self {
            kind,
            artifact_family: artifact_family.map(str::to_owned),
            detail,
            rejected_resource_release: None,
        }
    }

    pub const fn kind(&self) -> WorthQueryArtifactDenialKind {
        self.kind
    }

    pub fn artifact_family(&self) -> Option<&str> {
        self.artifact_family.as_deref()
    }

    pub const fn detail(&self) -> &'static str {
        self.detail
    }

    pub const fn rejected_resource_release(
        &self,
    ) -> Option<super::WorthQueryArtifactProviderReleasePosture> {
        self.rejected_resource_release
    }

    pub(super) fn with_rejected_resource_release(
        mut self,
        release: super::WorthQueryArtifactProviderReleasePosture,
    ) -> Self {
        self.rejected_resource_release = Some(release);
        self
    }
}
