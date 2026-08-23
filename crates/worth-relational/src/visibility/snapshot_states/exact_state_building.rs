use std::collections::BTreeMap;

use crate::runtime::RelationalRuntime;
use crate::snapshots::data::{SnapshotHandle, SnapshotId, SnapshotReadPolicy};
use crate::storage::overlay::{PartitionAccess, SnapshotPartitionPins};
use crate::storage::partition::DenseSlotBitSet;

use super::pin_assembly::{assemble_snapshot_state, SelectedRelationSlots};
use super::{SnapshotState, SnapshotStateBasis, VisibilitySnapshotBasis};

pub(crate) fn build_partition_pins_for_version(
    runtime: &RelationalRuntime,
    version_id: crate::identity::data::VersionId,
) -> BTreeMap<crate::identity::data::PartitionId, SnapshotPartitionPins> {
    let branch_id =
        crate::visibility::branch_scope::authoritative_branch_for_version(runtime, version_id);
    build_partition_pins_for_branch_head(runtime, &branch_id, version_id)
}

pub(crate) fn build_partition_pins_for_branch_head(
    runtime: &RelationalRuntime,
    branch_id: &crate::history::data::BranchId,
    version_id: crate::identity::data::VersionId,
) -> BTreeMap<crate::identity::data::PartitionId, SnapshotPartitionPins> {
    let Some(basis) = VisibilitySnapshotBasis::capture_current(runtime, branch_id, version_id)
    else {
        return BTreeMap::new();
    };
    build_visibility_state(
        runtime,
        basis,
        SnapshotId(0),
        SnapshotReadPolicy::ImmutablePinnedNoLazyMutation,
    )
    .pinned_partitions
}

pub(crate) fn build_visibility_state(
    runtime: &RelationalRuntime,
    basis: VisibilitySnapshotBasis,
    snapshot_id: SnapshotId,
    read_policy: SnapshotReadPolicy,
) -> SnapshotState {
    let root = basis.root().clone();
    if let Some(root_version) = root
        .axes()
        .map(|axes| crate::identity::data::VersionId(axes.storage_version))
    {
        debug_assert_eq!(root_version, basis.version_id());
    }
    let handle = SnapshotHandle {
        runtime_instance_id: runtime.runtime_instance_id(),
        branch_id: basis.branch_id().clone(),
        snapshot_id,
        version_id: basis.version_id(),
        read_policy,
    };
    let entities = exact_entity_slots(root.as_ref());
    let relations = exact_relation_slots(runtime, root.as_ref());
    assemble_snapshot_state(
        handle,
        SnapshotStateBasis::Exact(basis),
        entities,
        relations,
    )
}

fn exact_entity_slots(
    state: &impl PartitionAccess,
) -> Vec<(crate::identity::data::PartitionId, DenseSlotBitSet)> {
    state
        .partition_ids()
        .into_iter()
        .filter_map(|partition_id| {
            state
                .get_partition(partition_id)
                .map(|partition| (partition_id, partition.entity_arena.live_bitset.clone()))
        })
        .collect()
}

fn exact_relation_slots(
    runtime: &RelationalRuntime,
    root: &crate::branch::RelationalBranchRoot,
) -> Vec<SelectedRelationSlots> {
    root.partition_ids()
        .into_iter()
        .filter_map(|partition_id| {
            let visible = root
                .get_partition(partition_id)?
                .relation_arena
                .live_bitset
                .clone();
            let retained = retained_exact_relations(runtime, root, partition_id, &visible);
            Some(SelectedRelationSlots {
                partition_id,
                visible,
                retained,
            })
        })
        .collect()
}

fn retained_exact_relations(
    runtime: &RelationalRuntime,
    root: &crate::branch::RelationalBranchRoot,
    partition_id: crate::identity::data::PartitionId,
    visible: &DenseSlotBitSet,
) -> DenseSlotBitSet {
    let reader = runtime.read_truth();
    let mut retained = DenseSlotBitSet::with_capacity(visible.represented_slot_capacity());
    for slot in visible.iter_set_slots() {
        let relation_id = crate::identity::data::RelationId::new(partition_id, slot as u64, 0);
        let record = reader.authoritative_relation_record_for_id_from_exact_state(
            root,
            root.schema_authority().registry(),
            relation_id,
        );
        if record.is_some_and(|record| {
            record.lifecycle == crate::storage::data::RecordLifecycleState::RetainedDanglingForAudit
        }) {
            retained.set(slot, true);
        }
    }
    retained
}
