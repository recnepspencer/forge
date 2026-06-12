use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};
use crate::runtime::{
    ForgeQueryBatchWriteReceipt, ForgeQueryBranchIntentReceipt, ForgeQueryEffectHandle,
    ForgeQueryEffectIntentReceipt, ForgeQueryInspectionTarget, ForgeQueryIntentDenialEvidence,
    ForgeQueryIntentReceipt, ForgeQueryLiveView, ForgeQueryPreviewHandleBindingEvidence,
    ForgeQueryPreviewIntentReceipt, ForgeQueryPreviewOutcome, ForgeQueryWriteReceipt,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeQueryGenericInspectionIntentTargetSeed {
    LiveView { view_name: String },
    Effect { effect_name: String },
    WriteReceipt(ForgeQueryWriteReceipt),
    BatchWriteReceipt(ForgeQueryBatchWriteReceipt),
    IntentReceipt(ForgeQueryIntentReceipt),
    IntentDenial(ForgeQueryIntentDenialEvidence),
    EffectIntentReceipt(ForgeQueryEffectIntentReceipt),
    PreviewBinding(ForgeQueryPreviewHandleBindingEvidence),
    PreviewOutcome(ForgeQueryPreviewOutcome),
    PreviewIntentReceipt(ForgeQueryPreviewIntentReceipt),
    BranchIntentReceipt(ForgeQueryBranchIntentReceipt),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGenericInspectionRequestLabel(String);

impl ForgeQueryGenericInspectionRequestLabel {
    fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for ForgeQueryGenericInspectionRequestLabel {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGenericInspectionIntentSeed {
    target: ForgeQueryGenericInspectionIntentTargetSeed,
    request_label: ForgeQueryGenericInspectionRequestLabel,
    request_input_digest: String,
}

pub trait ForgeQueryGenericInspectionIntentTarget<'a> {
    fn into_generic_inspection_intent_seed(self) -> ForgeQueryGenericInspectionIntentSeed;
}

impl ForgeQueryGenericInspectionIntentSeed {
    pub(crate) fn from_target(target: ForgeQueryInspectionTarget<'_>) -> Option<Self> {
        match target {
            ForgeQueryInspectionTarget::LiveView { name } => Some(Self::from_live_view_name(name)),
            ForgeQueryInspectionTarget::DerivedView { .. } => None,
            ForgeQueryInspectionTarget::Effect { name } => Some(Self::from_effect_name(name)),
            ForgeQueryInspectionTarget::WriteReceipt(receipt) => Some(Self::write_receipt(receipt)),
            ForgeQueryInspectionTarget::BatchWriteReceipt(receipt) => {
                Some(Self::batch_write_receipt(receipt))
            }
            ForgeQueryInspectionTarget::IntentReceipt(receipt) => {
                Some(Self::intent_receipt(receipt))
            }
            ForgeQueryInspectionTarget::IntentDenial(evidence) => {
                Some(Self::intent_denial(evidence))
            }
            ForgeQueryInspectionTarget::EffectIntentReceipt(receipt) => {
                Some(Self::effect_intent_receipt(receipt))
            }
            ForgeQueryInspectionTarget::PreviewBinding(binding) => {
                Some(Self::preview_binding(binding))
            }
            ForgeQueryInspectionTarget::PreviewOutcome(outcome) => {
                Some(Self::preview_outcome(outcome))
            }
            ForgeQueryInspectionTarget::PreviewIntentReceipt(receipt) => {
                Some(Self::preview_intent_receipt(receipt))
            }
            ForgeQueryInspectionTarget::BranchIntentReceipt(receipt) => {
                Some(Self::branch_intent_receipt(receipt))
            }
            ForgeQueryInspectionTarget::AdmittedWorldBasis(_)
            | ForgeQueryInspectionTarget::ObservationBasisCapability(_)
            | ForgeQueryInspectionTarget::InspectionBasisCapability(_)
            | ForgeQueryInspectionTarget::SubscriptionDeclarationBasisCapability(_)
            | ForgeQueryInspectionTarget::SubscriptionActivationBasisCapability(_)
            | ForgeQueryInspectionTarget::ScopedObservationBasis(_)
            | ForgeQueryInspectionTarget::ScopedInspectionBasis(_)
            | ForgeQueryInspectionTarget::ScopedReplayBasis(_)
            | ForgeQueryInspectionTarget::ScopedSubscriptionDeclarationBasis(_)
            | ForgeQueryInspectionTarget::ScopedSubscriptionActivationBasis(_)
            | ForgeQueryInspectionTarget::LowerRuntimeBoundObservationBasis(_)
            | ForgeQueryInspectionTarget::LowerRuntimeBoundInspectionBasis(_)
            | ForgeQueryInspectionTarget::LowerRuntimeBoundSubscriptionDeclarationBasis(_)
            | ForgeQueryInspectionTarget::LowerRuntimeBoundSubscriptionActivationBasis(_)
            | ForgeQueryInspectionTarget::DeniedBasisCapability(_)
            | ForgeQueryInspectionTarget::BasisIntentDenial(_) => None,
        }
    }

    pub(crate) fn live_view<T>(view: &ForgeQueryLiveView<T>) -> Self {
        Self::from_live_view_name(view.name())
    }

    fn from_live_view_name(view_name: &str) -> Self {
        Self::new(
            ForgeQueryGenericInspectionIntentTargetSeed::LiveView {
                view_name: view_name.to_string(),
            },
            ForgeQueryGenericInspectionRequestLabel::new(format!("inspect.live.{view_name}")),
            generic_inspection_seed_identity("live_view")
                .field_shape(ForgeQueryEvidenceTag::new("view_name"), view_name)
                .seal()
                .as_str()
                .to_string(),
        )
    }

    pub(crate) fn effect<T>(effect: &ForgeQueryEffectHandle<T>) -> Self {
        Self::from_effect_name(effect.name())
    }

    fn from_effect_name(effect_name: &str) -> Self {
        Self::new(
            ForgeQueryGenericInspectionIntentTargetSeed::Effect {
                effect_name: effect_name.to_string(),
            },
            ForgeQueryGenericInspectionRequestLabel::new(format!("inspect.effect.{effect_name}")),
            generic_inspection_seed_identity("effect")
                .field_shape(ForgeQueryEvidenceTag::new("effect_name"), effect_name)
                .seal()
                .as_str()
                .to_string(),
        )
    }

    pub(crate) fn write_receipt(receipt: &ForgeQueryWriteReceipt) -> Self {
        Self::new(
            ForgeQueryGenericInspectionIntentTargetSeed::WriteReceipt(receipt.clone()),
            ForgeQueryGenericInspectionRequestLabel::new(format!(
                "inspect.write_receipt.{}",
                receipt.commit_identity()
            )),
            generic_inspection_seed_identity("write_receipt")
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("commit_identity"),
                    receipt.commit_evidence_identity(),
                )
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("snapshot_token"),
                    receipt.snapshot_evidence_identity(),
                )
                .seal()
                .as_str()
                .to_string(),
        )
    }

    pub(crate) fn batch_write_receipt(receipt: &ForgeQueryBatchWriteReceipt) -> Self {
        Self::new(
            ForgeQueryGenericInspectionIntentTargetSeed::BatchWriteReceipt(receipt.clone()),
            ForgeQueryGenericInspectionRequestLabel::new(format!(
                "inspect.batch_write_receipt.{}",
                receipt.batch_digest()
            )),
            generic_inspection_seed_identity("batch_write_receipt")
                .field_identity(
                    ForgeQueryEvidenceTag::new("batch_receipt"),
                    receipt.batch_digest(),
                )
                .seal()
                .as_str()
                .to_string(),
        )
    }

    pub(crate) fn intent_receipt(receipt: &ForgeQueryIntentReceipt) -> Self {
        Self::new(
            ForgeQueryGenericInspectionIntentTargetSeed::IntentReceipt(receipt.clone()),
            ForgeQueryGenericInspectionRequestLabel::new(format!(
                "inspect.intent_receipt.{}",
                receipt.intent_name()
            )),
            receipt_seed_input_digest("intent_receipt", receipt.receipt_identity()),
        )
    }

    pub(crate) fn intent_denial(evidence: &ForgeQueryIntentDenialEvidence) -> Self {
        Self::new(
            ForgeQueryGenericInspectionIntentTargetSeed::IntentDenial(evidence.clone()),
            ForgeQueryGenericInspectionRequestLabel::new(format!(
                "inspect.intent_denial.{}",
                evidence.intent_name()
            )),
            receipt_seed_input_digest("intent_denial", evidence.denial_digest()),
        )
    }

    pub(crate) fn effect_intent_receipt(receipt: &ForgeQueryEffectIntentReceipt) -> Self {
        Self::new(
            ForgeQueryGenericInspectionIntentTargetSeed::EffectIntentReceipt(receipt.clone()),
            ForgeQueryGenericInspectionRequestLabel::new(format!(
                "inspect.effect_intent_receipt.{}",
                receipt.effect_name()
            )),
            receipt_seed_input_digest("effect_intent_receipt", receipt.receipt_identity()),
        )
    }

    pub(crate) fn preview_binding(binding: &ForgeQueryPreviewHandleBindingEvidence) -> Self {
        Self::new(
            ForgeQueryGenericInspectionIntentTargetSeed::PreviewBinding(binding.clone()),
            ForgeQueryGenericInspectionRequestLabel::new(format!(
                "inspect.preview_binding.{}",
                binding.session_label().display()
            )),
            generic_inspection_seed_identity("preview_binding")
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("label_identity"),
                    binding.label_identity(),
                )
                .field_shape(
                    ForgeQueryEvidenceTag::new("handle_name"),
                    binding.handle_name(),
                )
                .seal()
                .as_str()
                .to_string(),
        )
    }

    pub(crate) fn preview_outcome(outcome: &ForgeQueryPreviewOutcome) -> Self {
        Self::new(
            ForgeQueryGenericInspectionIntentTargetSeed::PreviewOutcome(outcome.clone()),
            ForgeQueryGenericInspectionRequestLabel::new(format!(
                "inspect.preview_outcome.{}",
                outcome.label()
            )),
            generic_inspection_seed_identity("preview_outcome")
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("label_identity"),
                    outcome.session_label().identity_digest(),
                )
                .field_usize(
                    ForgeQueryEvidenceTag::new("write_count"),
                    outcome.write_count(),
                )
                .field_bool(ForgeQueryEvidenceTag::new("promoted"), outcome.promoted())
                .field_bool(ForgeQueryEvidenceTag::new("discarded"), outcome.discarded())
                .seal()
                .as_str()
                .to_string(),
        )
    }

    pub(crate) fn preview_intent_receipt(receipt: &ForgeQueryPreviewIntentReceipt) -> Self {
        Self::new(
            ForgeQueryGenericInspectionIntentTargetSeed::PreviewIntentReceipt(receipt.clone()),
            ForgeQueryGenericInspectionRequestLabel::new(format!(
                "inspect.preview_intent_receipt.{}",
                receipt.intent_name()
            )),
            receipt_seed_input_digest("preview_intent_receipt", receipt.receipt_identity()),
        )
    }

    pub(crate) fn branch_intent_receipt(receipt: &ForgeQueryBranchIntentReceipt) -> Self {
        Self::new(
            ForgeQueryGenericInspectionIntentTargetSeed::BranchIntentReceipt(receipt.clone()),
            ForgeQueryGenericInspectionRequestLabel::new(format!(
                "inspect.branch_intent_receipt.{}",
                receipt.intent_name()
            )),
            receipt_seed_input_digest("branch_intent_receipt", receipt.receipt_identity()),
        )
    }

    fn new(
        target: ForgeQueryGenericInspectionIntentTargetSeed,
        request_label: ForgeQueryGenericInspectionRequestLabel,
        request_input_digest: String,
    ) -> Self {
        Self {
            target,
            request_label,
            request_input_digest,
        }
    }

    pub fn target(&self) -> &ForgeQueryGenericInspectionIntentTargetSeed {
        &self.target
    }

    pub fn request_label(&self) -> &ForgeQueryGenericInspectionRequestLabel {
        &self.request_label
    }

    pub fn request_input_digest(&self) -> &str {
        &self.request_input_digest
    }
}

fn receipt_seed_input_digest(
    target_family: &'static str,
    receipt_identity: &ForgeQueryEvidenceIdentity,
) -> String {
    generic_inspection_seed_identity(target_family)
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("receipt_digest"),
            receipt_identity,
        )
        .seal()
        .as_str()
        .to_string()
}

fn generic_inspection_seed_identity(
    target_family: &'static str,
) -> crate::ForgeQueryEvidenceIdentityEncoder {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::GenericInspectionIntentSeed)
        .field_shape(ForgeQueryEvidenceTag::new("target_family"), target_family)
}

impl<'a, T> ForgeQueryGenericInspectionIntentTarget<'a> for &'a ForgeQueryLiveView<T> {
    fn into_generic_inspection_intent_seed(self) -> ForgeQueryGenericInspectionIntentSeed {
        ForgeQueryGenericInspectionIntentSeed::live_view(self)
    }
}

impl<'a, T> ForgeQueryGenericInspectionIntentTarget<'a> for &'a ForgeQueryEffectHandle<T> {
    fn into_generic_inspection_intent_seed(self) -> ForgeQueryGenericInspectionIntentSeed {
        ForgeQueryGenericInspectionIntentSeed::effect(self)
    }
}

impl<'a> ForgeQueryGenericInspectionIntentTarget<'a> for &'a ForgeQueryWriteReceipt {
    fn into_generic_inspection_intent_seed(self) -> ForgeQueryGenericInspectionIntentSeed {
        ForgeQueryGenericInspectionIntentSeed::write_receipt(self)
    }
}

impl<'a> ForgeQueryGenericInspectionIntentTarget<'a> for &'a ForgeQueryBatchWriteReceipt {
    fn into_generic_inspection_intent_seed(self) -> ForgeQueryGenericInspectionIntentSeed {
        ForgeQueryGenericInspectionIntentSeed::batch_write_receipt(self)
    }
}

impl<'a> ForgeQueryGenericInspectionIntentTarget<'a> for &'a ForgeQueryIntentReceipt {
    fn into_generic_inspection_intent_seed(self) -> ForgeQueryGenericInspectionIntentSeed {
        ForgeQueryGenericInspectionIntentSeed::intent_receipt(self)
    }
}

impl<'a> ForgeQueryGenericInspectionIntentTarget<'a> for &'a ForgeQueryIntentDenialEvidence {
    fn into_generic_inspection_intent_seed(self) -> ForgeQueryGenericInspectionIntentSeed {
        ForgeQueryGenericInspectionIntentSeed::intent_denial(self)
    }
}

impl<'a> ForgeQueryGenericInspectionIntentTarget<'a> for &'a ForgeQueryEffectIntentReceipt {
    fn into_generic_inspection_intent_seed(self) -> ForgeQueryGenericInspectionIntentSeed {
        ForgeQueryGenericInspectionIntentSeed::effect_intent_receipt(self)
    }
}

impl<'a> ForgeQueryGenericInspectionIntentTarget<'a>
    for &'a ForgeQueryPreviewHandleBindingEvidence
{
    fn into_generic_inspection_intent_seed(self) -> ForgeQueryGenericInspectionIntentSeed {
        ForgeQueryGenericInspectionIntentSeed::preview_binding(self)
    }
}

impl<'a> ForgeQueryGenericInspectionIntentTarget<'a> for &'a ForgeQueryPreviewOutcome {
    fn into_generic_inspection_intent_seed(self) -> ForgeQueryGenericInspectionIntentSeed {
        ForgeQueryGenericInspectionIntentSeed::preview_outcome(self)
    }
}

impl<'a> ForgeQueryGenericInspectionIntentTarget<'a> for &'a ForgeQueryPreviewIntentReceipt {
    fn into_generic_inspection_intent_seed(self) -> ForgeQueryGenericInspectionIntentSeed {
        ForgeQueryGenericInspectionIntentSeed::preview_intent_receipt(self)
    }
}

impl<'a> ForgeQueryGenericInspectionIntentTarget<'a> for &'a ForgeQueryBranchIntentReceipt {
    fn into_generic_inspection_intent_seed(self) -> ForgeQueryGenericInspectionIntentSeed {
        ForgeQueryGenericInspectionIntentSeed::branch_intent_receipt(self)
    }
}
