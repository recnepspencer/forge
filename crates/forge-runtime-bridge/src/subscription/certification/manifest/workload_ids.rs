use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BridgeSubscriptionReferenceWorkloadProductId {
    value: Arc<str>,
}

impl BridgeSubscriptionReferenceWorkloadProductId {
    fn from_declared_label(value: impl Into<Arc<str>>) -> Self {
        Self {
            value: value.into(),
        }
    }

    pub fn as_str(&self) -> &str {
        self.value.as_ref()
    }
}

impl AsRef<str> for BridgeSubscriptionReferenceWorkloadProductId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BridgeSubscriptionReferenceWorkloadComponentId {
    value: Arc<str>,
}

impl BridgeSubscriptionReferenceWorkloadComponentId {
    fn from_declared_label(value: impl Into<Arc<str>>) -> Self {
        Self {
            value: value.into(),
        }
    }

    pub fn as_str(&self) -> &str {
        self.value.as_ref()
    }
}

impl AsRef<str> for BridgeSubscriptionReferenceWorkloadComponentId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BridgeSubscriptionReferenceWorkloadLaneId {
    value: Arc<str>,
}

impl BridgeSubscriptionReferenceWorkloadLaneId {
    fn from_declared_label(value: impl Into<Arc<str>>) -> Self {
        Self {
            value: value.into(),
        }
    }

    pub fn as_str(&self) -> &str {
        self.value.as_ref()
    }
}

impl AsRef<str> for BridgeSubscriptionReferenceWorkloadLaneId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionReferenceWorkloadProductIdSet {
    ids: Vec<BridgeSubscriptionReferenceWorkloadProductId>,
}

impl BridgeSubscriptionReferenceWorkloadProductIdSet {
    pub fn from_declared_product_labels<I, S>(labels: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<Arc<str>>,
    {
        Self {
            ids: labels
                .into_iter()
                .map(BridgeSubscriptionReferenceWorkloadProductId::from_declared_label)
                .collect(),
        }
    }

    pub fn reversed(mut self) -> Self {
        self.ids.reverse();
        self
    }

    pub(super) fn into_sorted_unique_ids(
        self,
    ) -> Vec<BridgeSubscriptionReferenceWorkloadProductId> {
        let mut ids = self.ids;
        ids.sort();
        ids.dedup();
        ids
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionReferenceWorkloadComponentIdSet {
    ids: Vec<BridgeSubscriptionReferenceWorkloadComponentId>,
}

impl BridgeSubscriptionReferenceWorkloadComponentIdSet {
    pub fn from_declared_component_labels<I, S>(labels: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<Arc<str>>,
    {
        Self {
            ids: labels
                .into_iter()
                .map(BridgeSubscriptionReferenceWorkloadComponentId::from_declared_label)
                .collect(),
        }
    }

    pub fn reversed(mut self) -> Self {
        self.ids.reverse();
        self
    }

    pub(super) fn into_sorted_unique_ids(
        self,
    ) -> Vec<BridgeSubscriptionReferenceWorkloadComponentId> {
        let mut ids = self.ids;
        ids.sort();
        ids.dedup();
        ids
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionReferenceWorkloadLaneIdSet {
    ids: Vec<BridgeSubscriptionReferenceWorkloadLaneId>,
}

impl BridgeSubscriptionReferenceWorkloadLaneIdSet {
    pub fn from_declared_lane_labels<I, S>(labels: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<Arc<str>>,
    {
        Self {
            ids: labels
                .into_iter()
                .map(BridgeSubscriptionReferenceWorkloadLaneId::from_declared_label)
                .collect(),
        }
    }

    pub fn reversed(mut self) -> Self {
        self.ids.reverse();
        self
    }

    pub(super) fn into_sorted_unique_ids(self) -> Vec<BridgeSubscriptionReferenceWorkloadLaneId> {
        let mut ids = self.ids;
        ids.sort();
        ids.dedup();
        ids
    }
}
