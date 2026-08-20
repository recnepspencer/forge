#[derive(Clone, Copy, Eq, PartialEq)]
pub(super) enum PresentedSemanticPosture {
    Current,
    Superseded,
}

pub(super) fn settle(
    binding: worth_ui_host_contract::UiSurfaceBindingGeneration,
    completion_cost: worth_ui_host_contract::UiHostPresentationCostReport,
    semantic_receipts: Box<[worth_ui_query_binding::WorthUiPresentationRecoveryReceipt]>,
    owner: Option<&mut crate::native_platform::text_presentation::UiPresentationAsyncRuntime>,
) -> Result<PresentedSemanticPosture, super::surface_uncertainty::PresentationSurfaceUncertainty> {
    let Some(owner) = owner else {
        return semantic_receipts
            .is_empty()
            .then_some(PresentedSemanticPosture::Current)
            .ok_or_else(|| uncertainty(binding, completion_cost, semantic_receipts.into_vec()));
    };
    let payload_byte_len = std::mem::size_of::<
        worth_ui_host_contract::UiMountedSurfacePresentationCompletion,
    >() as u64;
    let mut receipts = std::collections::VecDeque::from(semantic_receipts.into_vec());
    let mut posture = None;
    while let Some(receipt) = receipts.pop_front() {
        let worth_ui_query_binding::WorthUiPresentationRecoveryReceipt::Pending(receipt) = receipt
        else {
            receipts.push_front(receipt);
            return Err(uncertainty(
                binding,
                completion_cost,
                receipts.into_iter().collect(),
            ));
        };
        let observed = match owner.admit_presented_after_validation(receipt, payload_byte_len) {
            Ok(crate::native_platform::text_presentation::UiPresentationAsyncPresentedAdmission::Current(_)) => PresentedSemanticPosture::Current,
            Ok(crate::native_platform::text_presentation::UiPresentationAsyncPresentedAdmission::Superseded(_)) => PresentedSemanticPosture::Superseded,
            Err((receipt, _)) => {
                receipts.push_front(receipt.into());
                return Err(uncertainty(
                    binding,
                    completion_cost,
                    receipts.into_iter().collect(),
                ));
            }
        };
        if posture.is_some_and(|existing| existing != observed) {
            return Err(uncertainty(
                binding,
                completion_cost,
                receipts.into_iter().collect(),
            ));
        }
        posture = Some(observed);
    }
    Ok(posture.unwrap_or(PresentedSemanticPosture::Current))
}

fn uncertainty(
    binding: worth_ui_host_contract::UiSurfaceBindingGeneration,
    completion_cost: worth_ui_host_contract::UiHostPresentationCostReport,
    receipts: Vec<worth_ui_query_binding::WorthUiPresentationRecoveryReceipt>,
) -> super::surface_uncertainty::PresentationSurfaceUncertainty {
    super::surface_uncertainty::PresentationSurfaceUncertainty::semantic(
        binding,
        Some(completion_cost),
        receipts,
    )
}
