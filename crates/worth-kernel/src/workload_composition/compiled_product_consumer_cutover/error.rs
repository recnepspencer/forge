#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum KernelCompiledProductConsumerDependencyErrorKind {
    MissingCurrentSourcePath,
    MissingCurrentConsumerSurface,
    QueryBackedConsumerMissingRealQueryLane,
    NonQueryConsumerNamedAsQueryLane,
    MissingRequiredCluster,
    DuplicateClusterBinding,
    MissingCoveredReuseSurface,
    DeclaredCoveredReuseSurfaceNotInventoryBacked,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KernelCompiledProductConsumerDependencyError {
    kind: KernelCompiledProductConsumerDependencyErrorKind,
    detail: String,
}

impl KernelCompiledProductConsumerDependencyError {
    pub(super) fn new(
        kind: KernelCompiledProductConsumerDependencyErrorKind,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }

    pub const fn kind(&self) -> KernelCompiledProductConsumerDependencyErrorKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}
