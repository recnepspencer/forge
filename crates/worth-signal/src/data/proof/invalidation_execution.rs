use serde::{Deserialize, Serialize};

use crate::data::aspect::Aspect;
use crate::data::handle::NodeId;

use super::invalidation_admission::{FrontierEntryClassification, FrontierInclusionBasis};
use super::locality::{PartitionScopeSet, TouchedScopeSummary};
use super::SummaryForm;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrontierWaveEntrySummary {
    pub node: NodeId,
    pub classification: FrontierEntryClassification,
    pub inclusion_basis: FrontierInclusionBasis,
    pub narrowed_scopes: PartitionScopeSet,
}

impl FrontierWaveEntrySummary {
    pub fn new(
        node: NodeId,
        classification: FrontierEntryClassification,
        inclusion_basis: FrontierInclusionBasis,
        narrowed_scopes: impl Into<PartitionScopeSet>,
    ) -> Self {
        Self {
            node,
            classification,
            inclusion_basis,
            narrowed_scopes: narrowed_scopes.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrontierWaveSummary {
    pub wave_index: u32,
    pub aspect: Aspect,
    pub entries: Vec<FrontierWaveEntrySummary>,
}

impl FrontierWaveSummary {
    pub fn new(
        wave_index: u32,
        aspect: Aspect,
        entries: impl IntoIterator<Item = FrontierWaveEntrySummary>,
    ) -> Self {
        Self {
            wave_index,
            aspect,
            entries: entries.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct FrontierExecutionCounters {
    pub frontier_seed_count: u64,
    pub frontier_group_count: u64,
    pub frontier_direct_wave_count: u64,
    pub frontier_transitive_wave_count: u64,
    pub frontier_partition_scoped_check_count: u64,
    pub frontier_direct_dirty_count: u64,
    pub frontier_maybe_stale_count: u64,
    pub frontier_partition_match_count: u64,
    pub frontier_detail_match_count: u64,
    pub frontier_cycle_check_candidate_count: u64,
    pub frontier_cycle_check_visited_count: u64,
    pub frontier_trace_retained_count: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrontierExecutionSummary {
    pub seed_count: u64,
    pub direct_waves: Vec<FrontierWaveSummary>,
    pub transitive_waves: Vec<FrontierWaveSummary>,
    pub touched_scope_summary: TouchedScopeSummary,
    pub counters: FrontierExecutionCounters,
}

impl FrontierExecutionSummary {
    pub fn new(
        seed_count: u64,
        direct_waves: Vec<FrontierWaveSummary>,
        transitive_waves: Vec<FrontierWaveSummary>,
        touched_scope_summary: TouchedScopeSummary,
        counters: FrontierExecutionCounters,
    ) -> Self {
        Self {
            seed_count,
            direct_waves,
            transitive_waves,
            touched_scope_summary,
            counters,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InvalidationTraceRecord {
    pub node: NodeId,
    pub aspect: Aspect,
    pub wave_index: u32,
    pub classification: FrontierEntryClassification,
    pub inclusion_basis: FrontierInclusionBasis,
}

impl InvalidationTraceRecord {
    pub fn new(
        node: NodeId,
        aspect: Aspect,
        wave_index: u32,
        classification: FrontierEntryClassification,
        inclusion_basis: FrontierInclusionBasis,
    ) -> Self {
        Self {
            node,
            aspect,
            wave_index,
            classification,
            inclusion_basis,
        }
    }
}

impl SummaryForm for FrontierExecutionSummary {}
