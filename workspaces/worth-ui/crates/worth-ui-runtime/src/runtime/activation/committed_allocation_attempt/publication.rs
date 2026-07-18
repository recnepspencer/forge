use super::UiCommittedAllocationValidation;
use crate::runtime::activation::committed_allocation_attempt::UiCommittedAllocationPreflightDenial;
use crate::runtime::WorthUiRuntime;
use crate::runtime::{WorthUiExecutionPlan, WorthUiFrameBoundary, WorthUiPlanSwapReceipt};

pub(super) fn publish_validated_committed_allocation(
    runtime: &mut WorthUiRuntime,
    mut ready_activation: UiCommittedAllocationValidation,
    candidate_plan: WorthUiExecutionPlan,
    boundary: WorthUiFrameBoundary,
) -> Result<WorthUiPlanSwapReceipt, crate::runtime::UiCommittedAllocationActivationDenial> {
    let attempt_identity = ready_activation.attempt_identity().clone();
    let runtime_frame_epoch = runtime.frame_epoch();
    ready_activation.record_graph_predecessor_check()?;
    ready_activation.record_scroll_binding_check()?;
    let prepared_catalog_transition = {
        let authority = runtime
            .allocation_invalidation_index
            .try_borrow()
            .map_err(|_| {
                crate::runtime::UiCommittedAllocationActivationDenial::preparation(
                    attempt_identity.clone(),
                    ready_activation.activation_counters(),
                    crate::runtime::UiCommittedAllocationActivationDenialReason::CommitResourceUnavailable,
                )
            })?;
        authority.prepare_catalog_transition(ready_activation.allocation_catalog_transition())
    };
    let prepared_catalog_transition = match prepared_catalog_transition {
            Ok(prepared) => prepared,
            Err(crate::runtime::invalidation_narrowing::UiAllocationNeighborhoodActivationDenial::ScrollBinding(denial)) => {
                return Err(crate::runtime::UiCommittedAllocationActivationDenial::preparation(
                    attempt_identity,
                    ready_activation.activation_counters(),
                    crate::runtime::UiCommittedAllocationActivationDenialReason::ScrollBinding(denial),
                ));
            }
            Err(crate::runtime::invalidation_narrowing::UiAllocationNeighborhoodActivationDenial::PortalBinding(denial)) => {
                return Err(crate::runtime::UiCommittedAllocationActivationDenial::preparation(
                    attempt_identity,
                    ready_activation.activation_counters(),
                    crate::runtime::UiCommittedAllocationActivationDenialReason::PortalBinding(denial),
                ));
            }
            Err(crate::runtime::invalidation_narrowing::UiAllocationNeighborhoodActivationDenial::StalePredecessor) => {
                return Err(crate::runtime::UiCommittedAllocationActivationDenial::preparation(
                    attempt_identity,
                    ready_activation.activation_counters(),
                    crate::runtime::UiCommittedAllocationActivationDenialReason::GraphPredecessorMismatch,
                ));
            }
        };
    let scroll_catalog_evidence = prepared_catalog_transition.scroll_catalog_evidence();
    ready_activation.record_frame_boundary_check()?;
    ready_activation.record_ledger_predecessor_check()?;
    let mut activation_counters = ready_activation.activation_counters();
    let preflight = super::preflight::preflight_committed_allocation(
            &runtime.active,
            ready_activation,
            candidate_plan,
            boundary,
            runtime_frame_epoch,
        )
        .map_err(|denial| {
            let reason = match denial {
                UiCommittedAllocationPreflightDenial::ActivationGate { denial, counters } => {
                    activation_counters = *counters;
                    crate::runtime::UiCommittedAllocationActivationDenialReason::FrameBoundary(
                        *denial,
                    )
                }
                UiCommittedAllocationPreflightDenial::CandidatePlanDigestMismatch { counters } => {
                    activation_counters = *counters;
                    crate::runtime::UiCommittedAllocationActivationDenialReason::CandidatePlanDigestMismatch
                }
                UiCommittedAllocationPreflightDenial::LedgerCommittedOutcomeMismatch { counters } => {
                    activation_counters = *counters;
                    crate::runtime::UiCommittedAllocationActivationDenialReason::LedgerCommittedOutcomeMismatch
                }
                UiCommittedAllocationPreflightDenial::CounterExhausted { counters, exhaustion } => {
                    return crate::runtime::UiCommittedAllocationActivationDenial::counter_exhausted(
                        attempt_identity.clone(),
                        *counters,
                        exhaustion,
                    );
                }
            };
            crate::runtime::UiCommittedAllocationActivationDenial::preparation(
                attempt_identity.clone(),
                activation_counters,
                reason,
            )
        })?;
    if let Err(exhaustion) = activation_counters.record_frame_replacement_check() {
        return Err(
            crate::runtime::UiCommittedAllocationActivationDenial::counter_exhausted(
                attempt_identity,
                activation_counters,
                exhaustion,
            ),
        );
    }
    let invalidation = match runtime.allocation_invalidation_index.try_borrow_mut() {
        Ok(invalidation) => invalidation,
        Err(_) => {
            return Err(crate::runtime::UiCommittedAllocationActivationDenial::preparation(
                    attempt_identity,
                    activation_counters,
                    crate::runtime::UiCommittedAllocationActivationDenialReason::CommitResourceUnavailable,
                ));
        }
    };
    let truth_resources = match preflight
        .acquire_truth_resources(&runtime.allocation_receipt_ledger, invalidation)
    {
        Ok(resources) => resources,
        Err(_) => {
            return Err(crate::runtime::UiCommittedAllocationActivationDenial::preparation(
                    attempt_identity,
                    activation_counters,
                    crate::runtime::UiCommittedAllocationActivationDenialReason::LedgerPredecessorMismatch,
                ));
        }
    };
    let frame_commit =
        match runtime
            .allocation_frame_scheduler
            .prepare_replacement_commit()
        {
            Ok(commit) => commit,
            Err(denial) => return Err(
                crate::runtime::UiCommittedAllocationActivationDenial::preparation(
                    attempt_identity,
                    activation_counters,
                    crate::runtime::UiCommittedAllocationActivationDenialReason::FrameReplacement(
                        denial,
                    ),
                ),
            ),
        };
    let (prepared, ledger_commit, invalidation) =
        truth_resources.seal(scroll_catalog_evidence, prepared_catalog_transition);
    Ok(prepared
        .bind_commit_resources(super::UiCommittedAllocationCommitResources {
            ledger_commit,
            invalidation,
            frame_commit,
            active: &mut runtime.active,
            last_valid: &mut runtime.last_valid,
            transient_interaction_admission: &mut runtime.transient_interaction_admission,
            durable_resize_source: &mut runtime.durable_resize_source,
        })
        .commit_once())
}
