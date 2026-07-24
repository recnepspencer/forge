use super::WorthQueryArtifactNativeAccessCounters;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryArtifactNativeAccessDenialKind {
    AccessPathDenied,
    NativeProviderUnavailable,
    ForeignRuntime,
    StaleInstallationGeneration,
    OperationMismatch,
    RunMismatch,
    StageMismatch,
    BasisMismatch,
    PayloadOwnerMismatch,
    ArtifactContractMismatch,
    ForeignThread,
    ProviderSessionMismatch,
    LayoutMismatch,
    FieldNotDeclared,
    FieldSliceDenied,
    RowBatchDenied,
    ProviderNativeProjectionRequired,
    ChunkingDenied,
    ProjectionDenied,
    ScalarFallbackDenied,
    BoundsExceeded,
    AlignmentMismatch,
    StaleBorrowGeneration,
    AlreadyDisposed,
    ProviderDenied,
    ProviderShapeMismatch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryArtifactNativeAccessDenial {
    kind: WorthQueryArtifactNativeAccessDenialKind,
    artifact_family: Option<String>,
    detail: &'static str,
    counters: WorthQueryArtifactNativeAccessCounters,
}

impl WorthQueryArtifactNativeAccessDenial {
    pub(crate) fn new(
        kind: WorthQueryArtifactNativeAccessDenialKind,
        artifact_family: Option<&str>,
        detail: &'static str,
        counters: WorthQueryArtifactNativeAccessCounters,
    ) -> Self {
        Self {
            kind,
            artifact_family: artifact_family.map(str::to_owned),
            detail,
            counters,
        }
    }

    pub const fn kind(&self) -> WorthQueryArtifactNativeAccessDenialKind {
        self.kind
    }

    pub fn artifact_family(&self) -> Option<&str> {
        self.artifact_family.as_deref()
    }

    pub const fn detail(&self) -> &'static str {
        self.detail
    }

    pub const fn counters(&self) -> WorthQueryArtifactNativeAccessCounters {
        self.counters
    }
}
