use crate::data::resource::completion::{
    StagedDeniedResourceCompletionEffect, StagedResourceCompletionEffect,
};
use serde::{Deserialize, Serialize};

use super::super::super::performance::ResourceBoundaryPerformanceEnvelope;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceCompletionStagingReport {
    staged_effect: StagedResourceCompletionEffect,
    performance: ResourceBoundaryPerformanceEnvelope,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceCompletionDenialStagingReport {
    staged_denial_effect: StagedDeniedResourceCompletionEffect,
    performance: ResourceBoundaryPerformanceEnvelope,
}

impl ResourceCompletionStagingReport {
    pub(crate) fn new(
        staged_effect: StagedResourceCompletionEffect,
        performance: ResourceBoundaryPerformanceEnvelope,
    ) -> Self {
        Self {
            staged_effect,
            performance,
        }
    }

    pub fn staged_effect(self) -> StagedResourceCompletionEffect {
        self.staged_effect
    }

    pub fn performance(&self) -> ResourceBoundaryPerformanceEnvelope {
        self.performance
    }
}

impl ResourceCompletionDenialStagingReport {
    pub(crate) fn new(
        staged_denial_effect: StagedDeniedResourceCompletionEffect,
        performance: ResourceBoundaryPerformanceEnvelope,
    ) -> Self {
        Self {
            staged_denial_effect,
            performance,
        }
    }

    pub fn staged_denial_effect(self) -> StagedDeniedResourceCompletionEffect {
        self.staged_denial_effect
    }

    pub fn performance(&self) -> ResourceBoundaryPerformanceEnvelope {
        self.performance
    }
}
