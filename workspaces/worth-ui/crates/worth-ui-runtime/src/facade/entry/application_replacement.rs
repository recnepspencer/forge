use super::{WorthUiActiveApplicationSession, WorthUiActiveApplicationSessionIdentity, WorthUiApp};
use crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationIdentity;

pub enum WorthUiApplicationReplacementPreparation {
    Prepared(Box<WorthUiPreparedApplicationReplacement>),
    NoOp(WorthUiApplicationReplacementNoOp),
}

pub struct WorthUiPreparedApplicationReplacement {
    next_app: WorthUiApp,
    admitted: crate::runtime::WorthUiAdmittedReplacementCandidate,
    basis: WorthUiPreparedApplicationReplacementBasis,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiPreparedApplicationReplacementBasis {
    origin_session: WorthUiActiveApplicationSessionIdentity,
    next_generation: WorthUiPreparedApplicationGenerationIdentity,
    candidate_basis: crate::runtime::WorthUiReplacementCandidateBasis,
    graph_authority_identity: crate::graph::UiGraphAuthorityIdentity,
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
}

pub struct WorthUiPendingApplicationCutover {
    next_app: WorthUiApp,
    pending_activation: crate::runtime::WorthUiPendingActivation,
    basis: WorthUiPreparedApplicationReplacementBasis,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiApplicationReplacementNoOp {
    active_generation: WorthUiPreparedApplicationGenerationIdentity,
    candidate_basis: crate::runtime::WorthUiReplacementCandidateBasis,
}

pub struct WorthUiApplicationCutoverReceipt {
    prior_generation: WorthUiPreparedApplicationGenerationIdentity,
    active_generation: WorthUiPreparedApplicationGenerationIdentity,
    plan_swap: crate::runtime::WorthUiPlanSwapReceipt,
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
    Activation(crate::runtime::WorthUiAllocationCatalogActivationDenial),
}

impl WorthUiActiveApplicationSession {
    pub fn prepare_replacement(
        &self,
        submission: crate::runtime::WorthUiWatchedCandidateSubmission,
    ) -> Result<
        WorthUiApplicationReplacementPreparation,
        WorthUiApplicationReplacementPreparationDenial,
    > {
        let (next_authority, candidate) =
            crate::facade::lifecycle::prepare_successor_application_authority(
                self.app.prepared_authority(),
                submission,
            )
            .map_err(WorthUiApplicationReplacementPreparationDenial::Preparation)?;
        let candidate_basis = candidate.basis();
        let admitted = crate::runtime::WorthUiCandidateAdmission::for_active_basis(
            self.runtime.replacement_admission_basis(),
        )
        .admit(candidate)
        .map_err(WorthUiApplicationReplacementPreparationDenial::Admission)?;
        let next_app = WorthUiApp::from_prepared_authority(next_authority);
        let active = self.runtime.inspect_active();
        if candidate_basis.artifact_digest().raw() == active.artifact_digest()
            && next_app.prepared_authority().declaration_source_identity()
                == self.app.prepared_authority().declaration_source_identity()
        {
            return Ok(WorthUiApplicationReplacementPreparation::NoOp(
                WorthUiApplicationReplacementNoOp {
                    active_generation: self.generation_identity().clone(),
                    candidate_basis,
                },
            ));
        }
        let Some(basis) = WorthUiPreparedApplicationReplacementBasis::bind(
            self.session_identity(),
            &next_app,
            &admitted,
        ) else {
            return Err(
                WorthUiApplicationReplacementPreparationDenial::PreparedApplicationBindingMismatch,
            );
        };
        Ok(WorthUiApplicationReplacementPreparation::Prepared(
            Box::new(WorthUiPreparedApplicationReplacement {
                next_app,
                admitted,
                basis,
            }),
        ))
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
        let lowering = self
            .runtime
            .prepare_application_replacement_lowering(prepared.admitted, configure)
            .map_err(WorthUiApplicationReplacementLoweringDenial::Lowering)?;
        Ok(WorthUiLoweredApplicationReplacement {
            next_app: prepared.next_app,
            lowering,
            basis: prepared.basis,
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
        })
    }

    pub fn activate_prepared_replacement(
        &mut self,
        pending: WorthUiPendingApplicationCutover,
        admitted_catalog: crate::graph::UiAdmittedAllocationCatalogBasisSet,
        boundary: crate::runtime::WorthUiFrameBoundary,
        lane_parity_report: Option<crate::runtime::WorthUiLaneParityReport>,
    ) -> Result<WorthUiApplicationCutoverReceipt, WorthUiApplicationCutoverDenial> {
        if !pending.basis.admits_session(self.session_identity()) {
            return Err(WorthUiApplicationCutoverDenial::ForeignActiveApplicationSession);
        }
        let prior_generation = self.generation_identity().clone();
        let active_generation = pending.basis.next_generation().clone();
        debug_assert_eq!(pending.next_app.generation_identity(), &active_generation);
        if !pending.basis.admits_catalog(&admitted_catalog) {
            return Err(WorthUiApplicationCutoverDenial::PreparedApplicationGraphMismatch);
        }
        let plan_swap = self
            .runtime
            .activate_admitted_allocation_catalog_at_frame_boundary(
                pending.pending_activation,
                admitted_catalog,
                boundary,
                lane_parity_report,
            )
            .map_err(WorthUiApplicationCutoverDenial::Activation)?;
        self.runtime
            .bind_active_application_generation(active_generation.clone());
        self.runtime
            .bind_retained_allocation_planning_evidence(std::rc::Rc::clone(
                pending.next_app.retained_planning_authority(),
            ));
        self.app = pending.next_app;
        Ok(WorthUiApplicationCutoverReceipt {
            prior_generation,
            active_generation,
            plan_swap,
        })
    }
}

impl WorthUiApplicationReplacementNoOp {
    pub fn active_generation(&self) -> &WorthUiPreparedApplicationGenerationIdentity {
        &self.active_generation
    }

    pub fn candidate_basis(&self) -> crate::runtime::WorthUiReplacementCandidateBasis {
        self.candidate_basis
    }
}

impl WorthUiPreparedApplicationReplacement {
    /// Borrow the candidate graph without promoting it to active truth.
    pub fn candidate_graph(&self) -> crate::graph::UiGraphAuthority<'_> {
        self.next_app.graph()
    }

    pub fn candidate_declaration_artifacts(&self) -> &[crate::declaration::UiDeclarationArtifact] {
        self.next_app.declaration_artifacts()
    }

    /// Advance candidate graph authority only after mounted-receipt transitions
    /// minted by that candidate graph have been re-admitted at its boundary.
    pub fn commit_candidate_mounted_layout_admissions(
        &mut self,
        transitions: Vec<crate::graph::UiGraphMountedReceiptTransition>,
    ) -> Result<(), crate::graph::UiGraphMountedLayoutAdmissionDenial> {
        let committed = self
            .candidate_graph()
            .commit_mounted_layout_admissions(transitions)?;
        self.next_app.advance_prepared_graph(committed);
        self.basis.rebind_graph(&self.next_app);
        Ok(())
    }

    /// Enter candidate-scoped obligation admission without touching the active
    /// application's admission authority.
    pub fn candidate_admission(&self) -> crate::admission::UiAdmissionBoundary<'_> {
        self.next_app.admission()
    }

    pub fn try_candidate_query_touch_for_node(
        &self,
        graph_node_identity: crate::graph::UiGraphNodeIdentity,
    ) -> Result<
        crate::obligations::touch::UiGraphTouchDescriptor,
        crate::obligations::touch::UiGraphTouchDenial,
    > {
        self.next_app.try_query_touch_for_node(graph_node_identity)
    }

    /// Seal one complete allocation catalog against the candidate graph that
    /// will become active. A catalog admitted by any other graph cannot cross
    /// the later cutover boundary.
    pub fn admit_candidate_allocation_catalog(
        &self,
        entries: Vec<(
            crate::evidence::UiMeasurementBasis,
            crate::obligations::selection::UiSelectedObligationSet,
        )>,
    ) -> Result<
        crate::graph::UiAdmittedAllocationCatalogBasisSet,
        crate::graph::UiAllocationCatalogBasisAdmissionDenial,
    > {
        self.next_app
            .graph_snapshot()
            .admit_allocation_catalog_basis_set(entries)
    }

    pub fn inspect_candidate(
        &self,
        query: worth_ui_inspection::UiInspectionQuery,
    ) -> WorthUiCandidateInspectionReceipt {
        WorthUiCandidateInspectionReceipt {
            generation_identity: self.next_app.generation_identity().clone(),
            candidate_basis: self.admitted.candidate().basis(),
            receipt: self.next_app.inspect(query),
        }
    }

    pub fn expand_candidate_evidence_ref(
        &self,
        evidence_ref: crate::evidence::UiEvidenceRef,
        requested_richness: worth_ui_inspection::UiEvidenceRichness,
    ) -> crate::evidence::UiEvidenceExpansion {
        self.next_app
            .expand_evidence_ref(evidence_ref, requested_richness)
    }
}

impl WorthUiPreparedApplicationReplacementBasis {
    fn bind(
        origin_session: WorthUiActiveApplicationSessionIdentity,
        next_app: &WorthUiApp,
        admitted: &crate::runtime::WorthUiAdmittedReplacementCandidate,
    ) -> Option<Self> {
        let candidate_basis = admitted.candidate().basis();
        (next_app
            .prepared_authority()
            .source_backed_candidate_basis()
            == Some(candidate_basis))
        .then(|| Self {
            origin_session,
            next_generation: next_app.generation_identity().clone(),
            candidate_basis,
            graph_authority_identity: next_app
                .prepared_authority()
                .graph_snapshot()
                .authority_identity(),
        })
    }

    fn admits_session(&self, session: WorthUiActiveApplicationSessionIdentity) -> bool {
        self.origin_session == session
    }

    fn rebind_graph(&mut self, next_app: &WorthUiApp) {
        self.next_generation = next_app.generation_identity().clone();
        self.graph_authority_identity = next_app.graph_snapshot().authority_identity();
    }

    pub(crate) fn next_generation(&self) -> &WorthUiPreparedApplicationGenerationIdentity {
        &self.next_generation
    }

    fn admits_catalog(&self, catalog: &crate::graph::UiAdmittedAllocationCatalogBasisSet) -> bool {
        self.graph_authority_identity == catalog.graph_authority_identity()
    }
}

impl WorthUiCandidateInspectionReceipt {
    pub fn generation_identity(&self) -> &WorthUiPreparedApplicationGenerationIdentity {
        &self.generation_identity
    }

    pub fn candidate_basis(&self) -> crate::runtime::WorthUiReplacementCandidateBasis {
        self.candidate_basis
    }

    pub fn receipt(&self) -> &crate::facade::inspection_bridge::UiInspectionReceipt {
        &self.receipt
    }
}

impl WorthUiApplicationCutoverReceipt {
    pub fn prior_generation(&self) -> &WorthUiPreparedApplicationGenerationIdentity {
        &self.prior_generation
    }

    pub fn active_generation(&self) -> &WorthUiPreparedApplicationGenerationIdentity {
        &self.active_generation
    }

    pub fn plan_swap(&self) -> &crate::runtime::WorthUiPlanSwapReceipt {
        &self.plan_swap
    }
}
