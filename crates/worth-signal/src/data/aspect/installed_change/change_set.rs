use std::collections::{BTreeMap, BTreeSet};

use worth_proof::TransitionOutcome;

use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::data::output::{CanonicalChangedRegions, ChangedRegion};
use crate::data::proof::{DirtyBatch, DirtyBatchEntry};

use super::super::{Aspect, InstalledSignalAspectCapability};
use super::{SignalInstalledScopedChangeDenial, SignalInstalledScopedChangeOutcome};

#[derive(Debug, PartialEq, Eq)]
pub struct InstalledSignalScopedChange {
    capability: InstalledSignalAspectCapability,
    changed_regions: CanonicalChangedRegions,
}

impl InstalledSignalScopedChange {
    pub fn new(
        capability: InstalledSignalAspectCapability,
        changed_regions: impl IntoIterator<Item = ChangedRegion>,
    ) -> Self {
        Self {
            capability,
            changed_regions: CanonicalChangedRegions::new(changed_regions),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct InstalledSignalScopedChangeView<'a> {
    node: NodeId,
    aspect: Aspect,
    changed_regions: &'a [ChangedRegion],
}

impl InstalledSignalScopedChangeView<'_> {
    pub const fn node(&self) -> NodeId {
        self.node
    }

    pub const fn aspect(&self) -> Aspect {
        self.aspect
    }

    pub const fn changed_regions(&self) -> &[ChangedRegion] {
        self.changed_regions
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct InstalledSignalScopedChangeSet {
    graph_instance_id: u64,
    dirty: DirtyBatch,
}

impl InstalledSignalScopedChangeSet {
    pub const fn graph_instance_id(&self) -> u64 {
        self.graph_instance_id
    }

    pub fn changes(&self) -> impl ExactSizeIterator<Item = InstalledSignalScopedChangeView<'_>> {
        self.dirty
            .as_slice()
            .iter()
            .map(|entry| InstalledSignalScopedChangeView {
                node: entry.source,
                aspect: entry.changed_aspect,
                changed_regions: entry.changed_regions.as_slice(),
            })
    }

    pub fn len(&self) -> usize {
        self.dirty.as_slice().len()
    }

    pub fn is_empty(&self) -> bool {
        self.dirty.is_empty()
    }
}

pub fn apply_installed_scoped_changes(
    graph: &mut SignalGraph,
    changes: impl IntoIterator<Item = InstalledSignalScopedChange>,
) -> SignalInstalledScopedChangeOutcome {
    let changes = changes.into_iter().collect::<Vec<_>>();
    if changes.is_empty() {
        return TransitionOutcome::Denied(SignalInstalledScopedChangeDenial::EmptyChangeSet);
    }

    let graph_instance_id = graph.runtime_instance_id();
    let mut unique = BTreeSet::new();
    let mut entries = Vec::with_capacity(changes.len());
    for change in changes {
        let capability = change.capability;
        if capability.graph_instance_id() != graph_instance_id {
            return TransitionOutcome::Denied(SignalInstalledScopedChangeDenial::ForeignCapability);
        }
        if !unique.insert((capability.node(), capability.aspect())) {
            return TransitionOutcome::Denied(SignalInstalledScopedChangeDenial::DuplicateTarget);
        }
        if graph.node_aspect_version(capability.node()).is_err() {
            return TransitionOutcome::Denied(
                SignalInstalledScopedChangeDenial::MissingOrStaleTarget,
            );
        }
        entries.push(DirtyBatchEntry::new(
            capability.node(),
            capability.aspect(),
            change.changed_regions,
        ));
    }

    let dirty = DirtyBatch::new(entries);
    match apply_scoped_change_effect(graph, &dirty) {
        Ok(()) => TransitionOutcome::Success(InstalledSignalScopedChangeSet {
            graph_instance_id,
            dirty,
        }),
        Err(error) => TransitionOutcome::Failed(error),
    }
}

fn apply_scoped_change_effect(
    graph: &mut SignalGraph,
    dirty: &DirtyBatch,
) -> Result<(), crate::data::error::SignalError> {
    let mut original = BTreeMap::new();
    let mut updated = BTreeMap::new();
    for entry in dirty.as_slice() {
        if let std::collections::btree_map::Entry::Vacant(original_entry) =
            original.entry(entry.source)
        {
            let current = graph.node_partition_version_map(entry.source)?;
            original_entry.insert(current.clone());
            updated.insert(entry.source, current);
        }
        let baseline = original
            .get(&entry.source)
            .expect("validated installed target has baseline versions");
        updated
            .get_mut(&entry.source)
            .expect("validated installed target has staged versions")
            .apply_scoped_aspect_bump(
                entry.changed_aspect,
                entry.changed_regions.as_slice(),
                baseline,
            );
    }

    for (node, versions) in updated {
        graph.replace_node_partition_version_map(node, versions)?;
    }
    if let Err(error) = crate::logic::invalidation::mark_dirty_batch(&mut *graph, dirty) {
        for (node, versions) in original {
            graph.replace_node_partition_version_map(node, versions)?;
        }
        return Err(error);
    }
    Ok(())
}
