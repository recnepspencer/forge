use serde::{Deserialize, Serialize};

use crate::data::temporal::TemporalWakeId;

use super::descriptor::ResourceDescriptorId;
use super::lifecycle::{ResourceLifecycleClass, ResourceLifecycleOrdinal};
use super::request::{
    ResourceAttemptId, ResourceBranchEpoch, ResourceGeneration, ResourceNodeId,
    ResourceRequestHandle,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceInFlightStatus {
    Active,
    Fulfilled,
    Superseded,
    Cancelled,
    TimedOut,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct InFlightResourceRequest {
    handle: ResourceRequestHandle,
    node: ResourceNodeId,
    descriptor_id: ResourceDescriptorId,
    generation: ResourceGeneration,
    attempt: ResourceAttemptId,
    lifecycle: ResourceLifecycleClass,
    lifecycle_ordinal: ResourceLifecycleOrdinal,
    status: ResourceInFlightStatus,
    timeout_wake_id: Option<TemporalWakeId>,
    superseded_by: Option<ResourceRequestHandle>,
}

impl InFlightResourceRequest {
    pub(crate) fn new(
        handle: ResourceRequestHandle,
        node: ResourceNodeId,
        descriptor_id: ResourceDescriptorId,
        generation: ResourceGeneration,
        attempt: ResourceAttemptId,
        lifecycle_ordinal: ResourceLifecycleOrdinal,
    ) -> Self {
        Self {
            handle,
            node,
            descriptor_id,
            generation,
            attempt,
            lifecycle: ResourceLifecycleClass::Pending,
            lifecycle_ordinal,
            status: ResourceInFlightStatus::Active,
            timeout_wake_id: None,
            superseded_by: None,
        }
    }

    pub(crate) fn attach_timeout_wake(&mut self, wake_id: TemporalWakeId) {
        self.timeout_wake_id = Some(wake_id);
    }

    pub(crate) fn supersede(
        &mut self,
        lifecycle_ordinal: ResourceLifecycleOrdinal,
        replacing: ResourceRequestHandle,
    ) {
        self.lifecycle = ResourceLifecycleClass::Superseded;
        self.lifecycle_ordinal = lifecycle_ordinal;
        self.status = ResourceInFlightStatus::Superseded;
        self.superseded_by = Some(replacing);
    }

    pub(crate) fn cancel(&mut self, lifecycle_ordinal: ResourceLifecycleOrdinal) {
        self.lifecycle = ResourceLifecycleClass::Cancelled;
        self.lifecycle_ordinal = lifecycle_ordinal;
        self.status = ResourceInFlightStatus::Cancelled;
    }

    pub(crate) fn timeout(&mut self, lifecycle_ordinal: ResourceLifecycleOrdinal) {
        self.lifecycle = ResourceLifecycleClass::TimedOut;
        self.lifecycle_ordinal = lifecycle_ordinal;
        self.status = ResourceInFlightStatus::TimedOut;
    }

    pub(crate) fn fulfill(&mut self, lifecycle_ordinal: ResourceLifecycleOrdinal) {
        self.lifecycle = ResourceLifecycleClass::Fulfilled;
        self.lifecycle_ordinal = lifecycle_ordinal;
        self.status = ResourceInFlightStatus::Fulfilled;
    }

    pub(crate) fn refresh_branch_epoch(&mut self, branch_epoch: ResourceBranchEpoch) {
        self.handle = self.handle.with_branch_epoch(branch_epoch);
    }

    pub fn handle(self) -> ResourceRequestHandle {
        self.handle
    }

    pub fn node(self) -> ResourceNodeId {
        self.node
    }

    pub fn descriptor_id(self) -> ResourceDescriptorId {
        self.descriptor_id
    }

    pub fn generation(self) -> ResourceGeneration {
        self.generation
    }

    pub fn attempt(self) -> ResourceAttemptId {
        self.attempt
    }

    pub fn lifecycle(self) -> ResourceLifecycleClass {
        self.lifecycle
    }

    pub fn lifecycle_ordinal(self) -> ResourceLifecycleOrdinal {
        self.lifecycle_ordinal
    }

    pub fn status(self) -> ResourceInFlightStatus {
        self.status
    }

    pub fn timeout_wake_id(self) -> Option<TemporalWakeId> {
        self.timeout_wake_id
    }

    pub fn superseded_by(self) -> Option<ResourceRequestHandle> {
        self.superseded_by
    }
}
