use super::report::WorthTopologyDomainQueryRequestFamily;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum WorthTopologyDomainQueryErrorKind {
    SnapshotIndexedResolution,
    CanonicalLoweringResolution,
    QueryNativeExecutionDenied,
    UnsupportedTraversalDepth,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct WorthTopologyDomainQueryError {
    kind: WorthTopologyDomainQueryErrorKind,
    detail: String,
}

impl WorthTopologyDomainQueryError {
    pub(crate) fn snapshot_indexed_resolution(detail: impl Into<String>) -> Self {
        Self {
            kind: WorthTopologyDomainQueryErrorKind::SnapshotIndexedResolution,
            detail: detail.into(),
        }
    }

    pub(crate) fn canonical_lowering_resolution(detail: impl Into<String>) -> Self {
        Self {
            kind: WorthTopologyDomainQueryErrorKind::CanonicalLoweringResolution,
            detail: detail.into(),
        }
    }

    pub(crate) fn query_native_execution_denied(detail: impl Into<String>) -> Self {
        Self {
            kind: WorthTopologyDomainQueryErrorKind::QueryNativeExecutionDenied,
            detail: detail.into(),
        }
    }

    pub(crate) fn unsupported_traversal_depth(
        request_family: WorthTopologyDomainQueryRequestFamily,
        requested_depth: usize,
        maximum_supported_depth: usize,
    ) -> Self {
        Self {
            kind: WorthTopologyDomainQueryErrorKind::UnsupportedTraversalDepth,
            detail: format!(
                "unsupported traversal depth `{requested_depth}` for `{request_family:?}`; maximum supported depth is `{maximum_supported_depth}`"
            ),
        }
    }

    #[cfg(test)]
    pub(crate) fn kind(&self) -> WorthTopologyDomainQueryErrorKind {
        self.kind
    }
}

impl std::fmt::Display for WorthTopologyDomainQueryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.detail.as_str())
    }
}

impl std::error::Error for WorthTopologyDomainQueryError {}
