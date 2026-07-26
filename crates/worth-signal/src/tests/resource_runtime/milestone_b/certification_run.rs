use super::super::diagnostics::resource_malformed_completion_report;
use super::support::*;
use super::*;

#[test]
fn resource_milestone_b_certification_run_requires_complete_passing_bundle() {
    let (complete, hostile_evidence, summary_read, diagnostics_summary, diagnostics_denial) =
        resource_certification_fixture_artifacts(ResourceRequestId::new(9_999));
    let scenario_matrix = resource_milestone_b_scenario_matrix(&complete, &hostile_evidence)
        .expect("complete passing resource bundle should produce scenario matrix");
    let performance_closeout = resource_milestone_b_performance_closeout(
        &scenario_matrix,
        summary_read,
        diagnostics_summary,
        diagnostics_denial,
    )
    .expect("complete passing resource evidence should produce performance closeout");
    let run = resource_milestone_b_certification_run(
        complete.clone(),
        scenario_matrix.clone(),
        performance_closeout.clone(),
    )
    .expect("complete passing resource bundle should close milestone B certification");

    assert!(run.passed());
    assert!(scenario_matrix.passed());
    assert!(performance_closeout.passed());
    assert_eq!(
        run.schema_version(),
        RESOURCE_MILESTONE_B_CERTIFICATION_RUN_SCHEMA_VERSION
    );
    assert_eq!(run.bundle().bundle_digest(), complete.bundle_digest());
    assert_eq!(run.scenario_matrix(), &scenario_matrix);
    assert_eq!(run.performance_closeout(), &performance_closeout);
    assert_eq!(
        scenario_matrix.schema_version(),
        RESOURCE_MILESTONE_B_SCENARIO_MATRIX_SCHEMA_VERSION
    );
    assert_eq!(scenario_matrix.bundle_digest(), complete.bundle_digest());
    assert_eq!(
        scenario_matrix.rows().len(),
        REQUIRED_RESOURCE_MILESTONE_B_SCENARIOS.len()
    );
    assert_eq!(
        hostile_evidence.rows().len(),
        REQUIRED_RESOURCE_MILESTONE_B_HOSTILE_SCENARIOS.len()
    );
    assert_eq!(
        hostile_evidence.schema_version(),
        RESOURCE_MILESTONE_B_HOSTILE_SCENARIO_EVIDENCE_SCHEMA_VERSION
    );
    assert_eq!(
        performance_closeout.schema_version(),
        RESOURCE_MILESTONE_B_PERFORMANCE_CLOSEOUT_SCHEMA_VERSION
    );
    assert_eq!(
        performance_closeout.scenario_matrix_digest(),
        scenario_matrix.matrix_digest()
    );
    assert_eq!(
        performance_closeout.rows().len(),
        REQUIRED_RESOURCE_MILESTONE_B_PERFORMANCE_CLAIMS.len()
    );
    assert_eq!(
        scenario_matrix.summary().required_scenario_count(),
        REQUIRED_RESOURCE_MILESTONE_B_SCENARIOS.len() as u32
    );
    assert_eq!(
        scenario_matrix.summary().certified_scenario_count(),
        REQUIRED_RESOURCE_MILESTONE_B_SCENARIOS.len() as u32
    );
    assert_eq!(scenario_matrix.summary().failed_scenario_count(), 0);
    assert_eq!(
        scenario_matrix.summary().bundle_digest(),
        complete.bundle_digest()
    );
    assert_eq!(
        performance_closeout.summary().required_claim_count(),
        REQUIRED_RESOURCE_MILESTONE_B_PERFORMANCE_CLAIMS.len() as u32
    );
    assert_eq!(
        performance_closeout.summary().certified_claim_count(),
        REQUIRED_RESOURCE_MILESTONE_B_PERFORMANCE_CLAIMS.len() as u32
    );
    assert_eq!(performance_closeout.summary().failed_claim_count(), 0);
    assert_eq!(
        performance_closeout.summary().scenario_matrix_digest(),
        scenario_matrix.matrix_digest()
    );

    for scenario in REQUIRED_RESOURCE_MILESTONE_B_SCENARIOS {
        let row = required_scenario_row(&scenario_matrix, scenario);
        assert_eq!(row.certification_family(), scenario.certification_family());
        assert_eq!(
            row.completion_denial_class(),
            scenario.completion_denial_class()
        );
        assert!(row.passed());
        assert!(!row.evidence_digest().is_empty());
    }
    for scenario in REQUIRED_RESOURCE_MILESTONE_B_HOSTILE_SCENARIOS {
        let evidence_row = required_hostile_evidence_row(&hostile_evidence, scenario);
        assert_hostile_evidence_shape(evidence_row);

        let matrix_row = required_scenario_row(&scenario_matrix, scenario);
        assert_eq!(
            matrix_row.evidence_kind(),
            ResourceMilestoneBScenarioEvidenceKind::HostileCompletionDenial
        );
        assert_eq!(
            matrix_row.completion_denial_class(),
            scenario.completion_denial_class()
        );
        assert_eq!(
            matrix_row.performance(),
            evidence_row.performance(),
            "hostile matrix row should preserve the source evidence performance envelope"
        );
        assert!(matrix_row.certification_family().is_none());
        assert!(matrix_row.passed());
    }
    for claim in REQUIRED_RESOURCE_MILESTONE_B_PERFORMANCE_CLAIMS {
        assert_performance_closeout_claim_shape(required_performance_claim_row(
            &performance_closeout,
            claim,
        ));
    }
    assert_eq!(
        run.summary().required_family_count(),
        REQUIRED_RESOURCE_CERTIFICATION_FAMILIES.len() as u32
    );
    assert_eq!(
        run.summary().certified_family_count(),
        REQUIRED_RESOURCE_CERTIFICATION_FAMILIES.len() as u32
    );
    assert_eq!(run.summary().failed_family_count(), 0);
    assert_eq!(run.summary().bundle_digest(), complete.bundle_digest());
    assert_eq!(
        run.summary().required_scenario_count(),
        REQUIRED_RESOURCE_MILESTONE_B_SCENARIOS.len() as u32
    );
    assert_eq!(
        run.summary().certified_scenario_count(),
        REQUIRED_RESOURCE_MILESTONE_B_SCENARIOS.len() as u32
    );
    assert_eq!(
        run.summary().scenario_matrix_digest(),
        scenario_matrix.matrix_digest()
    );
    assert_eq!(
        run.summary().required_performance_claim_count(),
        REQUIRED_RESOURCE_MILESTONE_B_PERFORMANCE_CLAIMS.len() as u32
    );
    assert_eq!(
        run.summary().certified_performance_claim_count(),
        REQUIRED_RESOURCE_MILESTONE_B_PERFORMANCE_CLAIMS.len() as u32
    );
    assert_eq!(
        run.summary().performance_closeout_digest(),
        performance_closeout.closeout_digest()
    );
    assert!(!run.run_digest().is_empty());
    let serialized_run =
        serde_json::to_value(&run).expect("closeout certification run should serialize");
    assert_eq!(
        serialized_run["scenarioMatrix"]["matrixDigest"],
        scenario_matrix.matrix_digest()
    );
    assert_eq!(
        serialized_run["summary"]["scenarioMatrixDigest"],
        scenario_matrix.matrix_digest()
    );
    assert_eq!(
        serialized_run["summary"]["performanceCloseoutDigest"],
        performance_closeout.closeout_digest()
    );

    let incomplete = resource_certification_bundle([]);
    let err = resource_milestone_b_scenario_matrix(&incomplete, &hostile_evidence)
        .expect_err("incomplete certification bundle must not become scenario evidence");
    assert!(format!("{err}").contains("resource certification bundle failed"));
    let err = resource_milestone_b_certification_run(
        incomplete,
        scenario_matrix.clone(),
        performance_closeout.clone(),
    )
    .expect_err("incomplete certification bundle must not become a milestone run");
    assert!(format!("{err}").contains("resource certification bundle failed"));
    let misclassified_hostile = resource_milestone_b_hostile_scenario_evidence(
        resource_late_cancelled_completion_report(),
        resource_late_cancelled_completion_report(),
        resource_late_timed_out_completion_report(),
        resource_malformed_completion_report(),
        &resource_async_inflight_pressure_workload().pressure_batch,
    )
    .expect_err("hostile scenario evidence must reject the wrong denial class per row");
    assert!(format!("{misclassified_hostile}").contains("requires Superseded denial evidence"));

    let (
        drifted,
        drifted_hostile_evidence,
        drifted_summary_read,
        drifted_diagnostics_summary,
        drifted_diagnostics_denial,
    ) = resource_certification_fixture_artifacts(ResourceRequestId::new(9_998));
    let drifted_matrix = resource_milestone_b_scenario_matrix(&drifted, &drifted_hostile_evidence)
        .expect("drifted but complete bundle should produce its own scenario matrix");
    let drifted_performance_closeout = resource_milestone_b_performance_closeout(
        &drifted_matrix,
        drifted_summary_read,
        drifted_diagnostics_summary,
        drifted_diagnostics_denial,
    )
    .expect("drifted but complete evidence should produce performance closeout");
    let wrong_matrix_err = resource_milestone_b_certification_run(
        complete,
        drifted_matrix.clone(),
        drifted_performance_closeout.clone(),
    )
    .expect_err("scenario matrix from a different bundle must not close the run");
    assert!(format!("{wrong_matrix_err}").contains("same bundle"));
    let wrong_performance_err = resource_milestone_b_certification_run(
        drifted.clone(),
        drifted_matrix.clone(),
        performance_closeout,
    )
    .expect_err("performance closeout from a different matrix must not close the run");
    assert!(format!("{wrong_performance_err}").contains("same scenario matrix"));
    let drifted_run = resource_milestone_b_certification_run(
        drifted,
        drifted_matrix,
        drifted_performance_closeout,
    )
    .expect("drifted but complete bundle should still produce its own run");
    assert_ne!(
        run.bundle().bundle_digest(),
        drifted_run.bundle().bundle_digest()
    );
    assert_ne!(
        run.scenario_matrix().matrix_digest(),
        drifted_run.scenario_matrix().matrix_digest()
    );
    assert_ne!(run.run_digest(), drifted_run.run_digest());
}
