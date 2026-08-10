//! Construct typed runtime requests from harness input.

use std::collections::{BTreeMap, BTreeSet};

use worth_harness::facade::MutationBatch;

use crate::data::aspect::Aspect;
use crate::data::error::SignalError;
use crate::data::handle::NodeId;
use crate::data::output::ChangedRegion;
use crate::data::proof::{DirtyBatch, DirtyBatchEntry};

use super::super::runtime::{SignalHarnessRuntime, SignalMutationAction, SignalMutationKind};

pub(super) fn resolve_targets(
    runtime: &SignalHarnessRuntime,
    labels: &[String],
) -> Result<Vec<NodeId>, SignalError> {
    labels.iter().map(|label| runtime.resolve(label)).collect()
}

pub(super) fn build_dirty_batch(
    runtime: &SignalHarnessRuntime,
    batch: &MutationBatch<SignalMutationAction>,
) -> Result<DirtyBatch, SignalError> {
    let mut pending_regions =
        BTreeMap::<(u32, u32, u8), (NodeId, Aspect, Option<BTreeSet<ChangedRegion>>)>::new();

    for operation in &batch.operations {
        match operation.kind() {
            SignalMutationKind::MarkDirty { label, aspect } => {
                let node = runtime.resolve(label)?;
                pending_regions.insert(
                    (node.index(), node.generation(), aspect.id()),
                    (node, *aspect, None),
                );
            }
            SignalMutationKind::MarkDirtyWithRegions {
                label,
                aspect,
                changed_regions,
            } => {
                let node = runtime.resolve(label)?;
                let key = (node.index(), node.generation(), aspect.id());
                let entry = pending_regions
                    .entry(key)
                    .or_insert_with(|| (node, *aspect, Some(BTreeSet::new())));
                if let Some(regions) = &mut entry.2 {
                    regions.extend(changed_regions.iter().cloned());
                }
            }
        }
    }

    Ok(DirtyBatch::new(pending_regions.into_values().map(
        |(node, aspect, regions)| {
            DirtyBatchEntry::new(
                node,
                aspect,
                regions
                    .map(|regions| regions.into_iter().collect::<Vec<_>>())
                    .unwrap_or_default(),
            )
        },
    )))
}
