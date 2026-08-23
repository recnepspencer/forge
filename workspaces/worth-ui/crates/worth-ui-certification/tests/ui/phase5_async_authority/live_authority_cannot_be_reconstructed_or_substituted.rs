use worth_ui_host_contract::UiMountedSurfacePresentationCompletion;
use worth_ui_query_binding::{WorthUiPresentationAsyncOwner, WorthUiPresentationPendingReceipt};

fn physical_completion_outside_signal_cannot_settle_query(
    owner: &mut WorthUiPresentationAsyncOwner,
    receipt: &WorthUiPresentationPendingReceipt,
    completion: UiMountedSurfacePresentationCompletion,
) {
    let _ = owner.admit_presented(receipt, completion);
}

fn main() {}
