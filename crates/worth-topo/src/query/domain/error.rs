use super::proof::report::TopologyDomainQueryRequestFamily;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TopologyDomainQueryErrorKind {
    CanonicalLoweringResolution,
    ReadFamilyExecutionDenied,
    UnsupportedTraversalDepth,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopologyDomainQueryError {
    kind: TopologyDomainQueryErrorKind,
    detail: String,
}

impl TopologyDomainQueryError {
    pub(crate) fn canonical_lowering_resolution(detail: impl Into<String>) -> Self {
        Self {
            kind: TopologyDomainQueryErrorKind::CanonicalLoweringResolution,
            detail: detail.into(),
        }
    }

    pub(crate) fn read_family_execution_denied(detail: impl Into<String>) -> Self {
        Self {
            kind: TopologyDomainQueryErrorKind::ReadFamilyExecutionDenied,
            detail: detail.into(),
        }
    }

    pub(crate) fn unsupported_traversal_depth(
        request_family: TopologyDomainQueryRequestFamily,
        requested_depth: usize,
        maximum_supported_depth: usize,
    ) -> Self {
        Self {
            kind: TopologyDomainQueryErrorKind::UnsupportedTraversalDepth,
            detail: format!(
                "unsupported traversal depth `{requested_depth}` for `{request_family:?}`; maximum supported depth is `{maximum_supported_depth}`"
            ),
        }
    }

    pub fn kind(&self) -> TopologyDomainQueryErrorKind {
        self.kind
    }
}

impl std::fmt::Display for TopologyDomainQueryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.detail.as_str())
    }
}

impl std::error::Error for TopologyDomainQueryError {}
