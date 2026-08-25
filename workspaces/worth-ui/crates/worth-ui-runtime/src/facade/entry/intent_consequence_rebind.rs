use super::WorthUiActiveApplicationSession;

pub(super) struct WorthUiIntentConsequenceRebindTransfer {
    pub(super) observation: crate::runtime::observation::UiPreparedObservationProgressCommit,
    pub(super) posture: Option<crate::mounting::UiIntentPostureCommit>,
    pub(super) consequence: crate::runtime::intent_execution::UiIntentConsequenceHandoff,
    pub(super) query_reference:
        Option<worth_ui_query_binding::WorthUiInstalledQueryBindingReference>,
}

impl WorthUiActiveApplicationSession {
    pub(super) fn prepare_intent_consequence_rebind(
        &mut self,
        plan: crate::runtime::rebind::UiRebindPlan,
        request: crate::runtime::rebind::UiRebindExecutionRequest,
        transfer: WorthUiIntentConsequenceRebindTransfer,
    ) -> Result<
        super::intent_consequence_publication::WorthUiPreparedIntentConsequenceRebind<'_>,
        crate::runtime::intent_execution::UiIntentConsequenceStop,
    > {
        if !plan.has_non_source_semantic_proof() {
            return Err(self.retain_intent_consequence_preparation_stop(
                crate::runtime::rebind::UiRebindPreparationDenial::InvalidSemanticProof,
                plan,
                transfer,
            ));
        }
        let reservation = match crate::runtime::rebind::admit_plan(
            &self.rebind,
            crate::runtime::rebind::UiRebindFinalAdmissionBasis::new(
                self.identity,
                self.capabilities().digest().as_u64(),
                self.generation_identity(),
            ),
            &plan,
            request,
        ) {
            Ok(reservation) => reservation,
            Err(denial) => {
                return Err(self.retain_intent_consequence_preparation_stop(denial, plan, transfer))
            }
        };
        let semantic_content = plan.content().clone();
        let frame = match self.prepare_intent_consequence_frame(semantic_content) {
            Ok(frame) => frame,
            Err(denial) => {
                drop(reservation);
                return Err(self.retain_intent_consequence_preparation_stop(denial, plan, transfer));
            }
        };
        Ok(
            super::intent_consequence_publication::WorthUiPreparedIntentConsequenceRebind::new(
                self,
                plan,
                reservation,
                frame,
                transfer,
            ),
        )
    }

    pub(super) fn prepare_intent_consequence_frame(
        &mut self,
        semantic_content: crate::mounting::UiMountedSemanticContentInput,
    ) -> Result<
        crate::mounting::UiPreparedMountedFrame,
        crate::runtime::rebind::UiRebindPreparationDenial,
    > {
        let completion = self.execute_framework_turn(|_| {}).map_err(|_| {
            crate::runtime::rebind::UiRebindPreparationDenial::FrameBoundaryUnavailable
        })?;
        let execution = completion.into_execution().map_err(|_| {
            crate::runtime::rebind::UiRebindPreparationDenial::FrameBoundaryUnavailable
        })?;
        let theme_values = execution.presentation.theme_values_source();
        execution
            .prepare_mounted_frame_with_content_internal(
                crate::mounting::UiMountedFrameRequest::all_bound_surfaces(),
                semantic_content,
                theme_values,
            )
            .map_err(|denial| {
                crate::runtime::rebind::UiRebindPreparationDenial::ContentMountedPreparation(
                    Box::new(denial),
                )
            })
    }

    fn retain_intent_consequence_preparation_stop(
        &mut self,
        denial: crate::runtime::rebind::UiRebindPreparationDenial,
        plan: crate::runtime::rebind::UiRebindPlan,
        mut transfer: WorthUiIntentConsequenceRebindTransfer,
    ) -> crate::runtime::intent_execution::UiIntentConsequenceStop {
        transfer
            .consequence
            .restore_query_from_facts(plan.into_retained_facts());
        self.intent_execution.retain_consequence_handoff(
            transfer.consequence,
            crate::runtime::intent_execution::UiIntentConsequenceStopReason::Preparation(Box::new(
                denial,
            )),
        )
    }
}
