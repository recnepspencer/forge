use crate::capability::TaskPresentationId;

use super::{
    TaskPresentationCancellationPosture, TaskPresentationFailurePosture, TaskPresentationFamily,
    TaskPresentationLifecyclePosture, TaskPresentationProjectionEligibility,
    TaskPresentationRuntimeAuthorityPosture,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TaskPresentationDescriptor {
    id: TaskPresentationId,
    family: TaskPresentationFamily,
    lifecycle_posture: Option<TaskPresentationLifecyclePosture>,
    cancellation_posture: Option<TaskPresentationCancellationPosture>,
    failure_posture: Option<TaskPresentationFailurePosture>,
    projection_eligibility: Option<TaskPresentationProjectionEligibility>,
    runtime_authority_posture: Option<TaskPresentationRuntimeAuthorityPosture>,
}

impl TaskPresentationDescriptor {
    pub fn new(id: TaskPresentationId, family: TaskPresentationFamily) -> Self {
        Self {
            id,
            family,
            lifecycle_posture: None,
            cancellation_posture: None,
            failure_posture: None,
            projection_eligibility: None,
            runtime_authority_posture: None,
        }
    }

    pub fn with_lifecycle_posture(mut self, posture: TaskPresentationLifecyclePosture) -> Self {
        self.lifecycle_posture = Some(posture);
        self
    }

    pub fn with_cancellation_posture(
        mut self,
        posture: TaskPresentationCancellationPosture,
    ) -> Self {
        self.cancellation_posture = Some(posture);
        self
    }

    pub fn with_failure_posture(mut self, posture: TaskPresentationFailurePosture) -> Self {
        self.failure_posture = Some(posture);
        self
    }

    pub fn with_projection_eligibility(
        mut self,
        eligibility: TaskPresentationProjectionEligibility,
    ) -> Self {
        self.projection_eligibility = Some(eligibility);
        self
    }

    pub fn with_runtime_authority_posture(
        mut self,
        posture: TaskPresentationRuntimeAuthorityPosture,
    ) -> Self {
        self.runtime_authority_posture = Some(posture);
        self
    }

    pub fn id(&self) -> &TaskPresentationId {
        &self.id
    }

    pub fn family(&self) -> &TaskPresentationFamily {
        &self.family
    }

    pub fn lifecycle_posture(&self) -> Option<&TaskPresentationLifecyclePosture> {
        self.lifecycle_posture.as_ref()
    }

    pub fn cancellation_posture(&self) -> Option<&TaskPresentationCancellationPosture> {
        self.cancellation_posture.as_ref()
    }

    pub fn failure_posture(&self) -> Option<&TaskPresentationFailurePosture> {
        self.failure_posture.as_ref()
    }

    pub fn projection_eligibility(&self) -> Option<&TaskPresentationProjectionEligibility> {
        self.projection_eligibility.as_ref()
    }

    pub fn runtime_authority_posture(&self) -> Option<&TaskPresentationRuntimeAuthorityPosture> {
        self.runtime_authority_posture.as_ref()
    }
}
