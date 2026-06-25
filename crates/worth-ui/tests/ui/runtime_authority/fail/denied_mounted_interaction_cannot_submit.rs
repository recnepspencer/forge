use worth_ui::facade::{
    WorthUiMountedInteractionActivationDeniedReceipt, WorthUiRuntimeHost,
};

fn submit_denied(
    runtime: &mut WorthUiRuntimeHost,
    denied: WorthUiMountedInteractionActivationDeniedReceipt,
) {
    let _ = runtime.submit_mounted_interaction(denied);
}

fn main() {}
