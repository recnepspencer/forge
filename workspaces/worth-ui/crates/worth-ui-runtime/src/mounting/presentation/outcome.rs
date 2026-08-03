use worth_ui_host_contract::{
    UiHostPresentationEpoch, UiHostSurfaceIdentity, UiMountedCompletedEffects,
    UiMountedFrameIdentity, UiMountedPresentationAttemptIdentity, UiSemanticSurfaceIdentity,
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
    cost: super::super::UiMountCostReport,
}

#[derive(Debug, Eq, PartialEq)]
pub struct UiMountedPresentationWitness {
    attempt: UiMountedPresentationAttemptIdentity,
    authority: worth_proof::AuthorityWitness<UiMountedPresentationAuthority>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiMountedSurfacePresentationReceipt {
    semantic_surface: UiSemanticSurfaceIdentity,
    host_surface: UiHostSurfaceIdentity,
    binding: UiSurfaceBindingGeneration,
    epoch: UiHostPresentationEpoch,
    effects: UiMountedCompletedEffects,
    adapter_cost: worth_ui_host_contract::UiHostPresentationCostReport,
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
    retention: super::super::retention::UiMountedRetentionReservation,
    receipt: UiMountedPresentationReceipt,
    witness: UiMountedPresentationWitness,
}

pub struct UiMountedRejectedFrame {
    attempt: UiMountedPresentationAttemptIdentity,
    frame: UiPreparedMountedFrame,
    rejections: Box<[UiMountedSurfacePresentationRejection]>,
    cost: super::super::UiMountCostReport,
}

pub struct UiMountedIndeterminateFrame {
    frame: UiPreparedMountedFrame,
    report: UiPresentationIndeterminateReport,
    cost: super::super::UiMountCostReport,
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
        cost: super::super::UiMountCostReport,
        surfaces: Vec<UiMountedSurfacePresentationReceipt>,
    ) -> Self {
        Self {
            attempt,
            frame,
            surfaces: surfaces.into_boxed_slice(),
            cost,
        }
    }

    pub(super) fn compose_cost(
        mounting_cost: super::super::UiMountCostReport,
        surfaces: &[UiMountedSurfacePresentationReceipt],
    ) -> Result<super::super::UiMountCostReport, super::super::UiMountCostOverflow> {
        let adapter_cost = surfaces.iter().try_fold(
            worth_ui_host_contract::UiHostPresentationCostReport::default(),
            |total, surface| {
                total
                    .checked_add(surface.adapter_cost())
                    .map_err(|_| super::super::UiMountCostOverflow)
            },
        )?;
        mounting_cost.with_adapter(adapter_cost)
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

    pub fn cost_report(&self) -> super::super::UiMountCostReport {
        self.cost
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
        requirement: worth_ui_host_contract::UiMountedSurfaceBindingRequirement,
        epoch: UiHostPresentationEpoch,
        effects: UiMountedCompletedEffects,
        adapter_cost: worth_ui_host_contract::UiHostPresentationCostReport,
    ) -> Self {
        Self {
            semantic_surface: requirement.semantic_surface(),
            host_surface: requirement.host_surface(),
            binding: requirement.binding(),
            epoch,
            effects,
            adapter_cost,
        }
    }

    pub fn binding(&self) -> UiSurfaceBindingGeneration {
        self.binding
    }

    pub fn semantic_surface(&self) -> UiSemanticSurfaceIdentity {
        self.semantic_surface
    }

    pub fn host_surface(&self) -> UiHostSurfaceIdentity {
        self.host_surface
    }

    pub fn epoch(&self) -> UiHostPresentationEpoch {
        self.epoch
    }

    pub fn effects(&self) -> &UiMountedCompletedEffects {
        &self.effects
    }

    pub fn adapter_cost(&self) -> worth_ui_host_contract::UiHostPresentationCostReport {
        self.adapter_cost
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
        retention: super::super::retention::UiMountedRetentionReservation,
        receipt: UiMountedPresentationReceipt,
        witness: UiMountedPresentationWitness,
    ) -> Self {
        Self {
            frame,
            retention,
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

    pub(crate) fn into_publication_parts(
        self,
    ) -> (
        UiPreparedMountedFrame,
        super::super::retention::UiMountedRetentionReservation,
    ) {
        (self.frame, self.retention)
    }
}

impl UiMountedRejectedFrame {
    pub(super) fn new(
        attempt: UiMountedPresentationAttemptIdentity,
        frame: UiPreparedMountedFrame,
        rejections: Vec<UiMountedSurfacePresentationRejection>,
    ) -> Self {
        let cost = frame
            .cost_report()
            .reclassified(super::super::UiMountWorkClass::RejectedPresentation)
            .with_rejected(rejections.len())
            .expect("bounded surface rejection rows fit cost accounting");
        Self {
            attempt,
            frame,
            rejections: rejections.into_boxed_slice(),
            cost,
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

    pub fn cost_report(&self) -> super::super::UiMountCostReport {
        self.cost
    }

    pub(crate) fn into_frame(self) -> UiPreparedMountedFrame {
        self.frame
    }
}

impl UiMountedIndeterminateFrame {
    pub(super) fn new(
        frame: UiPreparedMountedFrame,
        report: UiPresentationIndeterminateReport,
        cost: super::super::UiMountCostReport,
    ) -> Self {
        Self {
            frame,
            report,
            cost,
        }
    }

    pub fn frame(&self) -> &UiPreparedMountedFrame {
        &self.frame
    }

    pub fn report(&self) -> &UiPresentationIndeterminateReport {
        &self.report
    }

    pub fn cost_report(&self) -> super::super::UiMountCostReport {
        self.cost
    }
}
