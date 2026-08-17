use worth_ui_host_contract::{
    UiHostPresentationCompletionToken, UiMountedPresentationAttemptIdentity,
    UiPresentationDeadline, UiSurfaceBindingGeneration,
};

use super::super::UiPreparedMountedFrame;
use super::outcome::{UiMountedSurfacePresentationReceipt, UiMountedSurfacePresentationRejection};

pub struct UiMountedPresentationAdmission {
    pub(super) frame: UiPreparedMountedFrame,
    pub(super) retention: super::super::retention::UiMountedRetentionReservation,
    pub(super) attempt: UiMountedPresentationAttemptIdentity,
    pub(super) deadline: UiPresentationDeadline,
    lease: UiPresentationAdmissionLease,
}

struct UiPresentationAdmissionLease {
    active: Rc<RefCell<BTreeSet<UiMountedPresentationAttemptIdentity>>>,
    attempt: UiMountedPresentationAttemptIdentity,
    release_on_drop: bool,
}

pub struct UiMountedPresentationAttempt {
    admission: UiMountedPresentationAdmission,
}

pub struct UiMountedPresentationAdmissionRejection {
    denial: UiMountedPresentationAdmissionDenial,
    frame: Box<UiPreparedMountedFrame>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiMountedPresentationInFlight {
    attempt: UiMountedPresentationAttemptIdentity,
    deadline: UiPresentationDeadline,
    pending_bindings: Box<[UiSurfaceBindingGeneration]>,
    cost: super::super::UiMountCostReport,
}

pub(super) struct UiMountedPresentationInFlightState {
    pub(super) frame: UiPreparedMountedFrame,
    pub(super) retention: super::super::retention::UiMountedRetentionReservation,
    pub(super) attempt: UiMountedPresentationAttemptIdentity,
    pub(super) deadline: UiPresentationDeadline,
    pub(super) pending: Vec<UiPendingMountedSurface>,
    pub(super) pending_text: Vec<UiPendingMountedTextRaster>,
    pub(super) rejected: Vec<UiMountedSurfacePresentationRejection>,
    pub(super) completed: Vec<UiMountedSurfacePresentationReceipt>,
    pub(super) candidates: super::work_producer::UiMountedPresentationCandidates,
}

pub(super) struct UiPendingMountedTextRaster {
    pub(super) binding: UiSurfaceBindingGeneration,
    pub(super) pending: crate::native_platform::text_presentation::UiNativeMountedTextPending,
}

pub(super) struct UiPendingMountedSurface {
    pub(super) binding: UiSurfaceBindingGeneration,
    pub(super) token: UiHostPresentationCompletionToken,
    pub(super) expected_effects: Box<[worth_ui_host_contract::UiMountedEffectFamily]>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiMountedPresentationAdmissionDenial {
    CoordinatorShuttingDown,
    CapacityExceeded,
    DeadlineExpired,
    PreparedFrameBasisChanged,
    CapabilityGenerationChanged(UiSurfaceBindingGeneration),
    CapabilityProfileChanged(UiSurfaceBindingGeneration),
    BindingRequiresReconciliation(UiSurfaceBindingGeneration),
    BaselineReceiptUnavailable(UiSurfaceBindingGeneration),
    ReconciliationBasisMismatch,
    IdentityExhausted,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiMountedPresentationCompletionDenial {
    UnknownAttempt,
}

impl UiMountedPresentationAdmission {
    pub(super) fn new(
        prepared: super::super::retention::UiRetentionPreparedMountedFrame,
        attempt: UiMountedPresentationAttemptIdentity,
        deadline: UiPresentationDeadline,
        active: Rc<RefCell<BTreeSet<UiMountedPresentationAttemptIdentity>>>,
    ) -> Self {
        let (frame, retention) = prepared.into_parts();
        Self {
            frame,
            retention,
            attempt,
            deadline,
            lease: UiPresentationAdmissionLease {
                active,
                attempt,
                release_on_drop: true,
            },
        }
    }

    pub fn attempt(&self) -> UiMountedPresentationAttemptIdentity {
        self.attempt
    }

    pub fn deadline(&self) -> UiPresentationDeadline {
        self.deadline
    }

    pub fn frame(&self) -> &UiPreparedMountedFrame {
        &self.frame
    }

    pub fn into_attempt(self) -> UiMountedPresentationAttempt {
        UiMountedPresentationAttempt { admission: self }
    }
}

impl UiMountedPresentationAdmissionRejection {
    pub(crate) fn new(
        frame: UiPreparedMountedFrame,
        denial: UiMountedPresentationAdmissionDenial,
    ) -> Self {
        Self {
            denial,
            frame: Box::new(frame),
        }
    }

    pub fn denial(&self) -> UiMountedPresentationAdmissionDenial {
        self.denial
    }

    pub fn frame(&self) -> &UiPreparedMountedFrame {
        &self.frame
    }

    pub fn into_frame(self) -> UiPreparedMountedFrame {
        *self.frame
    }
}

impl std::fmt::Debug for UiMountedPresentationAdmissionRejection {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("UiMountedPresentationAdmissionRejection")
            .field("denial", &self.denial)
            .finish_non_exhaustive()
    }
}

impl UiMountedPresentationAttempt {
    pub(super) fn into_parts(
        mut self,
    ) -> (
        UiPreparedMountedFrame,
        super::super::retention::UiMountedRetentionReservation,
        UiMountedPresentationAttemptIdentity,
        UiPresentationDeadline,
    ) {
        self.admission.lease.release_on_drop = false;
        (
            self.admission.frame,
            self.admission.retention,
            self.admission.attempt,
            self.admission.deadline,
        )
    }
}

impl Drop for UiPresentationAdmissionLease {
    fn drop(&mut self) {
        if self.release_on_drop {
            self.active.borrow_mut().remove(&self.attempt);
        }
    }
}

impl UiMountedPresentationInFlight {
    pub(super) fn from_state(
        state: &UiMountedPresentationInFlightState,
        cost: super::super::UiMountCostReport,
    ) -> Self {
        Self {
            attempt: state.attempt,
            deadline: state.deadline,
            pending_bindings: state
                .pending
                .iter()
                .map(|pending| pending.binding)
                .chain(state.pending_text.iter().map(|pending| pending.binding))
                .collect(),
            cost,
        }
    }

    pub fn attempt(&self) -> UiMountedPresentationAttemptIdentity {
        self.attempt
    }

    pub fn deadline(&self) -> UiPresentationDeadline {
        self.deadline
    }

    pub fn pending_bindings(
        &self,
    ) -> impl ExactSizeIterator<Item = UiSurfaceBindingGeneration> + '_ {
        self.pending_bindings.iter().copied()
    }

    pub fn cost_report(&self) -> super::super::UiMountCostReport {
        self.cost
    }
}
use std::cell::RefCell;
use std::collections::BTreeSet;
use std::rc::Rc;
