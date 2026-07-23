use std::collections::{BTreeMap, BTreeSet};

use crate::domain_installation::{
    WorthQueryBoundCollectionWindow, WorthQueryCollectionDeliveryCounters,
    WorthQueryCollectionPatchOperation, WorthQueryImpactClass,
};
use crate::memory_workspace::WorthQueryEntityIdentity;

use super::reset_for;

pub(super) struct OperationDiff<'a> {
    pub prior: &'a WorthQueryBoundCollectionWindow,
    pub next: &'a WorthQueryBoundCollectionWindow,
    pub impact: WorthQueryImpactClass,
    pub affected: &'a BTreeSet<WorthQueryEntityIdentity>,
}

pub(super) fn operations_for(
    diff: OperationDiff<'_>,
    counters: &mut WorthQueryCollectionDeliveryCounters,
) -> Vec<WorthQueryCollectionPatchOperation> {
    if let Some(reset) = reset_for(diff.impact, diff.next.rows().len()) {
        return vec![reset];
    }
    if diff.impact == WorthQueryImpactClass::UnaffectedOrSuppressed {
        return Vec::new();
    }
    let mut operations = row_operations(&diff, counters);
    append_window_operations(&diff, &mut operations);
    operations
}

fn row_operations(
    diff: &OperationDiff<'_>,
    counters: &mut WorthQueryCollectionDeliveryCounters,
) -> Vec<WorthQueryCollectionPatchOperation> {
    let prior_positions = positions(diff.prior, &mut counters.prior_window_rows_visited);
    let next_positions = positions(diff.next, &mut counters.fresh_window_rows_visited);
    let mut operations = Vec::new();
    for (entity, from) in &prior_positions {
        if !next_positions.contains_key(entity) {
            operations.push(WorthQueryCollectionPatchOperation::Remove {
                entity: entity.clone(),
                from: *from,
            });
        }
    }
    for (to, row) in diff.next.rows().iter().enumerate() {
        counters.affected_identity_lookups += 1;
        match prior_positions.get(row.entity_identity()) {
            None => operations.push(WorthQueryCollectionPatchOperation::Insert {
                row: row.clone(),
                at: to,
            }),
            Some(from) if *from != to => {
                operations.push(WorthQueryCollectionPatchOperation::Move {
                    row: row.clone(),
                    from: *from,
                    to,
                })
            }
            Some(_) if diff.affected.contains(row.entity_identity()) => {
                operations.push(WorthQueryCollectionPatchOperation::Update { row: row.clone() })
            }
            Some(_) => {}
        }
    }
    operations
}

fn append_window_operations(
    diff: &OperationDiff<'_>,
    operations: &mut Vec<WorthQueryCollectionPatchOperation>,
) {
    let anchored_window_shifted = !diff.prior.cursor().is_beginning()
        && diff.prior.rows().first().map(|row| row.entity_identity())
            != diff.next.rows().first().map(|row| row.entity_identity());
    if anchored_window_shifted {
        operations.push(WorthQueryCollectionPatchOperation::WindowShift {
            first_row: diff
                .next
                .rows()
                .first()
                .map(|row| row.entity_identity().clone()),
        });
    }
    if diff.prior.result_state() != diff.next.result_state() {
        operations.push(WorthQueryCollectionPatchOperation::ResultState {
            state: diff.next.result_state(),
        });
    }
    if diff.prior.warnings() != diff.next.warnings() {
        operations.push(WorthQueryCollectionPatchOperation::Warnings {
            warnings: diff.next.warnings().to_vec(),
        });
    }
    if diff.prior.continuation() != diff.next.continuation() {
        operations.push(WorthQueryCollectionPatchOperation::Continuation {
            continuation: diff.next.continuation().clone(),
        });
    }
}

fn positions(
    window: &WorthQueryBoundCollectionWindow,
    visits: &mut usize,
) -> BTreeMap<WorthQueryEntityIdentity, usize> {
    window
        .rows()
        .iter()
        .enumerate()
        .map(|(index, row)| {
            *visits += 1;
            (row.entity_identity().clone(), index)
        })
        .collect()
}
