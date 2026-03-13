use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

use crate::data::aspect::{Aspect, AspectMask};
use crate::data::dependency::CanonicalDependencies;
use crate::data::dependency::{
    DependencySnapshot, DependencySnapshotUpdate, SharedDependencySnapshot, SnapshotDeltaRecord,
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
    nodes: Vec<NodeId>,
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
        Self { nodes }
    }

    pub fn from_ordered_unique(nodes: impl IntoIterator<Item = NodeId>) -> Self {
        let nodes = nodes.into_iter().collect::<Vec<_>>();
        debug_assert!(is_strict_node_order(nodes.as_slice()));
        Self { nodes }
    }

    pub fn from_slice(nodes: &[NodeId]) -> Self {
        Self::new(nodes.iter().copied())
    }

    pub fn as_slice(&self) -> &[NodeId] {
        &self.nodes
    }

    pub fn into_vec(self) -> Vec<NodeId> {
        self.nodes
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
    sources: Vec<NodeId>,
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
        Self { sources }
    }

    pub fn from_ordered_unique(sources: impl IntoIterator<Item = NodeId>) -> Self {
        let sources = sources.into_iter().collect::<Vec<_>>();
        debug_assert!(is_strict_node_order(sources.as_slice()));
        Self { sources }
    }

    pub fn from_slice(sources: &[NodeId]) -> Self {
        Self::new(sources.iter().copied())
    }

    pub fn as_slice(&self) -> &[NodeId] {
        &self.sources
    }

    pub fn into_vec(self) -> Vec<NodeId> {
        self.sources
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
    pub scopes: PartitionScopeSet,
    pub touched_nodes: DedupedNodeBatch,
    pub touched_sources: SortedSourceBatch,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PendingSnapshotCommit {
    pub node: NodeId,
    pub update: DependencySnapshotUpdate,
    pub delta: SnapshotDeltaRecord,
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
            PendingSnapshotCommit {
                node,
                delta: SnapshotDeltaRecord::between(node, &DependencySnapshot::empty(), &snapshot),
                update: DependencySnapshotUpdate::Replace(snapshot),
            }
        }))
    }

    pub(crate) fn from_pending_snapshots(
        entries: impl IntoIterator<Item = crate::logic::evaluation::PendingDependencySnapshot>,
    ) -> Self {
        Self::new(entries.into_iter().map(|pending| PendingSnapshotCommit {
            node: pending.node,
            update: pending.update,
            delta: pending.delta,
        }))
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
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct SnapshotBatchCommit {
    pending: PendingSnapshotBatch,
    target_nodes: DedupedNodeBatch,
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

    pub(crate) fn from_pending_snapshots(
        entries: impl IntoIterator<Item = crate::logic::evaluation::PendingDependencySnapshot>,
    ) -> Self {
        Self::new(PendingSnapshotBatch::from_pending_snapshots(entries))
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
        Self {
            scopes: scopes.into(),
            touched_nodes: touched_nodes.into(),
            touched_sources: touched_sources.into(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.scopes.is_empty() && self.touched_nodes.is_empty() && self.touched_sources.is_empty()
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
impl SummaryForm for SemanticBatchCommit {}
impl SummaryForm for SnapshotBatchCommit {}
impl SummaryForm for SubscriberRepairBatch {}
impl SummaryForm for NarrowedPropagationSet {}
impl SummaryForm for FrontierWave {}
impl SummaryForm for InvalidationFrontier {}

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
