use super::{ForgeQueryBatchWriteReceiptInspection, ForgeQueryWriteReceiptInspection};
use crate::runtime::{
    ForgeQueryBatchWriteReceipt, ForgeQueryBranchIntentReceipt,
    ForgeQueryBranchIntentReceiptInspection, ForgeQueryComputedInspectionEvidence,
    ForgeQueryDerivedViewHandle, ForgeQueryEffectHandle, ForgeQueryEffectInspectionEvidence,
    ForgeQueryEffectIntentReceipt, ForgeQueryEffectIntentReceiptInspection,
    ForgeQueryIntentDenialEvidence, ForgeQueryIntentDenialInspection, ForgeQueryIntentReceipt,
    ForgeQueryIntentReceiptInspection, ForgeQueryLiveView, ForgeQueryLiveViewInspection,
    ForgeQueryPreviewBindingInspection, ForgeQueryPreviewHandleBindingEvidence,
    ForgeQueryPreviewIntentReceipt, ForgeQueryPreviewIntentReceiptInspection,
    ForgeQueryPreviewOutcome, ForgeQueryPreviewOutcomeInspection, ForgeQueryWriteReceipt,
};

pub enum ForgeQueryInspectionTarget<'a> {
    LiveView { name: &'a str },
    DerivedView { name: &'a str },
    Effect { name: &'a str },
    WriteReceipt(&'a ForgeQueryWriteReceipt),
    BatchWriteReceipt(&'a ForgeQueryBatchWriteReceipt),
    IntentReceipt(&'a ForgeQueryIntentReceipt),
    IntentDenial(&'a ForgeQueryIntentDenialEvidence),
    EffectIntentReceipt(&'a ForgeQueryEffectIntentReceipt),
    PreviewBinding(&'a ForgeQueryPreviewHandleBindingEvidence),
    PreviewOutcome(&'a ForgeQueryPreviewOutcome),
    PreviewIntentReceipt(&'a ForgeQueryPreviewIntentReceipt),
    BranchIntentReceipt(&'a ForgeQueryBranchIntentReceipt),
}

impl<'a, T> From<&'a ForgeQueryLiveView<T>> for ForgeQueryInspectionTarget<'a> {
    fn from(value: &'a ForgeQueryLiveView<T>) -> Self {
        Self::LiveView { name: value.name() }
    }
}

impl<'a, T> From<&'a ForgeQueryDerivedViewHandle<T>> for ForgeQueryInspectionTarget<'a> {
    fn from(value: &'a ForgeQueryDerivedViewHandle<T>) -> Self {
        Self::DerivedView { name: value.name() }
    }
}

impl<'a, T> From<&'a ForgeQueryEffectHandle<T>> for ForgeQueryInspectionTarget<'a> {
    fn from(value: &'a ForgeQueryEffectHandle<T>) -> Self {
        Self::Effect { name: value.name() }
    }
}

impl<'a> From<&'a ForgeQueryWriteReceipt> for ForgeQueryInspectionTarget<'a> {
    fn from(value: &'a ForgeQueryWriteReceipt) -> Self {
        Self::WriteReceipt(value)
    }
}

impl<'a> From<&'a ForgeQueryBatchWriteReceipt> for ForgeQueryInspectionTarget<'a> {
    fn from(value: &'a ForgeQueryBatchWriteReceipt) -> Self {
        Self::BatchWriteReceipt(value)
    }
}

impl<'a> From<&'a ForgeQueryIntentReceipt> for ForgeQueryInspectionTarget<'a> {
    fn from(value: &'a ForgeQueryIntentReceipt) -> Self {
        Self::IntentReceipt(value)
    }
}

impl<'a> From<&'a ForgeQueryIntentDenialEvidence> for ForgeQueryInspectionTarget<'a> {
    fn from(value: &'a ForgeQueryIntentDenialEvidence) -> Self {
        Self::IntentDenial(value)
    }
}

impl<'a> From<&'a ForgeQueryEffectIntentReceipt> for ForgeQueryInspectionTarget<'a> {
    fn from(value: &'a ForgeQueryEffectIntentReceipt) -> Self {
        Self::EffectIntentReceipt(value)
    }
}

impl<'a> From<&'a ForgeQueryPreviewHandleBindingEvidence> for ForgeQueryInspectionTarget<'a> {
    fn from(value: &'a ForgeQueryPreviewHandleBindingEvidence) -> Self {
        Self::PreviewBinding(value)
    }
}

impl<'a> From<&'a ForgeQueryPreviewOutcome> for ForgeQueryInspectionTarget<'a> {
    fn from(value: &'a ForgeQueryPreviewOutcome) -> Self {
        Self::PreviewOutcome(value)
    }
}

impl<'a> From<&'a ForgeQueryPreviewIntentReceipt> for ForgeQueryInspectionTarget<'a> {
    fn from(value: &'a ForgeQueryPreviewIntentReceipt) -> Self {
        Self::PreviewIntentReceipt(value)
    }
}

impl<'a> From<&'a ForgeQueryBranchIntentReceipt> for ForgeQueryInspectionTarget<'a> {
    fn from(value: &'a ForgeQueryBranchIntentReceipt) -> Self {
        Self::BranchIntentReceipt(value)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeQueryInspection {
    LiveView(ForgeQueryLiveViewInspection),
    DerivedView(ForgeQueryComputedInspectionEvidence),
    Effect(ForgeQueryEffectInspectionEvidence),
    WriteReceipt(ForgeQueryWriteReceiptInspection),
    BatchWriteReceipt(ForgeQueryBatchWriteReceiptInspection),
    IntentReceipt(ForgeQueryIntentReceiptInspection),
    IntentDenial(ForgeQueryIntentDenialInspection),
    EffectIntentReceipt(ForgeQueryEffectIntentReceiptInspection),
    PreviewBinding(ForgeQueryPreviewBindingInspection),
    PreviewOutcome(ForgeQueryPreviewOutcomeInspection),
    PreviewIntentReceipt(ForgeQueryPreviewIntentReceiptInspection),
    BranchIntentReceipt(ForgeQueryBranchIntentReceiptInspection),
}
