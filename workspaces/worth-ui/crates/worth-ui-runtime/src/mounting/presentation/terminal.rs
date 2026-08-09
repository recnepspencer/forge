use worth_ui_host_contract::{
    UiHostSurfacePresentationDenial, UiMountedPresentationAttemptIdentity,
    UiMountedSurfacePresentationCompletion, UiSurfaceBindingGeneration,
};

use super::outcome::{
    UiMountedPresentationOutcome, UiMountedPresentationReceipt, UiMountedRejectedFrame,
    UiMountedSurfacePresentationReceipt, UiMountedSurfacePresentationRejection,
};

pub(super) struct UiIndeterminatePresentationEvidence {
    affected: Vec<UiSurfaceBindingGeneration>,
    completed: Vec<UiMountedSurfacePresentationReceipt>,
    additional_adapter_cost: Option<worth_ui_host_contract::UiHostPresentationCostReport>,
}

impl UiIndeterminatePresentationEvidence {
    pub(super) fn new(
        affected: Vec<UiSurfaceBindingGeneration>,
        completed: Vec<UiMountedSurfacePresentationReceipt>,
    ) -> Self {
        Self {
            affected,
            completed,
            additional_adapter_cost: None,
        }
    }

    pub(super) fn with_additional_adapter_cost(
        mut self,
        cost: worth_ui_host_contract::UiHostPresentationCostReport,
    ) -> Self {
        self.additional_adapter_cost = Some(cost);
        self
    }

    pub(super) fn into_terminal_parts(
        self,
        mounting_cost: super::super::UiMountCostReport,
    ) -> (
        Vec<UiSurfaceBindingGeneration>,
        super::super::UiMountCostReport,
    ) {
        let composed = UiMountedPresentationReceipt::compose_cost(mounting_cost, &self.completed)
            .and_then(|cost| match self.additional_adapter_cost {
                Some(additional) => cost.with_adapter(additional),
                None => Ok(cost),
            });
        let cost = composed
            .map(|cost| {
                cost.reclassified(super::super::UiMountWorkClass::IndeterminatePresentation)
            })
            .unwrap_or_else(|_| {
                mounting_cost
                    .reclassified(super::super::UiMountWorkClass::IndeterminatePresentation)
                    .with_cost_overflow()
                    .expect("one cost overflow marker fits accounting")
            });
        (self.affected, cost)
    }
}

pub(super) fn rejected_outcome(
    attempt: UiMountedPresentationAttemptIdentity,
    frame: super::super::UiPreparedMountedFrame,
    retention: super::super::retention::UiMountedRetentionReservation,
    rejections: Vec<UiMountedSurfacePresentationRejection>,
) -> UiMountedPresentationOutcome {
    drop(retention);
    UiMountedPresentationOutcome::RejectedBeforeEffects(UiMountedRejectedFrame::new(
        attempt, frame, rejections,
    ))
}

pub(super) fn aggregate_affected(
    completed: &[UiMountedSurfacePresentationReceipt],
    pending: &[super::state::UiPendingMountedSurface],
    rejected: &[UiMountedSurfacePresentationRejection],
) -> Vec<UiSurfaceBindingGeneration> {
    completed
        .iter()
        .map(UiMountedSurfacePresentationReceipt::binding)
        .chain(pending.iter().map(|surface| surface.binding))
        .chain(rejected.iter().map(|rejection| rejection.binding()))
        .collect()
}

pub(super) fn frame_rejections(
    frame: &super::super::UiPreparedMountedFrame,
    denial: UiHostSurfacePresentationDenial,
) -> Vec<UiMountedSurfacePresentationRejection> {
    frame
        .surfaces()
        .iter()
        .map(|surface| {
            UiMountedSurfacePresentationRejection::new(surface.requirement().binding(), denial)
        })
        .collect()
}

pub(super) fn completion_satisfies(
    surface: &super::super::UiMountedSurfaceReceipt,
    expected_effects: &[worth_ui_host_contract::UiMountedEffectFamily],
    completion: &UiMountedSurfacePresentationCompletion,
) -> bool {
    if completion.mode() != surface.requirement().presentation_mode() {
        return false;
    }
    completion.effects().families() == expected_effects
}
