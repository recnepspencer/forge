use super::source;

#[test]
fn degraded_rebind_and_execution_retain_exact_source_authority() {
    let rebind = source("src/access/execution/degraded_scan/rebind.rs");
    let execution = source("src/access/execution/degraded_scan/executed.rs");

    for required in [
        "validate_equivalent_request(stale, replacement)",
        "require_current_at(expected_frontier)",
        "stale_selected.admitted_family() != replacement.admitted_family()",
        "stale_selected.request_identity() != replacement.request_identity()",
    ] {
        assert!(
            rebind.contains(required),
            "degraded rebind omits source-binding check {required}"
        );
    }
    for required in [
        "selected.admitted_family().authority_identity()",
        "physical.store_identity().authority_identity()",
        "PhysicalDegradedExecutionDenial::StoreAuthorityMismatch",
    ] {
        assert!(
            execution.contains(required),
            "physical degraded execution omits Store binding {required}"
        );
    }
}
