use super::UiCommittedAllocationValidation;
use crate::runtime::activation::committed_allocation_attempt::UiCommittedAllocationPreflightDenial;
use crate::runtime::WorthUiFrameBoundary;
use crate::runtime::WorthUiRuntime;

pub(super) struct UiCommittedAllocationPublicationInput {
    pub candidate_bundle: crate::runtime::active::WorthUiSealedExecutionPlanBundle,
    pub query_succession: worth_ui_query_binding::WorthUiPreparedQueryBindingSuccession,
    pub successor_application_authority:
        crate::facade::prepared_application_authority::WorthUiPreparedApplicationLoweringAuthority,
    pub successor_planning_authority:
        std::rc::Rc<crate::runtime::WorthUiRetainedAllocationPlanningEvidenceRegistry>,
    pub application_publication: Option<crate::runtime::WorthUiPreparedApplicationPublication>,
    pub boundary: WorthUiFrameBoundary,
}

pub(super) fn publish_validated_committed_allocation(
    runtime: &mut WorthUiRuntime,
    mut ready_activation: UiCommittedAllocationValidation,
    input: UiCommittedAllocationPublicationInput,
) -> Result<
    super::prepared::UiPreparedCommittedAllocationActivation,
    crate::runtime::UiCommittedAllocationActivationDenial,
> {
    let UiCommittedAllocationPublicationInput {
        candidate_bundle,
        query_succession,
        successor_application_authority,
        successor_planning_authority,
        application_publication,
        boundary,
    } = input;
    let attempt_identity = ready_activation.attempt_identity().clone();
    let runtime_frame_epoch = runtime.frame_epoch();
    deny_if_interrupted("graph predecessor check", &ready_activation)?;
    ready_activation.record_graph_predecessor_check()?;
    deny_if_interrupted("scroll binding check", &ready_activation)?;
    ready_activation.record_scroll_binding_check()?;
    let prepared_catalog_transition = {
        deny_if_interrupted("catalog transition read", &ready_activation)?;
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
        deny_if_interrupted("catalog transition preparation", &ready_activation)?;
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
            Err(crate::runtime::invalidation_narrowing::UiAllocationNeighborhoodActivationDenial::DerivedIndexDiverged) => {
                return Err(crate::runtime::UiCommittedAllocationActivationDenial::preparation(
                    attempt_identity,
                    ready_activation.activation_counters(),
                    crate::runtime::UiCommittedAllocationActivationDenialReason::DerivedIndexDiverged,
                ));
            }
    };
    let scroll_catalog_evidence = prepared_catalog_transition.scroll_catalog_evidence();
    deny_if_interrupted("frame boundary check", &ready_activation)?;
    ready_activation.record_frame_boundary_check()?;
    deny_if_interrupted("ledger predecessor check", &ready_activation)?;
    ready_activation.record_ledger_predecessor_check()?;
    let mut activation_counters = ready_activation.activation_counters();
    deny_if_interrupted("committed preflight", &ready_activation)?;
    let preflight = super::preflight::preflight_committed_allocation(
            &runtime.active,
            ready_activation,
            candidate_bundle,
            boundary,
            runtime_frame_epoch,
            runtime.host_plan_binding.session_identity(),
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
    activation_counters = preflight.activation_counters();
    deny_after_preflight_if_interrupted(
        "frame replacement check",
        &attempt_identity,
        activation_counters,
    )?;
    if let Err(exhaustion) = activation_counters.record_frame_replacement_check() {
        return Err(
            crate::runtime::UiCommittedAllocationActivationDenial::counter_exhausted(
                attempt_identity,
                activation_counters,
                exhaustion,
            ),
        );
    }
    deny_after_preflight_if_interrupted(
        "invalidation write",
        &attempt_identity,
        activation_counters,
    )?;
    let invalidation_guard = match runtime.allocation_invalidation_index.try_borrow_mut() {
        Ok(invalidation) => invalidation,
        Err(_) => {
            return Err(crate::runtime::UiCommittedAllocationActivationDenial::preparation(
                    attempt_identity,
                    activation_counters,
                    crate::runtime::UiCommittedAllocationActivationDenialReason::CommitResourceUnavailable,
                ));
        }
    };
    drop(invalidation_guard);
    deny_after_preflight_if_interrupted(
        "ledger commit preparation",
        &attempt_identity,
        activation_counters,
    )?;
    let truth_resources = match preflight
        .acquire_truth_resources(&runtime.allocation_receipt_ledger)
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
    deny_after_preflight_if_interrupted(
        "frame commit preparation",
        &attempt_identity,
        activation_counters,
    )?;
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
    let (prepared, ledger_commit) =
        truth_resources.seal(scroll_catalog_evidence, prepared_catalog_transition);
    let last_valid_successor =
        crate::runtime::launch::WorthUiLastValidRuntimeState::record_from_active(&runtime.active);
    Ok(
        prepared.bind_commit_resources(super::UiCommittedAllocationCommitResources {
            ledger_commit,
            frame_commit,
            query_succession,
            successor_application_authority,
            successor_planning_authority,
            application_publication,
            last_valid_successor,
        }),
    )
}

fn deny_if_interrupted(
    stage: &'static str,
    ready: &UiCommittedAllocationValidation,
) -> Result<(), crate::runtime::UiCommittedAllocationActivationDenial> {
    deny_after_preflight_if_interrupted(
        stage,
        ready.attempt_identity(),
        ready.activation_counters(),
    )
}

fn deny_after_preflight_if_interrupted(
    stage: &'static str,
    identity: &super::UiCommittedAllocationActivationIdentity,
    counters: super::UiCommittedAllocationActivationCounters,
) -> Result<(), crate::runtime::UiCommittedAllocationActivationDenial> {
    if crate::runtime::activation::certification_precommit_interruption(stage) {
        Err(
            crate::runtime::UiCommittedAllocationActivationDenial::preparation(
                identity.clone(),
                counters,
                crate::runtime::UiCommittedAllocationActivationDenialReason::CommitResourceUnavailable,
            ),
        )
    } else {
        Ok(())
    }
}
