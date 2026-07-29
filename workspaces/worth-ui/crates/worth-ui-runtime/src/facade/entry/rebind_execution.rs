use super::{
    WorthUiActiveApplicationSession, WorthUiMountedReplacementPreparationOutcome,
    WorthUiPreparedApplicationReplacement,
};

pub(crate) struct WorthUiPreparedEvidenceOnlyApplicationRebind<'session> {
    application: &'session mut crate::runtime::session::WorthUiApplicationSessionState,
    successor_authority:
        crate::facade::prepared_application_authority::WorthUiPreparedApplicationAuthority,
    _admitted_candidate: crate::runtime::WorthUiAdmittedReplacementCandidate,
    _comparison: crate::runtime::WorthUiRuntimeArtifactComparison,
}

impl<'session> WorthUiPreparedEvidenceOnlyApplicationRebind<'session> {
    fn new(
        application: &'session mut crate::runtime::session::WorthUiApplicationSessionState,
        succession: crate::runtime::observation::UiAuthoredSourceSuccession,
    ) -> Result<Self, crate::runtime::rebind::UiRebindPreparationDenial> {
        let crate::runtime::observation::UiAuthoredSourceSuccession::EvidenceOnly {
            successor_authority,
            admitted_candidate,
            comparison,
        } = succession
        else {
            return Err(crate::runtime::rebind::UiRebindPreparationDenial::InvalidSemanticProof);
        };
        Ok(Self {
            application,
            successor_authority,
            _admitted_candidate: admitted_candidate,
            _comparison: comparison,
        })
    }

    pub(crate) fn commit(
        self,
    ) -> (
        crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationIdentity,
        crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationIdentity,
    ) {
        self.application
            .commit_evidence_only_rebind(self.successor_authority)
    }

    pub(crate) fn generation_identity(
        &self,
    ) -> &crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationIdentity
    {
        self.successor_authority.generation_identity()
    }
}

impl WorthUiActiveApplicationSession {
    pub fn prepare_rebind(
        &mut self,
        mut plan: crate::runtime::rebind::UiRebindPlan,
        request: crate::runtime::rebind::UiRebindExecutionRequest,
    ) -> Result<
        crate::runtime::rebind::UiPreparedRebind<'_>,
        crate::runtime::rebind::UiRebindPreparationDenial,
    > {
        let reservation = crate::runtime::rebind::admit_plan(
            &self.rebind,
            crate::runtime::rebind::UiRebindFinalAdmissionBasis::new(
                self.identity,
                self.capabilities().digest().as_u64(),
                self.generation_identity(),
            ),
            &plan,
            request,
        )?;
        match plan.take_semantic_proof() {
            crate::runtime::rebind::UiRebindSemanticProof::Changed(changed) => {
                self.prepare_changed_rebind(plan, reservation, changed)
            }
            crate::runtime::rebind::UiRebindSemanticProof::EvidenceOnly(succession) => {
                let prepared = WorthUiPreparedEvidenceOnlyApplicationRebind::new(
                    &mut self.application,
                    *succession,
                )?;
                crate::runtime::rebind::UiPreparedRebind::evidence_only(plan, reservation, prepared)
            }
            crate::runtime::rebind::UiRebindSemanticProof::NonSource => {
                Err(crate::runtime::rebind::UiRebindPreparationDenial::UnsupportedNonSourcePlan)
            }
            crate::runtime::rebind::UiRebindSemanticProof::Transferred => {
                Err(crate::runtime::rebind::UiRebindPreparationDenial::InvalidSemanticProof)
            }
        }
    }

    fn prepare_changed_rebind(
        &mut self,
        plan: crate::runtime::rebind::UiRebindPlan,
        reservation: crate::runtime::rebind::UiRebindReservation,
        changed: Box<crate::runtime::rebind::UiChangedRebindSemanticProof>,
    ) -> Result<
        crate::runtime::rebind::UiPreparedRebind<'_>,
        crate::runtime::rebind::UiRebindPreparationDenial,
    > {
        let mut prepared = WorthUiPreparedApplicationReplacement::from_changed_rebind_plan(
            self.identity,
            *changed,
        )
        .ok_or(crate::runtime::rebind::UiRebindPreparationDenial::CandidateBindingMismatch)?;
        let catalog = self
            .admit_native_replacement_allocation_catalog(&mut prepared)
            .map_err(|_| crate::runtime::rebind::UiRebindPreparationDenial::CandidateAllocation)?;
        let lowered = self
            .lower_prepared_replacement(prepared)
            .map_err(|_| crate::runtime::rebind::UiRebindPreparationDenial::CandidateLowering)?;
        let pending = self
            .stage_prepared_replacement(lowered)
            .map_err(|_| crate::runtime::rebind::UiRebindPreparationDenial::CandidateStaging)?;
        let boundary = self
            .execute_framework_turn(|_| {})
            .map_err(|_| {
                crate::runtime::rebind::UiRebindPreparationDenial::FrameBoundaryUnavailable
            })?
            .into_completion()
            .into_execution()
            .map_err(|_| {
                crate::runtime::rebind::UiRebindPreparationDenial::FrameBoundaryUnavailable
            })?
            .into_activation_boundary();
        let replacement = self
            .prepare_mounted_replacement(
                pending,
                catalog,
                boundary,
                None,
                crate::mounting::UiMountedFrameRequest::all_bound_surfaces(),
            )
            .map_err(|_| crate::runtime::rebind::UiRebindPreparationDenial::MountedPreparation)?;
        let replacement = match replacement {
            WorthUiMountedReplacementPreparationOutcome::Prepared(replacement) => replacement,
            WorthUiMountedReplacementPreparationOutcome::SemanticNoOp(_) => return Err(
                crate::runtime::rebind::UiRebindPreparationDenial::PlannedChangeBecameSemanticNoOp,
            ),
        };
        Ok(crate::runtime::rebind::UiPreparedRebind::changed(
            plan,
            reservation,
            replacement,
        ))
    }
}
