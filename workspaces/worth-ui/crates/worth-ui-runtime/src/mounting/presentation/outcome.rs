use worth_ui_host_contract::{
    UiMountedCompletedEffects, UiMountedFrameIdentity, UiMountedPresentationAttemptIdentity,
    UiSurfaceBindingGeneration,
};

use super::super::UiPreparedMountedFrame;

#[derive(Debug, Eq, PartialEq)]
struct UiMountedPresentationAuthority;

impl worth_proof::AuthorityMarker for UiMountedPresentationAuthority {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiMountedPresentationReceipt {
    attempt: UiMountedPresentationAttemptIdentity,
    frame: UiMountedFrameIdentity,
    surfaces: Box<[UiMountedSurfacePresentationReceipt]>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct UiMountedPresentationWitness {
    attempt: UiMountedPresentationAttemptIdentity,
    authority: worth_proof::AuthorityWitness<UiMountedPresentationAuthority>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiMountedSurfacePresentationReceipt {
    binding: UiSurfaceBindingGeneration,
    effects: UiMountedCompletedEffects,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiMountedSurfacePresentationRejection {
    binding: UiSurfaceBindingGeneration,
    denial: worth_ui_host_contract::UiHostSurfacePresentationDenial,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiPresentationIndeterminateReport {
    attempt: UiMountedPresentationAttemptIdentity,
    affected_bindings: Box<[UiSurfaceBindingGeneration]>,
}

pub struct UiMountedPresentedFrame {
    frame: UiPreparedMountedFrame,
    receipt: UiMountedPresentationReceipt,
    witness: UiMountedPresentationWitness,
}

pub struct UiMountedRejectedFrame {
    attempt: UiMountedPresentationAttemptIdentity,
    frame: UiPreparedMountedFrame,
    rejections: Box<[UiMountedSurfacePresentationRejection]>,
}

pub struct UiMountedIndeterminateFrame {
    frame: UiPreparedMountedFrame,
    report: UiPresentationIndeterminateReport,
}

pub enum UiMountedPresentationOutcome {
    RejectedBeforeEffects(UiMountedRejectedFrame),
    InFlight(super::UiMountedPresentationInFlight),
    Presented(UiMountedPresentedFrame),
    PresentationIndeterminate(UiMountedIndeterminateFrame),
}

impl UiMountedPresentationReceipt {
    pub(super) fn new(
        attempt: UiMountedPresentationAttemptIdentity,
        frame: UiMountedFrameIdentity,
        surfaces: Vec<UiMountedSurfacePresentationReceipt>,
    ) -> Self {
        Self {
            attempt,
            frame,
            surfaces: surfaces.into_boxed_slice(),
        }
    }

    pub fn attempt(&self) -> UiMountedPresentationAttemptIdentity {
        self.attempt
    }

    pub fn frame(&self) -> UiMountedFrameIdentity {
        self.frame
    }

    pub fn surfaces(&self) -> &[UiMountedSurfacePresentationReceipt] {
        &self.surfaces
    }
}

impl UiMountedPresentationWitness {
    pub(super) fn new(attempt: UiMountedPresentationAttemptIdentity) -> Self {
        Self {
            attempt,
            authority: worth_proof::AuthorityWitness::from_authority_marker(
                UiMountedPresentationAuthority,
            ),
        }
    }

    pub fn attempt(&self) -> UiMountedPresentationAttemptIdentity {
        self.attempt
    }
}

impl UiMountedSurfacePresentationReceipt {
    pub(super) fn new(
        binding: UiSurfaceBindingGeneration,
        effects: UiMountedCompletedEffects,
    ) -> Self {
        Self { binding, effects }
    }

    pub fn binding(&self) -> UiSurfaceBindingGeneration {
        self.binding
    }

    pub fn effects(&self) -> &UiMountedCompletedEffects {
        &self.effects
    }
}

impl UiMountedSurfacePresentationRejection {
    pub(super) fn new(
        binding: UiSurfaceBindingGeneration,
        denial: worth_ui_host_contract::UiHostSurfacePresentationDenial,
    ) -> Self {
        Self { binding, denial }
    }

    pub fn binding(self) -> UiSurfaceBindingGeneration {
        self.binding
    }

    pub fn denial(self) -> worth_ui_host_contract::UiHostSurfacePresentationDenial {
        self.denial
    }
}

impl UiPresentationIndeterminateReport {
    pub(super) fn new(
        attempt: UiMountedPresentationAttemptIdentity,
        mut affected_bindings: Vec<UiSurfaceBindingGeneration>,
    ) -> Self {
        affected_bindings.sort();
        affected_bindings.dedup();
        Self {
            attempt,
            affected_bindings: affected_bindings.into_boxed_slice(),
        }
    }

    pub fn attempt(&self) -> UiMountedPresentationAttemptIdentity {
        self.attempt
    }

    pub fn affected_bindings(&self) -> &[UiSurfaceBindingGeneration] {
        &self.affected_bindings
    }
}

impl UiMountedPresentedFrame {
    pub(super) fn new(
        frame: UiPreparedMountedFrame,
        receipt: UiMountedPresentationReceipt,
        witness: UiMountedPresentationWitness,
    ) -> Self {
        Self {
            frame,
            receipt,
            witness,
        }
    }

    pub fn receipt(&self) -> &UiMountedPresentationReceipt {
        &self.receipt
    }

    pub fn frame(&self) -> &UiPreparedMountedFrame {
        &self.frame
    }

    pub fn witness(&self) -> &UiMountedPresentationWitness {
        &self.witness
    }

    pub(crate) fn into_frame(self) -> UiPreparedMountedFrame {
        self.frame
    }
}

impl UiMountedRejectedFrame {
    pub(super) fn new(
        attempt: UiMountedPresentationAttemptIdentity,
        frame: UiPreparedMountedFrame,
        rejections: Vec<UiMountedSurfacePresentationRejection>,
    ) -> Self {
        Self {
            attempt,
            frame,
            rejections: rejections.into_boxed_slice(),
        }
    }

    pub fn attempt(&self) -> UiMountedPresentationAttemptIdentity {
        self.attempt
    }

    pub fn frame(&self) -> &UiPreparedMountedFrame {
        &self.frame
    }

    pub fn rejections(&self) -> &[UiMountedSurfacePresentationRejection] {
        &self.rejections
    }

    pub(crate) fn into_frame(self) -> UiPreparedMountedFrame {
        self.frame
    }
}

impl UiMountedIndeterminateFrame {
    pub(super) fn new(
        frame: UiPreparedMountedFrame,
        report: UiPresentationIndeterminateReport,
    ) -> Self {
        Self { frame, report }
    }

    pub fn frame(&self) -> &UiPreparedMountedFrame {
        &self.frame
    }

    pub fn report(&self) -> &UiPresentationIndeterminateReport {
        &self.report
    }
}
