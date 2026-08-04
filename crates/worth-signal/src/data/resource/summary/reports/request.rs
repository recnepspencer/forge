use crate::data::resource::lifecycle::ResourceLifecycleTransition;
use crate::data::resource::proof::AdmittedResourceRequest;
use crate::data::resource::request::ResourceRequestHandle;
use crate::data::resource::supersession::{
    ResourceIntentEquivalenceCoalescing, ResourceSupersessionRecord,
};
use serde::{Deserialize, Serialize};

use super::super::performance::ResourceBoundaryPerformanceEnvelope;
use super::lifecycle::ResourceLifecycleSummary;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceRequestAdmissionReport {
    admitted_request: AdmittedResourceRequest,
    lifecycle: ResourceLifecycleSummary,
    transition: ResourceLifecycleTransition,
    supersession_record: Option<ResourceSupersessionRecord>,
    intent_equivalence_coalescing: Option<ResourceIntentEquivalenceCoalescing>,
    superseded_request: Option<ResourceRequestHandle>,
    superseded_transition: Option<ResourceLifecycleTransition>,
    performance: ResourceBoundaryPerformanceEnvelope,
}

impl ResourceRequestAdmissionReport {
    pub(crate) fn new(
        admitted_request: AdmittedResourceRequest,
        lifecycle: ResourceLifecycleSummary,
        transition: ResourceLifecycleTransition,
        supersession_record: Option<ResourceSupersessionRecord>,
        intent_equivalence_coalescing: Option<ResourceIntentEquivalenceCoalescing>,
        performance: ResourceBoundaryPerformanceEnvelope,
    ) -> Self {
        Self {
            admitted_request,
            lifecycle,
            transition,
            supersession_record: supersession_record.clone(),
            intent_equivalence_coalescing,
            superseded_request: supersession_record
                .as_ref()
                .map(ResourceSupersessionRecord::previous),
            superseded_transition: supersession_record
                .as_ref()
                .map(ResourceSupersessionRecord::lifecycle_transition),
            performance,
        }
    }

    pub fn admitted_request(&self) -> AdmittedResourceRequest {
        self.admitted_request
    }

    pub fn lifecycle(&self) -> ResourceLifecycleSummary {
        self.lifecycle
    }

    pub fn transition(&self) -> ResourceLifecycleTransition {
        self.transition
    }

    pub fn supersession_record(&self) -> Option<ResourceSupersessionRecord> {
        self.supersession_record.clone()
    }

    pub fn intent_equivalence_coalescing(&self) -> Option<ResourceIntentEquivalenceCoalescing> {
        self.intent_equivalence_coalescing.clone()
    }

    pub fn superseded_request(&self) -> Option<ResourceRequestHandle> {
        self.superseded_request
    }

    pub fn superseded_transition(&self) -> Option<ResourceLifecycleTransition> {
        self.superseded_transition
    }

    pub fn performance(&self) -> ResourceBoundaryPerformanceEnvelope {
        self.performance
    }
}
