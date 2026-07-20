use std::collections::BTreeSet;

use crate::evidence::UiHostMeasurementAuthorityWitness;
use crate::runtime::persistent_index::UiPersistentOrdMap;

type ActivationRow = crate::runtime::allocation_receipt::UiCommittedAllocationCatalogActivationRow;

impl super::UiAllocationInvalidationAuthority {
    /// Updates derived invalidation indexes from the admitted catalog delta.
    /// Untouched persistent-map branches remain shared with the predecessor.
    pub(super) fn apply_index_delta(
        &mut self,
        removed: &[ActivationRow],
        changed: &[ActivationRow],
    ) -> Result<(), ()> {
        let mut touched_host_witnesses = BTreeSet::new();
        for row in removed {
            self.remove_row_indexes(row, &mut touched_host_witnesses)?;
        }
        for row in changed {
            self.insert_row_indexes(row, &mut touched_host_witnesses);
        }
        for witness in touched_host_witnesses {
            self.refresh_host_target(witness);
        }
        Ok(())
    }

    fn remove_row_indexes(
        &mut self,
        row: &ActivationRow,
        touched: &mut BTreeSet<UiHostMeasurementAuthorityWitness>,
    ) -> Result<(), ()> {
        let scope = row.scope();
        let context = row.committed_invalidation_context();
        for (source, _) in context.basis.query_allocation_mappings() {
            remove_value(&mut self.query_contexts, source, &scope)?;
        }
        for request in context.basis.host_allocation_requests() {
            let Some(result) = context.basis.host_measurement_result(request) else {
                continue;
            };
            let witness = result.authority_witness();
            touched.insert(witness);
            remove_value(&mut self.host_scopes_by_witness, &witness, &scope)?;
            remove_value(
                &mut self.host_witnesses_by_request,
                &(request, result.evidence_category()),
                &witness,
            )?;
        }
        for input in context.basis.durable_resize_inputs() {
            remove_value(&mut self.durable_contexts, &input, &scope)?;
        }
        Ok(())
    }

    fn insert_row_indexes(
        &mut self,
        row: &ActivationRow,
        touched: &mut BTreeSet<UiHostMeasurementAuthorityWitness>,
    ) {
        let scope = row.scope();
        let context = row.committed_invalidation_context();
        for (source, _) in context.basis.query_allocation_mappings() {
            insert_value(&mut self.query_contexts, source.clone(), scope.clone());
        }
        for request in context.basis.host_allocation_requests() {
            let Some(result) = context.basis.host_measurement_result(request) else {
                continue;
            };
            let witness = result.authority_witness();
            touched.insert(witness);
            insert_value(&mut self.host_scopes_by_witness, witness, scope.clone());
            insert_value(
                &mut self.host_witnesses_by_request,
                (request, result.evidence_category()),
                witness,
            );
        }
        for input in context.basis.durable_resize_inputs() {
            insert_value(&mut self.durable_contexts, input, scope.clone());
        }
    }

    fn refresh_host_target(&mut self, witness: UiHostMeasurementAuthorityWitness) {
        let mut nodes = Vec::new();
        if let Some(scopes) = self.host_scopes_by_witness.get(&witness) {
            for scope in scopes {
                let Some(row) = self.catalog.row(scope) else {
                    continue;
                };
                let basis = &row.committed_invalidation_context().basis;
                for request in basis.host_allocation_requests() {
                    if basis
                        .host_measurement_result(request)
                        .is_some_and(|result| result.authority_witness() == witness)
                    {
                        if let Some(node) = basis.host_allocation_target(request) {
                            nodes.push(node);
                        }
                    }
                }
            }
        }
        nodes.sort_unstable();
        nodes.dedup();
        if let Some(mapping) = super::authority::UiHostInvalidationTargetMapping::seal(
            nodes.into_boxed_slice(),
            &self.graph_replan,
        ) {
            self.host_targets_by_witness.insert(witness, mapping);
        } else {
            self.host_targets_by_witness.remove(&witness);
        }
    }
}

fn insert_value<K: Ord + Clone, V: Ord + Clone>(
    index: &mut UiPersistentOrdMap<K, Box<[V]>>,
    key: K,
    value: V,
) {
    let mut values = index.get(&key).map_or_else(Vec::new, |rows| rows.to_vec());
    if let Err(ordinal) = values.binary_search(&value) {
        values.insert(ordinal, value);
        index.insert(key, values.into_boxed_slice());
    }
}

fn remove_value<K: Ord + Clone, V: Ord + Clone>(
    index: &mut UiPersistentOrdMap<K, Box<[V]>>,
    key: &K,
    value: &V,
) -> Result<(), ()> {
    let existing = index.get(key).ok_or(())?;
    let mut values = existing.to_vec();
    let ordinal = values.binary_search(value).map_err(|_| ())?;
    values.remove(ordinal);
    if values.is_empty() {
        index.remove(key);
    } else {
        index.insert(key.clone(), values.into_boxed_slice());
    }
    Ok(())
}
