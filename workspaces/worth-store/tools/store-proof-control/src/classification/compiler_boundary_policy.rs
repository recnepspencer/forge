use super::ClassifiedProof;
use crate::discovery::CaseKind;

pub(super) fn violations(proof: &ClassifiedProof) -> Vec<String> {
    let mut violations = Vec::new();
    if proof.case.compiler_boundary_harness.is_some()
        && (!proof.case.launches_child_process
            || !proof.case.launches_nested_cargo
            || proof.case.process_model != "standardized-ui-harness"
            || !proof.case.external_tools.iter().any(|tool| tool == "cargo"))
    {
        violations.push(format!(
            "standardized UI harness has dishonest process metadata: {}",
            proof.case.identity.stable_id
        ));
    }
    if proof.case.kind == CaseKind::UiFixture && proof.case.compiler_boundary_harness.is_none() {
        violations.push(format!(
            "UI fixture is not owned by the standardized harness: {}",
            proof.case.identity.stable_id
        ));
    }
    violations
}
