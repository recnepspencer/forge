use worth_ui::facade::{
    WorthUiAppearanceStatePosture, WorthUiPrimitiveObservedPostureReceipt,
};

fn main() {
    let _receipt = WorthUiPrimitiveObservedPostureReceipt {
        posture: posture(),
    };

    panic!("compile-fail fixture only checks observed posture receipt field privacy");
}

fn posture() -> WorthUiAppearanceStatePosture {
    panic!("fixture only checks observed posture receipt field privacy")
}
