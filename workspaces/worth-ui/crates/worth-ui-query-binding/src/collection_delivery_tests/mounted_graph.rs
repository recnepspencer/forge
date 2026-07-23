use worth_query::facade::domain::{WorthQueryBoundCollectionWindow, WorthQueryCollectionRowHandle};

use crate::{WorthUiCollectionGraphMutation, WorthUiCollectionPatchConsequences};

pub(super) struct MountedUiCollection {
    rows: Vec<WorthQueryCollectionRowHandle>,
    result_state: worth_query::facade::domain::WorthQueryOperationResultState,
    warnings: Vec<worth_query::facade::domain::WorthQueryCollectionWindowWarning>,
    continuation: worth_query::facade::domain::WorthQueryCollectionContinuation,
}

impl MountedUiCollection {
    pub(super) fn from_query_window(window: &WorthQueryBoundCollectionWindow) -> Self {
        Self {
            rows: window.rows().to_vec(),
            result_state: window.result_state(),
            warnings: window.warnings().to_vec(),
            continuation: window.continuation().clone(),
        }
    }

    pub(super) fn apply(&mut self, consequences: &WorthUiCollectionPatchConsequences) {
        for mutation in consequences.graph_mutations() {
            match mutation {
                WorthUiCollectionGraphMutation::Insert { row, at }
                | WorthUiCollectionGraphMutation::Move { row, to: at } => {
                    self.remove(row);
                    self.rows.insert((*at).min(self.rows.len()), row.clone());
                }
                WorthUiCollectionGraphMutation::Remove { row } => {
                    if let Some(slot) = self
                        .rows
                        .iter()
                        .position(|candidate| candidate.entity_identity() == row)
                    {
                        self.rows.remove(slot);
                    }
                }
                WorthUiCollectionGraphMutation::Update { row } => {
                    let slot = self
                        .rows
                        .iter()
                        .position(|candidate| candidate.entity_identity() == row.entity_identity())
                        .expect("mounted UI update must target an existing row");
                    self.rows[slot] = row.clone();
                }
                WorthUiCollectionGraphMutation::ResultState { state } => {
                    self.result_state = *state;
                }
                WorthUiCollectionGraphMutation::Warnings { warnings } => {
                    self.warnings = warnings.clone();
                }
                WorthUiCollectionGraphMutation::Continuation { continuation } => {
                    self.continuation = continuation.clone();
                }
            }
        }
    }

    fn remove(&mut self, row: &WorthQueryCollectionRowHandle) {
        if let Some(slot) = self
            .rows
            .iter()
            .position(|candidate| candidate.entity_identity() == row.entity_identity())
        {
            self.rows.remove(slot);
        }
    }

    pub(super) fn assert_fresh_parity(&self, fresh: &WorthQueryBoundCollectionWindow) {
        assert_eq!(self.rows.len(), fresh.rows().len());
        for (mounted, fresh) in self.rows.iter().zip(fresh.rows()) {
            assert_eq!(mounted.entity_identity(), fresh.entity_identity());
            assert_eq!(mounted.view_local_identity(), fresh.view_local_identity());
        }
        assert_eq!(self.result_state, fresh.result_state());
        assert_eq!(self.warnings, fresh.warnings());
        assert_eq!(self.continuation, *fresh.continuation());
    }
}
