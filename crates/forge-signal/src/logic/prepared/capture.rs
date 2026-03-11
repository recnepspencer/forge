use std::cmp::Ordering;

use serde::{Deserialize, Serialize};

use crate::data::aspect::Aspect;
use crate::data::handle::NodeId;
use crate::data::output::PartitionSubscription;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PreparedDependencyCapture {
    edges: Vec<PreparedDependencyEdge>,
}

impl PreparedDependencyCapture {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record(&mut self, source: NodeId, aspect: Aspect, scope: Option<PartitionSubscription>) {
        let edge = PreparedDependencyEdge {
            source,
            aspect,
            scope,
        };
        match self
            .edges
            .binary_search_by(|candidate| compare_prepared_dependency_edges(candidate, &edge))
        {
            Ok(_) => {}
            Err(index) => self.edges.insert(index, edge),
        }
    }

    pub fn as_slice(&self) -> &[PreparedDependencyEdge] {
        &self.edges
    }

    pub fn into_sorted_unique(mut self) -> Self {
        self.edges.sort_by(compare_prepared_dependency_edges);
        self.edges.dedup_by(|left, right| {
            compare_prepared_dependency_edges(left, right) == Ordering::Equal
        });
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreparedDependencyEdge {
    pub source: NodeId,
    pub aspect: Aspect,
    #[serde(default)]
    pub scope: Option<PartitionSubscription>,
}

pub(crate) fn compare_prepared_dependency_edges(
    left: &PreparedDependencyEdge,
    right: &PreparedDependencyEdge,
) -> Ordering {
    (
        left.source.index(),
        left.source.generation(),
        left.aspect.index(),
        left.scope.as_ref(),
    )
        .cmp(&(
            right.source.index(),
            right.source.generation(),
            right.aspect.index(),
            right.scope.as_ref(),
        ))
}
