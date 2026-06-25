use worth_ui::facade::{
    WorthUiMountedInteractionActivationEligibleReceipt, WorthUiMountedInteractionGesture,
};

fn main() {
    let _forged = WorthUiMountedInteractionActivationEligibleReceipt {
        surface_id: fake(),
        component_id: fake(),
        interaction_id: "worth.interaction.forged".to_owned(),
        kind: fake(),
        gesture: WorthUiMountedInteractionGesture::primary_click(),
        receipt: fake(),
        operability: fake(),
        receipt_digest: 7,
    };
}

fn fake<T>() -> T {
    unsafe { std::mem::MaybeUninit::<T>::uninit().assume_init() }
}
