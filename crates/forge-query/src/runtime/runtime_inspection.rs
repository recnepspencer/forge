use super::*;

impl ForgeQueryRuntime {
    pub(crate) fn inspect_live_view_name_installation(
        &self,
        view_name: &str,
    ) -> Result<&ForgeQueryRuntimeLiveSubscriptionInstallation, ForgeQueryRuntimeError> {
        self.admit_facade_family(ForgeQueryRuntimeFacadeFamily::Inspect)?;
        self.live_subscriptions
            .get(view_name)
            .map(|state| &state.installation)
            .ok_or_else(|| ForgeQueryRuntimeError::MissingLiveSubscription(view_name.to_string()))
    }

    pub fn inspect_live_view<T>(
        &self,
        view: &ForgeQueryLiveView<T>,
    ) -> Result<&ForgeQueryRuntimeLiveSubscriptionInstallation, ForgeQueryRuntimeError> {
        self.inspect_live_view_name_installation(view.name())
    }

    pub fn inspect_live_view_explanation<T>(
        &self,
        view: &ForgeQueryLiveView<T>,
    ) -> Result<ForgeQueryLiveViewInspection, ForgeQueryRuntimeError> {
        let state = self.live_subscriptions.get(view.name()).ok_or_else(|| {
            ForgeQueryRuntimeError::MissingLiveSubscription(view.name().to_string())
        })?;
        Ok(ForgeQueryLiveViewInspection::from_state(state))
    }

    pub fn inspect_receipt<'a>(
        &'a self,
        receipt: &'a ForgeQueryWriteReceipt,
    ) -> ForgeQueryArtifactInspector<'a> {
        self.try_inspect_receipt(receipt)
            .expect("inspect support must be admitted before inspecting receipts")
    }

    pub fn try_inspect_receipt<'a>(
        &'a self,
        receipt: &'a ForgeQueryWriteReceipt,
    ) -> Result<ForgeQueryArtifactInspector<'a>, ForgeQueryRuntimeError> {
        self.admit_facade_family(ForgeQueryRuntimeFacadeFamily::Inspect)?;
        let runtime_evidence = self
            .backend
            .inspect_write_receipt(receipt, &self.evidence_authority)?;
        Ok(ForgeQueryArtifactInspector {
            receipt,
            runtime_evidence,
        })
    }

    pub fn inspect_intent_receipt(
        &self,
        receipt: &ForgeQueryIntentReceipt,
    ) -> Result<ForgeQueryIntentReceiptInspection, ForgeQueryRuntimeError> {
        match self.inspect(receipt)? {
            ForgeQueryInspection::IntentReceipt(inspection) => Ok(inspection),
            other => panic!("expected intent receipt inspection, got {other:?}"),
        }
    }

    pub fn inspect_effect_intent_receipt(
        &self,
        receipt: &ForgeQueryEffectIntentReceipt,
    ) -> Result<ForgeQueryEffectIntentReceiptInspection, ForgeQueryRuntimeError> {
        match self.inspect(receipt)? {
            ForgeQueryInspection::EffectIntentReceipt(inspection) => Ok(inspection),
            other => panic!("expected effect intent receipt inspection, got {other:?}"),
        }
    }

    pub fn inspect_intent_denial(
        &self,
        evidence: &ForgeQueryIntentDenialEvidence,
    ) -> Result<ForgeQueryIntentDenialInspection, ForgeQueryRuntimeError> {
        match self.inspect(evidence)? {
            ForgeQueryInspection::IntentDenial(inspection) => Ok(inspection),
            other => panic!("expected intent denial inspection, got {other:?}"),
        }
    }

    pub fn inspect_preview_binding(
        &self,
        binding: &ForgeQueryPreviewHandleBindingEvidence,
    ) -> Result<ForgeQueryPreviewBindingInspection, ForgeQueryRuntimeError> {
        match self.inspect(binding)? {
            ForgeQueryInspection::PreviewBinding(inspection) => Ok(inspection),
            other => panic!("expected preview binding inspection, got {other:?}"),
        }
    }

    pub fn inspect_preview_outcome(
        &self,
        outcome: &ForgeQueryPreviewOutcome,
    ) -> Result<ForgeQueryPreviewOutcomeInspection, ForgeQueryRuntimeError> {
        match self.inspect(outcome)? {
            ForgeQueryInspection::PreviewOutcome(inspection) => Ok(inspection),
            other => panic!("expected preview outcome inspection, got {other:?}"),
        }
    }

    pub fn inspect_preview_intent_receipt(
        &self,
        receipt: &ForgeQueryPreviewIntentReceipt,
    ) -> Result<ForgeQueryPreviewIntentReceiptInspection, ForgeQueryRuntimeError> {
        match self.inspect(receipt)? {
            ForgeQueryInspection::PreviewIntentReceipt(inspection) => Ok(inspection),
            other => panic!("expected preview intent receipt inspection, got {other:?}"),
        }
    }

    pub fn inspect_branch_intent_receipt(
        &self,
        receipt: &ForgeQueryBranchIntentReceipt,
    ) -> Result<ForgeQueryBranchIntentReceiptInspection, ForgeQueryRuntimeError> {
        match self.inspect(receipt)? {
            ForgeQueryInspection::BranchIntentReceipt(inspection) => Ok(inspection),
            other => panic!("expected branch intent receipt inspection, got {other:?}"),
        }
    }

    pub fn inspect_feedback_path<T>(
        &self,
        effect: &ForgeQueryEffectHandle<T>,
    ) -> Result<ForgeQueryFeedbackPhaseGraphInspection, ForgeQueryRuntimeError> {
        self.admit_facade_family(ForgeQueryRuntimeFacadeFamily::Inspect)?;
        let runtime = self
            .effects
            .get(effect.name())
            .ok_or_else(|| ForgeQueryRuntimeError::MissingEffect(effect.name().to_string()))?;
        ForgeQueryFeedbackPhaseGraphInspection::from_effect_runtime(runtime).ok_or_else(|| {
            ForgeQueryRuntimeError::MissingEffect(format!(
                "{} has no retained feedback delivery",
                effect.name()
            ))
        })
    }

    pub fn inspect_effect_feedback_receipt(
        &self,
        receipt: &ForgeQueryEffectIntentReceipt,
    ) -> Result<ForgeQueryFeedbackPhaseGraphInspection, ForgeQueryRuntimeError> {
        self.admit_facade_family(ForgeQueryRuntimeFacadeFamily::Inspect)?;
        Ok(ForgeQueryFeedbackPhaseGraphInspection::from_effect_intent_receipt(receipt))
    }

    pub fn inspect<'a, T>(
        &'a self,
        target: T,
    ) -> Result<ForgeQueryInspection, ForgeQueryRuntimeError>
    where
        T: Into<ForgeQueryInspectionTarget<'a>>,
    {
        self.admit_facade_family(ForgeQueryRuntimeFacadeFamily::Inspect)?;
        match target.into() {
            ForgeQueryInspectionTarget::AdmittedWorldBasis(basis) => {
                Ok(ForgeQueryInspection::BasisLifecycle(
                    ForgeQueryBasisLifecycleInspection::from_admitted_world_basis(basis),
                ))
            }
            ForgeQueryInspectionTarget::ObservationBasisCapability(basis) => {
                Ok(ForgeQueryInspection::BasisLifecycle(basis.into()))
            }
            ForgeQueryInspectionTarget::InspectionBasisCapability(basis) => {
                Ok(ForgeQueryInspection::BasisLifecycle(basis.into()))
            }
            ForgeQueryInspectionTarget::SubscriptionDeclarationBasisCapability(basis) => {
                Ok(ForgeQueryInspection::BasisLifecycle(basis.into()))
            }
            ForgeQueryInspectionTarget::SubscriptionActivationBasisCapability(basis) => {
                Ok(ForgeQueryInspection::BasisLifecycle(basis.into()))
            }
            ForgeQueryInspectionTarget::ScopedObservationBasis(basis) => {
                Ok(ForgeQueryInspection::BasisLifecycle(basis.into()))
            }
            ForgeQueryInspectionTarget::ScopedInspectionBasis(basis) => {
                Ok(ForgeQueryInspection::BasisLifecycle(basis.into()))
            }
            ForgeQueryInspectionTarget::ScopedReplayBasis(basis) => {
                Ok(ForgeQueryInspection::BasisLifecycle(basis.into()))
            }
            ForgeQueryInspectionTarget::ScopedSubscriptionDeclarationBasis(basis) => {
                Ok(ForgeQueryInspection::BasisLifecycle(basis.into()))
            }
            ForgeQueryInspectionTarget::ScopedSubscriptionActivationBasis(basis) => {
                Ok(ForgeQueryInspection::BasisLifecycle(basis.into()))
            }
            ForgeQueryInspectionTarget::LowerRuntimeBoundObservationBasis(basis) => {
                Ok(ForgeQueryInspection::BasisLifecycle(basis.into()))
            }
            ForgeQueryInspectionTarget::LowerRuntimeBoundInspectionBasis(basis) => {
                Ok(ForgeQueryInspection::BasisLifecycle(basis.into()))
            }
            ForgeQueryInspectionTarget::LowerRuntimeBoundSubscriptionDeclarationBasis(basis) => {
                Ok(ForgeQueryInspection::BasisLifecycle(basis.into()))
            }
            ForgeQueryInspectionTarget::LowerRuntimeBoundSubscriptionActivationBasis(basis) => {
                Ok(ForgeQueryInspection::BasisLifecycle(basis.into()))
            }
            ForgeQueryInspectionTarget::DeniedBasisCapability(denial) => {
                Ok(ForgeQueryInspection::BasisLifecycle(denial.into()))
            }
            ForgeQueryInspectionTarget::BasisIntentDenial(denial) => {
                Ok(ForgeQueryInspection::BasisLifecycle(denial.into()))
            }
            ForgeQueryInspectionTarget::DerivedView { name } => {
                let review = self.review_runtime_derived_inspection(name.to_string())?;
                let handoff = self.resolve_reviewed_admitted_derived_inspection_handoff(review)?;
                let binding = self.prepare_derived_inspection_execution_binding(handoff);
                let result = self.execute_derived_inspection_execution_binding(binding)?;
                Ok(ForgeQueryInspection::DerivedView(result.evidence().clone()))
            }
            target => {
                let seed = crate::intent_admission::ForgeQueryGenericInspectionIntentSeed::from_target(target)
                    .expect("derived inspection targets should route through the derived inspection lane");
                let review = self.review_unified_inspection(seed)?;
                let handoff = self.resolve_reviewed_admitted_unified_inspection_handoff(review)?;
                let binding = self.prepare_unified_inspection_execution_binding(handoff);
                let result = self.execute_unified_inspection_execution_binding(binding)?;
                Ok(result.inspection().clone())
            }
        }
    }
}
