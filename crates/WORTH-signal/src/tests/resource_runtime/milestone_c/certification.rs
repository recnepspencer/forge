use super::*;

#[test]
fn resource_milestone_c_policy_certification_bundle_and_scenario_matrix_use_production_reports() {
    let fixture = resource_milestone_c_policy_fixture();

    let bundle = fixture.bundle.clone();
    let matrix = resource_milestone_c_policy_scenario_matrix(
        &bundle,
        &fixture.freeze_report,
        &fixture.denied_retry_report,
        &fixture.heartbeat_denial_report,
        &fixture.retention_report,
        &fixture.diagnostics_denial,
        &fixture.compatible_restore,
        &fixture.incompatible_restore,
        &fixture.missing_restore,
    )
    .expect("complete milestone C policy scenario evidence should admit matrix");

    assert_eq!(
        bundle.summary().required_family_count(),
        REQUIRED_RESOURCE_MILESTONE_C_POLICY_CERTIFICATION_FAMILIES.len() as u32
    );
    assert_eq!(
        bundle.summary().certified_family_count(),
        REQUIRED_RESOURCE_MILESTONE_C_POLICY_CERTIFICATION_FAMILIES.len() as u32
    );
    assert!(bundle.passed());
    assert_eq!(
        matrix.summary().required_scenario_count(),
        REQUIRED_RESOURCE_MILESTONE_C_POLICY_SCENARIOS.len() as u32
    );
    assert_eq!(
        matrix.summary().certified_scenario_count(),
        REQUIRED_RESOURCE_MILESTONE_C_POLICY_SCENARIOS.len() as u32
    );
    assert_eq!(matrix.summary().failed_scenario_count(), 0);
    assert_eq!(matrix.summary().bundle_digest(), bundle.bundle_digest());
    assert!(matrix.passed());
    assert!(matrix
        .rows()
        .iter()
        .all(|row| row.passed() && !row.evidence_digest().is_empty()));

    let closeout = resource_milestone_c_policy_performance_closeout(&matrix)
        .expect("passing milestone C policy scenario matrix should yield a performance closeout");
    assert_eq!(
        closeout.schema_version(),
        RESOURCE_MILESTONE_C_POLICY_PERFORMANCE_CLOSEOUT_SCHEMA_VERSION
    );
    assert_eq!(closeout.scenario_matrix_digest(), matrix.matrix_digest());
    assert_eq!(
        closeout.summary().required_claim_count(),
        REQUIRED_RESOURCE_MILESTONE_C_POLICY_PERFORMANCE_CLAIMS.len() as u32
    );
    assert_eq!(
        closeout.summary().certified_claim_count(),
        REQUIRED_RESOURCE_MILESTONE_C_POLICY_PERFORMANCE_CLAIMS.len() as u32
    );
    assert_eq!(closeout.summary().failed_claim_count(), 0);
    assert_eq!(
        closeout.summary().scenario_matrix_digest(),
        matrix.matrix_digest()
    );
    assert_eq!(
        closeout.rows().len(),
        REQUIRED_RESOURCE_MILESTONE_C_POLICY_PERFORMANCE_CLAIMS.len()
    );
    assert!(closeout.passed());
    assert!(!closeout.closeout_digest().is_empty());
    for claim in REQUIRED_RESOURCE_MILESTONE_C_POLICY_PERFORMANCE_CLAIMS {
        let row = required_milestone_c_policy_performance_claim_row(&closeout, claim);
        assert_milestone_c_policy_performance_closeout_claim_shape(row);
    }

    let run =
        resource_milestone_c_certification_run(bundle.clone(), matrix.clone(), closeout.clone())
            .expect(
            "passing milestone C bundle, matrix, and closeout should yield final certification run",
        );
    assert_eq!(
        run.schema_version(),
        RESOURCE_MILESTONE_C_CERTIFICATION_RUN_SCHEMA_VERSION
    );
    assert_eq!(run.bundle().bundle_digest(), bundle.bundle_digest());
    assert_eq!(
        run.scenario_matrix().matrix_digest(),
        matrix.matrix_digest()
    );
    assert_eq!(
        run.performance_closeout().closeout_digest(),
        closeout.closeout_digest()
    );
    assert_eq!(
        run.summary().required_family_count(),
        REQUIRED_RESOURCE_MILESTONE_C_POLICY_CERTIFICATION_FAMILIES.len() as u32
    );
    assert_eq!(
        run.summary().certified_family_count(),
        REQUIRED_RESOURCE_MILESTONE_C_POLICY_CERTIFICATION_FAMILIES.len() as u32
    );
    assert_eq!(run.summary().failed_family_count(), 0);
    assert_eq!(run.summary().bundle_digest(), bundle.bundle_digest());
    assert_eq!(
        run.summary().required_scenario_count(),
        REQUIRED_RESOURCE_MILESTONE_C_POLICY_SCENARIOS.len() as u32
    );
    assert_eq!(
        run.summary().certified_scenario_count(),
        REQUIRED_RESOURCE_MILESTONE_C_POLICY_SCENARIOS.len() as u32
    );
    assert_eq!(
        run.summary().scenario_matrix_digest(),
        matrix.matrix_digest()
    );
    assert_eq!(
        run.summary().required_performance_claim_count(),
        REQUIRED_RESOURCE_MILESTONE_C_POLICY_PERFORMANCE_CLAIMS.len() as u32
    );
    assert_eq!(
        run.summary().certified_performance_claim_count(),
        REQUIRED_RESOURCE_MILESTONE_C_POLICY_PERFORMANCE_CLAIMS.len() as u32
    );
    assert_eq!(
        run.summary().performance_closeout_digest(),
        closeout.closeout_digest()
    );
    assert!(run.passed());
    assert!(!run.run_digest().is_empty());

    let reordered_bundle =
        resource_milestone_c_policy_certification_bundle(bundle.records().iter().cloned().rev());
    assert_eq!(
        reordered_bundle.bundle_digest(),
        bundle.bundle_digest(),
        "equivalent certification-family evidence order must not perturb bundle identity"
    );
    let reordered_matrix = resource_milestone_c_policy_scenario_matrix(
        &reordered_bundle,
        &fixture.freeze_report,
        &fixture.denied_retry_report,
        &fixture.heartbeat_denial_report,
        &fixture.retention_report,
        &fixture.diagnostics_denial,
        &fixture.compatible_restore,
        &fixture.incompatible_restore,
        &fixture.missing_restore,
    )
    .expect("equivalent certification-family evidence order should preserve scenario matrix");
    assert_eq!(
        reordered_matrix.matrix_digest(),
        matrix.matrix_digest(),
        "equivalent policy scenario evidence order must not perturb matrix identity"
    );
    let reordered_closeout = resource_milestone_c_policy_performance_closeout(&reordered_matrix)
        .expect("equivalent scenario evidence order should preserve performance closeout");
    assert_eq!(
        reordered_closeout.closeout_digest(),
        closeout.closeout_digest(),
        "equivalent performance claim evidence order must not perturb closeout identity"
    );
    let reordered_run = resource_milestone_c_certification_run(
        reordered_bundle,
        reordered_matrix,
        reordered_closeout,
    )
    .expect("equivalent milestone C certification evidence order should preserve final run");
    assert_eq!(
        reordered_run.run_digest(),
        run.run_digest(),
        "equivalent certification evidence order must not perturb final milestone C run identity"
    );

    let incomplete_bundle = resource_milestone_c_policy_certification_bundle(
        bundle.records()[..bundle.records().len() - 1]
            .iter()
            .cloned(),
    );
    let err = resource_milestone_c_certification_run(incomplete_bundle, matrix, closeout)
        .expect_err("final certification run should deny incomplete bundle coverage");
    assert!(
        err.to_string().contains("failed completeness checks"),
        "unexpected error: {err}"
    );
}
