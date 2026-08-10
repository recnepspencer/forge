use serde::{Deserialize, Serialize};

use crate::data::aspect::AspectMask;
use crate::data::handle::NodeId;
use crate::data::output::CanonicalChangedRegions;

use super::{DedupedNodeBatch, DeltaForm, SortedSourceBatch, TouchedScopeSummary};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DirtyDelta {
    pub changed_aspects: AspectMask,
    pub changed_regions: CanonicalChangedRegions,
    pub touched_nodes: DedupedNodeBatch,
}

impl DirtyDelta {
    pub fn new(
        changed_aspects: impl Into<AspectMask>,
        changed_regions: impl Into<CanonicalChangedRegions>,
        touched_nodes: impl Into<DedupedNodeBatch>,
    ) -> Self {
        Self {
            changed_aspects: changed_aspects.into(),
            changed_regions: changed_regions.into(),
            touched_nodes: touched_nodes.into(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.changed_aspects.is_empty()
            && self.changed_regions.is_empty()
            && self.touched_nodes.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct StructuralDelta {
    pub dirty: Option<DirtyDelta>,
    pub touched_scope: Option<TouchedScopeSummary>,
}

impl StructuralDelta {
    pub fn new(dirty: Option<DirtyDelta>, touched_scope: Option<TouchedScopeSummary>) -> Self {
        Self {
            dirty,
            touched_scope,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.dirty.as_ref().is_none_or(DirtyDelta::is_empty)
            && self
                .touched_scope
                .as_ref()
                .is_none_or(TouchedScopeSummary::is_empty)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PatchPlan {
    pub target_nodes: DedupedNodeBatch,
    pub delta: StructuralDelta,
}

impl PatchPlan {
    pub fn new(target_nodes: impl Into<DedupedNodeBatch>, delta: StructuralDelta) -> Self {
        Self {
            target_nodes: target_nodes.into(),
            delta,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.target_nodes.is_empty() && self.delta.is_empty()
    }
}

impl From<CanonicalChangedRegions> for DirtyDelta {
    fn from(changed_regions: CanonicalChangedRegions) -> Self {
        Self::new(
            AspectMask::EMPTY,
            changed_regions,
            DedupedNodeBatch::default(),
        )
    }
}

impl From<&[NodeId]> for DedupedNodeBatch {
    fn from(nodes: &[NodeId]) -> Self {
        Self::from_slice(nodes)
    }
}

impl From<Vec<NodeId>> for DedupedNodeBatch {
    fn from(nodes: Vec<NodeId>) -> Self {
        Self::new(nodes)
    }
}

impl From<&[NodeId]> for SortedSourceBatch {
    fn from(sources: &[NodeId]) -> Self {
        Self::from_slice(sources)
    }
}

impl From<Vec<NodeId>> for SortedSourceBatch {
    fn from(sources: Vec<NodeId>) -> Self {
        Self::new(sources)
    }
}

impl From<Vec<crate::data::output::PartitionSubscription>> for TouchedScopeSummary {
    fn from(scopes: Vec<crate::data::output::PartitionSubscription>) -> Self {
        Self::new(
            scopes,
            DedupedNodeBatch::default(),
            SortedSourceBatch::default(),
        )
    }
}

impl DeltaForm for DirtyDelta {}
impl DeltaForm for StructuralDelta {}
impl DeltaForm for PatchPlan {}
