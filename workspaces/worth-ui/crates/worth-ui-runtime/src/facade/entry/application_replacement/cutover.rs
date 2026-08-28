use super::publication_observation::WorthUiApplicationPublicationPreparation;
use super::*;
use crate::facade::WorthUiActiveApplicationSession;

pub(super) struct WorthUiCutoverGenerationBasis {
    pub(super) prior: WorthUiPreparedApplicationGenerationIdentity,
    pub(super) active: WorthUiPreparedApplicationGenerationIdentity,
}

struct WorthUiPreparedCutoverEvidence {
    generations: WorthUiCutoverGenerationBasis,
    visual_trace_source:
        crate::facade::prepared_application_authority::WorthUiPreparedVisualTraceSource,
    reload_cost_seed: crate::runtime::WorthUiReloadCostSeed,
    runtime_basis: crate::runtime::session::WorthUiRuntimePublicationBasis,
    host_session: crate::facade::WorthUiHostSessionIdentity,
    font_collection: std::sync::Arc<worth_ui_text::UiGlobalFontCollection>,
    candidate_graph: crate::graph::UiGraphSnapshot,
    candidate_application_authority:
        crate::facade::prepared_application_authority::WorthUiPreparedApplicationLoweringAuthority,
}

struct WorthUiCutoverPreparationInput {
    pending: WorthUiPendingApplicationCutover,
    admitted_delta: crate::graph::UiAdmittedAllocationCatalogDelta,
    boundary: crate::runtime::WorthUiFrameBoundary,
    lane_parity_report: Option<crate::runtime::WorthUiLaneParityReport>,
}

struct WorthUiPreparedCatalogActivation {
    prepared: crate::runtime::WorthUiPreparedQueryAwarePlanOutcome,
    reload_cost_seed: crate::runtime::WorthUiReloadCostSeed,
    visual_trace_source:
        crate::facade::prepared_application_authority::WorthUiPreparedVisualTraceSource,
    font_collection: std::sync::Arc<worth_ui_text::UiGlobalFontCollection>,
}

impl WorthUiActiveApplicationSession {
    pub fn activate_prepared_replacement(
        &mut self,
        pending: WorthUiPendingApplicationCutover,
        admitted_delta: crate::graph::UiAdmittedAllocationCatalogDelta,
        boundary: crate::runtime::WorthUiFrameBoundary,
        lane_parity_report: Option<crate::runtime::WorthUiLaneParityReport>,
    ) -> Result<WorthUiApplicationReplacementOutcome, WorthUiApplicationCutoverDenial> {
        if !self.mounted.view().surface_bindings().is_empty() {
            return Err(
                WorthUiApplicationCutoverDenial::MountedPresentationRequired {
                    retry: Box::new(WorthUiApplicationCutoverRetry {
                        pending,
                        admitted_delta,
                        lane_parity_report,
                    }),
                },
            );
        }
        let candidate_graph = pending.next_app.graph_snapshot().clone();
        let prepared = self.prepare_application_cutover(
            pending,
            admitted_delta,
            boundary,
            lane_parity_report,
        )?;
        match prepared {
            WorthUiPreparedApplicationCutoverOutcome::SemanticNoOp(receipt) => {
                Ok(WorthUiApplicationReplacementOutcome::SemanticNoOp(receipt))
            }
            WorthUiPreparedApplicationCutoverOutcome::Activation(activation) => {
                let next_mounted = self
                    .mounted
                    .prepare_graph_replacement_successor(crate::graph::UiGraphAuthority::new(
                        &candidate_graph,
                    ))
                    .map_err(WorthUiApplicationCutoverDenial::MountedIdentity)?;
                let scroll = self.prepare_scroll_replacement(&activation, &next_mounted, None);
                let selection = self.prepare_selection_replacement(&next_mounted, false);
                let receipt =
                    self.commit_application_activation(activation, next_mounted, scroll, selection);
                Ok(WorthUiApplicationReplacementOutcome::Activated(Box::new(
                    receipt,
                )))
            }
        }
    }

    pub(super) fn prepare_application_cutover(
        &mut self,
        pending: WorthUiPendingApplicationCutover,
        admitted_delta: crate::graph::UiAdmittedAllocationCatalogDelta,
        boundary: crate::runtime::WorthUiFrameBoundary,
        lane_parity_report: Option<crate::runtime::WorthUiLaneParityReport>,
    ) -> Result<WorthUiPreparedApplicationCutoverOutcome, WorthUiApplicationCutoverDenial> {
        let candidate_graph = pending.next_app.graph_snapshot().clone();
        let candidate_application_authority =
            pending.next_app.prepared_authority().lowering_authority();
        if self.mounted.has_active_presentation_attempt() {
            return Err(WorthUiApplicationCutoverDenial::MountedPresentationInFlight);
        }
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
        let generations = self.validate_cutover_generation_basis(&pending, &admitted_delta)?;
        let prepared_catalog = self.prepare_catalog_activation(WorthUiCutoverPreparationInput {
            pending,
            admitted_delta,
            boundary,
            lane_parity_report,
        })?;
        let evidence = WorthUiPreparedCutoverEvidence {
            generations,
            visual_trace_source: prepared_catalog.visual_trace_source,
            reload_cost_seed: prepared_catalog.reload_cost_seed,
            runtime_basis: self.application.runtime_publication_basis(),
            host_session: self.host_session.identity(),
            font_collection: prepared_catalog.font_collection,
            candidate_graph,
            candidate_application_authority,
        };
        match prepared_catalog.prepared.into_activation() {
            Err(receipt) => Ok(seal_semantic_no_op(evidence, receipt)),
            Ok(activation) => {
                let activation = activation.into_application_activation().map_err(|_| {
                    WorthUiApplicationCutoverDenial::MissingAllocationCatalogSuccessorReceipt
                })?;
                Ok(seal_prepared_activation(evidence, activation))
            }
        }
    }

    fn prepare_catalog_activation(
        &mut self,
        input: WorthUiCutoverPreparationInput,
    ) -> Result<WorthUiPreparedCatalogActivation, WorthUiApplicationCutoverDenial> {
        let pending = input.pending;
        let reload_cost_seed = pending.reload_cost_seed;
        let visual_trace_source = pending.next_app.visual_trace_source();
        let font_collection = std::sync::Arc::clone(pending.next_app.font_collection());
        let successor_planning_authority =
            std::rc::Rc::clone(pending.next_app.retained_planning_authority());
        let application_publication =
            crate::runtime::WorthUiPreparedApplicationPublication::replacement(
                self.application.prepared_authority(),
                pending.next_app,
            );
        let prepared = self
            .application
            .prepare_admitted_allocation_catalog_delta(
                pending.pending_activation,
                crate::runtime::UiAllocationCatalogDeltaActivationInput {
                    admitted_delta: input.admitted_delta,
                    active_graph: self.application.graph_snapshot().clone(),
                    graph_changed_nodes: pending.candidate_graph_changed_nodes,
                    boundary: input.boundary,
                    lane_parity_report: input.lane_parity_report,
                    candidate_query_binding: pending.candidate_query_binding,
                    successor_planning_authority,
                    application_publication,
                },
            )
            .map_err(WorthUiApplicationCutoverDenial::Activation)?;
        Ok(WorthUiPreparedCatalogActivation {
            prepared,
            reload_cost_seed,
            visual_trace_source,
            font_collection,
        })
    }

    pub(super) fn commit_application_activation(
        &mut self,
        mut prepared: Box<WorthUiPreparedApplicationActivation>,
        mounted_successor: crate::mounting::UiMountedGraphReplacementSuccessor,
        scroll: super::scroll_replacement::UiPreparedScrollReplacement,
        selection: super::selection_replacement::UiPreparedSelectionReplacement,
    ) -> WorthUiApplicationCutoverReceipt {
        let transition = prepared
            .transition
            .take()
            .expect("prepared application transition is present");
        let activation = match transition {
            WorthUiApplicationCutoverTransition::Prepared(activation) => activation,
            WorthUiApplicationCutoverTransition::Committed { .. } => {
                unreachable!("prepared application transition cannot already be committed")
            }
        };
        let publication = self.application.commit_application_activation(activation);
        self.intent_application_facts =
            crate::runtime::intent::UiIntentApplicationFactState::activate(
                self.application.intent_application_fact_plan(),
            );
        self.intent_confirmation.cancel_all(
            crate::runtime::intent::UiIntentConfirmationCancellationReason::ApplicationRebound,
        );
        self.intent_admission.cancel_all(
            &mut self.intent_execution,
            crate::runtime::intent::UiIntentAdmissionCancellationReason::ApplicationRebound,
        );
        self.scroll = scroll.into_state();
        self.selection = selection.into_state();
        self.mounted
            .commit_graph_replacement_successor(mounted_successor);
        self.cancel_all_interactions(
            crate::runtime::interaction::UiInteractionLifecycleStopReason::ApplicationRebound,
        );
        let observation_resources = self.application.retire_observation_resources(
            crate::runtime::observation::UiObservationResourceRetirementCause::
                ApplicationReplacement,
        );
        let intent_evidence = self
            .intent_evidence
            .retire(worth_ui_inspection::UiIntentEvidenceRetirementCause::ApplicationReplacement);
        let (plan_swap, query_retirement, plan_decision, allocation_catalog_successor) =
            publication.into_parts();
        prepared.transition = Some(WorthUiApplicationCutoverTransition::Committed {
            plan_swap,
            plan_decision,
            query_retirement,
            allocation_catalog_successor,
        });
        WorthUiApplicationCutoverReceipt {
            transition: prepared,
            observation_resources,
            intent_evidence,
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
        (
            self.application.into_runtime_for_test(),
            pending.pending_activation,
        )
    }
}

impl WorthUiPreparedApplicationActivation {
    pub(super) fn visual_trace_source(
        &self,
    ) -> crate::facade::prepared_application_authority::WorthUiPreparedVisualTraceSource {
        self.visual_trace_source.clone()
    }

    pub(super) fn candidate_plan(&self) -> &crate::runtime::WorthUiActiveExecutionPlan {
        self.prepared_transition().candidate_plan()
    }

    pub(super) fn candidate_query_binding(
        &self,
    ) -> &worth_ui_query_binding::WorthUiRuntimeQueryBinding {
        self.prepared_transition().candidate_query_binding()
    }

    pub(super) fn candidate_allocation_catalog(
        &self,
    ) -> crate::runtime::UiMountedAllocationProjectionCatalog {
        self.prepared_transition().candidate_allocation_catalog()
    }

    pub(super) fn candidate_plan_digest(&self) -> u64 {
        self.prepared_transition().candidate_plan_digest()
    }

    pub(super) fn candidate_allocation_truth_revision(&self) -> u64 {
        self.prepared_transition()
            .candidate_allocation_truth_revision()
    }

    fn prepared_transition(&self) -> &crate::runtime::WorthUiPreparedApplicationPlanSwap {
        match self
            .transition
            .as_ref()
            .expect("prepared application transition is present")
        {
            WorthUiApplicationCutoverTransition::Prepared(activation) => activation,
            WorthUiApplicationCutoverTransition::Committed { .. } => {
                unreachable!("prepared application transition cannot already be committed")
            }
        }
    }
}

fn seal_semantic_no_op(
    evidence: WorthUiPreparedCutoverEvidence,
    receipt: Box<crate::runtime::WorthUiSemanticNoOpReceipt>,
) -> WorthUiPreparedApplicationCutoverOutcome {
    let WorthUiPreparedCutoverEvidence {
        generations,
        reload_cost_seed,
        ..
    } = evidence;
    debug_assert_eq!(receipt.active_generation(), &generations.prior);
    debug_assert_eq!(receipt.candidate_generation(), &generations.active);
    let reload_cost = reload_cost_seed.finish(
        generations.prior,
        generations.active,
        receipt.equivalence().previous_fingerprint(),
        receipt.candidate_construction(),
        receipt.equivalence(),
    );
    WorthUiPreparedApplicationCutoverOutcome::SemanticNoOp(Box::new(
        WorthUiApplicationSemanticNoOpReceipt {
            receipt: *receipt,
            reload_cost,
        },
    ))
}

fn seal_prepared_activation(
    evidence: WorthUiPreparedCutoverEvidence,
    activation: crate::runtime::WorthUiPreparedApplicationPlanSwap,
) -> WorthUiPreparedApplicationCutoverOutcome {
    let successor_runtime = activation.candidate_runtime_observation();
    let publication = WorthUiApplicationPublicationObservation::prepare_successor(
        WorthUiApplicationPublicationPreparation {
            application_generation: evidence.generations.active.clone(),
            successor_runtime: successor_runtime.clone(),
            runtime_basis: evidence.runtime_basis,
            host_session: evidence.host_session,
            successor_scheduler: activation.candidate_scheduler_state(),
        },
    );
    let reload_cost = evidence.reload_cost_seed.finish(
        evidence.generations.prior.clone(),
        evidence.generations.active.clone(),
        activation.previous_active_plan_digest(),
        successor_runtime
            .cross_lane_bundle()
            .construction_counters(),
        activation
            .plan_decision()
            .summary()
            .expect("prepared activation carries comparison evidence"),
    );
    WorthUiPreparedApplicationCutoverOutcome::Activation(Box::new(
        WorthUiPreparedApplicationActivation {
            identity: Box::new(WorthUiApplicationCutoverIdentityEvidence {
                prior_generation: evidence.generations.prior,
                active_generation: evidence.generations.active,
            }),
            publication: Box::new(publication),
            visual_trace_source: evidence.visual_trace_source,
            font_collection: evidence.font_collection,
            candidate_graph: evidence.candidate_graph,
            candidate_application_authority: evidence.candidate_application_authority,
            reload_cost,
            transition: Some(WorthUiApplicationCutoverTransition::Prepared(activation)),
        },
    ))
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
    (boundary.frame_epoch() != session.application.frame_epoch())
        .then_some(crate::runtime::WorthUiActivationGateDenialReason::BoundaryFrameEpochMismatch)
}
