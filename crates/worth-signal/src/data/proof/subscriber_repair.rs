use serde::{Deserialize, Serialize};

use crate::data::handle::NodeId;

use super::{DedupedNodeBatch, SummaryForm};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubscriberRepair {
    pub source: NodeId,
    pub subscribers: DedupedNodeBatch,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct SubscriberRepairBatch {
    repairs: Vec<SubscriberRepair>,
}

impl SubscriberRepairBatch {
    pub fn new(repairs: impl IntoIterator<Item = SubscriberRepair>) -> Self {
        let mut repairs = repairs.into_iter().collect::<Vec<_>>();
        if repairs.len() > 1 {
            repairs.sort_unstable_by_key(|repair| super::locality::node_sort_key(&repair.source));
            repairs.dedup_by(|left, right| left.source == right.source);
        }
        Self { repairs }
    }

    pub fn as_slice(&self) -> &[SubscriberRepair] {
        &self.repairs
    }

    pub fn into_vec(self) -> Vec<SubscriberRepair> {
        self.repairs
    }

    pub fn is_empty(&self) -> bool {
        self.repairs.is_empty()
    }
}

impl SummaryForm for SubscriberRepairBatch {}
