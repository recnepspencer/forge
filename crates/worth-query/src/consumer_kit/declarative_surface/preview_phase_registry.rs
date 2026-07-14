use super::model::{
    WorthQueryDeclarativeCapabilityFamily as Family,
    WorthQueryDeclarativePhaseResponsibility as Phase, WorthQueryDeclarativeSurfaceClass as Class,
    WorthQueryDeclarativeSurfaceRow as Row,
};

pub(super) fn preview_phase_surface_rows() -> &'static [Row] {
    PREVIEW_PHASE_ROWS
}

#[rustfmt::skip]
const PREVIEW_PHASE_ROWS: &[Row] = &[
    preview("admit_preview_workflow_foundation", Phase::Admit),
    preview("admit_preview_workflow_foundation_request", Phase::Admit),
    preview("admit_authoritative_preview_comparison_candidate", Phase::Admit),
    preview("admit_preview_promotion_parity_comparison", Phase::Admit),
    preview("admit_read_only_preview_session_plan_binding", Phase::Admit),
    preview("admit_promotion_eligible_preview_session_plan_binding", Phase::Admit),
    preview("execute_read_only_preview_session_plan", Phase::Execute),
    preview("execute_promotion_eligible_preview_session_plan", Phase::Execute),
    preview("bind_preflight_to_preview_session", Phase::Bind),
];

const fn preview(function: &'static str, phase: Phase) -> Row {
    Row::new(
        "src/preview/mod.rs",
        function,
        Family::Preview,
        phase,
        Class::Compatibility,
        Class::InternalMechanism,
        "advanced preview integration or Query implementation",
        "ordinary preview declaration",
    )
}
