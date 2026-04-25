use serde::{Deserialize, Serialize};

use super::proof::AdmittedResourceRequest;
use super::request::{ResourceNodeId, ResourceRequestHandle, ResourceRequestId};
use super::supersession::ResourceSupersessionRecord;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceRevalidationIntent {
    node: ResourceNodeId,
    expected_active: Option<ResourceRequestHandle>,
}

impl ResourceRevalidationIntent {
    pub fn new(node: ResourceNodeId) -> Self {
        Self {
            node,
            expected_active: None,
        }
    }

    pub fn with_expected_active(
        node: ResourceNodeId,
        expected_active: ResourceRequestHandle,
    ) -> Self {
        Self {
            node,
            expected_active: Some(expected_active),
        }
    }

    pub fn node(self) -> ResourceNodeId {
        self.node
    }

    pub fn expected_active(self) -> Option<ResourceRequestHandle> {
        self.expected_active
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceRevalidationDenialClass {
    UndeclaredResourceNode,
    ActiveRequestRequiresExpectedHandle,
    ExpectedActiveRequestMismatch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdmittedResourceRevalidation {
    admitted_request: AdmittedResourceRequest,
    expected_active: Option<ResourceRequestHandle>,
    supersession_record: Option<ResourceSupersessionRecord>,
}

impl AdmittedResourceRevalidation {
    pub(crate) fn new(
        admitted_request: AdmittedResourceRequest,
        expected_active: Option<ResourceRequestHandle>,
        supersession_record: Option<ResourceSupersessionRecord>,
    ) -> Self {
        Self {
            admitted_request,
            expected_active,
            supersession_record,
        }
    }

    pub fn admitted_request(self) -> AdmittedResourceRequest {
        self.admitted_request
    }

    pub fn expected_active(self) -> Option<ResourceRequestHandle> {
        self.expected_active
    }

    pub fn supersession_record(self) -> Option<ResourceSupersessionRecord> {
        self.supersession_record
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeniedResourceRevalidation {
    node: ResourceNodeId,
    request_id: Option<ResourceRequestId>,
    class: ResourceRevalidationDenialClass,
}

impl DeniedResourceRevalidation {
    pub(crate) fn new(
        node: ResourceNodeId,
        request_id: Option<ResourceRequestId>,
        class: ResourceRevalidationDenialClass,
    ) -> Self {
        Self {
            node,
            request_id,
            class,
        }
    }

    pub fn node(self) -> ResourceNodeId {
        self.node
    }

    pub fn request_id(self) -> Option<ResourceRequestId> {
        self.request_id
    }

    pub fn class(self) -> ResourceRevalidationDenialClass {
        self.class
    }
}
