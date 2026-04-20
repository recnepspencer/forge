use serde::{Deserialize, Serialize};

use super::StableBasisReadRequest;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StableBasisId(String);

impl StableBasisId {
    pub(crate) fn from_request(request: &StableBasisReadRequest) -> Self {
        Self(format!(
            "stable-basis|{}|{}|{}|{}|{}|{}|{}",
            request.branch_id().0,
            request.frontier_commit_id().0,
            request.read_scope().fingerprint(),
            request.support_context_digest(),
            request.schema_boundary_artifact_id(),
            request.layout_posture().as_str(),
            request.authority_basis_digest(),
        ))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub(crate) fn from_string(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}
