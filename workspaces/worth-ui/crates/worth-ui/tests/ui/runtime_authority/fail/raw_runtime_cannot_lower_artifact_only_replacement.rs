use worth_ui::facade::{
    WorthUiAdmittedReplacementCandidate, WorthUiDurableStateInventory, WorthUiRuntime,
};

fn lower_without_prepared_application(
    runtime: &WorthUiRuntime,
    admitted: WorthUiAdmittedReplacementCandidate,
    inventory: &WorthUiDurableStateInventory,
) {
    let _ = runtime.prepare_replacement_lowering(admitted, inventory);
}

fn main() {}
