use worth_ui::facade::app::WorthUiActiveApplicationSession;
use worth_ui::facade::observation::UiClassifiedChange;
use worth_ui::facade::rebind::{
    UiAuthoredChangedFact, UiAuthoredFactKind, UiAuthoredFactSelector,
    UiRebindExecutionPolicy, UiRebindPlan, UiResolvedAffectedScope,
};

fn forge_fact() {
    let _ = UiAuthoredChangedFact::new(
        UiAuthoredFactSelector::Node("component:forged".into()),
        UiAuthoredFactKind::SemanticsChanged,
    );
}

fn construct_raw_scope() {
    let _ = UiResolvedAffectedScope::new(());
}

fn substitute_wrong_phase(
    session: &WorthUiActiveApplicationSession,
    change: UiClassifiedChange,
) {
    let _ = session.compile_rebind_plan(change, UiRebindExecutionPolicy::ordinary());
}

fn mutate_compiled_plan(plan: &mut UiRebindPlan) {
    plan.set_effects(Vec::new());
}

fn attach_executor_mechanism() {
    let _ = UiRebindExecutionPolicy::ordinary().with_executor(|| {});
}

fn main() {
    let _ = (
        forge_fact,
        construct_raw_scope,
        substitute_wrong_phase,
        mutate_compiled_plan,
        attach_executor_mechanism,
    );
}
