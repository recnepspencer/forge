use super::RelationalExecutionBasisCounters;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RelationalExecutionBasisDenialKind {
    VersionUnavailable,
    BranchMismatch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RelationalExecutionBasisDenial {
    kind: RelationalExecutionBasisDenialKind,
    detail: &'static str,
    counters: RelationalExecutionBasisCounters,
}

impl RelationalExecutionBasisDenial {
    pub(crate) fn new(
        kind: RelationalExecutionBasisDenialKind,
        detail: &'static str,
        counters: RelationalExecutionBasisCounters,
    ) -> Self {
        Self {
            kind,
            detail,
            counters,
        }
    }

    pub fn kind(&self) -> RelationalExecutionBasisDenialKind {
        self.kind
    }

    pub fn detail(&self) -> &'static str {
        self.detail
    }

    pub fn counters(&self) -> &RelationalExecutionBasisCounters {
        &self.counters
    }
}
