use crate::data::resource::completion::{AdmittedResourceCompletion, DeniedResourceCompletion};
use serde::{Deserialize, Serialize};

use super::super::super::performance::ResourceBoundaryPerformanceEnvelope;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceCompletionAdmissionReport {
    admitted_completion: Option<AdmittedResourceCompletion>,
    denied_completion: Option<DeniedResourceCompletion>,
    performance: ResourceBoundaryPerformanceEnvelope,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceCompletionBatchAdmissionReport {
    admitted_completions: Vec<AdmittedResourceCompletion>,
    denied_completions: Vec<DeniedResourceCompletion>,
    input_width: u32,
    deduplicated_width: u32,
    duplicate_width: u32,
    performance: ResourceBoundaryPerformanceEnvelope,
}

impl ResourceCompletionAdmissionReport {
    pub(crate) fn admitted(
        admitted_completion: AdmittedResourceCompletion,
        performance: ResourceBoundaryPerformanceEnvelope,
    ) -> Self {
        Self {
            admitted_completion: Some(admitted_completion),
            denied_completion: None,
            performance,
        }
    }

    pub(crate) fn denied(
        denied_completion: DeniedResourceCompletion,
        performance: ResourceBoundaryPerformanceEnvelope,
    ) -> Self {
        Self {
            admitted_completion: None,
            denied_completion: Some(denied_completion),
            performance,
        }
    }

    pub fn admitted_completion(self) -> Option<AdmittedResourceCompletion> {
        self.admitted_completion
    }

    pub fn denied_completion(self) -> Option<DeniedResourceCompletion> {
        self.denied_completion
    }

    pub fn performance(&self) -> ResourceBoundaryPerformanceEnvelope {
        self.performance
    }
}

impl ResourceCompletionBatchAdmissionReport {
    pub(crate) fn new(
        admitted_completions: Vec<AdmittedResourceCompletion>,
        denied_completions: Vec<DeniedResourceCompletion>,
        input_width: u32,
        duplicate_width: u32,
        performance: ResourceBoundaryPerformanceEnvelope,
    ) -> Self {
        Self {
            admitted_completions,
            denied_completions,
            input_width,
            deduplicated_width: input_width.saturating_sub(duplicate_width),
            duplicate_width,
            performance,
        }
    }

    pub fn admitted_completions(&self) -> &[AdmittedResourceCompletion] {
        &self.admitted_completions
    }

    pub fn denied_completions(&self) -> &[DeniedResourceCompletion] {
        &self.denied_completions
    }

    pub fn into_parts(
        self,
    ) -> (
        Vec<AdmittedResourceCompletion>,
        Vec<DeniedResourceCompletion>,
    ) {
        (self.admitted_completions, self.denied_completions)
    }

    pub fn input_width(&self) -> u32 {
        self.input_width
    }

    pub fn deduplicated_width(&self) -> u32 {
        self.deduplicated_width
    }

    pub fn duplicate_width(&self) -> u32 {
        self.duplicate_width
    }

    pub fn performance(&self) -> ResourceBoundaryPerformanceEnvelope {
        self.performance
    }
}
