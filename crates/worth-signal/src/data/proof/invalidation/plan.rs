use serde::{Deserialize, Serialize};

use crate::data::aspect::Aspect;
use crate::data::handle::NodeId;

use super::super::locality::{PartitionScopeSet, TouchedScopeSummary};
use super::super::SummaryForm;
use super::frontier_admission::{
    FrontierEntryClassification, FrontierInclusionBasis, InvalidationSeedBatch,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct FrontierWaveEntryPlan {
    pub(crate) node: NodeId,
    pub(crate) classification: FrontierEntryClassification,
    pub(crate) inclusion_basis: FrontierInclusionBasis,
    pub(crate) narrowed_scopes: PartitionScopeSet,
    pub(crate) source_seed_refs: Vec<u32>,
}

impl FrontierWaveEntryPlan {
    pub(crate) fn new(
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
pub(crate) struct FrontierWavePlan {
    pub(crate) wave_index: u32,
    pub(crate) aspect: Aspect,
    pub(crate) entries: Vec<FrontierWaveEntryPlan>,
}

impl FrontierWavePlan {
    pub(crate) fn new(
        wave_index: u32,
        aspect: Aspect,
        entries: impl IntoIterator<Item = FrontierWaveEntryPlan>,
    ) -> Self {
        let mut entries = entries.into_iter().collect::<Vec<_>>();
        if entries.len() > 1 {
            entries.sort_unstable_by_key(|entry| {
                (
                    super::super::locality::node_sort_key(&entry.node),
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct InvalidationPlanningEstimate {
    pub(crate) seed_count: u64,
    pub(crate) group_count: u64,
    pub(crate) direct_wave_count: u64,
    pub(crate) transitive_wave_count: u64,
    pub(crate) direct_dirty_count: u64,
    pub(crate) maybe_stale_count: u64,
    pub(crate) partition_scoped_checks: u64,
    pub(crate) partition_match_count: u64,
    pub(crate) detail_match_count: u64,
    pub(crate) cycle_check_candidate_count: u64,
}

impl InvalidationPlanningEstimate {
    pub const fn seed_count(&self) -> u64 {
        self.seed_count
    }

    pub const fn direct_candidate_count(&self) -> u64 {
        self.direct_dirty_count + self.maybe_stale_count
    }

    pub const fn partition_scoped_check_count(&self) -> u64 {
        self.partition_scoped_checks
    }
}

impl SummaryForm for InvalidationPlanningEstimate {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct FrontierPlan {
    pub(crate) seed_batch: InvalidationSeedBatch,
    pub(crate) direct_waves: Vec<FrontierWavePlan>,
    pub(crate) touched_scope_summary: TouchedScopeSummary,
    pub(crate) predicted: InvalidationPlanningEstimate,
}

impl FrontierPlan {
    pub(crate) fn new(
        seed_batch: InvalidationSeedBatch,
        direct_waves: Vec<FrontierWavePlan>,
        touched_scope_summary: TouchedScopeSummary,
        predicted: InvalidationPlanningEstimate,
    ) -> Self {
        Self {
            seed_batch,
            direct_waves,
            touched_scope_summary,
            predicted,
        }
    }
}

impl SummaryForm for FrontierPlan {}
