use worth_query::facade::domain::WorthQueryCollectionResetReason;
use worth_query::facade::installed::{
    collection::{
        WorthQueryCollectionContinuation, WorthQueryCollectionPatchApplicationReceipt,
        WorthQueryCollectionPatchOperation, WorthQueryCollectionWindowWarning,
    },
    operation::WorthQueryOperationResultState,
};

use super::{
    consequence::WorthUiCollectionChangeConsequenceParts, WorthUiCollectionAllocationEffect,
    WorthUiCollectionAllocationPolicy, WorthUiCollectionChangeConsequence,
    WorthUiCollectionChangeCounters, WorthUiCollectionChangeInspection,
    WorthUiCollectionChangeKind, WorthUiCollectionChangeSourceReference,
    WorthUiCollectionContinuationPosture, WorthUiCollectionGraphEffect,
    WorthUiCollectionIncrementalConsequence, WorthUiCollectionMeasurementEffect,
    WorthUiCollectionQueryWorkInspection, WorthUiCollectionResetConsequence,
    WorthUiCollectionResultPosture, WorthUiCollectionRowReference,
};

pub(crate) fn mint_collection_change_consequence(
    installed_reference: crate::WorthUiInstalledQueryBindingReference,
    source: WorthUiCollectionChangeSourceReference,
    change_order: u64,
    allocation_policy: WorthUiCollectionAllocationPolicy,
    receipt: &WorthQueryCollectionPatchApplicationReceipt,
) -> WorthUiCollectionChangeConsequence {
    let mut translator = CollectionChangeTranslator::new(source.clone(), receipt);
    let kind = if receipt.reset_required() {
        translator.translate_reset(receipt)
    } else {
        translator.translate_incremental(receipt, allocation_policy)
    };
    WorthUiCollectionChangeConsequence::new(WorthUiCollectionChangeConsequenceParts {
        installed_reference,
        source,
        change_order,
        kind,
        inspection: translator.inspection,
        ui_counters: translator.counters,
        query_work: WorthUiCollectionQueryWorkInspection::from_query(receipt.counters()),
    })
}

struct CollectionChangeTranslator {
    source: WorthUiCollectionChangeSourceReference,
    inspection: WorthUiCollectionChangeInspection,
    counters: WorthUiCollectionChangeCounters,
}

impl CollectionChangeTranslator {
    fn new(
        source: WorthUiCollectionChangeSourceReference,
        receipt: &WorthQueryCollectionPatchApplicationReceipt,
    ) -> Self {
        Self {
            source,
            inspection: WorthUiCollectionChangeInspection::new(
                receipt.foundational_invalidation().scopes().len(),
            ),
            counters: WorthUiCollectionChangeCounters::default(),
        }
    }

    fn translate_incremental(
        &mut self,
        receipt: &WorthQueryCollectionPatchApplicationReceipt,
        allocation_policy: WorthUiCollectionAllocationPolicy,
    ) -> WorthUiCollectionChangeKind {
        let mut graph = Vec::new();
        let mut measurement = Vec::new();
        let mut allocation = Vec::new();
        for operation in receipt.operations() {
            self.counters.visit_operation();
            self.translate_operation(
                operation,
                allocation_policy,
                &mut graph,
                &mut measurement,
                &mut allocation,
            );
        }
        self.counters.record_reported_facts(receipt.facts().len());
        if !receipt.facts().is_empty() {
            measurement.push(WorthUiCollectionMeasurementEffect::ChangedNativeFacts {
                count: receipt.facts().len(),
            });
            self.counters.mint_measurement_effect();
        }
        WorthUiCollectionChangeKind::Incremental(WorthUiCollectionIncrementalConsequence::new(
            graph,
            measurement,
            allocation,
        ))
    }

    fn translate_reset(
        &mut self,
        receipt: &WorthQueryCollectionPatchApplicationReceipt,
    ) -> WorthUiCollectionChangeKind {
        self.counters.record_reported_facts(receipt.facts().len());
        let reset = receipt.operations().iter().find_map(|operation| {
            self.counters.visit_operation();
            match operation {
                WorthQueryCollectionPatchOperation::ResetRequired { reason, cost } => {
                    Some(WorthUiCollectionResetConsequence::new(
                        map_reset_reason(*reason),
                        cost.fresh_execution_required,
                        cost.maximum_replacement_rows,
                    ))
                }
                other => {
                    self.observe_terminal_operation(other);
                    None
                }
            }
        });
        WorthUiCollectionChangeKind::Reset(
            reset.expect("Query reset receipt contains its typed reset operation"),
        )
    }

    fn translate_operation(
        &mut self,
        operation: &WorthQueryCollectionPatchOperation,
        policy: WorthUiCollectionAllocationPolicy,
        graph: &mut Vec<WorthUiCollectionGraphEffect>,
        measurement: &mut Vec<WorthUiCollectionMeasurementEffect>,
        allocation: &mut Vec<WorthUiCollectionAllocationEffect>,
    ) {
        match operation {
            WorthQueryCollectionPatchOperation::Insert { row: query_row, at } => {
                let row = self.mint_row(query_row.entity_identity());
                graph.push(WorthUiCollectionGraphEffect::Insert {
                    row: row.clone(),
                    at: *at,
                });
                measurement.push(WorthUiCollectionMeasurementEffect::RowChanged(row));
                self.counters.mint_graph_effect();
                self.counters.mint_measurement_effect();
            }
            WorthQueryCollectionPatchOperation::Remove { entity, from } => {
                graph.push(WorthUiCollectionGraphEffect::Remove {
                    row: self.mint_row(entity),
                    from: *from,
                });
                self.counters.mint_graph_effect();
            }
            WorthQueryCollectionPatchOperation::Move {
                row: query_row,
                from,
                to,
            } => {
                let row = self.mint_row(query_row.entity_identity());
                graph.push(WorthUiCollectionGraphEffect::Move {
                    row: row.clone(),
                    from: *from,
                    to: *to,
                });
                allocation.push(WorthUiCollectionAllocationEffect::RowPreservationCandidate(
                    row,
                ));
                self.counters.mint_graph_effect();
                self.counters.mint_allocation_effect();
            }
            WorthQueryCollectionPatchOperation::Update { row: query_row } => {
                let row = self.mint_row(query_row.entity_identity());
                graph.push(WorthUiCollectionGraphEffect::Update { row: row.clone() });
                measurement.push(WorthUiCollectionMeasurementEffect::RowChanged(row.clone()));
                allocation.push(WorthUiCollectionAllocationEffect::RowPreservationCandidate(
                    row,
                ));
                self.counters.mint_graph_effect();
                self.counters.mint_measurement_effect();
                self.counters.mint_allocation_effect();
            }
            WorthQueryCollectionPatchOperation::WindowShift { .. } => {
                allocation.push(WorthUiCollectionAllocationEffect::WindowShift { policy });
                self.counters.mint_allocation_effect();
            }
            WorthQueryCollectionPatchOperation::ResetRequired { .. } => {
                unreachable!("incremental receipt cannot contain a reset operation")
            }
            terminal => self.observe_terminal_operation(terminal),
        }
    }

    fn mint_row(
        &mut self,
        identity: &worth_query::facade::foundation::WorthQueryEntityIdentity,
    ) -> WorthUiCollectionRowReference {
        self.counters.mint_row_reference();
        WorthUiCollectionRowReference::mint(&self.source, identity)
    }

    fn observe_terminal_operation(&mut self, operation: &WorthQueryCollectionPatchOperation) {
        match operation {
            WorthQueryCollectionPatchOperation::ResultState { state } => {
                self.inspection.set_result(map_result(*state));
            }
            WorthQueryCollectionPatchOperation::Warnings { warnings } => {
                for warning in warnings {
                    map_warning(*warning, self.inspection.warnings_mut());
                }
            }
            WorthQueryCollectionPatchOperation::Continuation { continuation } => {
                self.inspection
                    .set_continuation(map_continuation(continuation));
            }
            _ => return,
        }
        self.counters.observe_diagnostic_effect();
    }
}

fn map_result(value: WorthQueryOperationResultState) -> WorthUiCollectionResultPosture {
    match value {
        WorthQueryOperationResultState::Ready => WorthUiCollectionResultPosture::Ready,
        WorthQueryOperationResultState::Advisory => WorthUiCollectionResultPosture::Advisory,
        WorthQueryOperationResultState::Pending => WorthUiCollectionResultPosture::Pending,
        WorthQueryOperationResultState::Partial => WorthUiCollectionResultPosture::Partial,
        WorthQueryOperationResultState::Violation => WorthUiCollectionResultPosture::Violation,
    }
}

fn map_warning(
    value: WorthQueryCollectionWindowWarning,
    target: &mut super::WorthUiCollectionWarningPosture,
) {
    match value {
        WorthQueryCollectionWindowWarning::ExecutionWarningsPresent { count } => {
            target.record_execution_warnings(count);
        }
        WorthQueryCollectionWindowWarning::ProjectionWarningsPresent => {
            target.record_projection_warning();
        }
        WorthQueryCollectionWindowWarning::MountingBudgetClamped => {
            target.record_allocation_budget_clamp();
        }
    }
}

fn map_continuation(
    value: &WorthQueryCollectionContinuation,
) -> WorthUiCollectionContinuationPosture {
    match value {
        WorthQueryCollectionContinuation::Complete => {
            WorthUiCollectionContinuationPosture::Complete
        }
        WorthQueryCollectionContinuation::SnapshotMore(_) => {
            WorthUiCollectionContinuationPosture::AdditionalSnapshotRows
        }
        WorthQueryCollectionContinuation::LiveMore(_) => {
            WorthUiCollectionContinuationPosture::AdditionalLiveRows
        }
    }
}

pub(crate) fn map_reset_reason(
    value: WorthQueryCollectionResetReason,
) -> super::WorthUiCollectionResetReason {
    match value {
        WorthQueryCollectionResetReason::ReexecutionRequired => {
            super::WorthUiCollectionResetReason::ReexecutionRequired
        }
        WorthQueryCollectionResetReason::CapabilityRebindRequired => {
            super::WorthUiCollectionResetReason::CapabilityRebindRequired
        }
        WorthQueryCollectionResetReason::ReplacementRequired => {
            super::WorthUiCollectionResetReason::ReplacementRequired
        }
        WorthQueryCollectionResetReason::RetirementRequired => {
            super::WorthUiCollectionResetReason::RetirementRequired
        }
        WorthQueryCollectionResetReason::UnsupportedIncrementalMeaning => {
            super::WorthUiCollectionResetReason::UnsupportedIncrementalMeaning
        }
        WorthQueryCollectionResetReason::UnappliedPriorPatch => {
            super::WorthUiCollectionResetReason::UnappliedPriorChange
        }
    }
}
