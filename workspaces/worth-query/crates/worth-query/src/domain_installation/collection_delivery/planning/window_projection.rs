use crate::domain_installation::{
    WorthQueryBoundCollectionWindow, WorthQueryCollectionContinuation,
    WorthQueryCollectionDeliveryCounters, WorthQueryCollectionPatchOperation,
    WorthQueryCollectionResetCost, WorthQueryCollectionResetReason,
    WorthQueryCollectionWindowParts, WorthQueryImpactClass, WorthQueryOperationContinuationPosture,
};

use super::WorthQueryCollectionConsumerWindow;

pub(super) fn next_window(
    state: &WorthQueryCollectionConsumerWindow,
    rows: Vec<crate::domain_installation::WorthQueryCollectionRowHandle>,
    has_more: bool,
    counters: WorthQueryCollectionDeliveryCounters,
) -> WorthQueryBoundCollectionWindow {
    let continuation = if has_more {
        let cursor = crate::domain_installation::WorthQueryCollectionCursor::mint(
            crate::domain_installation::WorthQueryCollectionCursorParts {
                capability_identity: state.window.capability_identity,
                capability_generation: state.window.capability_generation,
                basis_identity: state.window.basis_identity.clone(),
                ordering_identity: state.window.ordering_identity.clone(),
                next_row_ordinal: state.window.cursor().next_row_ordinal + rows.len(),
            },
        );
        match state.index.continuation_posture() {
            WorthQueryOperationContinuationPosture::SnapshotCursor => {
                WorthQueryCollectionContinuation::SnapshotMore(cursor)
            }
            WorthQueryOperationContinuationPosture::LiveCursor => {
                WorthQueryCollectionContinuation::LiveMore(cursor)
            }
            WorthQueryOperationContinuationPosture::NotRequired => {
                WorthQueryCollectionContinuation::Complete
            }
        }
    } else {
        WorthQueryCollectionContinuation::Complete
    };
    WorthQueryBoundCollectionWindow::from_parts(WorthQueryCollectionWindowParts {
        capability_identity: state.window.capability_identity,
        capability_generation: state.window.capability_generation,
        source_identity: state.window.source_identity.clone(),
        binding_identity: state.window.binding_identity.clone(),
        result_shape_identity: state.window.result_shape_identity.clone(),
        collection_delivery_contract_identity: state
            .window
            .collection_delivery_contract_identity
            .clone(),
        window_contract_identity: state.window.window_contract_identity.clone(),
        basis_identity: state.window.basis_identity.clone(),
        ordering_identity: state.window.ordering_identity.clone(),
        admitted_width: state.window.admitted_width(),
        cursor: state.window.cursor().clone(),
        rows,
        continuation,
        result_state: state.window.result_state(),
        warnings: state.window.warnings().to_vec(),
        counters: crate::domain_installation::WorthQueryCollectionWindowCounters {
            ordered_index_probes: 1,
            rows_visited: counters.fresh_window_rows_visited,
            window_rows_materialized: counters.fresh_window_rows_visited,
            ..Default::default()
        },
    })
}

pub(super) fn reset_for(
    impact: WorthQueryImpactClass,
    width: usize,
) -> Option<WorthQueryCollectionPatchOperation> {
    let reason = match impact {
        WorthQueryImpactClass::Reexecute => WorthQueryCollectionResetReason::ReexecutionRequired,
        WorthQueryImpactClass::ExplicitRebind => {
            WorthQueryCollectionResetReason::CapabilityRebindRequired
        }
        WorthQueryImpactClass::Replacement => WorthQueryCollectionResetReason::ReplacementRequired,
        WorthQueryImpactClass::Retirement => WorthQueryCollectionResetReason::RetirementRequired,
        WorthQueryImpactClass::UnsupportedEscalation => {
            WorthQueryCollectionResetReason::UnsupportedIncrementalMeaning
        }
        _ => return None,
    };
    Some(WorthQueryCollectionPatchOperation::ResetRequired {
        reason,
        cost: WorthQueryCollectionResetCost {
            fresh_execution_required: true,
            maximum_replacement_rows: width,
        },
    })
}
