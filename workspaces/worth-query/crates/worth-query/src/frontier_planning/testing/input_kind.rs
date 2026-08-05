use super::FrontierPlanningInput;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::frontier_planning::testing) enum FrontierInputKind {
    ExecutionPreflight,
    LivePlan,
}

pub(in crate::frontier_planning::testing) fn frontier_input_kind(
    input: &FrontierPlanningInput,
) -> FrontierInputKind {
    match input {
        FrontierPlanningInput::ExecutionPreflight(_) => FrontierInputKind::ExecutionPreflight,
        FrontierPlanningInput::LivePlan(_) => FrontierInputKind::LivePlan,
    }
}
