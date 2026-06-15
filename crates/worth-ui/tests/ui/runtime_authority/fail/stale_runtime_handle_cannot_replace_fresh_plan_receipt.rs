use worth_ui::facade::{
    WorthUiComponentHandle, WorthUiHandlePlanGeneration, WorthUiRuntimeHandleAllocationReceipt,
};

fn main() {
    let generation = generation_from_public_option(None);
    let old_handle = WorthUiComponentHandle {
        plan_index: 0,
        plan_generation: generation,
    };
    let _receipt = WorthUiRuntimeHandleAllocationReceipt {
        basis_digest: 2,
        plan_generation: old_handle.plan_generation(),
    };
}

fn generation_from_public_option(
    generation: Option<WorthUiHandlePlanGeneration>,
) -> WorthUiHandlePlanGeneration {
    generation.expect("test fixture never runs")
}
