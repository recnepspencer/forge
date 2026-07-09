use super::{WorthQueryBatchWriteReceiptInspection, WorthQueryWriteReceiptInspection};
use crate::application::WorthQueryAdmittedWorldBasis;
use crate::query_basis_lifecycle::{
    BasisIntentDenial, DeniedBasisCapability, InspectionBasisCapability,
    LowerRuntimeBoundInspectionBasis, LowerRuntimeBoundObservationBasis,
    LowerRuntimeBoundSubscriptionActivationBasis, LowerRuntimeBoundSubscriptionDeclarationBasis,
    ObservationBasisCapability, ScopedInspectionBasis, ScopedObservationBasis, ScopedReplayBasis,
    ScopedSubscriptionActivationBasis, ScopedSubscriptionDeclarationBasis,
    SubscriptionActivationBasisCapability, SubscriptionDeclarationBasisCapability,
};
use crate::runtime::{
    WorthQueryBasisLifecycleInspection, WorthQueryBatchWriteReceipt, WorthQueryBranchIntentReceipt,
    WorthQueryBranchIntentReceiptInspection, WorthQueryComputedInspectionEvidence,
    WorthQueryDerivedViewHandle, WorthQueryEffectHandle, WorthQueryEffectInspectionEvidence,
    WorthQueryEffectIntentReceipt, WorthQueryEffectIntentReceiptInspection,
    WorthQueryIntentDenialEvidence, WorthQueryIntentDenialInspection, WorthQueryIntentReceipt,
    WorthQueryIntentReceiptInspection, WorthQueryLiveView, WorthQueryLiveViewInspection,
    WorthQueryPreviewBindingInspection, WorthQueryPreviewHandleBindingEvidence,
    WorthQueryPreviewIntentReceipt, WorthQueryPreviewIntentReceiptInspection,
    WorthQueryPreviewOutcome, WorthQueryPreviewOutcomeInspection, WorthQueryWriteReceipt,
};

pub enum WorthQueryInspectionTarget<'a> {
    LiveView {
        name: &'a str,
    },
    DerivedView {
        name: &'a str,
    },
    Effect {
        name: &'a str,
    },
    WriteReceipt(&'a WorthQueryWriteReceipt),
    BatchWriteReceipt(&'a WorthQueryBatchWriteReceipt),
    IntentReceipt(&'a WorthQueryIntentReceipt),
    IntentDenial(&'a WorthQueryIntentDenialEvidence),
    EffectIntentReceipt(&'a WorthQueryEffectIntentReceipt),
    PreviewBinding(&'a WorthQueryPreviewHandleBindingEvidence),
    PreviewOutcome(&'a WorthQueryPreviewOutcome),
    PreviewIntentReceipt(&'a WorthQueryPreviewIntentReceipt),
    BranchIntentReceipt(&'a WorthQueryBranchIntentReceipt),
    AdmittedWorldBasis(&'a WorthQueryAdmittedWorldBasis),
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

impl<'a, T> From<&'a WorthQueryLiveView<T>> for WorthQueryInspectionTarget<'a> {
    fn from(value: &'a WorthQueryLiveView<T>) -> Self {
        Self::LiveView { name: value.name() }
    }
}

impl<'a, T> From<&'a WorthQueryDerivedViewHandle<T>> for WorthQueryInspectionTarget<'a> {
    fn from(value: &'a WorthQueryDerivedViewHandle<T>) -> Self {
        Self::DerivedView { name: value.name() }
    }
}

impl<'a, T> From<&'a WorthQueryEffectHandle<T>> for WorthQueryInspectionTarget<'a> {
    fn from(value: &'a WorthQueryEffectHandle<T>) -> Self {
        Self::Effect { name: value.name() }
    }
}

impl<'a> From<&'a WorthQueryWriteReceipt> for WorthQueryInspectionTarget<'a> {
    fn from(value: &'a WorthQueryWriteReceipt) -> Self {
        Self::WriteReceipt(value)
    }
}

impl<'a> From<&'a WorthQueryBatchWriteReceipt> for WorthQueryInspectionTarget<'a> {
    fn from(value: &'a WorthQueryBatchWriteReceipt) -> Self {
        Self::BatchWriteReceipt(value)
    }
}

impl<'a> From<&'a WorthQueryIntentReceipt> for WorthQueryInspectionTarget<'a> {
    fn from(value: &'a WorthQueryIntentReceipt) -> Self {
        Self::IntentReceipt(value)
    }
}

impl<'a> From<&'a WorthQueryIntentDenialEvidence> for WorthQueryInspectionTarget<'a> {
    fn from(value: &'a WorthQueryIntentDenialEvidence) -> Self {
        Self::IntentDenial(value)
    }
}

impl<'a> From<&'a WorthQueryEffectIntentReceipt> for WorthQueryInspectionTarget<'a> {
    fn from(value: &'a WorthQueryEffectIntentReceipt) -> Self {
        Self::EffectIntentReceipt(value)
    }
}

impl<'a> From<&'a WorthQueryPreviewHandleBindingEvidence> for WorthQueryInspectionTarget<'a> {
    fn from(value: &'a WorthQueryPreviewHandleBindingEvidence) -> Self {
        Self::PreviewBinding(value)
    }
}

impl<'a> From<&'a WorthQueryPreviewOutcome> for WorthQueryInspectionTarget<'a> {
    fn from(value: &'a WorthQueryPreviewOutcome) -> Self {
        Self::PreviewOutcome(value)
    }
}

impl<'a> From<&'a WorthQueryPreviewIntentReceipt> for WorthQueryInspectionTarget<'a> {
    fn from(value: &'a WorthQueryPreviewIntentReceipt) -> Self {
        Self::PreviewIntentReceipt(value)
    }
}

impl<'a> From<&'a WorthQueryBranchIntentReceipt> for WorthQueryInspectionTarget<'a> {
    fn from(value: &'a WorthQueryBranchIntentReceipt) -> Self {
        Self::BranchIntentReceipt(value)
    }
}

macro_rules! impl_basis_target {
    ($target:ty, $variant:ident) => {
        impl<'a> From<&'a $target> for WorthQueryInspectionTarget<'a> {
            fn from(value: &'a $target) -> Self {
                Self::$variant(value)
            }
        }
    };
}

impl_basis_target!(WorthQueryAdmittedWorldBasis, AdmittedWorldBasis);
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
pub enum WorthQueryInspection {
    LiveView(WorthQueryLiveViewInspection),
    DerivedView(WorthQueryComputedInspectionEvidence),
    Effect(WorthQueryEffectInspectionEvidence),
    WriteReceipt(WorthQueryWriteReceiptInspection),
    BatchWriteReceipt(WorthQueryBatchWriteReceiptInspection),
    IntentReceipt(WorthQueryIntentReceiptInspection),
    IntentDenial(WorthQueryIntentDenialInspection),
    EffectIntentReceipt(WorthQueryEffectIntentReceiptInspection),
    PreviewBinding(WorthQueryPreviewBindingInspection),
    PreviewOutcome(WorthQueryPreviewOutcomeInspection),
    PreviewIntentReceipt(WorthQueryPreviewIntentReceiptInspection),
    BranchIntentReceipt(WorthQueryBranchIntentReceiptInspection),
    BasisLifecycle(WorthQueryBasisLifecycleInspection),
}
