#![allow(unreachable_code)]

use worth_ui::facade::WorthUiLiveViewProjectionRebindReceipt;

fn main() {
    let _forged = WorthUiLiveViewProjectionRebindReceipt {
        control_rebind: panic!("fixture only checks receipt field privacy"),
        conditional_rebind: panic!("fixture only checks receipt field privacy"),
        counters: Default::default(),
    };
}
