use crate::projection::diagnostic_surfaces::read_proof::report::TopologyReadRequestFamily;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TopologyReadErrorKind {
    CanonicalLoweringResolution,
    ReadFamilyExecutionDenied,
    UnsupportedTraversalDepth,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopologyReadError {
    kind: TopologyReadErrorKind,
    detail: String,
}

impl TopologyReadError {
    pub(crate) fn canonical_lowering_resolution(detail: impl Into<String>) -> Self {
        Self {
            kind: TopologyReadErrorKind::CanonicalLoweringResolution,
            detail: detail.into(),
        }
    }

    pub(crate) fn read_family_execution_denied(detail: impl Into<String>) -> Self {
        Self {
            kind: TopologyReadErrorKind::ReadFamilyExecutionDenied,
            detail: detail.into(),
        }
    }

    pub(crate) fn unsupported_traversal_depth(
        request_family: TopologyReadRequestFamily,
        requested_depth: usize,
        maximum_supported_depth: usize,
    ) -> Self {
        Self {
            kind: TopologyReadErrorKind::UnsupportedTraversalDepth,
            detail: format!(
                "unsupported traversal depth `{requested_depth}` for `{request_family:?}`; maximum supported depth is `{maximum_supported_depth}`"
            ),
        }
    }

    pub fn kind(&self) -> TopologyReadErrorKind {
        self.kind
    }
}

impl std::fmt::Display for TopologyReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.detail.as_str())
    }
}

<<<<<<< HEAD
impl std::error::Error for TopologyDomainQueryError {}
=======
impl std::error::Error for TopologyReadError {}
>>>>>>> origin/master
