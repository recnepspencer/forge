use crate::runtime::WorthUiQuerySupportStatus;
use crate::source::WorthUiArtifactEquivalenceBasis;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiRuntimeEquivalenceBasis {
    artifact_equivalence_basis: WorthUiArtifactEquivalenceBasis,
    required_query_support_status: WorthUiQuerySupportStatus,
}

impl WorthUiRuntimeEquivalenceBasis {
    pub fn semantic_artifact_meaning() -> Self {
        Self {
            artifact_equivalence_basis: WorthUiArtifactEquivalenceBasis::semantic(),
            required_query_support_status: WorthUiQuerySupportStatus::Supported,
        }
    }

    pub(crate) fn artifact_equivalence_basis(self) -> WorthUiArtifactEquivalenceBasis {
        self.artifact_equivalence_basis
    }

    pub fn required_query_support_status(self) -> WorthUiQuerySupportStatus {
        self.required_query_support_status
    }

    #[cfg(test)]
    pub(crate) fn with_required_query_support_status_for_test(
        mut self,
        required_query_support_status: WorthUiQuerySupportStatus,
    ) -> Self {
        self.required_query_support_status = required_query_support_status;
        self
    }
}
