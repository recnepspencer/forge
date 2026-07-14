use super::*;

#[test]
fn resource_milestone_c_policy_scenario_matrix_rejects_wrong_restore_denial_class() {
    let fixture = resource_milestone_c_policy_fixture();

    let err = resource_milestone_c_policy_scenario_matrix(
        &fixture.bundle,
        &fixture.freeze_report,
        &fixture.denied_retry_report,
        &fixture.heartbeat_denial_report,
        &fixture.retention_report,
        &fixture.diagnostics_denial,
        &fixture.compatible_restore,
        &fixture.missing_restore,
        &fixture.missing_restore,
    )
    .expect_err("wrong restore denial class should reject the matrix");
    assert!(format!("{err}").contains("requires VersionIncompatible denial evidence"));
}
