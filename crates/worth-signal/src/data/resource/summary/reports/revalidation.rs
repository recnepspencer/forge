use crate::data::resource::lifecycle::ResourceLifecycleTransition;
use crate::data::resource::revalidation::{
    AdmittedResourceRevalidation, DeniedResourceRevalidation,
};
use serde::Serialize;

use super::super::performance::ResourceBoundaryPerformanceEnvelope;
use super::lifecycle::ResourceLifecycleSummary;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ResourceRevalidationReport {
    admitted_revalidation: Option<AdmittedResourceRevalidation>,
    denied_revalidation: Option<DeniedResourceRevalidation>,
    lifecycle: Option<ResourceLifecycleSummary>,
    transition: Option<ResourceLifecycleTransition>,
    performance: ResourceBoundaryPerformanceEnvelope,
}

impl ResourceRevalidationReport {
    pub(crate) fn admitted(
        admitted_revalidation: AdmittedResourceRevalidation,
        lifecycle: ResourceLifecycleSummary,
        transition: ResourceLifecycleTransition,
        performance: ResourceBoundaryPerformanceEnvelope,
    ) -> Self {
        Self {
            admitted_revalidation: Some(admitted_revalidation),
            denied_revalidation: None,
            lifecycle: Some(lifecycle),
            transition: Some(transition),
            performance,
        }
    }

    pub(crate) fn denied(
        denied_revalidation: DeniedResourceRevalidation,
        performance: ResourceBoundaryPerformanceEnvelope,
    ) -> Self {
        Self {
            admitted_revalidation: None,
            denied_revalidation: Some(denied_revalidation),
            lifecycle: None,
            transition: None,
            performance,
        }
    }

    pub fn admitted_revalidation(&self) -> Option<AdmittedResourceRevalidation> {
        self.admitted_revalidation.clone()
    }

    pub fn denied_revalidation(&self) -> Option<DeniedResourceRevalidation> {
        self.denied_revalidation
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
