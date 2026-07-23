use super::{WorthUiActiveApplicationSession, WorthUiActiveApplicationSessionIdentity, WorthUiApp};
use crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationIdentity;

#[cfg(test)]
#[path = "application_replacement_exact_authority_tests.rs"]
mod application_replacement_exact_authority_tests;

mod basis;
mod candidate;
mod publication_observation;
mod receipt;
mod retry;

pub use candidate::{WorthUiReplacementCandidateSummary, WorthUiReplacementPlannedCostEnvelope};
pub use publication_observation::WorthUiApplicationPublicationObservation;

pub struct WorthUiPreparedApplicationReplacement {
    next_app: WorthUiApp,
    admitted: crate::runtime::WorthUiAdmittedReplacementCandidate,
    basis: WorthUiPreparedApplicationReplacementBasis,
    candidate_query_binding: worth_ui_query_binding::WorthUiRuntimeQueryBinding,
    candidate_graph_changed_nodes: std::collections::BTreeSet<crate::graph::UiGraphNodeIdentity>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiPreparedApplicationReplacementBasis {
    origin_session: WorthUiActiveApplicationSessionIdentity,
    next_generation: WorthUiPreparedApplicationGenerationIdentity,
    candidate_basis: crate::runtime::WorthUiReplacementCandidateBasis,
    graph_authority_identity: crate::graph::UiGraphAuthorityIdentity,
    candidate_application_authority:
        crate::facade::prepared_application_authority::WorthUiPreparedApplicationLoweringAuthority,
}

pub struct WorthUiCandidateInspectionReceipt {
    generation_identity: WorthUiPreparedApplicationGenerationIdentity,
    candidate_basis: crate::runtime::WorthUiReplacementCandidateBasis,
    receipt: crate::facade::inspection_bridge::UiInspectionReceipt,
}

pub struct WorthUiLoweredApplicationReplacement {
    next_app: WorthUiApp,
    lowering: crate::runtime::WorthUiReplacementLoweringReady,
    basis: WorthUiPreparedApplicationReplacementBasis,
    candidate_query_binding: worth_ui_query_binding::WorthUiRuntimeQueryBinding,
    candidate_graph_changed_nodes: std::collections::BTreeSet<crate::graph::UiGraphNodeIdentity>,
    reload_cost_seed: crate::runtime::WorthUiReloadCostSeed,
    active_generation: WorthUiPreparedApplicationGenerationIdentity,
}

pub struct WorthUiPendingApplicationCutover {
    next_app: WorthUiApp,
    pending_activation: crate::runtime::WorthUiPendingActivation,
    basis: WorthUiPreparedApplicationReplacementBasis,
    candidate_query_binding: worth_ui_query_binding::WorthUiRuntimeQueryBinding,
    candidate_graph_changed_nodes: std::collections::BTreeSet<crate::graph::UiGraphNodeIdentity>,
    reload_cost_seed: crate::runtime::WorthUiReloadCostSeed,
}

/// Candidate ownership returned when a transient frame boundary cannot admit
/// publication yet.
#[must_use = "a denied frame-boundary cutover remains retryable"]
pub struct WorthUiApplicationCutoverRetry {
    pending: WorthUiPendingApplicationCutover,
    admitted_delta: crate::graph::UiAdmittedAllocationCatalogDelta,
    lane_parity_report: Option<crate::runtime::WorthUiLaneParityReport>,
}

#[must_use = "application cutover receipts may carry Query resources requiring explicit retirement"]
pub struct WorthUiApplicationCutoverReceipt {
    prior_generation: WorthUiPreparedApplicationGenerationIdentity,
    active_generation: WorthUiPreparedApplicationGenerationIdentity,
    plan_swap: crate::runtime::WorthUiPlanSwapReceipt,
    plan_decision: crate::runtime::WorthUiExecutablePlanDecision,
    query_retirement:
        worth_ui_query_binding::compatibility::managed_live::WorthUiQueryLiveRetirement,
    allocation_catalog_successor: crate::runtime::UiAllocationCatalogSuccessorReceipt,
    publication: WorthUiApplicationPublicationObservation,
    reload_cost: Result<
        crate::runtime::WorthUiReloadLoweringCounterReceipt,
        crate::runtime::WorthUiReloadCounterBoundaryDenial,
    >,
}

pub struct WorthUiApplicationSemanticNoOpReceipt {
    receipt: crate::runtime::WorthUiSemanticNoOpReceipt,
    reload_cost: Result<
        crate::runtime::WorthUiReloadLoweringCounterReceipt,
        crate::runtime::WorthUiReloadCounterBoundaryDenial,
    >,
}

#[must_use = "replacement outcomes distinguish authority-preserving no-op from publication"]
pub enum WorthUiApplicationReplacementOutcome {
    SemanticNoOp(Box<WorthUiApplicationSemanticNoOpReceipt>),
    Activated(Box<WorthUiApplicationCutoverReceipt>),
}

#[derive(Debug)]
pub enum WorthUiApplicationReplacementPreparationDenial {
    Preparation(crate::facade::lifecycle::WorthUiApplicationPreparationDenial),
    Admission(crate::runtime::WorthUiCandidateAdmissionReport),
    PreparedApplicationBindingMismatch,
}

#[derive(Debug)]
pub enum WorthUiApplicationReplacementLoweringDenial {
    ForeignActiveApplicationSession,
    Lowering(crate::runtime::WorthUiReplacementLoweringDenial),
}

#[derive(Debug)]
pub enum WorthUiApplicationReplacementStagingDenial {
    ForeignActiveApplicationSession,
    Staging(crate::runtime::WorthUiActivationStagingDenial),
}

#[derive(Debug)]
pub enum WorthUiApplicationCutoverDenial {
    ForeignActiveApplicationSession,
    PreparedApplicationGraphMismatch,
    PreparedApplicationAuthorityMismatch,
    FrameBoundaryUnavailable {
        reason: crate::runtime::WorthUiActivationGateDenialReason,
        retry: Box<WorthUiApplicationCutoverRetry>,
    },
    Activation(crate::runtime::WorthUiAllocationCatalogActivationDenial),
}

impl WorthUiActiveApplicationSession {
    pub fn prepare_replacement(
        &self,
        submission: crate::runtime::WorthUiWatchedCandidateSubmission,
    ) -> Result<
        Box<WorthUiPreparedApplicationReplacement>,
        WorthUiApplicationReplacementPreparationDenial,
    > {
        let (next_authority, candidate) =
            crate::facade::lifecycle::prepare_successor_application_authority(
                self.app.prepared_authority(),
                submission,
            )
            .map_err(WorthUiApplicationReplacementPreparationDenial::Preparation)?;
        let admitted = crate::runtime::WorthUiCandidateAdmission::for_active_basis(
            self.runtime.replacement_admission_basis(),
        )
        .admit(candidate)
        .map_err(WorthUiApplicationReplacementPreparationDenial::Admission)?;
        let next_app = WorthUiApp::from_prepared_authority(next_authority);
        let Some(basis) = WorthUiPreparedApplicationReplacementBasis::bind(
            self.session_identity(),
            &next_app,
            &admitted,
        ) else {
            return Err(
                WorthUiApplicationReplacementPreparationDenial::PreparedApplicationBindingMismatch,
            );
        };
        Ok(Box::new(WorthUiPreparedApplicationReplacement {
            candidate_query_binding: next_app
                .prepared_authority()
                .query_binding_plan()
                .prepare_downstream_state(),
            next_app,
            admitted,
            basis,
            candidate_graph_changed_nodes: Default::default(),
        }))
    }

    pub fn lower_prepared_replacement(
        &self,
        prepared: WorthUiPreparedApplicationReplacement,
    ) -> Result<WorthUiLoweredApplicationReplacement, WorthUiApplicationReplacementLoweringDenial>
    {
        self.lower_prepared_replacement_with_state_hooks(prepared, |inventory| inventory)
    }

    pub fn lower_prepared_replacement_with_state_hooks(
        &self,
        prepared: WorthUiPreparedApplicationReplacement,
        configure: impl FnOnce(
            crate::runtime::WorthUiDurableStateInventoryBuilder,
        ) -> crate::runtime::WorthUiDurableStateInventoryBuilder,
    ) -> Result<WorthUiLoweredApplicationReplacement, WorthUiApplicationReplacementLoweringDenial>
    {
        if !prepared.basis.admits_session(self.session_identity()) {
            return Err(
                WorthUiApplicationReplacementLoweringDenial::ForeignActiveApplicationSession,
            );
        }
        let candidate_application_authority =
            prepared.next_app.prepared_authority().lowering_authority();
        let lowering = self
            .runtime
            .prepare_application_replacement_lowering(
                prepared.admitted,
                candidate_application_authority,
                &prepared.candidate_query_binding,
                configure,
            )
            .map_err(WorthUiApplicationReplacementLoweringDenial::Lowering)?;
        let reload_cost_seed = lowering.reload_cost_seed();
        Ok(WorthUiLoweredApplicationReplacement {
            next_app: prepared.next_app,
            lowering,
            basis: prepared.basis,
            candidate_query_binding: prepared.candidate_query_binding,
            candidate_graph_changed_nodes: prepared.candidate_graph_changed_nodes,
            reload_cost_seed,
            active_generation: self.generation_identity().clone(),
        })
    }

    pub fn stage_prepared_replacement(
        &self,
        lowered: WorthUiLoweredApplicationReplacement,
    ) -> Result<WorthUiPendingApplicationCutover, WorthUiApplicationReplacementStagingDenial> {
        if !lowered.basis.admits_session(self.session_identity()) {
            return Err(
                WorthUiApplicationReplacementStagingDenial::ForeignActiveApplicationSession,
            );
        }
        let pending_activation = self
            .runtime
            .stage_replacement_activation_from_lowering(lowered.lowering)
            .map_err(WorthUiApplicationReplacementStagingDenial::Staging)?;
        Ok(WorthUiPendingApplicationCutover {
            next_app: lowered.next_app,
            pending_activation,
            basis: lowered.basis,
            candidate_query_binding: lowered.candidate_query_binding,
            candidate_graph_changed_nodes: lowered.candidate_graph_changed_nodes,
            reload_cost_seed: lowered.reload_cost_seed,
        })
    }

    pub fn activate_prepared_replacement(
        &mut self,
        pending: WorthUiPendingApplicationCutover,
        admitted_delta: crate::graph::UiAdmittedAllocationCatalogDelta,
        boundary: crate::runtime::WorthUiFrameBoundary,
        lane_parity_report: Option<crate::runtime::WorthUiLaneParityReport>,
    ) -> Result<WorthUiApplicationReplacementOutcome, WorthUiApplicationCutoverDenial> {
        if !pending.basis.admits_session(self.session_identity()) {
            return Err(WorthUiApplicationCutoverDenial::ForeignActiveApplicationSession);
        }
        if let Some(reason) = retryable_boundary_denial(self, &pending, boundary) {
            return Err(WorthUiApplicationCutoverDenial::FrameBoundaryUnavailable {
                reason,
                retry: Box::new(WorthUiApplicationCutoverRetry {
                    pending,
                    admitted_delta,
                    lane_parity_report,
                }),
            });
        }
        let prior_generation = self.generation_identity().clone();
        let reload_cost_seed = pending.reload_cost_seed;
        if !pending.basis.admits_catalog_delta(&admitted_delta) {
            return Err(WorthUiApplicationCutoverDenial::PreparedApplicationGraphMismatch);
        }
        let candidate_application_authority = pending
            .pending_activation
            .candidate_application_authority()
            .clone();
        let active_generation = candidate_application_authority
            .generation_identity()
            .clone();
        debug_assert_eq!(pending.basis.next_generation(), &active_generation);
        debug_assert_eq!(pending.next_app.generation_identity(), &active_generation);
        if !pending
            .basis
            .admits_application_authority(&candidate_application_authority)
        {
            return Err(WorthUiApplicationCutoverDenial::PreparedApplicationAuthorityMismatch);
        }
        let publication = self
            .runtime
            .activate_admitted_allocation_catalog_delta_with_query_binding(
                pending.pending_activation,
                crate::runtime::UiAllocationCatalogDeltaActivationInput {
                    admitted_delta,
                    active_graph: self.app.graph_snapshot().clone(),
                    graph_changed_nodes: pending.candidate_graph_changed_nodes,
                    boundary,
                    lane_parity_report,
                    candidate_query_binding: pending.candidate_query_binding,
                    successor_planning_authority: std::rc::Rc::clone(
                        pending.next_app.retained_planning_authority(),
                    ),
                    application_publication:
                        crate::runtime::WorthUiPreparedApplicationPublication::new(
                            &mut self.app,
                            pending.next_app,
                        ),
                },
            )
            .map_err(WorthUiApplicationCutoverDenial::Activation)?;
        match publication {
            crate::runtime::WorthUiQueryAwarePlanOutcome::SemanticNoOp(receipt) => {
                debug_assert_eq!(receipt.active_generation(), &prior_generation);
                debug_assert_eq!(receipt.candidate_generation(), &active_generation);
                let reload_cost = reload_cost_seed.finish(
                    prior_generation,
                    active_generation,
                    receipt.equivalence().previous_fingerprint(),
                    receipt.candidate_construction(),
                    receipt.equivalence(),
                );
                Ok(WorthUiApplicationReplacementOutcome::SemanticNoOp(
                    Box::new(WorthUiApplicationSemanticNoOpReceipt {
                        receipt: *receipt,
                        reload_cost,
                    }),
                ))
            }
            crate::runtime::WorthUiQueryAwarePlanOutcome::Activated(publication) => {
                let (plan_swap, query_retirement, plan_decision, allocation_catalog_successor) =
                    publication.into_parts();
                let publication = WorthUiApplicationPublicationObservation::capture(
                    &self.app,
                    &self.runtime,
                    &self.host_session,
                );
                let reload_cost = reload_cost_seed.finish(
                    prior_generation.clone(),
                    active_generation.clone(),
                    plan_swap.previous_active_plan_digest(),
                    publication
                        .runtime()
                        .cross_lane_bundle()
                        .construction_counters(),
                    plan_decision
                        .summary()
                        .expect("an activated plan decision carries comparison evidence"),
                );
                Ok(WorthUiApplicationReplacementOutcome::Activated(Box::new(
                    WorthUiApplicationCutoverReceipt {
                        prior_generation,
                        active_generation,
                        plan_swap,
                        plan_decision,
                        query_retirement,
                        allocation_catalog_successor: allocation_catalog_successor
                            .expect("public application replacement always uses a catalog delta"),
                        publication,
                        reload_cost,
                    },
                )))
            }
        }
    }

    /// Lets transaction tests retain the exact production-staged pending
    /// authority while inspecting denial behavior below the public cutover.
    #[cfg(test)]
    pub(crate) fn into_runtime_and_pending_after_staging_for_test(
        self,
        pending: WorthUiPendingApplicationCutover,
    ) -> (
        crate::runtime::WorthUiRuntime,
        crate::runtime::WorthUiPendingActivation,
    ) {
        assert!(pending.basis.admits_session(self.session_identity()));
        (self.runtime, pending.pending_activation)
    }
}

fn retryable_boundary_denial(
    session: &WorthUiActiveApplicationSession,
    pending: &WorthUiPendingApplicationCutover,
    boundary: crate::runtime::WorthUiFrameBoundary,
) -> Option<crate::runtime::WorthUiActivationGateDenialReason> {
    if !boundary.is_safe_to_activate() {
        return Some(crate::runtime::WorthUiActivationGateDenialReason::UnsafeFrameBoundary);
    }
    if boundary.host_session() != session.host_session_identity() {
        return Some(
            crate::runtime::WorthUiActivationGateDenialReason::ForeignFrameBoundarySession,
        );
    }
    let readiness_epoch = pending.pending_activation.frame_epoch();
    if boundary.frame_epoch() < readiness_epoch {
        return Some(crate::runtime::WorthUiActivationGateDenialReason::StaleFrameEpoch);
    }
    if boundary.frame_epoch() > readiness_epoch {
        return Some(crate::runtime::WorthUiActivationGateDenialReason::FutureFrameEpochMismatch);
    }
    (boundary.frame_epoch() != session.runtime.frame_epoch())
        .then_some(crate::runtime::WorthUiActivationGateDenialReason::BoundaryFrameEpochMismatch)
}
