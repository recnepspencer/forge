use forge_relational::facade::history::{BranchId, CommitId};
use serde::Serialize;

use crate::live_query::evidence::LiveQueryComplexityStatus;
use crate::live_query::restart::StableBasisSurvival;
use crate::live_query::retention_descriptor::{
    descriptor_for_stable_basis, ContinuationRetentionDescriptor, ContinuationRetentionStatus,
};

use super::{
    validation::StableBasisPublicationPlan, StableBasisId, StableBasisLayoutPosture,
    StableBasisReadRequest, StableBasisReadScope,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct StableBasisReadPlan {
    request: StableBasisReadRequest,
    stable_basis_id: StableBasisId,
}

impl StableBasisReadPlan {
    pub(crate) fn new(request: StableBasisReadRequest, stable_basis_id: StableBasisId) -> Self {
        Self {
            request,
            stable_basis_id,
        }
    }

    pub fn request(&self) -> &StableBasisReadRequest {
        &self.request
    }

    pub fn stable_basis_id(&self) -> &StableBasisId {
        &self.stable_basis_id
    }

    pub(crate) fn into_publication_plan(self) -> StableBasisPublicationPlan {
        let descriptor = descriptor_for_stable_basis(&self.stable_basis_id, &self.request);
        StableBasisPublicationPlan::new(self.request, self.stable_basis_id, descriptor)
    }

    pub fn into_handle(
        self,
        retention_descriptor: ContinuationRetentionDescriptor,
    ) -> StableBasisHandle {
        StableBasisHandle::new(self.request, self.stable_basis_id, retention_descriptor)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct StableBasisHandle {
    request: StableBasisReadRequest,
    stable_basis_id: StableBasisId,
    retention_descriptor: ContinuationRetentionDescriptor,
}

impl StableBasisHandle {
    pub(crate) fn new(
        request: StableBasisReadRequest,
        stable_basis_id: StableBasisId,
        retention_descriptor: ContinuationRetentionDescriptor,
    ) -> Self {
        Self {
            request,
            stable_basis_id,
            retention_descriptor,
        }
    }

    pub fn stable_basis_id(&self) -> &StableBasisId {
        &self.stable_basis_id
    }

    pub fn request(&self) -> &StableBasisReadRequest {
        &self.request
    }

    pub fn branch_id(&self) -> &BranchId {
        self.request.branch_id()
    }

    pub fn frontier_commit_id(&self) -> CommitId {
        self.request.frontier_commit_id()
    }

    pub fn read_scope(&self) -> &StableBasisReadScope {
        self.request.read_scope()
    }

    pub fn schema_boundary_artifact_id(&self) -> &str {
        self.request.schema_boundary_artifact_id()
    }

    pub fn support_context_digest(&self) -> &str {
        self.request.support_context_digest()
    }

    pub fn layout_posture(&self) -> StableBasisLayoutPosture {
        self.request.layout_posture()
    }

    pub fn authority_basis_digest(&self) -> &str {
        self.request.authority_basis_digest()
    }

    pub fn retention_status(&self) -> &ContinuationRetentionStatus {
        self.request.retention_status()
    }

    pub fn retention_descriptor(&self) -> &ContinuationRetentionDescriptor {
        &self.retention_descriptor
    }

    pub fn fallback_class(&self) -> Option<&str> {
        match self.retention_status() {
            ContinuationRetentionStatus::Retained => None,
            ContinuationRetentionStatus::Degraded { fallback_class } => Some(fallback_class),
            ContinuationRetentionStatus::Rejected { reason } => Some(reason),
        }
    }

    pub fn complexity_status(&self) -> LiveQueryComplexityStatus {
        StableBasisSurvival::from_handle(self).complexity_status()
    }

    pub fn required_support_artifact_set(&self) -> &[String] {
        self.retention_descriptor.required_support_artifact_set()
    }
}
