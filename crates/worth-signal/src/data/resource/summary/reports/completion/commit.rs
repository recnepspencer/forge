use crate::data::resource::completion::{
    CommittedResourceCompletionArtifact, RolledBackResourceCompletionArtifact,
};
use crate::data::resource::lifecycle::ResourceLifecycleTransition;
use serde::{Deserialize, Serialize};

use super::super::super::performance::ResourceBoundaryPerformanceEnvelope;
use super::super::lifecycle::ResourceLifecycleSummary;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceCompletionCommitReport {
    committed_completion: CommittedResourceCompletionArtifact,
    lifecycle: ResourceLifecycleSummary,
    transition: ResourceLifecycleTransition,
    performance: ResourceBoundaryPerformanceEnvelope,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceCompletionRollbackReport {
    rolled_back_completion: RolledBackResourceCompletionArtifact,
    performance: ResourceBoundaryPerformanceEnvelope,
}

impl ResourceCompletionCommitReport {
    pub(crate) fn new(
        committed_completion: CommittedResourceCompletionArtifact,
        lifecycle: ResourceLifecycleSummary,
        transition: ResourceLifecycleTransition,
        performance: ResourceBoundaryPerformanceEnvelope,
    ) -> Self {
        Self {
            committed_completion,
            lifecycle,
            transition,
            performance,
        }
    }

    pub fn committed_completion(self) -> CommittedResourceCompletionArtifact {
        self.committed_completion
    }

    pub fn lifecycle(&self) -> ResourceLifecycleSummary {
        self.lifecycle
    }

    pub fn transition(&self) -> ResourceLifecycleTransition {
        self.transition
    }

    pub fn performance(&self) -> ResourceBoundaryPerformanceEnvelope {
        self.performance
    }
}

impl ResourceCompletionRollbackReport {
    pub(crate) fn new(
        rolled_back_completion: RolledBackResourceCompletionArtifact,
        performance: ResourceBoundaryPerformanceEnvelope,
    ) -> Self {
        Self {
            rolled_back_completion,
            performance,
        }
    }

    pub fn rolled_back_completion(self) -> RolledBackResourceCompletionArtifact {
        self.rolled_back_completion
    }

    pub fn performance(&self) -> ResourceBoundaryPerformanceEnvelope {
        self.performance
    }
}
