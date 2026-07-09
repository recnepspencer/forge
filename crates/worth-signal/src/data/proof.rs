use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

use crate::data::aspect::{Aspect, AspectMask};
use crate::data::dependency::CanonicalDependencies;
use crate::data::dependency::{
    CommittedSnapshotUpdate, DependencySnapshot, ReplacementSnapshotUpdate,
    SharedDependencySnapshot, SnapshotDeltaRecord, VersionOnlySnapshotUpdate,
};
use crate::data::handle::NodeId;
use crate::data::output::{CanonicalChangedRegions, PartitionSubscription};
use crate::data::performance::{
    ResolvedExecutionStrategy, ResolvedMaintenanceStrategy, ResolvedPerformancePolicy,
};

pub trait CanonicalForm {}

pub trait LoweredForm {}

pub trait ResolvedForm {}

pub trait DeltaForm {}

pub trait SummaryForm {}

pub trait OrderedStreamItem {
    type OrderKey: Ord + Copy;

    fn order_key(&self) -> Self::OrderKey;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrderedStreamMergeError<K> {
    DuplicateKey(K),
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct LocallyOrderedShard<T> {
    items: Vec<T>,
}

impl<T: OrderedStreamItem> LocallyOrderedShard<T> {
    pub fn new(items: impl IntoIterator<Item = T>) -> Self {
        let items = items.into_iter().collect::<Vec<_>>();
        assert_strict_order(items.as_slice());
        Self { items }
    }

    pub fn canonicalize_unordered(items: impl IntoIterator<Item = T>) -> Self {
        let mut items = items.into_iter().collect::<Vec<_>>();
        if items.len() > 1 {
            items.sort_unstable_by_key(|item| item.order_key());
        }
        Self::new(items)
    }

    pub fn as_slice(&self) -> &[T] {
        &self.items
    }

    pub fn into_vec(self) -> Vec<T> {
        self.items
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct MergeableOrderedStream<T> {
    shards: Vec<LocallyOrderedShard<T>>,
}

impl<T> MergeableOrderedStream<T> {
    pub fn new(shards: impl IntoIterator<Item = LocallyOrderedShard<T>>) -> Self {
        Self {
            shards: shards.into_iter().collect(),
        }
    }
}

impl<T: OrderedStreamItem> MergeableOrderedStream<T> {
    pub fn try_into_vec(self) -> Result<Vec<T>, OrderedStreamMergeError<T::OrderKey>> {
        let mut merged = Vec::<T>::new();
        for shard in self.shards {
            merged = merge_ordered_streams(merged, shard.into_vec())?;
        }
        Ok(merged)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DependencySetEdit {
    pub node: NodeId,
    pub dependencies: CanonicalDependencies,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct DependencyBatchEdit {
    edits: Vec<DependencySetEdit>,
}

impl DependencyBatchEdit {
    pub fn new(edits: impl IntoIterator<Item = DependencySetEdit>) -> Self {
        let mut edits = edits.into_iter().collect::<Vec<_>>();
        if edits.len() > 1 {
            edits.sort_unstable_by_key(|edit| node_sort_key(&edit.node));
            let duplicate = edits
                .windows(2)
                .find(|pair| pair[0].node == pair[1].node)
                .map(|pair| pair[0].node);
            assert!(
                duplicate.is_none(),
                "dependency batch edit cannot contain multiple edits for node {:?}",
                duplicate
            );
        }
        Self { edits }
    }

    pub fn from_pairs(
        edits: impl IntoIterator<Item = (NodeId, impl Into<CanonicalDependencies>)>,
    ) -> Self {
        Self::new(
            edits
                .into_iter()
                .map(|(node, dependencies)| DependencySetEdit {
                    node,
                    dependencies: dependencies.into(),
                }),
        )
    }

    pub fn singleton(node: NodeId, dependencies: impl Into<CanonicalDependencies>) -> Self {
        Self::new(std::iter::once(DependencySetEdit {
            node,
            dependencies: dependencies.into(),
        }))
    }

    pub fn as_slice(&self) -> &[DependencySetEdit] {
        &self.edits
    }

    pub fn into_vec(self) -> Vec<DependencySetEdit> {
        self.edits
    }

    pub fn is_empty(&self) -> bool {
        self.edits.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DirtyBatchEntry {
    pub source: NodeId,
    pub changed_aspect: Aspect,
    pub changed_regions: CanonicalChangedRegions,
}

impl DirtyBatchEntry {
    pub fn new(
        source: NodeId,
        changed_aspect: Aspect,
        changed_regions: impl Into<CanonicalChangedRegions>,
    ) -> Self {
        Self {
            source,
            changed_aspect,
            changed_regions: changed_regions.into(),
        }
    }

    pub fn without_regions(source: NodeId, changed_aspect: Aspect) -> Self {
        Self::new(source, changed_aspect, CanonicalChangedRegions::default())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct DirtyBatch {
    entries: Vec<DirtyBatchEntry>,
}

impl DirtyBatch {
    pub fn new(entries: impl IntoIterator<Item = DirtyBatchEntry>) -> Self {
        let mut entries = entries.into_iter().collect::<Vec<_>>();
        if entries.len() > 1 {
            entries.sort_unstable_by_key(|entry| {
                (entry.changed_aspect.index(), node_sort_key(&entry.source))
            });
            let mut merged = Vec::<DirtyBatchEntry>::with_capacity(entries.len());
            for entry in entries {
                if let Some(previous) = merged.last_mut() {
                    if previous.source == entry.source
                        && previous.changed_aspect == entry.changed_aspect
                    {
                        previous.changed_regions = CanonicalChangedRegions::new(
                            previous
                                .changed_regions
                                .as_slice()
                                .iter()
                                .cloned()
                                .chain(entry.changed_regions.into_vec()),
                        );
                        continue;
                    }
                }
                merged.push(entry);
            }
            entries = merged;
        }
        Self { entries }
    }

    pub fn from_sources(entries: impl IntoIterator<Item = (NodeId, Aspect)>) -> Self {
        Self::new(entries.into_iter().map(|(source, changed_aspect)| {
            DirtyBatchEntry::without_regions(source, changed_aspect)
        }))
    }

    pub fn singleton(
        source: NodeId,
        changed_aspect: Aspect,
        changed_regions: impl Into<CanonicalChangedRegions>,
    ) -> Self {
        Self::new(std::iter::once(DirtyBatchEntry::new(
            source,
            changed_aspect,
            changed_regions,
        )))
    }

    pub fn as_slice(&self) -> &[DirtyBatchEntry] {
        &self.entries
    }

    pub fn into_vec(self) -> Vec<DirtyBatchEntry> {
        self.entries
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn changed_aspects_mask(&self) -> AspectMask {
        self.entries.iter().fold(AspectMask::EMPTY, |mask, entry| {
            mask | AspectMask::from_aspect(entry.changed_aspect)
        })
    }

    pub fn changed_regions(&self) -> CanonicalChangedRegions {
        CanonicalChangedRegions::new(
            self.entries
                .iter()
                .flat_map(|entry| entry.changed_regions.as_slice().iter().cloned()),
        )
    }

    pub fn touched_sources(&self) -> SortedSourceBatch {
        SortedSourceBatch::new(self.entries.iter().map(|entry| entry.source))
    }

    pub fn locality_footprint(&self) -> LocalityFootprint {
        let changed_regions = self.changed_regions();
        LocalityFootprint::new(
            PartitionScopeSet::from_changed_regions(&changed_regions),
            DedupedNodeBatch::new(self.entries.iter().map(|entry| entry.source)),
            self.touched_sources(),
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default)]
pub enum FrontierSeedCause {
    #[default]
    DirtySource,
    StructuralDelta,
    BatchRevalidation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default)]
pub enum FrontierEntryClassification {
    DirectDirty,
    #[default]
    MaybeStale,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, PartialOrd, Ord)]
pub enum FrontierInclusionBasis {
    #[default]
    DirectSubscriptionMatch,
    PartitionScopeOverlap,
    DetailScopeOverlap,
    TransitiveReachability,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum FrontierValidationDecision {
    #[default]
    ReachableCycleCheck,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InvalidationSeed {
    pub source_node: NodeId,
    pub aspect: Aspect,
    pub changed_scopes: PartitionScopeSet,
    pub cause: FrontierSeedCause,
}

impl InvalidationSeed {
    pub fn new(
        source_node: NodeId,
        aspect: Aspect,
        changed_scopes: impl Into<PartitionScopeSet>,
        cause: FrontierSeedCause,
    ) -> Self {
        Self {
            source_node,
            aspect,
            changed_scopes: changed_scopes.into(),
            cause,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct InvalidationSeedBatch {
    pub seeds: Vec<InvalidationSeed>,
}

impl InvalidationSeedBatch {
    pub fn new(seeds: impl IntoIterator<Item = InvalidationSeed>) -> Self {
        let mut seeds = seeds.into_iter().collect::<Vec<_>>();
        if seeds.len() > 1 {
            seeds.sort_unstable_by_key(|seed| {
                (
                    seed.aspect.index(),
                    node_sort_key(&seed.source_node),
                    seed.changed_scopes.as_slice().to_vec(),
                    seed.cause,
                )
            });
            let mut merged = Vec::<InvalidationSeed>::with_capacity(seeds.len());
            for seed in seeds {
                if let Some(previous) = merged.last_mut() {
                    if previous.source_node == seed.source_node
                        && previous.aspect == seed.aspect
                        && previous.cause == seed.cause
                    {
                        let mut scopes = previous.changed_scopes.as_slice().to_vec();
                        scopes.extend_from_slice(seed.changed_scopes.as_slice());
                        previous.changed_scopes = PartitionScopeSet::new(scopes);
                        continue;
                    }
                }
                merged.push(seed);
            }
            seeds = merged;
        }
        Self { seeds }
    }

    pub fn as_slice(&self) -> &[InvalidationSeed] {
        &self.seeds
    }

    pub fn is_empty(&self) -> bool {
        self.seeds.is_empty()
    }
}

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
                    node_sort_key(&entry.node),
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemanticBatchCommit {
    pub dirty: DirtyBatch,
    pub changed_aspects: AspectMask,
    pub changed_regions: CanonicalChangedRegions,
    pub locality: LocalityFootprint,
    pub touched_scope: TouchedScopeSummary,
}

impl SemanticBatchCommit {
    pub fn new(dirty: DirtyBatch) -> Self {
        let changed_aspects = dirty.changed_aspects_mask();
        let changed_regions = dirty.changed_regions();
        let scopes = PartitionScopeSet::from_changed_regions(&changed_regions);
        let touched_nodes =
            DedupedNodeBatch::new(dirty.as_slice().iter().map(|entry| entry.source));
        let touched_sources = dirty.touched_sources();
        let locality = LocalityFootprint::new(
            scopes.clone(),
            touched_nodes.clone(),
            touched_sources.clone(),
        );
        let touched_scope =
            TouchedScopeSummary::new(scopes, touched_nodes, touched_sources.clone());
        Self {
            dirty,
            changed_aspects,
            changed_regions,
            locality,
            touched_scope,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.dirty.is_empty()
    }
}

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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DesiredState<T> {
    value: T,
}

impl<T> DesiredState<T> {
    pub fn new(value: T) -> Self {
        Self { value }
    }

    pub fn value(&self) -> &T {
        &self.value
    }

    pub fn into_inner(self) -> T {
        self.value
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SingleConsumer<T>(T);

impl<T> SingleConsumer<T> {
    pub fn new(value: T) -> Self {
        Self(value)
    }

    pub fn as_ref(&self) -> &T {
        &self.0
    }

    pub fn as_mut(&mut self) -> &mut T {
        &mut self.0
    }

    pub fn into_inner(self) -> T {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct DedupedNodeBatch {
    nodes: std::sync::Arc<Vec<NodeId>>,
}

impl DedupedNodeBatch {
    pub fn new(nodes: impl IntoIterator<Item = NodeId>) -> Self {
        Self::canonicalize_unordered(nodes)
    }

    pub fn canonicalize_unordered(nodes: impl IntoIterator<Item = NodeId>) -> Self {
        let mut nodes = nodes.into_iter().collect::<Vec<_>>();
        if nodes.len() > 1 {
            nodes.sort_unstable_by_key(node_sort_key);
            nodes.dedup();
        }
        Self {
            nodes: std::sync::Arc::new(nodes),
        }
    }

    pub fn from_ordered_unique(nodes: impl IntoIterator<Item = NodeId>) -> Self {
        let nodes = nodes.into_iter().collect::<Vec<_>>();
        debug_assert!(is_strict_node_order(nodes.as_slice()));
        Self {
            nodes: std::sync::Arc::new(nodes),
        }
    }

    pub fn from_slice(nodes: &[NodeId]) -> Self {
        Self::new(nodes.iter().copied())
    }

    pub fn as_slice(&self) -> &[NodeId] {
        self.nodes.as_slice()
    }

    pub fn into_vec(self) -> Vec<NodeId> {
        match std::sync::Arc::try_unwrap(self.nodes) {
            Ok(nodes) => nodes,
            Err(nodes) => nodes.as_ref().clone(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct SortedSourceBatch {
    sources: std::sync::Arc<Vec<NodeId>>,
}

impl SortedSourceBatch {
    pub fn new(sources: impl IntoIterator<Item = NodeId>) -> Self {
        Self::canonicalize_unordered(sources)
    }

    pub fn canonicalize_unordered(sources: impl IntoIterator<Item = NodeId>) -> Self {
        let mut sources = sources.into_iter().collect::<Vec<_>>();
        if sources.len() > 1 {
            sources.sort_unstable_by_key(node_sort_key);
            sources.dedup();
        }
        Self {
            sources: std::sync::Arc::new(sources),
        }
    }

    pub fn from_ordered_unique(sources: impl IntoIterator<Item = NodeId>) -> Self {
        let sources = sources.into_iter().collect::<Vec<_>>();
        debug_assert!(is_strict_node_order(sources.as_slice()));
        Self {
            sources: std::sync::Arc::new(sources),
        }
    }

    pub fn from_slice(sources: &[NodeId]) -> Self {
        Self::new(sources.iter().copied())
    }

    pub fn as_slice(&self) -> &[NodeId] {
        self.sources.as_slice()
    }

    pub fn into_vec(self) -> Vec<NodeId> {
        match std::sync::Arc::try_unwrap(self.sources) {
            Ok(sources) => sources,
            Err(sources) => sources.as_ref().clone(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.sources.is_empty()
    }

    pub fn len(&self) -> usize {
        self.sources.len()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PartitionScopeSet(SmallVec<[PartitionSubscription; 8]>);

impl PartitionScopeSet {
    pub fn new(scopes: impl IntoIterator<Item = PartitionSubscription>) -> Self {
        let mut scopes = SmallVec::<[PartitionSubscription; 8]>::from_iter(scopes);
        if scopes.len() > 1 {
            scopes.sort_unstable();
            scopes.dedup();
        }
        Self(scopes)
    }

    pub fn from_changed_regions(changed_regions: &CanonicalChangedRegions) -> Self {
        Self::new(
            changed_regions
                .as_slice()
                .iter()
                .map(|region| match &region.detail {
                    Some(detail) => PartitionSubscription::partition_and_detail(
                        region.partition.clone(),
                        detail.clone(),
                    ),
                    None => PartitionSubscription::whole_partition(region.partition.clone()),
                }),
        )
    }

    pub fn as_slice(&self) -> &[PartitionSubscription] {
        self.0.as_slice()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &PartitionSubscription> {
        self.0.iter()
    }

    pub fn intersects(&self, other: &Self) -> bool {
        let mut left = 0usize;
        let mut right = 0usize;
        while left < self.0.len() && right < other.0.len() {
            match self.0[left].cmp(&other.0[right]) {
                std::cmp::Ordering::Less => left += 1,
                std::cmp::Ordering::Greater => right += 1,
                std::cmp::Ordering::Equal => return true,
            }
        }
        false
    }
}

impl From<Vec<PartitionSubscription>> for PartitionScopeSet {
    fn from(scopes: Vec<PartitionSubscription>) -> Self {
        Self::new(scopes)
    }
}

impl From<&[PartitionSubscription]> for PartitionScopeSet {
    fn from(scopes: &[PartitionSubscription]) -> Self {
        Self::new(scopes.iter().cloned())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct LocalityFootprint {
    pub partitions: PartitionScopeSet,
    pub nodes: DedupedNodeBatch,
    pub sources: SortedSourceBatch,
}

impl LocalityFootprint {
    pub fn new(
        partitions: impl Into<PartitionScopeSet>,
        nodes: impl Into<DedupedNodeBatch>,
        sources: impl Into<SortedSourceBatch>,
    ) -> Self {
        Self {
            partitions: partitions.into(),
            nodes: nodes.into(),
            sources: sources.into(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.partitions.is_empty() && self.nodes.is_empty() && self.sources.is_empty()
    }

    pub fn conflicts_with(&self, other: &Self) -> bool {
        self.nodes
            .as_slice()
            .iter()
            .any(|node| other.nodes.as_slice().contains(node))
            || self
                .sources
                .as_slice()
                .iter()
                .any(|node| other.sources.as_slice().contains(node))
    }

    pub fn merge(&mut self, other: &Self) {
        let mut partitions = self.partitions.as_slice().to_vec();
        partitions.extend_from_slice(other.partitions.as_slice());
        self.partitions = partitions.into();

        let mut nodes = self.nodes.as_slice().to_vec();
        nodes.extend_from_slice(other.nodes.as_slice());
        self.nodes = nodes.into();

        let mut sources = self.sources.as_slice().to_vec();
        sources.extend_from_slice(other.sources.as_slice());
        self.sources = sources.into();
    }
}

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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct TouchedScopeSummary {
    pub seed_scopes: PartitionScopeSet,
    pub inclusion_scopes: PartitionScopeSet,
    pub transitive_reached_scopes: PartitionScopeSet,
    pub direct_dirty_scopes: PartitionScopeSet,
    pub maybe_stale_scopes: PartitionScopeSet,
    pub touched_nodes: DedupedNodeBatch,
    pub touched_sources: SortedSourceBatch,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PendingSnapshotCommit {
    pub node: NodeId,
    pub update: CommittedSnapshotUpdate,
    pub delta: SnapshotDeltaRecord,
}

impl PendingSnapshotCommit {
    pub fn is_stable_shape(&self) -> bool {
        matches!(self.update, CommittedSnapshotUpdate::VersionOnly(_))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PendingStableShapeSnapshotCommit {
    node: NodeId,
    update: VersionOnlySnapshotUpdate,
    delta: SnapshotDeltaRecord,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PendingReplacementSnapshotCommit {
    node: NodeId,
    update: ReplacementSnapshotUpdate,
    delta: SnapshotDeltaRecord,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PendingSnapshotBatch {
    entries: Vec<PendingSnapshotCommit>,
}

impl PendingSnapshotBatch {
    pub fn new(entries: impl IntoIterator<Item = PendingSnapshotCommit>) -> Self {
        let mut entries = entries.into_iter().collect::<Vec<_>>();
        if entries.len() > 1 {
            entries.sort_unstable_by_key(|entry| node_sort_key(&entry.node));
            entries.dedup_by(|left, right| left.node == right.node);
        }
        Self { entries }
    }

    pub fn from_pairs(entries: impl IntoIterator<Item = (NodeId, DependencySnapshot)>) -> Self {
        Self::new(entries.into_iter().map(|(node, snapshot)| {
            let snapshot = SharedDependencySnapshot::new(snapshot);
            let mut shape_store = crate::data::dependency::DependencySnapshotShapeStore::default();
            PendingSnapshotCommit {
                node,
                delta: SnapshotDeltaRecord::between(node, &DependencySnapshot::empty(), &snapshot),
                update: CommittedSnapshotUpdate::Replace(
                    crate::data::dependency::ReplacementSnapshotUpdate::from_snapshot(
                        snapshot.into_snapshot(),
                        &mut shape_store,
                    ),
                ),
            }
        }))
    }

    pub(crate) fn from_unique_pending_snapshots_in_stage_order(
        entries: impl IntoIterator<Item = crate::logic::evaluation::PendingDependencySnapshot>,
    ) -> Self {
        let entries = entries
            .into_iter()
            .map(|pending| PendingSnapshotCommit {
                node: pending.node,
                update: pending.update,
                delta: pending.delta,
            })
            .collect::<Vec<_>>();
        debug_assert!(pending_snapshot_nodes_are_unique(entries.as_slice()));
        Self { entries }
    }

    pub fn as_slice(&self) -> &[PendingSnapshotCommit] {
        &self.entries
    }

    pub fn into_vec(self) -> Vec<PendingSnapshotCommit> {
        self.entries
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn is_stable_shape_only(&self) -> bool {
        !self.entries.is_empty()
            && self
                .entries
                .iter()
                .all(PendingSnapshotCommit::is_stable_shape)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct SnapshotBatchCommit {
    pending: PendingSnapshotBatch,
    target_nodes: DedupedNodeBatch,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct StableShapeSnapshotBatchCommit {
    pending: Vec<PendingStableShapeSnapshotCommit>,
    target_nodes: DedupedNodeBatch,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct MixedSnapshotBatchCommit {
    stable_shape: Vec<PendingStableShapeSnapshotCommit>,
    replacements: Vec<PendingReplacementSnapshotCommit>,
    target_nodes: DedupedNodeBatch,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClassifiedSnapshotBatchCommit {
    StableShape(StableShapeSnapshotBatchCommit),
    Mixed(MixedSnapshotBatchCommit),
}

impl SnapshotBatchCommit {
    pub fn new(pending: PendingSnapshotBatch) -> Self {
        let target_nodes = DedupedNodeBatch::new(pending.as_slice().iter().map(|entry| entry.node));
        Self {
            pending,
            target_nodes,
        }
    }

    pub fn from_pairs(entries: impl IntoIterator<Item = (NodeId, DependencySnapshot)>) -> Self {
        Self::new(PendingSnapshotBatch::from_pairs(entries))
    }

    pub(crate) fn from_unique_pending_snapshots_in_stage_order(
        entries: impl IntoIterator<Item = crate::logic::evaluation::PendingDependencySnapshot>,
    ) -> Self {
        Self::new(PendingSnapshotBatch::from_unique_pending_snapshots_in_stage_order(entries))
    }

    pub fn pending(&self) -> &PendingSnapshotBatch {
        &self.pending
    }

    pub fn target_nodes(&self) -> &DedupedNodeBatch {
        &self.target_nodes
    }

    pub fn is_empty(&self) -> bool {
        self.pending.is_empty()
    }

    pub fn classify(self) -> ClassifiedSnapshotBatchCommit {
        if self.pending.is_stable_shape_only() {
            let pending = self
                .pending
                .into_vec()
                .into_iter()
                .map(|entry| match entry.update {
                    CommittedSnapshotUpdate::VersionOnly(update) => {
                        PendingStableShapeSnapshotCommit {
                            node: entry.node,
                            update,
                            delta: entry.delta,
                        }
                    }
                    CommittedSnapshotUpdate::Replace(_) => {
                        unreachable!("stable-shape classification must exclude replacement entries")
                    }
                })
                .collect::<Vec<_>>();
            ClassifiedSnapshotBatchCommit::StableShape(StableShapeSnapshotBatchCommit {
                pending,
                target_nodes: self.target_nodes,
            })
        } else {
            let mut stable_shape = Vec::new();
            let mut replacements = Vec::new();
            for entry in self.pending.into_vec() {
                match entry.update {
                    CommittedSnapshotUpdate::VersionOnly(update) => {
                        stable_shape.push(PendingStableShapeSnapshotCommit {
                            node: entry.node,
                            update,
                            delta: entry.delta,
                        });
                    }
                    CommittedSnapshotUpdate::Replace(update) => {
                        replacements.push(PendingReplacementSnapshotCommit {
                            node: entry.node,
                            update,
                            delta: entry.delta,
                        });
                    }
                }
            }
            ClassifiedSnapshotBatchCommit::Mixed(MixedSnapshotBatchCommit {
                stable_shape,
                replacements,
                target_nodes: self.target_nodes,
            })
        }
    }
}

impl StableShapeSnapshotBatchCommit {
    pub fn node(&self, index: usize) -> Option<NodeId> {
        self.pending
            .get(index)
            .map(PendingStableShapeSnapshotCommit::node)
    }

    pub fn pending(&self) -> &[PendingStableShapeSnapshotCommit] {
        self.pending.as_slice()
    }

    pub fn target_nodes(&self) -> &DedupedNodeBatch {
        &self.target_nodes
    }

    pub fn is_empty(&self) -> bool {
        self.pending.is_empty()
    }
}

impl MixedSnapshotBatchCommit {
    pub fn stable_shape(&self) -> &[PendingStableShapeSnapshotCommit] {
        self.stable_shape.as_slice()
    }

    pub fn replacements(&self) -> &[PendingReplacementSnapshotCommit] {
        self.replacements.as_slice()
    }

    pub fn target_nodes(&self) -> &DedupedNodeBatch {
        &self.target_nodes
    }

    pub fn is_empty(&self) -> bool {
        self.stable_shape.is_empty() && self.replacements.is_empty()
    }
}

impl ClassifiedSnapshotBatchCommit {
    pub fn is_empty(&self) -> bool {
        match self {
            Self::StableShape(commit) => commit.is_empty(),
            Self::Mixed(commit) => commit.is_empty(),
        }
    }

    pub fn target_nodes(&self) -> &DedupedNodeBatch {
        match self {
            Self::StableShape(commit) => commit.target_nodes(),
            Self::Mixed(commit) => commit.target_nodes(),
        }
    }
}

impl PendingStableShapeSnapshotCommit {
    pub fn node(&self) -> NodeId {
        self.node
    }

    pub fn update(&self) -> &VersionOnlySnapshotUpdate {
        &self.update
    }

    pub fn delta(&self) -> SnapshotDeltaRecord {
        self.delta
    }
}

impl PendingReplacementSnapshotCommit {
    pub fn node(&self) -> NodeId {
        self.node
    }

    pub fn update(&self) -> &ReplacementSnapshotUpdate {
        &self.update
    }

    pub fn delta(&self) -> SnapshotDeltaRecord {
        self.delta
    }
}

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
            repairs.sort_unstable_by_key(|repair| node_sort_key(&repair.source));
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

impl TouchedScopeSummary {
    pub fn new(
        scopes: impl Into<PartitionScopeSet>,
        touched_nodes: impl Into<DedupedNodeBatch>,
        touched_sources: impl Into<SortedSourceBatch>,
    ) -> Self {
        let scopes = scopes.into();
        Self {
            seed_scopes: scopes.clone(),
            inclusion_scopes: scopes.clone(),
            transitive_reached_scopes: PartitionScopeSet::default(),
            direct_dirty_scopes: scopes,
            maybe_stale_scopes: PartitionScopeSet::default(),
            touched_nodes: touched_nodes.into(),
            touched_sources: touched_sources.into(),
        }
    }

    pub fn new_invalidation(
        seed_scopes: impl Into<PartitionScopeSet>,
        inclusion_scopes: impl Into<PartitionScopeSet>,
        transitive_reached_scopes: impl Into<PartitionScopeSet>,
        direct_dirty_scopes: impl Into<PartitionScopeSet>,
        maybe_stale_scopes: impl Into<PartitionScopeSet>,
        touched_nodes: impl Into<DedupedNodeBatch>,
        touched_sources: impl Into<SortedSourceBatch>,
    ) -> Self {
        Self {
            seed_scopes: seed_scopes.into(),
            inclusion_scopes: inclusion_scopes.into(),
            transitive_reached_scopes: transitive_reached_scopes.into(),
            direct_dirty_scopes: direct_dirty_scopes.into(),
            maybe_stale_scopes: maybe_stale_scopes.into(),
            touched_nodes: touched_nodes.into(),
            touched_sources: touched_sources.into(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.seed_scopes.is_empty()
            && self.inclusion_scopes.is_empty()
            && self.transitive_reached_scopes.is_empty()
            && self.direct_dirty_scopes.is_empty()
            && self.maybe_stale_scopes.is_empty()
            && self.touched_nodes.is_empty()
            && self.touched_sources.is_empty()
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

impl From<Vec<PartitionSubscription>> for TouchedScopeSummary {
    fn from(scopes: Vec<PartitionSubscription>) -> Self {
        Self::new(
            scopes,
            DedupedNodeBatch::default(),
            SortedSourceBatch::default(),
        )
    }
}

impl CanonicalForm for CanonicalDependencies {}
impl CanonicalForm for CanonicalChangedRegions {}
impl CanonicalForm for DedupedNodeBatch {}
impl CanonicalForm for SortedSourceBatch {}
impl CanonicalForm for DependencyBatchEdit {}
impl CanonicalForm for PartitionScopeSet {}

impl ResolvedForm for ResolvedExecutionStrategy {}
impl ResolvedForm for ResolvedMaintenanceStrategy {}
impl ResolvedForm for ResolvedPerformancePolicy {}

impl DeltaForm for DirtyDelta {}
impl DeltaForm for StructuralDelta {}
impl DeltaForm for PatchPlan {}
impl DeltaForm for DirtyBatch {}
impl SummaryForm for LocalityFootprint {}
impl SummaryForm for TouchedScopeSummary {}
impl SummaryForm for PendingSnapshotBatch {}

fn pending_snapshot_nodes_are_unique(entries: &[PendingSnapshotCommit]) -> bool {
    let mut seen = std::collections::HashSet::with_capacity(entries.len());
    entries.iter().all(|entry| seen.insert(entry.node))
}
impl SummaryForm for SemanticBatchCommit {}
impl SummaryForm for SnapshotBatchCommit {}
impl SummaryForm for StableShapeSnapshotBatchCommit {}
impl SummaryForm for MixedSnapshotBatchCommit {}
impl SummaryForm for SubscriberRepairBatch {}
impl SummaryForm for NarrowedPropagationSet {}
impl SummaryForm for FrontierWave {}
impl SummaryForm for InvalidationFrontier {}
impl SummaryForm for InvalidationSeedBatch {}
impl SummaryForm for FrontierPlan {}
impl SummaryForm for FrontierExecutionSummary {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::aspect::Aspect;

    #[test]
    fn snapshot_batch_commit_classifies_stable_shape_batches() {
        let node = NodeId::new(0, 0);
        let mut snapshot = DependencySnapshot::empty();
        snapshot.record(NodeId::new(1, 0), Aspect::new(0), 3, None);
        let mut shape_store = crate::data::dependency::DependencySnapshotShapeStore::default();
        let basis = crate::data::dependency::StableShapeSnapshotBasis::prove(
            &crate::data::dependency::DependencyInputScan::stable_shape(
                node,
                crate::data::dependency::DependencySnapshotId::EMPTY,
                1,
                1,
                vec![5],
            ),
            snapshot.shape().intern(&mut shape_store),
        )
        .expect("proof should exist");
        let update = crate::data::dependency::CommittedSnapshotUpdate::VersionOnly(
            crate::data::dependency::VersionOnlySnapshotUpdate::from_basis_and_versions(
                basis.clone(),
                crate::data::dependency::VersionVector::from_scan(
                    &basis,
                    &crate::data::dependency::DependencyInputScan::stable_shape(
                        node,
                        crate::data::dependency::DependencySnapshotId::EMPTY,
                        1,
                        1,
                        vec![5],
                    ),
                ),
            ),
        );
        let batch = SnapshotBatchCommit::new(PendingSnapshotBatch::new([PendingSnapshotCommit {
            node,
            update,
            delta: SnapshotDeltaRecord::for_version_update(node, &snapshot, &[5]),
        }]));

        assert!(matches!(
            batch.classify(),
            ClassifiedSnapshotBatchCommit::StableShape(_)
        ));
    }
}

fn assert_strict_order<T: OrderedStreamItem>(items: &[T]) {
    for pair in items.windows(2) {
        if let [left, right] = pair {
            assert!(
                left.order_key() < right.order_key(),
                "ordered shard must be strictly increasing by stream key"
            );
        }
    }
}

fn merge_ordered_streams<T: OrderedStreamItem>(
    left: Vec<T>,
    right: Vec<T>,
) -> Result<Vec<T>, OrderedStreamMergeError<T::OrderKey>> {
    let mut merged = Vec::with_capacity(left.len() + right.len());
    let mut left = left.into_iter().peekable();
    let mut right = right.into_iter().peekable();

    loop {
        match (left.peek(), right.peek()) {
            (Some(left_item), Some(right_item)) => {
                match left_item.order_key().cmp(&right_item.order_key()) {
                    std::cmp::Ordering::Less => {
                        merged.push(left.next().expect("left item should exist"));
                    }
                    std::cmp::Ordering::Greater => {
                        merged.push(right.next().expect("right item should exist"));
                    }
                    std::cmp::Ordering::Equal => {
                        return Err(OrderedStreamMergeError::DuplicateKey(left_item.order_key()));
                    }
                }
            }
            (Some(_), None) => {
                merged.extend(left);
                return Ok(merged);
            }
            (None, Some(_)) => {
                merged.extend(right);
                return Ok(merged);
            }
            (None, None) => return Ok(merged),
        }
    }
}

fn node_sort_key(node: &NodeId) -> (u32, u32) {
    (node.index(), node.generation())
}

fn is_strict_node_order(nodes: &[NodeId]) -> bool {
    nodes
        .windows(2)
        .all(|pair| node_sort_key(&pair[0]) < node_sort_key(&pair[1]))
}
