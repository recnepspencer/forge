use serde::{Deserialize, Serialize};

use crate::data::handle::NodeId;

use super::denial::HostComputedDenialClass;
use super::dependency_patch::HostComputedDependencyPatch;
use super::descriptor::{HostComputedApiFamily, HostComputedDescriptor, HostComputedDescriptorId};
use super::read_set::AdmittedHostComputedReadSet;
use super::request::HostComputedEvaluationRequest;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HostComputedOutcomeClass {
    Prepared,
    Committed,
    Denied,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HostComputedDiagnosticsSummary {
    descriptor_id: HostComputedDescriptorId,
    node: NodeId,
    api_family: HostComputedApiFamily,
    outcome: HostComputedOutcomeClass,
    previous_dependency_count: u32,
    admitted_read_count: u32,
    retained_dependency_count: u32,
    added_dependency_count: u32,
    removed_dependency_count: u32,
    denial_class: Option<HostComputedDenialClass>,
    failure_class: Option<String>,
}

impl HostComputedDiagnosticsSummary {
    pub(crate) fn prepared(
        request: &HostComputedEvaluationRequest,
        admitted_reads: &AdmittedHostComputedReadSet,
        dependency_patch: &HostComputedDependencyPatch,
    ) -> Self {
        Self {
            descriptor_id: request.descriptor().descriptor_id(),
            node: request.node(),
            api_family: request.descriptor().api_family(),
            outcome: HostComputedOutcomeClass::Prepared,
            previous_dependency_count: request.previous_dependency_count() as u32,
            admitted_read_count: admitted_reads.dependencies().len() as u32,
            retained_dependency_count: dependency_patch.retained_dependency_count() as u32,
            added_dependency_count: dependency_patch.added_dependencies().len() as u32,
            removed_dependency_count: dependency_patch.removed_dependencies().len() as u32,
            denial_class: None,
            failure_class: None,
        }
    }

    pub(crate) fn denied(
        request: &HostComputedEvaluationRequest,
        denial_class: HostComputedDenialClass,
    ) -> Self {
        Self {
            descriptor_id: request.descriptor().descriptor_id(),
            node: request.node(),
            api_family: request.descriptor().api_family(),
            outcome: HostComputedOutcomeClass::Denied,
            previous_dependency_count: request.previous_dependency_count() as u32,
            admitted_read_count: 0,
            retained_dependency_count: 0,
            added_dependency_count: 0,
            removed_dependency_count: 0,
            denial_class: Some(denial_class),
            failure_class: None,
        }
    }

    pub(crate) fn failed(descriptor: &HostComputedDescriptor, failure_class: &str) -> Self {
        Self {
            descriptor_id: descriptor.descriptor_id(),
            node: descriptor.node(),
            api_family: descriptor.api_family(),
            outcome: HostComputedOutcomeClass::Failed,
            previous_dependency_count: 0,
            admitted_read_count: 0,
            retained_dependency_count: 0,
            added_dependency_count: 0,
            removed_dependency_count: 0,
            denial_class: None,
            failure_class: Some(failure_class.to_owned()),
        }
    }

    pub(crate) fn with_outcome(mut self, outcome: HostComputedOutcomeClass) -> Self {
        self.outcome = outcome;
        self
    }

    pub fn descriptor_id(&self) -> HostComputedDescriptorId {
        self.descriptor_id
    }

    pub fn node(&self) -> NodeId {
        self.node
    }

    pub fn api_family(&self) -> HostComputedApiFamily {
        self.api_family
    }

    pub fn outcome(&self) -> HostComputedOutcomeClass {
        self.outcome
    }

    pub fn previous_dependency_count(&self) -> u32 {
        self.previous_dependency_count
    }

    pub fn admitted_read_count(&self) -> u32 {
        self.admitted_read_count
    }

    pub fn retained_dependency_count(&self) -> u32 {
        self.retained_dependency_count
    }

    pub fn added_dependency_count(&self) -> u32 {
        self.added_dependency_count
    }

    pub fn removed_dependency_count(&self) -> u32 {
        self.removed_dependency_count
    }

    pub fn denial_class(&self) -> Option<HostComputedDenialClass> {
        self.denial_class
    }

    pub fn failure_class(&self) -> Option<&str> {
        self.failure_class.as_deref()
    }
}
