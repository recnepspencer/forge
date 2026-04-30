use serde::{Deserialize, Serialize};

use crate::data::handle::NodeId;
use crate::data::resource::{
    ResourceNodeId, ResourceRequestHandle, ResourceRequestIntent, ResourceRevalidationIntent,
};
use crate::data::temporal::{TemporalDuration, TemporalPreviousValueReference};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AsyncNodeRequestIntent {
    inner: ResourceRequestIntent,
    previous_value_reference: Option<TemporalPreviousValueReference>,
}

impl AsyncNodeRequestIntent {
    pub(crate) fn new(node: NodeId) -> Self {
        Self {
            inner: ResourceRequestIntent::new(ResourceNodeId::from_node(node)),
            previous_value_reference: None,
        }
    }

    pub(crate) fn with_transaction_deadline(node: NodeId, deadline: TemporalDuration) -> Self {
        Self {
            inner: ResourceRequestIntent::with_transaction_deadline(
                ResourceNodeId::from_node(node),
                deadline,
            ),
            previous_value_reference: None,
        }
    }

    pub fn with_previous_value_reference(
        mut self,
        previous_value_reference: TemporalPreviousValueReference,
    ) -> Self {
        self.previous_value_reference = Some(previous_value_reference);
        self
    }

    pub fn node(&self) -> NodeId {
        self.inner.node().node()
    }

    pub fn transaction_deadline(&self) -> Option<TemporalDuration> {
        self.inner.transaction_deadline()
    }

    pub fn previous_value_reference(&self) -> Option<&TemporalPreviousValueReference> {
        self.previous_value_reference.as_ref()
    }

    pub(crate) fn into_resource_intent(self) -> ResourceRequestIntent {
        self.inner
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AsyncNodeRevalidationIntent {
    inner: ResourceRevalidationIntent,
    previous_value_reference: Option<TemporalPreviousValueReference>,
}

impl AsyncNodeRevalidationIntent {
    pub(crate) fn new(node: NodeId) -> Self {
        Self {
            inner: ResourceRevalidationIntent::new(ResourceNodeId::from_node(node)),
            previous_value_reference: None,
        }
    }

    pub(crate) fn with_expected_active(
        node: NodeId,
        expected_active: ResourceRequestHandle,
    ) -> Self {
        Self {
            inner: ResourceRevalidationIntent::with_expected_active(
                ResourceNodeId::from_node(node),
                expected_active,
            ),
            previous_value_reference: None,
        }
    }

    pub(crate) fn with_transaction_deadline(node: NodeId, deadline: TemporalDuration) -> Self {
        Self {
            inner: ResourceRevalidationIntent::with_transaction_deadline(
                ResourceNodeId::from_node(node),
                deadline,
            ),
            previous_value_reference: None,
        }
    }

    pub fn with_previous_value_reference(
        mut self,
        previous_value_reference: TemporalPreviousValueReference,
    ) -> Self {
        self.previous_value_reference = Some(previous_value_reference);
        self
    }

    pub fn node(&self) -> NodeId {
        self.inner.node().node()
    }

    pub fn expected_active(&self) -> Option<ResourceRequestHandle> {
        self.inner.expected_active()
    }

    pub fn transaction_deadline(&self) -> Option<TemporalDuration> {
        self.inner.transaction_deadline()
    }

    pub fn previous_value_reference(&self) -> Option<&TemporalPreviousValueReference> {
        self.previous_value_reference.as_ref()
    }

    pub(crate) fn into_resource_intent(self) -> ResourceRevalidationIntent {
        self.inner
    }
}
