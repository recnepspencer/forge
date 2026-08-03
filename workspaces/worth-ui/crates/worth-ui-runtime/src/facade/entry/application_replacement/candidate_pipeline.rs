use super::*;
use crate::facade::WorthUiActiveApplicationSession;

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
                self.application.prepared_authority(),
                submission,
            )
            .map_err(WorthUiApplicationReplacementPreparationDenial::Preparation)?;
        let admitted = crate::runtime::WorthUiCandidateAdmission::for_active_basis(
            self.application.replacement_admission_basis(),
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
            semantic_input: WorthUiPreparedReplacementSemanticInput::Admitted(admitted),
            basis,
            candidate_graph_changed_nodes: Default::default(),
        }))
    }

    pub fn lower_prepared_replacement(
        &self,
        prepared: WorthUiPreparedApplicationReplacement,
    ) -> Result<WorthUiLoweredApplicationReplacement, WorthUiApplicationReplacementLoweringDenial>
    {
        if !prepared.basis.admits_session(self.session_identity()) {
            return Err(
                WorthUiApplicationReplacementLoweringDenial::ForeignActiveApplicationSession,
            );
        }
        let lowering = match prepared.semantic_input {
            WorthUiPreparedReplacementSemanticInput::Admitted(admitted) => {
                let candidate_application_authority =
                    prepared.next_app.prepared_authority().lowering_authority();
                self.application
                    .prepare_application_replacement_lowering(
                        admitted,
                        candidate_application_authority,
                        &prepared.candidate_query_binding,
                    )
                    .map_err(WorthUiApplicationReplacementLoweringDenial::Lowering)?
            }
            WorthUiPreparedReplacementSemanticInput::Prelowered(lowering) => lowering,
        };
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
            .application
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
}
