use worth_ui::facade::{WorthUiLiveViewInteractionIntentReceipt, WorthUiRuntimeHost};

fn main() {
    let runtime = unsafe { std::mem::zeroed::<WorthUiRuntimeHost>() };
    let interaction = unsafe { std::mem::zeroed::<WorthUiLiveViewInteractionIntentReceipt>() };

    let _receipt = runtime.submit_live_view_interaction(&interaction);
}
