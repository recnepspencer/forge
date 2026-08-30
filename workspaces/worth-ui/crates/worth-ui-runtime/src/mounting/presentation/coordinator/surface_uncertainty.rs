use super::super::terminal::{aggregate_affected, UiIndeterminatePresentationEvidence};
use crate::facade::UiHostEffectPort;

pub(super) struct PresentationSurfaceUncertainty(Box<PresentationSurfaceUncertaintyInner>);

struct PresentationSurfaceUncertaintyInner {
    binding: worth_ui_host_contract::UiSurfaceBindingGeneration,
    additional_cost: Option<worth_ui_host_contract::UiHostPresentationCostReport>,
    semantic_receipts: Vec<worth_ui_query_binding::WorthUiPresentationRecoveryReceipt>,
    recovery_required: Vec<worth_ui_query_binding::WorthUiPresentationRecoveryRequiredReceipt>,
    physical_recovery_bindings: Vec<worth_ui_host_contract::UiSurfaceBindingGeneration>,
}

impl PresentationSurfaceUncertainty {
    pub(super) fn semantic(
        binding: worth_ui_host_contract::UiSurfaceBindingGeneration,
        additional_cost: Option<worth_ui_host_contract::UiHostPresentationCostReport>,
        semantic_receipts: Vec<worth_ui_query_binding::WorthUiPresentationRecoveryReceipt>,
    ) -> Self {
        Self(Box::new(PresentationSurfaceUncertaintyInner {
            binding,
            additional_cost,
            semantic_receipts,
            recovery_required: Vec::new(),
            physical_recovery_bindings: Vec::new(),
        }))
    }

    pub(super) fn effects_indeterminate(
        binding: worth_ui_host_contract::UiSurfaceBindingGeneration,
        additional_cost: Option<worth_ui_host_contract::UiHostPresentationCostReport>,
        semantic_receipts: Box<[worth_ui_query_binding::WorthUiPresentationRecoveryReceipt]>,
        owner: Option<&mut crate::native_platform::text_presentation::UiPresentationAsyncRuntime>,
        awaits_physical_recovery: bool,
    ) -> Self {
        let (semantic_receipts, recovery_required) =
            settle_effects_indeterminate(owner, semantic_receipts);
        Self(Box::new(PresentationSurfaceUncertaintyInner {
            binding,
            additional_cost,
            semantic_receipts,
            recovery_required,
            physical_recovery_bindings: awaits_physical_recovery
                .then_some(binding)
                .into_iter()
                .collect(),
        }))
    }
}

pub(super) fn terminalize(
    progress: &mut super::UiMountedPresentationProgress,
    host: UiHostEffectPort<'_>,
    presentation_async: Option<
        &mut crate::native_platform::text_presentation::UiPresentationAsyncRuntime,
    >,
    uncertainty: PresentationSurfaceUncertainty,
) -> UiIndeterminatePresentationEvidence {
    let PresentationSurfaceUncertainty(inner) = uncertainty;
    let PresentationSurfaceUncertaintyInner {
        binding,
        additional_cost,
        mut semantic_receipts,
        mut recovery_required,
        mut physical_recovery_bindings,
    } = *inner;
    let mut affected =
        aggregate_affected(&progress.completed, &progress.pending, &progress.rejected);
    affected.push(binding);
    let stopped = super::cancellation::cancel_all(std::mem::take(&mut progress.pending), host);
    let cancellation = super::cancellation_settlement::settle(
        stopped,
        presentation_async,
        worth_ui_host_contract::UiHostSurfacePresentationDenial::CancelledBeforeEffects,
    );
    let (_, cancelled_recoveries, cancelled_required, cancelled_physical) =
        cancellation.into_parts();
    semantic_receipts.extend(cancelled_recoveries);
    recovery_required.extend(cancelled_required);
    physical_recovery_bindings.extend(cancelled_physical);
    let evidence =
        UiIndeterminatePresentationEvidence::new(affected, std::mem::take(&mut progress.completed))
            .with_semantic_receipts(semantic_receipts)
            .with_recovery_required(recovery_required);
    let evidence = if physical_recovery_bindings.is_empty() {
        evidence
    } else {
        evidence.with_physical_recovery_bindings(physical_recovery_bindings)
    };
    match additional_cost {
        Some(cost) => evidence.with_additional_adapter_cost(cost),
        None => evidence,
    }
}

fn settle_effects_indeterminate(
    owner: Option<&mut crate::native_platform::text_presentation::UiPresentationAsyncRuntime>,
    semantic_receipts: Box<[worth_ui_query_binding::WorthUiPresentationRecoveryReceipt]>,
) -> (
    Vec<worth_ui_query_binding::WorthUiPresentationRecoveryReceipt>,
    Vec<worth_ui_query_binding::WorthUiPresentationRecoveryRequiredReceipt>,
) {
    let Some(owner) = owner else {
        return (semantic_receipts.into_vec(), Vec::new());
    };
    let mut retry = Vec::new();
    let mut required = Vec::new();
    for recovery in semantic_receipts {
        match recovery {
            worth_ui_query_binding::WorthUiPresentationRecoveryReceipt::Pending(pending) => {
                match owner.admit_effects_indeterminate_requiring_reconstruction(&pending, 0) {
                    Ok(required_receipt) => required.push(required_receipt),
                    Err(_) => retry.push(pending.into()),
                }
            }
            worth_ui_query_binding::WorthUiPresentationRecoveryReceipt::Admission(admission) => {
                let recovery =
                    worth_ui_query_binding::WorthUiPresentationRecoveryReceipt::Admission(
                        admission,
                    );
                if owner.reject_recovery_before_effects(&recovery).is_err() {
                    retry.push(recovery);
                }
            }
        }
    }
    (retry, required)
}
