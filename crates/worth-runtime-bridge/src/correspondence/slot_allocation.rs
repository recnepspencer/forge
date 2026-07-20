use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Arc, RwLock};

use worth_signal::facade::{Aspect, PartitionToken, MAX_ASPECTS};

use super::{
    BridgeSignalAspectTargetDeclaration, BridgeSignalSlotRequest, InstalledCorrespondenceTarget,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(super) struct AllocationKey {
    pub(super) signal_graph_instance_id: u64,
    pub(super) partition: PartitionToken,
    pub(super) node: worth_signal::facade::NodeId,
    pub(super) aspect: Aspect,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(super) struct AuthoritativeAllocationRecord {
    pub(super) key: AllocationKey,
    pub(super) owner: String,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct CorrespondenceAllocationRegistry {
    pub(super) authoritative_records: BTreeSet<AuthoritativeAllocationRecord>,
    pub(super) owners: BTreeMap<AllocationKey, BTreeSet<String>>,
}

impl CorrespondenceAllocationRegistry {
    pub(crate) fn admits_source_set(&self, target: &InstalledCorrespondenceTarget) -> bool {
        let key = AllocationKey {
            signal_graph_instance_id: target.signal_graph_instance_id,
            partition: target.partition.clone(),
            node: target.node,
            aspect: target.aspect,
        };
        self.owners
            .get(&key)
            .is_some_and(|owners| owners.iter().eq(target.allocation_sources.iter()))
    }

    #[cfg(test)]
    pub(crate) fn is_empty(&self) -> bool {
        self.authoritative_records.is_empty() && self.owners.is_empty()
    }
}

pub(crate) type SharedCorrespondenceAllocationRegistry =
    Arc<RwLock<CorrespondenceAllocationRegistry>>;

pub(super) enum SlotAllocation {
    Allocated(Aspect),
    CapacityExhausted,
    NodeContractMismatch,
}

pub(super) fn rebuild_owners(
    records: &BTreeSet<AuthoritativeAllocationRecord>,
) -> BTreeMap<AllocationKey, BTreeSet<String>> {
    let mut owners = BTreeMap::<AllocationKey, BTreeSet<String>>::new();
    for record in records {
        owners
            .entry(record.key.clone())
            .or_default()
            .insert(record.owner.clone());
    }
    owners
}

pub(super) fn allocate_slot(
    registry: &CorrespondenceAllocationRegistry,
    pending: &BTreeMap<AllocationKey, BTreeSet<String>>,
    signal_graph_instance_id: u64,
    declaration: &BridgeSignalAspectTargetDeclaration,
) -> (SlotAllocation, usize) {
    match &declaration.slot {
        BridgeSignalSlotRequest::Exact(aspect) => (
            if signal_node_admits(declaration, aspect.aspect()) {
                SlotAllocation::Allocated(aspect.aspect())
            } else {
                SlotAllocation::NodeContractMismatch
            },
            1,
        ),
        BridgeSignalSlotRequest::Allocate => {
            let mut examined = 0;
            let mut admitted_slot_exists = false;
            let aspect = (0..MAX_ASPECTS as u8).map(Aspect::new).find(|aspect| {
                examined += 1;
                let admitted = signal_node_admits(declaration, *aspect);
                admitted_slot_exists |= admitted;
                admitted
                    && !registry.owners.contains_key(&AllocationKey {
                        signal_graph_instance_id,
                        partition: declaration.partition.clone(),
                        node: declaration.node,
                        aspect: *aspect,
                    })
                    && !pending.contains_key(&AllocationKey {
                        signal_graph_instance_id,
                        partition: declaration.partition.clone(),
                        node: declaration.node,
                        aspect: *aspect,
                    })
            });
            (
                aspect.map_or_else(
                    || {
                        if admitted_slot_exists {
                            SlotAllocation::CapacityExhausted
                        } else {
                            SlotAllocation::NodeContractMismatch
                        }
                    },
                    SlotAllocation::Allocated,
                ),
                examined,
            )
        }
    }
}

pub(super) fn signal_node_admits(
    declaration: &BridgeSignalAspectTargetDeclaration,
    aspect: Aspect,
) -> bool {
    let contract = declaration.node_capability.contract();
    let slot = worth_signal::facade::AspectMask::from_aspect(aspect);
    let partition_admitted = |scopes: &Option<Vec<worth_signal::facade::PartitionSubscription>>| {
        scopes.as_ref().is_none_or(|scopes| {
            scopes
                .iter()
                .any(|scope| scope.partition == declaration.partition)
        })
    };
    contract.semantics.reads.contains(slot)
        && contract.projection.consumes.contains(slot)
        && partition_admitted(&contract.semantics.partition_scope)
        && partition_admitted(&contract.projection.consumes_partitions)
}
