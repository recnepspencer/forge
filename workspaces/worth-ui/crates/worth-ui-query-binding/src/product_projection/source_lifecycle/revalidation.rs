use worth_query::facade::runtime;
use worth_runtime_bridge::facade::{
    AdmittedBridgeAsyncRequestIdentity, BridgeMixedCauseOrderingInput,
    BridgeMixedCauseOrderingLaneKind, BridgeMixedCauseOrderingRequest, RuntimeBridge,
};

use crate::{
    UiProjectionConsumptionBudget, UiScalarProjectionBatchOutcome, UiScalarProjectionBinding,
    UiScalarProjectionFactReceipt,
};

use super::{declaration::truth_basis, ScalarLiveView, WorthUiScalarProjectionAdvanceError};

pub(super) struct WorthUiScalarProjectionRevalidationStop {
    error: WorthUiScalarProjectionAdvanceError,
    request: AdmittedBridgeAsyncRequestIdentity,
    predecessor: UiScalarProjectionFactReceipt,
}

pub(super) fn revalidate(
    bridge: &RuntimeBridge,
    workspace: &mut runtime::WorthQueryWorkspace,
    view: &ScalarLiveView,
    binding: &mut UiScalarProjectionBinding,
    request: &AdmittedBridgeAsyncRequestIdentity,
    predecessor: UiScalarProjectionFactReceipt,
    revision: u64,
) -> Result<
    (
        AdmittedBridgeAsyncRequestIdentity,
        UiScalarProjectionFactReceipt,
    ),
    Box<WorthUiScalarProjectionRevalidationStop>,
> {
    let revalidation = match bridge.revalidate_async_request(request, truth_basis(revision)) {
        Ok(revalidation) => revalidation,
        Err(error) => return Err(bridge_stop(error, request.clone(), predecessor)),
    };
    let request = revalidation.newer_request().clone();
    let ordering = bridge.order_mixed_causes(&BridgeMixedCauseOrderingRequest::new(
        BridgeMixedCauseOrderingLaneKind::Authoritative,
        vec![BridgeMixedCauseOrderingInput::AsyncRevalidationLineage(
            revalidation,
        )],
    ));
    let batch = match workspace.admit_bridge_async_result_transitions(view, &ordering) {
        Ok(batch) => batch,
        Err(error) => {
            return Err(Box::new(WorthUiScalarProjectionRevalidationStop {
                error: WorthUiScalarProjectionAdvanceError::Query(error),
                request,
                predecessor,
            }));
        }
    };
    let transition = match binding.consume_async_result_batch(
        workspace,
        batch,
        Some(predecessor),
        UiProjectionConsumptionBudget::platform_pulse(),
    ) {
        UiScalarProjectionBatchOutcome::Advanced(transition) => transition,
        UiScalarProjectionBatchOutcome::Unchanged(unchanged) => {
            return Err(Box::new(WorthUiScalarProjectionRevalidationStop {
                error: WorthUiScalarProjectionAdvanceError::UnexpectedUnchanged,
                request,
                predecessor: unchanged
                    .into_predecessor()
                    .expect("revalidation always submits one predecessor"),
            }));
        }
    };
    let (fact, retained) = transition.into_fact_and_predecessor();
    debug_assert!(retained.is_none());
    Ok((request, fact))
}

impl WorthUiScalarProjectionRevalidationStop {
    pub(super) fn into_error(self) -> WorthUiScalarProjectionAdvanceError {
        self.error
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        WorthUiScalarProjectionAdvanceError,
        AdmittedBridgeAsyncRequestIdentity,
        UiScalarProjectionFactReceipt,
    ) {
        (self.error, self.request, self.predecessor)
    }
}

fn bridge_stop(
    error: impl std::fmt::Debug,
    request: AdmittedBridgeAsyncRequestIdentity,
    predecessor: UiScalarProjectionFactReceipt,
) -> Box<WorthUiScalarProjectionRevalidationStop> {
    Box::new(WorthUiScalarProjectionRevalidationStop {
        error: WorthUiScalarProjectionAdvanceError::Bridge(format!("{error:?}")),
        request,
        predecessor,
    })
}
