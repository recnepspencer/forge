use worth_query::facade::foundation::WorthQueryEntityIdentity;
use worth_query::facade::installed::collection::{
    WorthQueryCollectionContinuation, WorthQueryCollectionPatchApplicationReceipt,
    WorthQueryCollectionPatchOperation, WorthQueryCollectionRowHandle,
    WorthQueryCollectionWindowWarning,
};
use worth_query::facade::installed::operation::WorthQueryOperationResultState;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiCollectionAllocationPolicy {
    PreserveMounted,
    ReleaseOutsideWindow,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiCollectionGraphMutation {
    Insert {
        row: WorthQueryCollectionRowHandle,
        at: usize,
    },
    Remove {
        row: WorthQueryEntityIdentity,
    },
    Move {
        row: WorthQueryCollectionRowHandle,
        to: usize,
    },
    Update {
        row: WorthQueryCollectionRowHandle,
    },
    ResultState {
        state: WorthQueryOperationResultState,
    },
    Warnings {
        warnings: Vec<WorthQueryCollectionWindowWarning>,
    },
    Continuation {
        continuation: WorthQueryCollectionContinuation,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiCollectionMeasurementInvalidation {
    Row(WorthQueryEntityIdentity),
    EntireWindow,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct WorthUiCollectionPatchConsequences {
    graph_mutations: Vec<WorthUiCollectionGraphMutation>,
    measurement_invalidation_plan: Vec<WorthUiCollectionMeasurementInvalidation>,
    graph_row_insertions: usize,
    graph_row_removals: usize,
    graph_row_moves: usize,
    graph_value_touches: usize,
    native_fact_touches: usize,
    measurement_invalidations: usize,
    virtualization_shifts: usize,
    mounted_identity_preservations: usize,
    reset_required: bool,
    maintenance_ordinal: u64,
}

impl WorthUiCollectionPatchConsequences {
    pub fn from_query_receipt(
        receipt: &WorthQueryCollectionPatchApplicationReceipt,
        allocation: WorthUiCollectionAllocationPolicy,
    ) -> Self {
        let mut consequences = Self {
            maintenance_ordinal: receipt.maintenance_ordinal(),
            native_fact_touches: receipt.facts().len(),
            ..Self::default()
        };
        for operation in receipt.operations() {
            consequences.apply(operation, allocation);
        }
        consequences
    }

    fn apply(
        &mut self,
        operation: &WorthQueryCollectionPatchOperation,
        allocation: WorthUiCollectionAllocationPolicy,
    ) {
        match operation {
            WorthQueryCollectionPatchOperation::Insert { row, at } => self.insert(row, *at),
            WorthQueryCollectionPatchOperation::Remove { entity, .. } => self.remove(entity),
            WorthQueryCollectionPatchOperation::Move { row, to, .. } => self.move_row(row, *to),
            WorthQueryCollectionPatchOperation::Update { row } => self.update(row),
            WorthQueryCollectionPatchOperation::WindowShift { .. } => self.shift_window(allocation),
            WorthQueryCollectionPatchOperation::ResultState { state } => {
                self.update_result_state(*state)
            }
            WorthQueryCollectionPatchOperation::Warnings { warnings } => {
                self.update_warnings(warnings)
            }
            WorthQueryCollectionPatchOperation::Continuation { continuation } => {
                self.update_continuation(continuation)
            }
            WorthQueryCollectionPatchOperation::ResetRequired { .. } => self.require_reset(),
        }
    }

    fn insert(&mut self, row: &WorthQueryCollectionRowHandle, at: usize) {
        self.graph_mutations
            .push(WorthUiCollectionGraphMutation::Insert {
                row: row.clone(),
                at,
            });
        self.invalidate_row_measurement(row.entity_identity().clone());
        self.graph_row_insertions += 1;
    }

    fn remove(&mut self, entity: &WorthQueryEntityIdentity) {
        self.graph_mutations
            .push(WorthUiCollectionGraphMutation::Remove {
                row: entity.clone(),
            });
        self.invalidate_row_measurement(entity.clone());
        self.graph_row_removals += 1;
    }

    fn move_row(&mut self, row: &WorthQueryCollectionRowHandle, to: usize) {
        self.graph_mutations
            .push(WorthUiCollectionGraphMutation::Move {
                row: row.clone(),
                to,
            });
        self.graph_row_moves += 1;
        self.mounted_identity_preservations += 1;
    }

    fn update(&mut self, row: &WorthQueryCollectionRowHandle) {
        self.graph_mutations
            .push(WorthUiCollectionGraphMutation::Update { row: row.clone() });
        self.invalidate_row_measurement(row.entity_identity().clone());
        self.graph_value_touches += 1;
        self.mounted_identity_preservations += 1;
    }

    fn shift_window(&mut self, allocation: WorthUiCollectionAllocationPolicy) {
        self.virtualization_shifts += 1;
        if allocation == WorthUiCollectionAllocationPolicy::PreserveMounted {
            self.mounted_identity_preservations += 1;
        }
    }

    fn update_result_state(&mut self, state: WorthQueryOperationResultState) {
        self.graph_mutations
            .push(WorthUiCollectionGraphMutation::ResultState { state });
        self.graph_value_touches += 1;
    }

    fn update_warnings(&mut self, warnings: &[WorthQueryCollectionWindowWarning]) {
        self.graph_mutations
            .push(WorthUiCollectionGraphMutation::Warnings {
                warnings: warnings.to_vec(),
            });
        self.graph_value_touches += 1;
    }

    fn update_continuation(&mut self, continuation: &WorthQueryCollectionContinuation) {
        self.graph_mutations
            .push(WorthUiCollectionGraphMutation::Continuation {
                continuation: continuation.clone(),
            });
        self.graph_value_touches += 1;
    }

    fn require_reset(&mut self) {
        self.measurement_invalidation_plan
            .push(WorthUiCollectionMeasurementInvalidation::EntireWindow);
        self.reset_required = true;
        self.measurement_invalidations += 1;
    }

    fn invalidate_row_measurement(&mut self, row: WorthQueryEntityIdentity) {
        self.measurement_invalidation_plan
            .push(WorthUiCollectionMeasurementInvalidation::Row(row));
        self.measurement_invalidations += 1;
    }

    pub fn graph_mutations(&self) -> &[WorthUiCollectionGraphMutation] {
        &self.graph_mutations
    }

    pub fn measurement_invalidation_plan(&self) -> &[WorthUiCollectionMeasurementInvalidation] {
        &self.measurement_invalidation_plan
    }

    pub const fn graph_row_insertions(&self) -> usize {
        self.graph_row_insertions
    }

    pub const fn graph_row_removals(&self) -> usize {
        self.graph_row_removals
    }

    pub const fn graph_row_moves(&self) -> usize {
        self.graph_row_moves
    }

    pub const fn graph_value_touches(&self) -> usize {
        self.graph_value_touches
    }

    pub const fn native_fact_touches(&self) -> usize {
        self.native_fact_touches
    }

    pub const fn measurement_invalidations(&self) -> usize {
        self.measurement_invalidations
    }

    pub const fn virtualization_shifts(&self) -> usize {
        self.virtualization_shifts
    }

    pub const fn mounted_identity_preservations(&self) -> usize {
        self.mounted_identity_preservations
    }

    pub const fn reset_required(&self) -> bool {
        self.reset_required
    }

    pub const fn maintenance_ordinal(&self) -> u64 {
        self.maintenance_ordinal
    }
}
