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
}

pub(crate) struct UiMountedFrameReconciliationCandidate {
    replacements: Box<[super::UiMountedSurfaceReconciliationBinding]>,
    receipt: UiMountedFramePublicationReceipt,
}

pub enum UiMountedFrameOutcome {
    Published(UiMountedFramePublicationReceipt),
    Unchanged(UiMountedFramePublicationReceipt),
    Reconciled(UiMountedFramePublicationReceipt),
    RejectedBeforeEffects(super::UiMountedRejectedFrame),
    InFlight(super::UiMountedPresentationInFlight),
    PresentationIndeterminate(super::UiMountedIndeterminateFrame),
    RetentionDenied(super::UiMountedFrameRetentionRejection),
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
        Self {
            replacements: replacements.to_vec().into_boxed_slice(),
            receipt,
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
        let Self { receipt, .. } = self;
        let mount_cost = retained_publication_cost(presented.receipt().cost_report());
        let presentation = presented.receipt().clone();
        receipt.finalize_cost(mount_cost);
        let (frame, reservation) = presented.into_publication_parts();
        state.publish_reconciled_frame(frame, receipt.clone());
        reservation.commit(mount_cost, presentation);
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
        Self { receipt }
    }

    pub(crate) fn commit_presented(
        self,
        presented: super::UiMountedPresentedFrame,
        state: &mut super::UiMountedIdentityState,
    ) -> UiMountedFramePublicationReceipt {
        let Self { receipt } = self;
        let mount_cost = retained_publication_cost(presented.receipt().cost_report());
        let presentation = presented.receipt().clone();
        receipt.finalize_cost(mount_cost);
        let (frame, reservation) = presented.into_publication_parts();
        state.publish_presented_frame(frame, receipt.clone());
        reservation.commit(mount_cost, presentation);
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

fn retained_publication_cost(cost: super::UiMountCostReport) -> super::UiMountCostReport {
    cost.with_retained(1)
        .expect("one published current frame fits retained cost accounting")
}

impl UiMountedFrameOutcome {
    pub fn cost_report(&self) -> Option<super::UiMountCostReport> {
        match self {
            Self::Published(receipt) | Self::Reconciled(receipt) => Some(receipt.cost_report()),
            Self::Unchanged(_) => Some(super::UiMountCostReport::unchanged_reuse()),
            Self::RejectedBeforeEffects(frame) => Some(frame.cost_report()),
            Self::InFlight(frame) => Some(frame.cost_report()),
            Self::PresentationIndeterminate(frame) => Some(frame.cost_report()),
            Self::RetentionDenied(rejection) => Some(
                rejection
                    .frame()
                    .cost_report()
                    .reclassified(super::UiMountWorkClass::RejectedPreparation)
                    .with_rejected(1)
                    .expect("one rejected retained frame fits cost accounting"),
            ),
            Self::AdmissionDenied(rejection) => Some(
                rejection
                    .frame()
                    .cost_report()
                    .reclassified(super::UiMountWorkClass::RejectedPresentation)
                    .with_rejected(1)
                    .expect("one rejected presentation frame fits cost accounting"),
            ),
            Self::CompletionDenied(_) => None,
        }
    }
}
