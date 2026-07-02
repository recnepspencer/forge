use worth_ui::facade::declaration::{UiDeclarationArtifact, UiDeclarationGraphHandoff};
use worth_ui::facade::graph::{
    UiGraphInstantiationPlan, UiGraphWorldProfile, UiRuntimeInstanceBasisAdmission,
};

fn main() {
    let artifact = unsafe { std::mem::MaybeUninit::<UiDeclarationArtifact>::zeroed().assume_init() };
    let handoff =
        unsafe { std::mem::MaybeUninit::<UiDeclarationGraphHandoff>::zeroed().assume_init() };

    let _ = UiGraphInstantiationPlan::admit_handoffs(&[artifact], &[]);
    let _ = UiRuntimeInstanceBasisAdmission::admit_runtime_data_keyed(
        unsafe { std::mem::MaybeUninit::zeroed().assume_init_ref() },
        "row:user-7",
    );

    let plan = UiGraphInstantiationPlan {
        node_entries: Vec::new(),
        local_denials: Vec::new(),
    };

    let entry = &plan.node_entries()[0];
    let _ = entry.touch_meaning();
    let _ = entry.measurement_policy();
    let _ = entry.host_capability();
    let _ = plan.committed_snapshot(UiGraphWorldProfile::authoritative());
    let _ = handoff;
}
