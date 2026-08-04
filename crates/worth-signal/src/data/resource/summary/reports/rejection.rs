use crate::data::resource::lifecycle::ResourceLifecycleTransition;
use crate::data::resource::rejection::{DeniedResourceRejection, RejectedResourceRequest};
use serde::{Deserialize, Serialize};

use super::super::performance::ResourceBoundaryPerformanceEnvelope;
use super::lifecycle::ResourceLifecycleSummary;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceRejectionReport {
    rejected_request: Option<RejectedResourceRequest>,
    denied_rejection: Option<DeniedResourceRejection>,
    lifecycle: Option<ResourceLifecycleSummary>,
    transition: Option<ResourceLifecycleTransition>,
    performance: ResourceBoundaryPerformanceEnvelope,
}

impl ResourceRejectionReport {
    pub(crate) fn admitted(
        rejected_request: RejectedResourceRequest,
        lifecycle: ResourceLifecycleSummary,
        transition: ResourceLifecycleTransition,
        performance: ResourceBoundaryPerformanceEnvelope,
    ) -> Self {
        Self {
            rejected_request: Some(rejected_request),
            denied_rejection: None,
            lifecycle: Some(lifecycle),
            transition: Some(transition),
            performance,
        }
    }

    pub(crate) fn denied(
        denied_rejection: DeniedResourceRejection,
        performance: ResourceBoundaryPerformanceEnvelope,
    ) -> Self {
        Self {
            rejected_request: None,
            denied_rejection: Some(denied_rejection),
            lifecycle: None,
            transition: None,
            performance,
        }
    }

    pub fn rejected_request(&self) -> Option<RejectedResourceRequest> {
        self.rejected_request.clone()
    }

    pub fn denied_rejection(&self) -> Option<DeniedResourceRejection> {
        self.denied_rejection
    }

    pub fn lifecycle(&self) -> Option<ResourceLifecycleSummary> {
        self.lifecycle
    }

    pub fn transition(&self) -> Option<ResourceLifecycleTransition> {
        self.transition
    }

    pub fn performance(&self) -> ResourceBoundaryPerformanceEnvelope {
        self.performance
    }
}
