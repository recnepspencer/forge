use std::collections::{BTreeMap, BTreeSet};

use worth_foundational::facade::{
    AspectBinding, AspectContractRevision, AspectIdentity, AspectKey,
    AuthoritativeAspectChangeKind, CanonicalFieldPath, ProjectionMask, TruthPartitionRole,
};
use worth_runtime_bridge::facade::{
    BridgeDeliveredCorrespondenceChange, BridgeSemanticDependencyCandidate, BridgeSemanticLocality,
    RelationalBridgeRecordIdentityParts,
};

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct BoundPrimaryDependencyKey {
    aspect_key: AspectKey,
    aspect_identity: AspectIdentity,
    contract_revision: AspectContractRevision,
    binding: AspectBinding,
    locality: BoundPrimaryLocalityKey,
}

#[derive(Default)]
struct BoundPrimaryProjectionBucket {
    whole_aspect: Vec<usize>,
    exact_paths: BTreeMap<CanonicalFieldPath, Vec<usize>>,
    descendants_by_prefix: BTreeMap<CanonicalFieldPath, Vec<usize>>,
}

#[derive(Default)]
struct BoundPrimaryChangeBuckets {
    by_kind: BTreeMap<AuthoritativeAspectChangeKind, BoundPrimaryProjectionBucket>,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum BoundPrimaryLocalityKey {
    ExactRecord(RelationalBridgeRecordIdentityParts),
    AnyManagedRecord,
    Partition(TruthPartitionRole),
    WholeGraph,
}

/// Rebuildable, non-authoritative lookup over one consumer's exact installed
/// Bridge dependencies. Every returned candidate is still checked against the
/// current Query invalidation manifest before it can admit work.
pub(super) struct WorthQueryBoundPrimaryDependencyIndex {
    buckets: BTreeMap<BoundPrimaryDependencyKey, BoundPrimaryChangeBuckets>,
}

impl WorthQueryBoundPrimaryDependencyIndex {
    pub(super) fn build(dependencies: &[BridgeSemanticDependencyCandidate]) -> Self {
        let mut buckets = BTreeMap::<_, BoundPrimaryChangeBuckets>::new();
        for (index, dependency) in dependencies.iter().enumerate() {
            let locality = match dependency.locality() {
                BridgeSemanticLocality::SourceRecord => BoundPrimaryLocalityKey::ExactRecord(
                    dependency
                        .source_record_identity()
                        .expect("an admitted source-record dependency retains its record"),
                ),
                BridgeSemanticLocality::ManagedSourceRecord => {
                    BoundPrimaryLocalityKey::AnyManagedRecord
                }
                BridgeSemanticLocality::SourcePartition(partition) => {
                    BoundPrimaryLocalityKey::Partition(partition.clone())
                }
                BridgeSemanticLocality::WholeLogicalGraph => BoundPrimaryLocalityKey::WholeGraph,
            };
            for &change_kind in dependency.relevant_changes() {
                let projection = buckets
                    .entry(key_for(dependency, locality.clone()))
                    .or_default()
                    .by_kind
                    .entry(change_kind)
                    .or_default();
                index_projection(projection, dependency.projection_mask(), index);
            }
        }
        Self { buckets }
    }

    pub(super) fn lookup(
        &self,
        delivered: &BridgeSemanticDependencyCandidate,
        changes: &[BridgeDeliveredCorrespondenceChange],
    ) -> (Vec<usize>, usize) {
        let lookup_changes = lookup_changes(delivered, changes);
        let probes = lookup_changes
            .iter()
            .map(|change| change.localities.len())
            .sum();
        let mut indices = BTreeSet::new();
        for change in &lookup_changes {
            for locality in &change.localities {
                let Some(buckets) = self.buckets.get(&key_for(delivered, locality.clone())) else {
                    continue;
                };
                for (kind, bucket) in &buckets.by_kind {
                    if change.kind_intersects(*kind) {
                        select_projection_candidates(bucket, change.projection(), &mut indices);
                    }
                }
            }
        }
        (indices.into_iter().collect(), probes)
    }
}

fn lookup_localities(
    delivered: &BridgeSemanticDependencyCandidate,
    changes: &[BridgeDeliveredCorrespondenceChange],
) -> Vec<BoundPrimaryLocalityKey> {
    let mut localities = changes
        .iter()
        .filter_map(BridgeDeliveredCorrespondenceChange::relational_record_identity)
        .map(BoundPrimaryLocalityKey::ExactRecord)
        .collect::<BTreeSet<_>>();
    if let Some(record) = delivered.source_record_identity() {
        localities.insert(BoundPrimaryLocalityKey::ExactRecord(record));
    }
    if localities
        .iter()
        .any(|locality| matches!(locality, BoundPrimaryLocalityKey::ExactRecord(_)))
    {
        localities.insert(BoundPrimaryLocalityKey::AnyManagedRecord);
    }
    if let BridgeSemanticLocality::SourcePartition(partition) = delivered.locality() {
        localities.insert(BoundPrimaryLocalityKey::Partition(partition.clone()));
    }
    // A whole-graph consumer contains every narrower delivered locality.
    localities.insert(BoundPrimaryLocalityKey::WholeGraph);
    localities.into_iter().collect()
}

enum BoundPrimaryLookupProjection<'a> {
    WholeAspect,
    Paths(&'a [CanonicalFieldPath]),
}

struct BoundPrimaryLookupChange<'a> {
    semantic: Option<&'a worth_runtime_bridge::facade::BridgeSemanticAspectChange>,
    structural_kind: Option<AuthoritativeAspectChangeKind>,
    fallback_kind: Option<AuthoritativeAspectChangeKind>,
    projection: BoundPrimaryLookupProjection<'a>,
    localities: Vec<BoundPrimaryLocalityKey>,
}

impl BoundPrimaryLookupChange<'_> {
    fn kind_intersects(&self, admitted: AuthoritativeAspectChangeKind) -> bool {
        self.semantic
            .is_some_and(|change| change.intersects_relevant_change(admitted))
            || self.structural_kind == Some(admitted)
            || self.fallback_kind == Some(admitted)
    }

    fn projection(&self) -> &BoundPrimaryLookupProjection<'_> {
        &self.projection
    }
}

fn lookup_changes<'a>(
    delivered: &'a BridgeSemanticDependencyCandidate,
    changes: &'a [BridgeDeliveredCorrespondenceChange],
) -> Vec<BoundPrimaryLookupChange<'a>> {
    let actual = changes
        .iter()
        .filter_map(|delivered_change| {
            let semantic = delivered_change.semantic_change();
            let structural_kind = semantic
                .is_none()
                .then(|| delivered_change.effective_change_kind_for(delivered))
                .flatten();
            if semantic.is_none() && structural_kind.is_none() {
                return None;
            }
            Some(BoundPrimaryLookupChange {
                semantic,
                structural_kind,
                fallback_kind: None,
                projection: semantic
                    .and_then(|change| change.effective_field_path())
                    .map_or(BoundPrimaryLookupProjection::WholeAspect, |path| {
                        BoundPrimaryLookupProjection::Paths(std::slice::from_ref(path))
                    }),
                localities: lookup_localities_for_record(
                    delivered,
                    delivered_change.relational_record_identity(),
                ),
            })
        })
        .collect::<Vec<_>>();
    if !actual.is_empty() {
        return actual;
    }
    let projection = if delivered.projection_mask().is_whole_aspect() {
        BoundPrimaryLookupProjection::WholeAspect
    } else {
        BoundPrimaryLookupProjection::Paths(delivered.projection_mask().paths())
    };
    let localities = lookup_localities(delivered, changes);
    delivered
        .relevant_changes()
        .iter()
        .copied()
        .map(|kind| BoundPrimaryLookupChange {
            semantic: None,
            structural_kind: None,
            fallback_kind: Some(kind),
            projection: match &projection {
                BoundPrimaryLookupProjection::WholeAspect => {
                    BoundPrimaryLookupProjection::WholeAspect
                }
                BoundPrimaryLookupProjection::Paths(paths) => {
                    BoundPrimaryLookupProjection::Paths(paths)
                }
            },
            localities: localities.clone(),
        })
        .collect()
}

fn lookup_localities_for_record(
    delivered: &BridgeSemanticDependencyCandidate,
    record: Option<RelationalBridgeRecordIdentityParts>,
) -> Vec<BoundPrimaryLocalityKey> {
    let mut localities = BTreeSet::new();
    let record = record.or_else(|| delivered.source_record_identity());
    if let Some(record) = record {
        localities.insert(BoundPrimaryLocalityKey::ExactRecord(record));
        localities.insert(BoundPrimaryLocalityKey::AnyManagedRecord);
    }
    if let BridgeSemanticLocality::SourcePartition(partition) = delivered.locality() {
        localities.insert(BoundPrimaryLocalityKey::Partition(partition.clone()));
    }
    localities.insert(BoundPrimaryLocalityKey::WholeGraph);
    localities.into_iter().collect()
}

fn key_for(
    dependency: &BridgeSemanticDependencyCandidate,
    locality: BoundPrimaryLocalityKey,
) -> BoundPrimaryDependencyKey {
    BoundPrimaryDependencyKey {
        aspect_key: dependency.contract().key().clone(),
        aspect_identity: dependency.contract().identity(),
        contract_revision: dependency.contract().revision(),
        binding: dependency.binding().clone(),
        locality,
    }
}

fn index_projection(
    bucket: &mut BoundPrimaryProjectionBucket,
    mask: &worth_foundational::facade::AspectMask<ProjectionMask>,
    index: usize,
) {
    if mask.is_whole_aspect() {
        bucket.whole_aspect.push(index);
        return;
    }
    for path in mask.paths() {
        bucket
            .exact_paths
            .entry(path.clone())
            .or_default()
            .push(index);
        for prefix in path_prefixes(path) {
            bucket
                .descendants_by_prefix
                .entry(prefix)
                .or_default()
                .push(index);
        }
    }
}

fn select_projection_candidates(
    bucket: &BoundPrimaryProjectionBucket,
    projection: &BoundPrimaryLookupProjection<'_>,
    selected: &mut BTreeSet<usize>,
) {
    selected.extend(bucket.whole_aspect.iter().copied());
    match projection {
        BoundPrimaryLookupProjection::WholeAspect => {
            selected.extend(bucket.exact_paths.values().flatten().copied());
        }
        BoundPrimaryLookupProjection::Paths(paths) => {
            for path in *paths {
                if let Some(descendants) = bucket.descendants_by_prefix.get(path) {
                    selected.extend(descendants.iter().copied());
                }
                for prefix in path_prefixes(path) {
                    if let Some(ancestors) = bucket.exact_paths.get(&prefix) {
                        selected.extend(ancestors.iter().copied());
                    }
                }
            }
        }
    }
}

fn path_prefixes(path: &CanonicalFieldPath) -> Vec<CanonicalFieldPath> {
    (1..=path.fields().len())
        .filter_map(|length| CanonicalFieldPath::new(path.fields()[..length].to_vec()))
        .collect()
}

#[cfg(test)]
#[path = "dependency_index/tests.rs"]
mod tests;
