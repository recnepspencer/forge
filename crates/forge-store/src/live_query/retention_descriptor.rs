use crate::live_query::basis::StableBasisId;
use crate::live_query::basis::StableBasisReadRequest;
use crate::live_query::restart::StableBasisSurvival;
use forge_relational::facade::history::CommitId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContinuationRetentionStatus {
    Retained,
    Degraded { fallback_class: String },
    Rejected { reason: String },
}

impl ContinuationRetentionStatus {
    pub fn is_retained(&self) -> bool {
        matches!(self, Self::Retained)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ContinuationRetentionDescriptor {
    stable_basis_id: StableBasisId,
    minimum_retained_commit_id: CommitId,
    required_support_artifact_set: Vec<String>,
    schema_boundary_dependency: String,
    authority_replay_fallback_class: String,
    snapshot_tail_fallback_class: String,
    version: u32,
}

impl ContinuationRetentionDescriptor {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        stable_basis_id: StableBasisId,
        minimum_retained_commit_id: CommitId,
        required_support_artifact_set: Vec<String>,
        schema_boundary_dependency: impl Into<String>,
        authority_replay_fallback_class: impl Into<String>,
        snapshot_tail_fallback_class: impl Into<String>,
        version: u32,
    ) -> Self {
        Self {
            stable_basis_id,
            minimum_retained_commit_id,
            required_support_artifact_set,
            schema_boundary_dependency: schema_boundary_dependency.into(),
            authority_replay_fallback_class: authority_replay_fallback_class.into(),
            snapshot_tail_fallback_class: snapshot_tail_fallback_class.into(),
            version,
        }
    }

    pub fn stable_basis_id(&self) -> &StableBasisId {
        &self.stable_basis_id
    }

    pub fn minimum_retained_commit_id(&self) -> CommitId {
        self.minimum_retained_commit_id
    }

    pub fn required_support_artifact_set(&self) -> &[String] {
        &self.required_support_artifact_set
    }

    pub fn schema_boundary_dependency(&self) -> &str {
        &self.schema_boundary_dependency
    }

    pub fn authority_replay_fallback_class(&self) -> &str {
        &self.authority_replay_fallback_class
    }

    pub fn snapshot_tail_fallback_class(&self) -> &str {
        &self.snapshot_tail_fallback_class
    }

    pub fn version(&self) -> u32 {
        self.version
    }
}

pub(crate) fn descriptor_for_stable_basis(
    stable_basis_id: &StableBasisId,
    request: &StableBasisReadRequest,
) -> ContinuationRetentionDescriptor {
    let survival = StableBasisSurvival::from_request(request);
    let authority_replay_fallback_class = match &survival {
        StableBasisSurvival::Retained => "none",
        StableBasisSurvival::DegradedButRecoverable { fallback_class } => fallback_class.as_str(),
        StableBasisSurvival::Rejected { .. } => "rejected",
    };

    ContinuationRetentionDescriptor::new(
        stable_basis_id.clone(),
        request.frontier_commit_id(),
        vec![request.schema_boundary_artifact_id().to_string()],
        request.schema_boundary_artifact_id().to_string(),
        authority_replay_fallback_class.to_string(),
        "snapshot_tail".to_string(),
        1,
    )
}
