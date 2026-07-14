use super::model::{
    WorthQueryDeclarativeCapabilityFamily as Family,
    WorthQueryDeclarativePhaseResponsibility as Phase, WorthQueryDeclarativeSurfaceClass as Class,
    WorthQueryDeclarativeSurfaceRow as Row,
};

pub(super) fn policy_phase_surface_rows() -> &'static [Row] {
    POLICY_PHASE_ROWS
}

#[rustfmt::skip]
const POLICY_PHASE_ROWS: &[Row] = &[
    policy("src/policy_basis/admission.rs", "admit_policy_tenant_context", Phase::Admit, "declarative policy-tenant context"),
    policy("src/policy_delivery/shape.rs", "lower_policy_aware_delivery_shape", Phase::Lower, "Query-owned outcome delivery"),
    policy("src/policy_live/admission.rs", "admit_policy_aware_live_plan", Phase::Admit, "managed policy-aware live declaration"),
    policy("src/policy_plan/branch.rs", "lower_policy_aware_branch_plan", Phase::Lower, "declarative admitted read context"),
    policy("src/policy_plan/current.rs", "lower_policy_aware_current_plan", Phase::Lower, "declarative admitted read context"),
    policy("src/policy_plan/diff.rs", "lower_policy_aware_diff_plan", Phase::Lower, "ordinary comparison declaration"),
    policy("src/policy_plan/historical.rs", "lower_policy_aware_historical_plan", Phase::Lower, "ordinary historical declaration"),
    policy("src/policy_plan/optimizer.rs", "lower_policy_aware_optimizer_input", Phase::Lower, "Query-owned planning"),
];

const fn policy(
    source: &'static str,
    function: &'static str,
    phase: Phase,
    replacement: &'static str,
) -> Row {
    Row::new(
        source,
        function,
        Family::GeneralDeclaration,
        phase,
        Class::Compatibility,
        Class::InternalMechanism,
        "advanced policy integration or Query implementation",
        replacement,
    )
}
