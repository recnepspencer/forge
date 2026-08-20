use worth_ui_host_contract::{UiHostSurfaceCancellationOutcome, UiHostSurfacePresentationDenial};

use super::cancellation::UiStoppedMountedSurface;

pub(super) struct UiCancellationSettlement {
    rejections: Vec<super::super::UiMountedSurfacePresentationRejection>,
    semantic_recoveries: Vec<worth_ui_query_binding::WorthUiPresentationRecoveryReceipt>,
    recovery_required: Vec<worth_ui_query_binding::WorthUiPresentationRecoveryRequiredReceipt>,
    physical_recovery_bindings: Vec<worth_ui_host_contract::UiSurfaceBindingGeneration>,
}

pub(super) fn settle(
    stopped: Vec<UiStoppedMountedSurface>,
    owner: Option<&mut crate::native_platform::text_presentation::UiPresentationAsyncRuntime>,
    before_effects_denial: UiHostSurfacePresentationDenial,
) -> UiCancellationSettlement {
    let mut settlement = UiCancellationSettlement {
        rejections: Vec::new(),
        semantic_recoveries: Vec::new(),
        recovery_required: Vec::new(),
        physical_recovery_bindings: Vec::new(),
    };
    let mut owner = owner;
    for surface in stopped {
        let (binding, outcome, receipts) = surface.into_parts();
        match outcome {
            UiHostSurfaceCancellationOutcome::CancelledBeforeEffects => {
                settlement.rejections.push(
                    super::super::UiMountedSurfacePresentationRejection::new(
                        binding,
                        before_effects_denial,
                    ),
                );
                settle_before_effects(owner.as_deref_mut(), receipts, &mut settlement);
            }
            UiHostSurfaceCancellationOutcome::EffectsMayHaveBegun => {
                settlement.physical_recovery_bindings.push(binding);
                settle_after_effects(owner.as_deref_mut(), receipts, &mut settlement);
            }
        }
    }
    settlement
}

fn settle_before_effects(
    owner: Option<&mut crate::native_platform::text_presentation::UiPresentationAsyncRuntime>,
    receipts: Box<[worth_ui_query_binding::WorthUiPresentationRecoveryReceipt]>,
    settlement: &mut UiCancellationSettlement,
) {
    let Some(owner) = owner else {
        settlement.semantic_recoveries.extend(receipts);
        return;
    };
    for receipt in receipts {
        if owner.cancel_recovery_before_effects(&receipt).is_err() {
            settlement.semantic_recoveries.push(receipt);
        }
    }
}

fn settle_after_effects(
    owner: Option<&mut crate::native_platform::text_presentation::UiPresentationAsyncRuntime>,
    receipts: Box<[worth_ui_query_binding::WorthUiPresentationRecoveryReceipt]>,
    settlement: &mut UiCancellationSettlement,
) {
    let Some(owner) = owner else {
        settlement.semantic_recoveries.extend(receipts);
        return;
    };
    for receipt in receipts {
        match owner.cancel_recovery_after_effects_may_have_begun(&receipt, 0) {
            Ok(required) => settlement.recovery_required.push(required),
            Err(_) => settlement.semantic_recoveries.push(receipt),
        }
    }
}

impl UiCancellationSettlement {
    pub(super) fn requires_indeterminate(&self) -> bool {
        !self.physical_recovery_bindings.is_empty() || !self.semantic_recoveries.is_empty()
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        Vec<super::super::UiMountedSurfacePresentationRejection>,
        Vec<worth_ui_query_binding::WorthUiPresentationRecoveryReceipt>,
        Vec<worth_ui_query_binding::WorthUiPresentationRecoveryRequiredReceipt>,
        Vec<worth_ui_host_contract::UiSurfaceBindingGeneration>,
    ) {
        (
            self.rejections,
            self.semantic_recoveries,
            self.recovery_required,
            self.physical_recovery_bindings,
        )
    }
}
