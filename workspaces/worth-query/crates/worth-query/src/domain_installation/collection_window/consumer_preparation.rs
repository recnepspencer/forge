use crate::basis_lifecycle::BasisOperationLane;
use crate::domain_installation::{
    WorthQueryAdmittedCollectionWindow, WorthQueryBoundCollectionWindow,
    WorthQueryCollectionCapabilityCounters, WorthQueryCollectionCapabilityDenial,
    WorthQueryCollectionContinuation, WorthQueryCollectionCursor, WorthQueryCollectionCursorParts,
    WorthQueryCollectionWindowBreadth, WorthQueryCollectionWindowCounters,
    WorthQueryCollectionWindowDenial, WorthQueryCollectionWindowDenialKind,
    WorthQueryCollectionWindowParts, WorthQueryCollectionWindowWarning,
    WorthQueryOperationContinuationPosture, WorthQueryOperationWindowPolicy,
    WorthQuerySettledDomainProjection,
};

use super::capability::prepare_collection_binding;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryCollectionConsumerPreparationDenial {
    Collection(WorthQueryCollectionCapabilityDenial),
    Window(WorthQueryCollectionWindowDenial),
}

impl<D, O, F, L: BasisOperationLane> WorthQuerySettledDomainProjection<D, O, F, L> {
    pub fn prepare_collection_consumer(
        &self,
        breadth: WorthQueryCollectionWindowBreadth,
    ) -> Result<
        crate::domain_installation::WorthQueryCollectionConsumerWindow,
        WorthQueryCollectionConsumerPreparationDenial,
    > {
        let mut collection_counters = WorthQueryCollectionCapabilityCounters::default();
        collection_counters.current_generation_checks += 1;
        if !self.bound_operation().installation_is_current() {
            return Err(WorthQueryCollectionConsumerPreparationDenial::Collection(
                WorthQueryCollectionCapabilityDenial::new(
                    crate::domain_installation::WorthQueryCollectionCapabilityDenialKind::StaleInstallationGeneration,
                    collection_counters,
                ),
            ));
        }
        let prepared =
            prepare_collection_binding(self, &mut collection_counters).map_err(|kind| {
                WorthQueryCollectionConsumerPreparationDenial::Collection(
                    WorthQueryCollectionCapabilityDenial::new(kind, collection_counters),
                )
            })?;
        let mut window_counters = WorthQueryCollectionWindowCounters {
            authority_checks: 1,
            cursor_checks: 3,
            breadth_checks: 1,
            ordered_index_probes: 1,
            ..WorthQueryCollectionWindowCounters::default()
        };
        let beginning = cursor(&prepared, self.consumer_contract().basis_identity(), 0);
        let admitted =
            WorthQueryAdmittedCollectionWindow::mint(beginning, breadth, window_counters);
        let width = admitted.basis().admitted_width;
        let end = collection_window_end(prepared.window_policy, prepared.rows.len(), width)
            .map_err(|kind| {
                WorthQueryCollectionConsumerPreparationDenial::Window(
                    WorthQueryCollectionWindowDenial::new(kind, window_counters),
                )
            })?;
        window_counters.rows_visited += end;
        window_counters.window_rows_materialized += end;
        let continuation = continuation(&prepared, self.consumer_contract().basis_identity(), end);
        let warnings = collection_warnings(self, breadth);
        let window = WorthQueryBoundCollectionWindow::from_parts(WorthQueryCollectionWindowParts {
            capability_identity: prepared.capability_identity,
            capability_generation: prepared.capability_generation,
            source_identity: self.identity().to_owned(),
            binding_identity: self.bound_operation().binding_identity().to_owned(),
            result_shape_identity: prepared.result_shape_identity,
            collection_delivery_contract_identity: prepared.delivery_contract_identity,
            window_contract_identity: admitted.identity().to_owned(),
            basis_identity: self.consumer_contract().basis_identity().to_owned(),
            ordering_identity: prepared.ordering_identity,
            admitted_width: width,
            cursor: admitted.cursor,
            rows: prepared.rows[..end].to_vec(),
            continuation,
            result_state: self.result_state(),
            warnings,
            counters: window_counters,
        });
        Ok(
            crate::domain_installation::WorthQueryCollectionConsumerWindow::from_prepared(
                prepared.maintenance_index,
                window,
            ),
        )
    }
}

fn collection_window_end(
    policy: WorthQueryOperationWindowPolicy,
    row_count: usize,
    admitted_width: usize,
) -> Result<usize, WorthQueryCollectionWindowDenialKind> {
    if policy == WorthQueryOperationWindowPolicy::CompleteCollection {
        if row_count > admitted_width {
            return Err(WorthQueryCollectionWindowDenialKind::CompleteCollectionExceedsBreadth);
        }
        return Ok(row_count);
    }
    Ok(admitted_width.min(row_count))
}

fn continuation(
    prepared: &super::capability::WorthQueryPreparedCollectionBinding,
    basis_identity: &str,
    end: usize,
) -> WorthQueryCollectionContinuation {
    if end >= prepared.rows.len() {
        return WorthQueryCollectionContinuation::Complete;
    }
    let next = cursor(prepared, basis_identity, end);
    match prepared.continuation_posture {
        WorthQueryOperationContinuationPosture::SnapshotCursor => {
            WorthQueryCollectionContinuation::SnapshotMore(next)
        }
        WorthQueryOperationContinuationPosture::LiveCursor => {
            WorthQueryCollectionContinuation::LiveMore(next)
        }
        WorthQueryOperationContinuationPosture::NotRequired => {
            unreachable!("validated complete collections cannot expose continuation")
        }
    }
}

fn cursor(
    prepared: &super::capability::WorthQueryPreparedCollectionBinding,
    basis_identity: &str,
    next_row_ordinal: usize,
) -> WorthQueryCollectionCursor {
    WorthQueryCollectionCursor::mint(WorthQueryCollectionCursorParts {
        capability_identity: prepared.capability_identity,
        capability_generation: prepared.capability_generation,
        basis_identity: basis_identity.to_owned(),
        ordering_identity: prepared.ordering_identity.clone(),
        next_row_ordinal,
    })
}

fn collection_warnings<D, O, F, L: BasisOperationLane>(
    projection: &WorthQuerySettledDomainProjection<D, O, F, L>,
    breadth: WorthQueryCollectionWindowBreadth,
) -> Vec<WorthQueryCollectionWindowWarning> {
    let mut warnings = Vec::new();
    if !projection.warnings().is_empty() {
        warnings.push(
            WorthQueryCollectionWindowWarning::ExecutionWarningsPresent {
                count: projection.warnings().len(),
            },
        );
    }
    if projection.projection_warnings().is_some() {
        warnings.push(WorthQueryCollectionWindowWarning::ProjectionWarningsPresent);
    }
    if breadth.mounting_budget_clamped() {
        warnings.push(WorthQueryCollectionWindowWarning::MountingBudgetClamped);
    }
    warnings
}
