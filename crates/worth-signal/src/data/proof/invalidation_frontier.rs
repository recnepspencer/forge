use serde::{Deserialize, Serialize};

use crate::data::aspect::Aspect;

use super::locality::{DedupedNodeBatch, PartitionScopeSet, SortedSourceBatch};
use super::SummaryForm;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NarrowedPropagationSet {
    pub changed_aspect: Aspect,
    pub dirty_sources: SortedSourceBatch,
    pub changed_scopes: PartitionScopeSet,
}

impl NarrowedPropagationSet {
    pub fn new(
        changed_aspect: Aspect,
        dirty_sources: impl Into<SortedSourceBatch>,
        changed_scopes: impl Into<PartitionScopeSet>,
    ) -> Self {
        Self {
            changed_aspect,
            dirty_sources: dirty_sources.into(),
            changed_scopes: changed_scopes.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrontierWave {
    pub direct_subscribers: DedupedNodeBatch,
    pub transitive_frontier: DedupedNodeBatch,
}

impl FrontierWave {
    pub fn new(
        direct_subscribers: impl Into<DedupedNodeBatch>,
        transitive_frontier: impl Into<DedupedNodeBatch>,
    ) -> Self {
        Self {
            direct_subscribers: direct_subscribers.into(),
            transitive_frontier: transitive_frontier.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InvalidationFrontier {
    pub narrowed: NarrowedPropagationSet,
    pub wave: FrontierWave,
}

impl InvalidationFrontier {
    pub fn new(narrowed: NarrowedPropagationSet, wave: FrontierWave) -> Self {
        Self { narrowed, wave }
    }
}

impl SummaryForm for NarrowedPropagationSet {}
impl SummaryForm for FrontierWave {}
impl SummaryForm for InvalidationFrontier {}
