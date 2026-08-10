use serde::{Deserialize, Serialize};

use crate::data::aspect::Aspect;
use crate::data::handle::NodeId;

use super::invalidation_admission::{
    FrontierEntryClassification, FrontierInclusionBasis, InvalidationSeedBatch,
};
use super::locality::{PartitionScopeSet, TouchedScopeSummary};
use super::SummaryForm;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrontierWaveEntryPlan {
    pub node: NodeId,
    pub classification: FrontierEntryClassification,
    pub inclusion_basis: FrontierInclusionBasis,
    pub narrowed_scopes: PartitionScopeSet,
    pub source_seed_refs: Vec<u32>,
}

impl FrontierWaveEntryPlan {
    pub fn new(
        node: NodeId,
        classification: FrontierEntryClassification,
        inclusion_basis: FrontierInclusionBasis,
        narrowed_scopes: impl Into<PartitionScopeSet>,
        source_seed_refs: impl IntoIterator<Item = u32>,
    ) -> Self {
        let mut source_seed_refs = source_seed_refs.into_iter().collect::<Vec<_>>();
        if source_seed_refs.len() > 1 {
            source_seed_refs.sort_unstable();
            source_seed_refs.dedup();
        }
        Self {
            node,
            classification,
            inclusion_basis,
            narrowed_scopes: narrowed_scopes.into(),
            source_seed_refs,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrontierWavePlan {
    pub wave_index: u32,
    pub aspect: Aspect,
    pub entries: Vec<FrontierWaveEntryPlan>,
}

impl FrontierWavePlan {
    pub fn new(
        wave_index: u32,
        aspect: Aspect,
        entries: impl IntoIterator<Item = FrontierWaveEntryPlan>,
    ) -> Self {
        let mut entries = entries.into_iter().collect::<Vec<_>>();
        if entries.len() > 1 {
            entries.sort_unstable_by_key(|entry| {
                (
                    super::locality::node_sort_key(&entry.node),
                    entry.classification,
                    entry.inclusion_basis,
                )
            });
        }
        Self {
            wave_index,
            aspect,
            entries,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransitiveFrontierRoot {
    pub node: NodeId,
    pub aspect: Aspect,
    pub classification: FrontierEntryClassification,
    pub narrowed_scopes: PartitionScopeSet,
    pub source_seed_refs: Vec<u32>,
}

impl TransitiveFrontierRoot {
    pub fn new(
        node: NodeId,
        aspect: Aspect,
        classification: FrontierEntryClassification,
        narrowed_scopes: impl Into<PartitionScopeSet>,
        source_seed_refs: impl IntoIterator<Item = u32>,
    ) -> Self {
        let mut source_seed_refs = source_seed_refs.into_iter().collect::<Vec<_>>();
        if source_seed_refs.len() > 1 {
            source_seed_refs.sort_unstable();
            source_seed_refs.dedup();
        }
        Self {
            node,
            aspect,
            classification,
            narrowed_scopes: narrowed_scopes.into(),
            source_seed_refs,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct FrontierPredictedCounters {
    pub seed_count: u64,
    pub group_count: u64,
    pub direct_wave_count: u64,
    pub transitive_wave_count: u64,
    pub direct_dirty_count: u64,
    pub maybe_stale_count: u64,
    pub partition_scoped_checks: u64,
    pub partition_match_count: u64,
    pub detail_match_count: u64,
    pub cycle_check_candidate_count: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrontierPlan {
    pub seed_batch: InvalidationSeedBatch,
    pub direct_waves: Vec<FrontierWavePlan>,
    pub transitive_roots: Vec<TransitiveFrontierRoot>,
    pub touched_scope_summary: TouchedScopeSummary,
    pub predicted: FrontierPredictedCounters,
}

impl FrontierPlan {
    pub fn new(
        seed_batch: InvalidationSeedBatch,
        direct_waves: Vec<FrontierWavePlan>,
        transitive_roots: Vec<TransitiveFrontierRoot>,
        touched_scope_summary: TouchedScopeSummary,
        predicted: FrontierPredictedCounters,
    ) -> Self {
        Self {
            seed_batch,
            direct_waves,
            transitive_roots,
            touched_scope_summary,
            predicted,
        }
    }
}

impl SummaryForm for FrontierPlan {}
