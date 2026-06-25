use worth_ui::facade::{
    WorthUiLiveViewInteractionActivationEligibleReceipt, WorthUiLiveViewInteractionIntentReceipt,
};

fn main() {
    let interaction = interaction();

    let _forged = WorthUiLiveViewInteractionActivationEligibleReceipt { interaction };
}

fn interaction() -> WorthUiLiveViewInteractionIntentReceipt {
    unimplemented!("the fixture only type-checks the sealed receipt boundary")
}
