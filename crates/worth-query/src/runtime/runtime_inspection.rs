use super::*;

impl WorthQueryRuntime {
    pub(crate) fn resolve_live_artifact_target(
        &self,
        view_name: &str,
    ) -> Result<WorthQueryLiveArtifactTarget, WorthQueryRuntimeError> {
        let target = WorthQueryLiveArtifactTarget::from_view_name(view_name);
        let state = self.live_subscriptions.get(&target).ok_or_else(|| {
            WorthQueryRuntimeError::MissingLiveSubscription(view_name.to_string())
        })?;
        Ok(WorthQueryLiveArtifactTarget::from_subscription_installation(&state.installation))
    }

    pub(crate) fn inspect_live_view_name_installation(
        &self,
        view_name: &str,
    ) -> Result<&WorthQueryRuntimeLiveSubscriptionInstallation, WorthQueryRuntimeError> {
        self.admit_facade_family(WorthQueryRuntimeFacadeFamily::Inspect)?;
        let target = WorthQueryLiveArtifactTarget::from_view_name(view_name);
        self.live_subscriptions
            .get(&target)
            .map(|state| &state.installation)
            .ok_or_else(|| WorthQueryRuntimeError::MissingLiveSubscription(view_name.to_string()))
    }

    pub fn inspect_live_view<T>(
        &self,
        view: &WorthQueryLiveView<T>,
    ) -> Result<&WorthQueryRuntimeLiveSubscriptionInstallation, WorthQueryRuntimeError> {
        self.inspect_live_view_name_installation(view.name())
    }

    pub fn inspect_live_view_explanation<T>(
        &self,
        view: &WorthQueryLiveView<T>,
    ) -> Result<WorthQueryLiveViewInspection, WorthQueryRuntimeError> {
        let target = WorthQueryLiveArtifactTarget::from_subscription_installation(
            view.subscription_installation(),
        );
        let state = self.live_subscriptions.get(&target).ok_or_else(|| {
            WorthQueryRuntimeError::MissingLiveSubscription(view.name().to_string())
        })?;
        Ok(WorthQueryLiveViewInspection::from_state(state))
    }

    pub fn inspect_receipt<'a>(
        &'a self,
        receipt: &'a WorthQueryWriteReceipt,
    ) -> WorthQueryArtifactInspector<'a> {
        self.try_inspect_receipt(receipt)
            .expect("inspect support must be admitted before inspecting receipts")
    }

    pub fn try_inspect_receipt<'a>(
        &'a self,
        receipt: &'a WorthQueryWriteReceipt,
    ) -> Result<WorthQueryArtifactInspector<'a>, WorthQueryRuntimeError> {
        self.admit_facade_family(WorthQueryRuntimeFacadeFamily::Inspect)?;
        let runtime_evidence = self
            .backend
            .inspect_write_receipt(receipt, &self.evidence_authority)?;
        Ok(WorthQueryArtifactInspector {
            receipt,
            runtime_evidence,
        })
    }

    pub fn inspect_intent_receipt(
        &self,
        receipt: &WorthQueryIntentReceipt,
    ) -> Result<WorthQueryIntentReceiptInspection, WorthQueryRuntimeError> {
        match self.inspect(receipt)? {
            WorthQueryInspection::IntentReceipt(inspection) => Ok(inspection),
            other => panic!("expected intent receipt inspection, got {other:?}"),
        }
    }

    pub fn inspect_effect_intent_receipt(
        &self,
        receipt: &WorthQueryEffectIntentReceipt,
    ) -> Result<WorthQueryEffectIntentReceiptInspection, WorthQueryRuntimeError> {
        match self.inspect(receipt)? {
            WorthQueryInspection::EffectIntentReceipt(inspection) => Ok(inspection),
            other => panic!("expected effect intent receipt inspection, got {other:?}"),
        }
    }

    pub fn inspect_intent_denial(
        &self,
        evidence: &WorthQueryIntentDenialEvidence,
    ) -> Result<WorthQueryIntentDenialInspection, WorthQueryRuntimeError> {
        match self.inspect(evidence)? {
            WorthQueryInspection::IntentDenial(inspection) => Ok(inspection),
            other => panic!("expected intent denial inspection, got {other:?}"),
        }
    }

    pub fn inspect_preview_binding(
        &self,
        binding: &WorthQueryPreviewHandleBindingEvidence,
    ) -> Result<WorthQueryPreviewBindingInspection, WorthQueryRuntimeError> {
        match self.inspect(binding)? {
            WorthQueryInspection::PreviewBinding(inspection) => Ok(inspection),
            other => panic!("expected preview binding inspection, got {other:?}"),
        }
    }

    pub fn inspect_preview_outcome(
        &self,
        outcome: &WorthQueryPreviewOutcome,
    ) -> Result<WorthQueryPreviewOutcomeInspection, WorthQueryRuntimeError> {
        match self.inspect(outcome)? {
            WorthQueryInspection::PreviewOutcome(inspection) => Ok(inspection),
            other => panic!("expected preview outcome inspection, got {other:?}"),
        }
    }

    pub fn inspect_preview_intent_receipt(
        &self,
        receipt: &WorthQueryPreviewIntentReceipt,
    ) -> Result<WorthQueryPreviewIntentReceiptInspection, WorthQueryRuntimeError> {
        match self.inspect(receipt)? {
            WorthQueryInspection::PreviewIntentReceipt(inspection) => Ok(inspection),
            other => panic!("expected preview intent receipt inspection, got {other:?}"),
        }
    }

    pub fn inspect_branch_intent_receipt(
        &self,
        receipt: &WorthQueryBranchIntentReceipt,
    ) -> Result<WorthQueryBranchIntentReceiptInspection, WorthQueryRuntimeError> {
        match self.inspect(receipt)? {
            WorthQueryInspection::BranchIntentReceipt(inspection) => Ok(inspection),
            other => panic!("expected branch intent receipt inspection, got {other:?}"),
        }
    }

    pub fn inspect_feedback_path<T>(
        &self,
        effect: &WorthQueryEffectHandle<T>,
    ) -> Result<WorthQueryFeedbackPhaseGraphInspection, WorthQueryRuntimeError> {
        self.admit_facade_family(WorthQueryRuntimeFacadeFamily::Inspect)?;
        let effect_target = WorthQueryEffectTarget::from_name(effect.name());
        let runtime = self
            .effects
            .get(&effect_target)
            .ok_or_else(|| WorthQueryRuntimeError::MissingEffect(effect.name().to_string()))?;
        WorthQueryFeedbackPhaseGraphInspection::from_effect_runtime(runtime).ok_or_else(|| {
            WorthQueryRuntimeError::MissingEffect(format!(
                "{} has no retained feedback delivery",
                effect.name()
            ))
        })
    }

    pub fn inspect_effect_feedback_receipt(
        &self,
        receipt: &WorthQueryEffectIntentReceipt,
    ) -> Result<WorthQueryFeedbackPhaseGraphInspection, WorthQueryRuntimeError> {
        self.admit_facade_family(WorthQueryRuntimeFacadeFamily::Inspect)?;
        Ok(WorthQueryFeedbackPhaseGraphInspection::from_effect_intent_receipt(receipt))
    }

    pub fn inspect<'a, T>(
        &'a self,
        target: T,
    ) -> Result<WorthQueryInspection, WorthQueryRuntimeError>
    where
        T: Into<WorthQueryInspectionTarget<'a>>,
    {
        self.admit_facade_family(WorthQueryRuntimeFacadeFamily::Inspect)?;
        match target.into() {
            WorthQueryInspectionTarget::AdmittedWorldBasis(basis) => {
                Ok(WorthQueryInspection::BasisLifecycle(
                    WorthQueryBasisLifecycleInspection::from_admitted_world_basis(basis),
                ))
            }
            WorthQueryInspectionTarget::ObservationBasisCapability(basis) => {
                Ok(WorthQueryInspection::BasisLifecycle(basis.into()))
            }
            WorthQueryInspectionTarget::InspectionBasisCapability(basis) => {
                Ok(WorthQueryInspection::BasisLifecycle(basis.into()))
            }
            WorthQueryInspectionTarget::SubscriptionDeclarationBasisCapability(basis) => {
                Ok(WorthQueryInspection::BasisLifecycle(basis.into()))
            }
            WorthQueryInspectionTarget::SubscriptionActivationBasisCapability(basis) => {
                Ok(WorthQueryInspection::BasisLifecycle(basis.into()))
            }
            WorthQueryInspectionTarget::ScopedObservationBasis(basis) => {
                Ok(WorthQueryInspection::BasisLifecycle(basis.into()))
            }
            WorthQueryInspectionTarget::ScopedInspectionBasis(basis) => {
                Ok(WorthQueryInspection::BasisLifecycle(basis.into()))
            }
            WorthQueryInspectionTarget::ScopedReplayBasis(basis) => {
                Ok(WorthQueryInspection::BasisLifecycle(basis.into()))
            }
            WorthQueryInspectionTarget::ScopedSubscriptionDeclarationBasis(basis) => {
                Ok(WorthQueryInspection::BasisLifecycle(basis.into()))
            }
            WorthQueryInspectionTarget::ScopedSubscriptionActivationBasis(basis) => {
                Ok(WorthQueryInspection::BasisLifecycle(basis.into()))
            }
            WorthQueryInspectionTarget::LowerRuntimeBoundObservationBasis(basis) => {
                Ok(WorthQueryInspection::BasisLifecycle(basis.into()))
            }
            WorthQueryInspectionTarget::LowerRuntimeBoundInspectionBasis(basis) => {
                Ok(WorthQueryInspection::BasisLifecycle(basis.into()))
            }
            WorthQueryInspectionTarget::LowerRuntimeBoundSubscriptionDeclarationBasis(basis) => {
                Ok(WorthQueryInspection::BasisLifecycle(basis.into()))
            }
            WorthQueryInspectionTarget::LowerRuntimeBoundSubscriptionActivationBasis(basis) => {
                Ok(WorthQueryInspection::BasisLifecycle(basis.into()))
            }
            WorthQueryInspectionTarget::DeniedBasisCapability(denial) => {
                Ok(WorthQueryInspection::BasisLifecycle(denial.into()))
            }
            WorthQueryInspectionTarget::BasisIntentDenial(denial) => {
                Ok(WorthQueryInspection::BasisLifecycle(denial.into()))
            }
            WorthQueryInspectionTarget::DerivedView { name } => {
                let review = self.review_runtime_derived_inspection(name.to_string())?;
                let handoff = self.resolve_reviewed_admitted_derived_inspection_handoff(review)?;
                let binding = self.prepare_derived_inspection_execution_binding(handoff);
                let result = self.execute_derived_inspection_execution_binding(binding)?;
                Ok(WorthQueryInspection::DerivedView(result.evidence().clone()))
            }
            target => {
                let seed = crate::intent_admission::WorthQueryGenericInspectionIntentSeed::from_target(target)
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
