use worth_ui::facade::{
    WorthUiPrimitiveProofTargetBinding, WorthUiRuntimeHost,
};

fn main() {}

fn primitive_target_cannot_authorize_mounted_interaction(
    runtime: &WorthUiRuntimeHost,
    target: WorthUiPrimitiveProofTargetBinding,
) {
    let _ = runtime.resolve_mounted_interaction_plan_for_target(target);
}
