use worth_ui_host_contract::{UiMountedFrameIdentity, UiSurfaceBindingGeneration};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiMountedFramePublicationReceipt {
    inner: std::rc::Rc<UiMountedFramePublicationReceiptInner>,
}

#[derive(Debug, Eq, PartialEq)]
struct UiMountedFramePublicationReceiptInner {
    attempt: worth_ui_host_contract::UiMountedPresentationAttemptIdentity,
    frame: UiMountedFrameIdentity,
    predecessor: Option<UiMountedFrameIdentity>,
    generation:
        crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationIdentity,
    bindings: Box<[UiSurfaceBindingGeneration]>,
    cost: std::cell::Cell<super::UiMountCostReport>,
}

pub struct UiMountedFramePublicationCandidate {
    receipt: UiMountedFramePublicationReceipt,
    presented_basis: super::retention::UiPreparedPresentedFrameBasis,
}

pub(crate) struct UiMountedFrameReconciliationCandidate {
    replacements: Box<[super::UiMountedSurfaceReconciliationBinding]>,
    receipt: UiMountedFramePublicationReceipt,
    presented_basis: super::retention::UiPreparedPresentedFrameBasis,
}

pub enum UiMountedFrameOutcome {
    Published(UiMountedFramePublicationReceipt),
    Unchanged(UiMountedFramePublicationReceipt),
    Reconciled(UiMountedFramePublicationReceipt),
    RejectedBeforeEffects(super::UiMountedRejectedFrame),
    InFlight(super::UiMountedPresentationInFlight),
    PresentationIndeterminate(super::UiMountedIndeterminateFrame),
    AdmissionDenied(super::UiMountedPresentationAdmissionRejection),
    CompletionDenied(super::UiMountedPresentationCompletionDenial),
}

impl UiMountedFrameReconciliationCandidate {
    pub(crate) fn reserve(
        admission: &super::UiMountedPresentationAdmission,
        current: &UiMountedFramePublicationReceipt,
        replacements: &[super::UiMountedSurfaceReconciliationBinding],
    ) -> Self {
        debug_assert_eq!(admission.frame().canonical_core().frame(), current.frame());
        let mut bindings = admission
            .frame()
            .manifest()
            .surfaces()
            .iter()
            .map(|surface| surface.binding())
            .collect::<Vec<_>>();
        bindings.sort();
        let receipt = UiMountedFramePublicationReceipt {
            inner: std::rc::Rc::new(UiMountedFramePublicationReceiptInner {
                attempt: admission.attempt(),
                frame: current.frame(),
                predecessor: current.predecessor(),
                generation: current.generation().clone(),
                bindings: bindings.into_boxed_slice(),
                cost: std::cell::Cell::new(admission.frame().cost_report()),
            }),
        };
        let presented_basis = super::retention::UiMountedPresentedFrameRetention::prepare(
            receipt.frame(),
            receipt.bindings(),
            admission.frame().presented_receipt_basis().clone(),
        );
        Self {
            replacements: replacements.to_vec().into_boxed_slice(),
            receipt,
            presented_basis,
        }
    }

    pub(crate) fn replacements(&self) -> &[super::UiMountedSurfaceReconciliationBinding] {
        &self.replacements
    }

    pub(crate) fn commit_presented(
        self,
        presented: super::UiMountedPresentedFrame,
        state: &mut super::UiMountedIdentityState,
    ) -> UiMountedFramePublicationReceipt {
        let Self {
            receipt,
            presented_basis,
            ..
        } = self;
        receipt.finalize_cost(presented.receipt().cost_report());
        state.publish_reconciled_frame(presented.into_frame(), receipt.clone(), presented_basis);
        receipt
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiMountedPublicationLeaseDenial {
    PresentationInFlight,
}

impl UiMountedFramePublicationCandidate {
    pub(crate) fn reserve(
        admission: &super::UiMountedPresentationAdmission,
        predecessor: Option<UiMountedFrameIdentity>,
    ) -> Self {
        let mut bindings = admission
            .frame()
            .manifest()
            .surfaces()
            .iter()
            .map(|surface| surface.binding())
            .collect::<Vec<_>>();
        bindings.sort();
        let receipt = UiMountedFramePublicationReceipt {
            inner: std::rc::Rc::new(UiMountedFramePublicationReceiptInner {
                attempt: admission.attempt(),
                frame: admission.frame().canonical_core().frame(),
                predecessor,
                generation: admission.frame().generation().clone(),
                bindings: bindings.into_boxed_slice(),
                cost: std::cell::Cell::new(admission.frame().cost_report()),
            }),
        };
        let presented_basis = super::retention::UiMountedPresentedFrameRetention::prepare(
            admission.frame().canonical_core().frame(),
            receipt.bindings(),
            admission.frame().presented_receipt_basis().clone(),
        );
        Self {
            receipt,
            presented_basis,
        }
    }

    pub(crate) fn commit_presented(
        self,
        presented: super::UiMountedPresentedFrame,
        state: &mut super::UiMountedIdentityState,
    ) -> UiMountedFramePublicationReceipt {
        let Self {
            receipt,
            presented_basis,
        } = self;
        receipt.finalize_cost(presented.receipt().cost_report());
        state.publish_presented_frame(presented.into_frame(), receipt.clone(), presented_basis);
        receipt
    }
}

impl UiMountedFramePublicationReceipt {
    pub fn attempt(&self) -> worth_ui_host_contract::UiMountedPresentationAttemptIdentity {
        self.inner.attempt
    }

    pub fn frame(&self) -> UiMountedFrameIdentity {
        self.inner.frame
    }

    pub fn predecessor(&self) -> Option<UiMountedFrameIdentity> {
        self.inner.predecessor
    }

    pub fn generation(
        &self,
    ) -> &crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationIdentity
    {
        &self.inner.generation
    }

    pub fn bindings(&self) -> &[UiSurfaceBindingGeneration] {
        &self.inner.bindings
    }

    pub fn cost_report(&self) -> super::UiMountCostReport {
        self.inner.cost.get()
    }

    fn finalize_cost(&self, cost: super::UiMountCostReport) {
        self.inner.cost.set(cost);
    }
}

impl UiMountedFrameOutcome {
    pub fn cost_report(&self) -> Option<super::UiMountCostReport> {
        match self {
            Self::Published(receipt) | Self::Reconciled(receipt) => Some(receipt.cost_report()),
            Self::Unchanged(_) => Some(super::UiMountCostReport::unchanged_reuse()),
            Self::RejectedBeforeEffects(frame) => Some(
                frame
                    .frame()
                    .cost_report()
                    .reclassified(super::UiMountWorkClass::RejectedPresentation),
            ),
            Self::InFlight(frame) => Some(frame.cost_report()),
            Self::PresentationIndeterminate(frame) => Some(
                frame
                    .frame()
                    .cost_report()
                    .reclassified(super::UiMountWorkClass::IndeterminatePresentation),
            ),
            Self::AdmissionDenied(rejection) => Some(
                rejection
                    .frame()
                    .cost_report()
                    .reclassified(super::UiMountWorkClass::RejectedPresentation),
            ),
            Self::CompletionDenied(_) => None,
        }
    }
}
