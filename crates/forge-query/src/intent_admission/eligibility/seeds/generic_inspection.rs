use crate::identity::hash_parts;
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
pub struct ForgeQueryGenericInspectionIntentSeed {
    target: ForgeQueryGenericInspectionIntentTargetSeed,
    request_label: String,
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
            format!("inspect.live.{view_name}"),
            hash_parts(&[
                "forge_query_generic_inspection_live_view_seed_v1".to_string(),
                format!("view:{view_name}"),
            ]),
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
            format!("inspect.effect.{effect_name}"),
            hash_parts(&[
                "forge_query_generic_inspection_effect_seed_v1".to_string(),
                format!("effect:{effect_name}"),
            ]),
        )
    }

    pub(crate) fn write_receipt(receipt: &ForgeQueryWriteReceipt) -> Self {
        Self::new(
            ForgeQueryGenericInspectionIntentTargetSeed::WriteReceipt(receipt.clone()),
            format!("inspect.write_receipt.{}", receipt.commit_identity()),
            hash_parts(&[
                "forge_query_generic_inspection_write_receipt_seed_v1".to_string(),
                format!("commit:{}", receipt.commit_identity()),
                format!("snapshot:{}", receipt.snapshot_token()),
            ]),
        )
    }

    pub(crate) fn batch_write_receipt(receipt: &ForgeQueryBatchWriteReceipt) -> Self {
        Self::new(
            ForgeQueryGenericInspectionIntentTargetSeed::BatchWriteReceipt(receipt.clone()),
            format!("inspect.batch_write_receipt.{}", receipt.batch_digest()),
            hash_parts(&[
                "forge_query_generic_inspection_batch_write_receipt_seed_v1".to_string(),
                format!("receipt:{}", receipt.batch_digest()),
            ]),
        )
    }

    pub(crate) fn intent_receipt(receipt: &ForgeQueryIntentReceipt) -> Self {
        Self::new(
            ForgeQueryGenericInspectionIntentTargetSeed::IntentReceipt(receipt.clone()),
            format!("inspect.intent_receipt.{}", receipt.intent_name()),
            hash_parts(&[
                "forge_query_generic_inspection_intent_receipt_seed_v1".to_string(),
                format!("receipt:{}", receipt.receipt_digest()),
            ]),
        )
    }

    pub(crate) fn intent_denial(evidence: &ForgeQueryIntentDenialEvidence) -> Self {
        Self::new(
            ForgeQueryGenericInspectionIntentTargetSeed::IntentDenial(evidence.clone()),
            format!("inspect.intent_denial.{}", evidence.intent_name()),
            hash_parts(&[
                "forge_query_generic_inspection_intent_denial_seed_v1".to_string(),
                format!("evidence:{}", evidence.denial_digest()),
            ]),
        )
    }

    pub(crate) fn effect_intent_receipt(receipt: &ForgeQueryEffectIntentReceipt) -> Self {
        Self::new(
            ForgeQueryGenericInspectionIntentTargetSeed::EffectIntentReceipt(receipt.clone()),
            format!("inspect.effect_intent_receipt.{}", receipt.effect_name()),
            hash_parts(&[
                "forge_query_generic_inspection_effect_intent_receipt_seed_v1".to_string(),
                format!("receipt:{}", receipt.receipt_digest()),
            ]),
        )
    }

    pub(crate) fn preview_binding(binding: &ForgeQueryPreviewHandleBindingEvidence) -> Self {
        Self::new(
            ForgeQueryGenericInspectionIntentTargetSeed::PreviewBinding(binding.clone()),
            format!("inspect.preview_binding.{}", binding.label()),
            hash_parts(&[
                "forge_query_generic_inspection_preview_binding_seed_v1".to_string(),
                format!("label:{}", binding.label()),
                format!("handle:{}", binding.handle_name()),
            ]),
        )
    }

    pub(crate) fn preview_outcome(outcome: &ForgeQueryPreviewOutcome) -> Self {
        Self::new(
            ForgeQueryGenericInspectionIntentTargetSeed::PreviewOutcome(outcome.clone()),
            format!("inspect.preview_outcome.{}", outcome.label()),
            hash_parts(&[
                "forge_query_generic_inspection_preview_outcome_seed_v1".to_string(),
                format!("label:{}", outcome.label()),
                format!("writes:{}", outcome.write_count()),
                format!("promoted:{}", outcome.promoted()),
                format!("discarded:{}", outcome.discarded()),
            ]),
        )
    }

    pub(crate) fn preview_intent_receipt(receipt: &ForgeQueryPreviewIntentReceipt) -> Self {
        Self::new(
            ForgeQueryGenericInspectionIntentTargetSeed::PreviewIntentReceipt(receipt.clone()),
            format!("inspect.preview_intent_receipt.{}", receipt.intent_name()),
            hash_parts(&[
                "forge_query_generic_inspection_preview_intent_receipt_seed_v1".to_string(),
                format!("receipt:{}", receipt.receipt_digest()),
            ]),
        )
    }

    pub(crate) fn branch_intent_receipt(receipt: &ForgeQueryBranchIntentReceipt) -> Self {
        Self::new(
            ForgeQueryGenericInspectionIntentTargetSeed::BranchIntentReceipt(receipt.clone()),
            format!("inspect.branch_intent_receipt.{}", receipt.intent_name()),
            hash_parts(&[
                "forge_query_generic_inspection_branch_intent_receipt_seed_v1".to_string(),
                format!("receipt:{}", receipt.receipt_digest()),
            ]),
        )
    }

    fn new(
        target: ForgeQueryGenericInspectionIntentTargetSeed,
        request_label: String,
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

    pub fn request_label(&self) -> &str {
        &self.request_label
    }

    pub fn request_input_digest(&self) -> &str {
        &self.request_input_digest
    }
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
