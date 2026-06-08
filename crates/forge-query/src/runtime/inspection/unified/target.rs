use super::{ForgeQueryBatchWriteReceiptInspection, ForgeQueryWriteReceiptInspection};
use crate::application::ForgeQueryAdmittedWorldBasis;
use crate::query_basis_lifecycle::{
    BasisIntentDenial, DeniedBasisCapability, InspectionBasisCapability,
    LowerRuntimeBoundInspectionBasis, LowerRuntimeBoundObservationBasis,
    LowerRuntimeBoundSubscriptionActivationBasis, LowerRuntimeBoundSubscriptionDeclarationBasis,
    ObservationBasisCapability, ScopedInspectionBasis, ScopedObservationBasis, ScopedReplayBasis,
    ScopedSubscriptionActivationBasis, ScopedSubscriptionDeclarationBasis,
    SubscriptionActivationBasisCapability, SubscriptionDeclarationBasisCapability,
};
use crate::runtime::{
    ForgeQueryBasisLifecycleInspection, ForgeQueryBatchWriteReceipt, ForgeQueryBranchIntentReceipt,
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
    LiveView {
        name: &'a str,
    },
    DerivedView {
        name: &'a str,
    },
    Effect {
        name: &'a str,
    },
    WriteReceipt(&'a ForgeQueryWriteReceipt),
    BatchWriteReceipt(&'a ForgeQueryBatchWriteReceipt),
    IntentReceipt(&'a ForgeQueryIntentReceipt),
    IntentDenial(&'a ForgeQueryIntentDenialEvidence),
    EffectIntentReceipt(&'a ForgeQueryEffectIntentReceipt),
    PreviewBinding(&'a ForgeQueryPreviewHandleBindingEvidence),
    PreviewOutcome(&'a ForgeQueryPreviewOutcome),
    PreviewIntentReceipt(&'a ForgeQueryPreviewIntentReceipt),
    BranchIntentReceipt(&'a ForgeQueryBranchIntentReceipt),
    AdmittedWorldBasis(&'a ForgeQueryAdmittedWorldBasis),
    ObservationBasisCapability(&'a ObservationBasisCapability),
    InspectionBasisCapability(&'a InspectionBasisCapability),
    SubscriptionDeclarationBasisCapability(&'a SubscriptionDeclarationBasisCapability),
    SubscriptionActivationBasisCapability(&'a SubscriptionActivationBasisCapability),
    ScopedObservationBasis(&'a ScopedObservationBasis),
    ScopedInspectionBasis(&'a ScopedInspectionBasis),
    ScopedReplayBasis(&'a ScopedReplayBasis),
    ScopedSubscriptionDeclarationBasis(&'a ScopedSubscriptionDeclarationBasis),
    ScopedSubscriptionActivationBasis(&'a ScopedSubscriptionActivationBasis),
    LowerRuntimeBoundObservationBasis(&'a LowerRuntimeBoundObservationBasis),
    LowerRuntimeBoundInspectionBasis(&'a LowerRuntimeBoundInspectionBasis),
    LowerRuntimeBoundSubscriptionDeclarationBasis(
        &'a LowerRuntimeBoundSubscriptionDeclarationBasis,
    ),
    LowerRuntimeBoundSubscriptionActivationBasis(&'a LowerRuntimeBoundSubscriptionActivationBasis),
    DeniedBasisCapability(&'a DeniedBasisCapability),
    BasisIntentDenial(&'a BasisIntentDenial),
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

macro_rules! impl_basis_target {
    ($target:ty, $variant:ident) => {
        impl<'a> From<&'a $target> for ForgeQueryInspectionTarget<'a> {
            fn from(value: &'a $target) -> Self {
                Self::$variant(value)
            }
        }
    };
}

impl_basis_target!(ForgeQueryAdmittedWorldBasis, AdmittedWorldBasis);
impl_basis_target!(ObservationBasisCapability, ObservationBasisCapability);
impl_basis_target!(InspectionBasisCapability, InspectionBasisCapability);
impl_basis_target!(
    SubscriptionDeclarationBasisCapability,
    SubscriptionDeclarationBasisCapability
);
impl_basis_target!(
    SubscriptionActivationBasisCapability,
    SubscriptionActivationBasisCapability
);
impl_basis_target!(ScopedObservationBasis, ScopedObservationBasis);
impl_basis_target!(ScopedInspectionBasis, ScopedInspectionBasis);
impl_basis_target!(ScopedReplayBasis, ScopedReplayBasis);
impl_basis_target!(
    ScopedSubscriptionDeclarationBasis,
    ScopedSubscriptionDeclarationBasis
);
impl_basis_target!(
    ScopedSubscriptionActivationBasis,
    ScopedSubscriptionActivationBasis
);
impl_basis_target!(
    LowerRuntimeBoundObservationBasis,
    LowerRuntimeBoundObservationBasis
);
impl_basis_target!(
    LowerRuntimeBoundInspectionBasis,
    LowerRuntimeBoundInspectionBasis
);
impl_basis_target!(
    LowerRuntimeBoundSubscriptionDeclarationBasis,
    LowerRuntimeBoundSubscriptionDeclarationBasis
);
impl_basis_target!(
    LowerRuntimeBoundSubscriptionActivationBasis,
    LowerRuntimeBoundSubscriptionActivationBasis
);
impl_basis_target!(DeniedBasisCapability, DeniedBasisCapability);
impl_basis_target!(BasisIntentDenial, BasisIntentDenial);

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
    BasisLifecycle(ForgeQueryBasisLifecycleInspection),
}
