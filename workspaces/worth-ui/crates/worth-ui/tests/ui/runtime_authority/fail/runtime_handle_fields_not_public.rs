use worth_ui::facade::{
    WorthUiHandlePlanGeneration, WorthUiPlanNodeInputFamily, WorthUiRuntimeHandle,
};

fn main() {
    let generation = generation_from_public_option(None);
    let _handle = WorthUiRuntimeHandle {
        family: WorthUiPlanNodeInputFamily::ComponentInvocation,
        plan_index: 0,
        plan_generation: generation,
    };
}

fn generation_from_public_option(
    generation: Option<WorthUiHandlePlanGeneration>,
) -> WorthUiHandlePlanGeneration {
    generation.expect("test fixture never runs")
}
