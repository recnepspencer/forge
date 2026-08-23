use worth_ui_host_native::UiNativePhysicalSignalTransitionObservation;
use worth_ui_query_binding::{WorthUiPresentationAsyncOwner, WorthUiPresentationPendingReceipt};

fn signal_observation_cannot_authorize_query_effect(
    owner: &mut WorthUiPresentationAsyncOwner,
    receipt: &WorthUiPresentationPendingReceipt,
    observation: UiNativePhysicalSignalTransitionObservation,
) {
    let _ = owner.admit_presented(receipt, observation);
}

fn main() {}
