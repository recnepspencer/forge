use super::*;

impl ForgeQueryRuntime {
    pub fn inspect_live_view<T>(
        &self,
        view: &ForgeQueryLiveView<T>,
    ) -> Result<&ForgeQueryRuntimeLiveSubscriptionInstallation, ForgeQueryRuntimeError> {
        self.admit_facade_family(ForgeQueryRuntimeFacadeFamily::Inspect)?;
        self.live_subscriptions
            .get(view.name())
            .map(|state| &state.installation)
            .ok_or_else(|| ForgeQueryRuntimeError::MissingLiveSubscription(view.name().to_string()))
    }

    pub fn inspect_live_view_explanation<T>(
        &self,
        view: &ForgeQueryLiveView<T>,
    ) -> Result<ForgeQueryLiveViewInspection, ForgeQueryRuntimeError> {
        let installation = self.inspect_live_view(view)?;
        Ok(ForgeQueryLiveViewInspection::from_installation(
            installation,
        ))
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
        self.admit_facade_family(ForgeQueryRuntimeFacadeFamily::Inspect)?;
        Ok(ForgeQueryIntentReceiptInspection::from_receipt(receipt))
    }

    pub fn inspect_effect_intent_receipt(
        &self,
        receipt: &ForgeQueryEffectIntentReceipt,
    ) -> Result<ForgeQueryEffectIntentReceiptInspection, ForgeQueryRuntimeError> {
        self.admit_facade_family(ForgeQueryRuntimeFacadeFamily::Inspect)?;
        Ok(ForgeQueryEffectIntentReceiptInspection::from_receipt(
            receipt,
        ))
    }

    pub fn inspect_intent_denial(
        &self,
        evidence: &ForgeQueryIntentDenialEvidence,
    ) -> Result<ForgeQueryIntentDenialInspection, ForgeQueryRuntimeError> {
        self.admit_facade_family(ForgeQueryRuntimeFacadeFamily::Inspect)?;
        Ok(ForgeQueryIntentDenialInspection::from_evidence(evidence))
    }

    pub fn inspect_preview_binding(
        &self,
        binding: &ForgeQueryPreviewHandleBindingEvidence,
    ) -> Result<ForgeQueryPreviewBindingInspection, ForgeQueryRuntimeError> {
        self.admit_facade_family(ForgeQueryRuntimeFacadeFamily::Inspect)?;
        Ok(ForgeQueryPreviewBindingInspection::from_binding(binding))
    }

    pub fn inspect_preview_outcome(
        &self,
        outcome: &ForgeQueryPreviewOutcome,
    ) -> Result<ForgeQueryPreviewOutcomeInspection, ForgeQueryRuntimeError> {
        self.admit_facade_family(ForgeQueryRuntimeFacadeFamily::Inspect)?;
        Ok(ForgeQueryPreviewOutcomeInspection::from_outcome(outcome))
    }

    pub fn inspect_preview_intent_receipt(
        &self,
        receipt: &ForgeQueryPreviewIntentReceipt,
    ) -> Result<ForgeQueryPreviewIntentReceiptInspection, ForgeQueryRuntimeError> {
        self.admit_facade_family(ForgeQueryRuntimeFacadeFamily::Inspect)?;
        Ok(ForgeQueryPreviewIntentReceiptInspection::from_receipt(
            receipt,
        ))
    }

    pub fn inspect_branch_intent_receipt(
        &self,
        receipt: &ForgeQueryBranchIntentReceipt,
    ) -> Result<ForgeQueryBranchIntentReceiptInspection, ForgeQueryRuntimeError> {
        self.admit_facade_family(ForgeQueryRuntimeFacadeFamily::Inspect)?;
        Ok(ForgeQueryBranchIntentReceiptInspection::from_receipt(
            receipt,
        ))
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
            ForgeQueryInspectionTarget::LiveView { name } => {
                let installation = self
                    .live_subscriptions
                    .get(name)
                    .map(|state| &state.installation)
                    .ok_or_else(|| {
                        ForgeQueryRuntimeError::MissingLiveSubscription(name.to_string())
                    })?;
                Ok(ForgeQueryInspection::LiveView(
                    ForgeQueryLiveViewInspection::from_installation(installation),
                ))
            }
            ForgeQueryInspectionTarget::DerivedView { name } => {
                Ok(ForgeQueryInspection::DerivedView(
                    self.derived_views
                        .get(name)
                        .map(ForgeQueryComputedInspectionEvidence::from_runtime)
                        .ok_or_else(|| {
                            ForgeQueryRuntimeError::MissingDerivedView(name.to_string())
                        })?,
                ))
            }
            ForgeQueryInspectionTarget::Effect { name } => Ok(ForgeQueryInspection::Effect(
                self.inspect_effect_by_name(name)?,
            )),
            ForgeQueryInspectionTarget::WriteReceipt(receipt) => {
                let runtime_evidence = self
                    .backend
                    .inspect_write_receipt(receipt, &self.evidence_authority)?;
                Ok(ForgeQueryInspection::WriteReceipt(
                    ForgeQueryWriteReceiptInspection::new(receipt, runtime_evidence),
                ))
            }
            ForgeQueryInspectionTarget::BatchWriteReceipt(receipt) => {
                Ok(ForgeQueryInspection::BatchWriteReceipt(
                    ForgeQueryBatchWriteReceiptInspection::new(receipt),
                ))
            }
            ForgeQueryInspectionTarget::IntentReceipt(receipt) => Ok(
                ForgeQueryInspection::IntentReceipt(self.inspect_intent_receipt(receipt)?),
            ),
            ForgeQueryInspectionTarget::IntentDenial(evidence) => Ok(
                ForgeQueryInspection::IntentDenial(self.inspect_intent_denial(evidence)?),
            ),
            ForgeQueryInspectionTarget::EffectIntentReceipt(receipt) => {
                Ok(ForgeQueryInspection::EffectIntentReceipt(
                    self.inspect_effect_intent_receipt(receipt)?,
                ))
            }
            ForgeQueryInspectionTarget::PreviewBinding(binding) => Ok(
                ForgeQueryInspection::PreviewBinding(self.inspect_preview_binding(binding)?),
            ),
            ForgeQueryInspectionTarget::PreviewOutcome(outcome) => Ok(
                ForgeQueryInspection::PreviewOutcome(self.inspect_preview_outcome(outcome)?),
            ),
            ForgeQueryInspectionTarget::PreviewIntentReceipt(receipt) => {
                Ok(ForgeQueryInspection::PreviewIntentReceipt(
                    self.inspect_preview_intent_receipt(receipt)?,
                ))
            }
            ForgeQueryInspectionTarget::BranchIntentReceipt(receipt) => {
                Ok(ForgeQueryInspection::BranchIntentReceipt(
                    self.inspect_branch_intent_receipt(receipt)?,
                ))
            }
        }
    }
}
