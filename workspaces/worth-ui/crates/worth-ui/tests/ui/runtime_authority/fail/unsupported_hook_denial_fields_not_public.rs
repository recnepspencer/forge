use worth_ui::facade::{WorthUiUnsupportedHookDenial, WorthUiUnsupportedHookDenialReason};

fn uninhabited<T>() -> T {
    panic!("compile-fail fixture never runs")
}

fn main() {
    let _denial = WorthUiUnsupportedHookDenial {
        hook: uninhabited(),
        reason: WorthUiUnsupportedHookDenialReason::ActivePlanTruthOverride,
        counters: uninhabited(),
    };
}
