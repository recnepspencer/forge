use crate::data::resource::lifecycle::ResourceLifecycleTransition;
use crate::data::resource::retry::{
    AdmittedResourceRetry, DeniedResourceRetry, ScheduledResourceRetry,
};
use serde::{Deserialize, Serialize};

use super::super::performance::ResourceBoundaryPerformanceEnvelope;
use super::lifecycle::ResourceLifecycleSummary;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceRetryScheduleReport {
    scheduled_retry: Option<ScheduledResourceRetry>,
    denied_retry: Option<DeniedResourceRetry>,
    performance: ResourceBoundaryPerformanceEnvelope,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceRetryAdmissionReport {
    admitted_retry: Option<AdmittedResourceRetry>,
    denied_retry: Option<DeniedResourceRetry>,
    lifecycle: Option<ResourceLifecycleSummary>,
    transition: Option<ResourceLifecycleTransition>,
    performance: ResourceBoundaryPerformanceEnvelope,
}

impl ResourceRetryScheduleReport {
    pub(crate) fn admitted(
        scheduled_retry: ScheduledResourceRetry,
        performance: ResourceBoundaryPerformanceEnvelope,
    ) -> Self {
        Self {
            scheduled_retry: Some(scheduled_retry),
            denied_retry: None,
            performance,
        }
    }

    pub(crate) fn denied(
        denied_retry: DeniedResourceRetry,
        performance: ResourceBoundaryPerformanceEnvelope,
    ) -> Self {
        Self {
            scheduled_retry: None,
            denied_retry: Some(denied_retry),
            performance,
        }
    }

    pub fn scheduled_retry(&self) -> Option<&ScheduledResourceRetry> {
        self.scheduled_retry.as_ref()
    }

    pub fn denied_retry(&self) -> Option<DeniedResourceRetry> {
        self.denied_retry.clone()
    }

    pub fn performance(&self) -> ResourceBoundaryPerformanceEnvelope {
        self.performance
    }
}

impl ResourceRetryAdmissionReport {
    pub(crate) fn admitted(
        admitted_retry: AdmittedResourceRetry,
        lifecycle: ResourceLifecycleSummary,
        transition: ResourceLifecycleTransition,
        performance: ResourceBoundaryPerformanceEnvelope,
    ) -> Self {
        Self {
            admitted_retry: Some(admitted_retry),
            denied_retry: None,
            lifecycle: Some(lifecycle),
            transition: Some(transition),
            performance,
        }
    }

    pub(crate) fn denied(
        denied_retry: DeniedResourceRetry,
        performance: ResourceBoundaryPerformanceEnvelope,
    ) -> Self {
        Self {
            admitted_retry: None,
            denied_retry: Some(denied_retry),
            lifecycle: None,
            transition: None,
            performance,
        }
    }

    pub fn admitted_retry(&self) -> Option<&AdmittedResourceRetry> {
        self.admitted_retry.as_ref()
    }

    pub fn denied_retry(&self) -> Option<DeniedResourceRetry> {
        self.denied_retry.clone()
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
