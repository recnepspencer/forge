use std::collections::BTreeSet;

use crate::runtime::persistent_index::{
    UiPersistentIndexMutationWork, UiPersistentOrdMap, UiPersistentOrdSet,
};

type ActivationRow = crate::runtime::allocation_receipt::UiCommittedAllocationCatalogActivationRow;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct UiDerivedIndexDeltaCounters {
    row_visits: usize,
    membership_mutations: usize,
    owner_mutations: usize,
    persistent_key_probes: usize,
    persistent_node_copies: usize,
}

impl UiDerivedIndexDeltaCounters {
    pub(crate) fn row_visits(self) -> usize {
        self.row_visits
    }

    pub(crate) fn membership_mutations(self) -> usize {
        self.membership_mutations
    }

    pub(crate) fn owner_mutations(self) -> usize {
        self.owner_mutations
    }

    pub(crate) fn persistent_key_probes(self) -> usize {
        self.persistent_key_probes
    }

    pub(crate) fn persistent_node_copies(self) -> usize {
        self.persistent_node_copies
    }

    fn visit_row(&mut self) -> Result<(), ()> {
        bump(&mut self.row_visits)
    }

    fn record_membership(&mut self) -> Result<(), ()> {
        bump(&mut self.membership_mutations)
    }

    fn record_owner(&mut self) -> Result<(), ()> {
        bump(&mut self.owner_mutations)
    }

    fn record_persistent_work(&mut self, work: UiPersistentIndexMutationWork) -> Result<(), ()> {
        self.persistent_key_probes = self
            .persistent_key_probes
            .checked_add(work.key_probes())
            .ok_or(())?;
        self.persistent_node_copies = self
            .persistent_node_copies
            .checked_add(work.node_copies())
            .ok_or(())?;
        Ok(())
    }
}

impl super::UiAllocationInvalidationAuthority {
    /// Updates only keys named by the admitted catalog delta. Every value is
    /// itself persistent, so shared-key fanout is not copied or scanned here.
    pub(super) fn apply_index_delta(
        &mut self,
        removed: &[ActivationRow],
        changed: &[ActivationRow],
    ) -> Result<UiDerivedIndexDeltaCounters, ()> {
        let mut counters = UiDerivedIndexDeltaCounters::default();
        for row in removed {
            counters.visit_row()?;
            self.remove_row_indexes(row, &mut counters)?;
        }
        for row in changed {
            counters.visit_row()?;
            self.insert_row_indexes(row, &mut counters)?;
        }
        Ok(counters)
    }

    fn remove_row_indexes(
        &mut self,
        row: &ActivationRow,
        counters: &mut UiDerivedIndexDeltaCounters,
    ) -> Result<(), ()> {
        let scope = row.scope();
        let context = row.committed_invalidation_context();
        let query_sources = context
            .basis
            .query_allocation_mappings()
            .map(|(source, _)| source.clone())
            .collect::<BTreeSet<_>>();
        for source in query_sources {
            let work = remove_member(&mut self.query_contexts, &source, &scope)?;
            counters.record_persistent_work(work)?;
            counters.record_membership()?;
            let work = remove_member(
                &mut self.query_contexts_by_binding,
                &source.binding_key(),
                &scope,
            )?;
            counters.record_persistent_work(work)?;
            counters.record_membership()?;
        }

        let mut row_witnesses = BTreeSet::new();
        for request in context.basis.host_allocation_requests() {
            let Some(result) = context.basis.host_measurement_result(request) else {
                continue;
            };
            let witness = result.authority_witness();
            row_witnesses.insert(witness);
            let work = remove_owner(
                &mut self.host_witnesses_by_request,
                &(request, result.evidence_category()),
                &witness,
            )?;
            counters.record_persistent_work(work)?;
            counters.record_owner()?;
            if let Some(node) = context.basis.host_allocation_target(request) {
                let work =
                    remove_host_target_owner(&mut self.host_targets_by_witness, witness, node)?;
                counters.record_persistent_work(work)?;
                counters.record_owner()?;
            }
        }
        for witness in row_witnesses {
            let work = remove_member(&mut self.host_scopes_by_witness, &witness, &scope)?;
            counters.record_persistent_work(work)?;
            counters.record_membership()?;
        }
        for input in context
            .basis
            .durable_resize_inputs()
            .collect::<BTreeSet<_>>()
        {
            let work = remove_member(&mut self.durable_contexts, &input, &scope)?;
            counters.record_persistent_work(work)?;
            counters.record_membership()?;
        }
        Ok(())
    }

    fn insert_row_indexes(
        &mut self,
        row: &ActivationRow,
        counters: &mut UiDerivedIndexDeltaCounters,
    ) -> Result<(), ()> {
        let scope = row.scope();
        let context = row.committed_invalidation_context();
        let query_sources = context
            .basis
            .query_allocation_mappings()
            .map(|(source, _)| source.clone())
            .collect::<BTreeSet<_>>();
        for source in query_sources {
            let binding = source.binding_key();
            let work = insert_member(&mut self.query_contexts, source, scope.clone())?;
            counters.record_persistent_work(work)?;
            counters.record_membership()?;
            let work = insert_member(&mut self.query_contexts_by_binding, binding, scope.clone())?;
            counters.record_persistent_work(work)?;
            counters.record_membership()?;
        }

        let mut row_witnesses = BTreeSet::new();
        for request in context.basis.host_allocation_requests() {
            let Some(result) = context.basis.host_measurement_result(request) else {
                continue;
            };
            let witness = result.authority_witness();
            row_witnesses.insert(witness);
            let work = insert_owner(
                &mut self.host_witnesses_by_request,
                (request, result.evidence_category()),
                witness,
            )?;
            counters.record_persistent_work(work)?;
            counters.record_owner()?;
            if let Some(node) = context.basis.host_allocation_target(request) {
                let work =
                    insert_host_target_owner(&mut self.host_targets_by_witness, witness, node)?;
                counters.record_persistent_work(work)?;
                counters.record_owner()?;
            }
        }
        for witness in row_witnesses {
            let work = insert_member(&mut self.host_scopes_by_witness, witness, scope.clone())?;
            counters.record_persistent_work(work)?;
            counters.record_membership()?;
        }
        for input in context
            .basis
            .durable_resize_inputs()
            .collect::<BTreeSet<_>>()
        {
            let work = insert_member(&mut self.durable_contexts, input, scope.clone())?;
            counters.record_persistent_work(work)?;
            counters.record_membership()?;
        }
        Ok(())
    }
}

fn insert_member<K: Ord + Clone, V: Ord + Clone>(
    index: &mut UiPersistentOrdMap<K, UiPersistentOrdSet<V>>,
    key: K,
    value: V,
) -> Result<UiPersistentIndexMutationWork, ()> {
    let (members, probes) = index.get_with_probes(&key);
    let mut work = UiPersistentIndexMutationWork::with_key_probes(probes);
    let mut members = members.cloned().unwrap_or_default();
    let (inserted, member_work) = members.insert_with_work(value);
    work.merge(member_work)?;
    if !inserted {
        return Err(());
    }
    work.merge(index.insert_with_work(key, members))?;
    Ok(work)
}

fn remove_member<K: Ord + Clone, V: Ord + Clone>(
    index: &mut UiPersistentOrdMap<K, UiPersistentOrdSet<V>>,
    key: &K,
    value: &V,
) -> Result<UiPersistentIndexMutationWork, ()> {
    let (members, probes) = index.get_with_probes(key);
    let mut work = UiPersistentIndexMutationWork::with_key_probes(probes);
    let mut members = members.cloned().ok_or(())?;
    let (removed, member_work) = members.remove_with_work(value);
    work.merge(member_work)?;
    if !removed {
        return Err(());
    }
    if members.is_empty() {
        let (removed, outer_work) = index.remove_with_work(key);
        if !removed {
            return Err(());
        }
        work.merge(outer_work)?;
    } else {
        work.merge(index.insert_with_work(key.clone(), members))?;
    }
    Ok(work)
}

fn insert_owner<K: Ord + Clone, V: Ord + Clone>(
    index: &mut UiPersistentOrdMap<K, UiPersistentOrdMap<V, usize>>,
    key: K,
    value: V,
) -> Result<UiPersistentIndexMutationWork, ()> {
    let (owners, probes) = index.get_with_probes(&key);
    let mut work = UiPersistentIndexMutationWork::with_key_probes(probes);
    let mut owners = owners.cloned().unwrap_or_default();
    let (prior, owner_probes) = owners.get_with_probes(&value);
    work.merge(UiPersistentIndexMutationWork::with_key_probes(owner_probes))?;
    let next = prior
        .copied()
        .unwrap_or_default()
        .checked_add(1)
        .ok_or(())?;
    work.merge(owners.insert_with_work(value, next))?;
    work.merge(index.insert_with_work(key, owners))?;
    Ok(work)
}

fn remove_owner<K: Ord + Clone, V: Ord + Clone>(
    index: &mut UiPersistentOrdMap<K, UiPersistentOrdMap<V, usize>>,
    key: &K,
    value: &V,
) -> Result<UiPersistentIndexMutationWork, ()> {
    let (owners, probes) = index.get_with_probes(key);
    let mut work = UiPersistentIndexMutationWork::with_key_probes(probes);
    let mut owners = owners.cloned().ok_or(())?;
    let (prior, owner_probes) = owners.get_with_probes(value);
    work.merge(UiPersistentIndexMutationWork::with_key_probes(owner_probes))?;
    let prior = prior.copied().ok_or(())?;
    if prior == 1 {
        let (removed, owner_work) = owners.remove_with_work(value);
        if !removed {
            return Err(());
        }
        work.merge(owner_work)?;
    } else {
        work.merge(owners.insert_with_work(value.clone(), prior - 1))?;
    }
    if owners.is_empty() {
        let (removed, outer_work) = index.remove_with_work(key);
        if !removed {
            return Err(());
        }
        work.merge(outer_work)?;
    } else {
        work.merge(index.insert_with_work(key.clone(), owners))?;
    }
    Ok(work)
}

fn insert_host_target_owner(
    index: &mut UiPersistentOrdMap<
        crate::evidence::UiHostMeasurementAuthorityWitness,
        super::authority::UiHostInvalidationTargetMapping,
    >,
    witness: crate::evidence::UiHostMeasurementAuthorityWitness,
    node: crate::graph::UiGraphNodeIdentity,
) -> Result<UiPersistentIndexMutationWork, ()> {
    let (mapping, probes) = index.get_with_probes(&witness);
    let mut work = UiPersistentIndexMutationWork::with_key_probes(probes);
    let mut mapping = mapping.cloned().unwrap_or_default();
    work.merge(mapping.add_owner(node)?)?;
    work.merge(index.insert_with_work(witness, mapping))?;
    Ok(work)
}

fn remove_host_target_owner(
    index: &mut UiPersistentOrdMap<
        crate::evidence::UiHostMeasurementAuthorityWitness,
        super::authority::UiHostInvalidationTargetMapping,
    >,
    witness: crate::evidence::UiHostMeasurementAuthorityWitness,
    node: crate::graph::UiGraphNodeIdentity,
) -> Result<UiPersistentIndexMutationWork, ()> {
    let (mapping, probes) = index.get_with_probes(&witness);
    let mut work = UiPersistentIndexMutationWork::with_key_probes(probes);
    let mut mapping = mapping.cloned().ok_or(())?;
    work.merge(mapping.remove_owner(node)?)?;
    if mapping.is_empty() {
        let (removed, outer_work) = index.remove_with_work(&witness);
        if !removed {
            return Err(());
        }
        work.merge(outer_work)?;
    } else {
        work.merge(index.insert_with_work(witness, mapping))?;
    }
    Ok(work)
}

fn bump(counter: &mut usize) -> Result<(), ()> {
    *counter = counter.checked_add(1).ok_or(())?;
    Ok(())
}

#[cfg(test)]
#[path = "index_delta/counter_tests.rs"]
mod tests;
