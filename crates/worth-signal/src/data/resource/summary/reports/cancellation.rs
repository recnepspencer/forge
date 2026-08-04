use crate::data::resource::cancellation::{
    CancelledResourceRequest, DeniedResourceCancellation, ResourceDependentCancellationPropagation,
};
use crate::data::resource::lifecycle::ResourceLifecycleTransition;
use serde::{Deserialize, Serialize};

use super::super::performance::ResourceBoundaryPerformanceEnvelope;
use super::lifecycle::ResourceLifecycleSummary;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceCancellationReport {
    cancelled_request: Option<CancelledResourceRequest>,
    dependent_propagation: Option<ResourceDependentCancellationPropagation>,
    denied_cancellation: Option<DeniedResourceCancellation>,
    lifecycle: Option<ResourceLifecycleSummary>,
    transition: Option<ResourceLifecycleTransition>,
    performance: ResourceBoundaryPerformanceEnvelope,
}

impl ResourceCancellationReport {
    pub(crate) fn admitted(
        cancelled_request: CancelledResourceRequest,
        dependent_propagation: Option<ResourceDependentCancellationPropagation>,
        lifecycle: ResourceLifecycleSummary,
        transition: ResourceLifecycleTransition,
        performance: ResourceBoundaryPerformanceEnvelope,
    ) -> Self {
        Self {
            cancelled_request: Some(cancelled_request),
            dependent_propagation,
            denied_cancellation: None,
            lifecycle: Some(lifecycle),
            transition: Some(transition),
            performance,
        }
    }

    pub(crate) fn denied(
        denied_cancellation: DeniedResourceCancellation,
        performance: ResourceBoundaryPerformanceEnvelope,
    ) -> Self {
        Self {
            cancelled_request: None,
            dependent_propagation: None,
            denied_cancellation: Some(denied_cancellation),
            lifecycle: None,
            transition: None,
            performance,
        }
    }

    pub fn cancelled_request(&self) -> Option<CancelledResourceRequest> {
        self.cancelled_request.clone()
    }

    pub fn denied_cancellation(&self) -> Option<DeniedResourceCancellation> {
        self.denied_cancellation
    }

    pub fn dependent_propagation(&self) -> Option<ResourceDependentCancellationPropagation> {
        self.dependent_propagation.clone()
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
