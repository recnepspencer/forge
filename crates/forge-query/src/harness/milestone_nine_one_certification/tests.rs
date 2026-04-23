use super::{
    MilestoneNineOneCertificationAdapter, MilestoneNineOneFailureClass,
    MILESTONE_NINE_ONE_REQUIRED_COMPILE_FAIL_TARGETS,
};
use crate::harness::certification::{
    contains_row, milestone_nine_one_requirements, unmet_required_assertion_classes,
    unmet_required_rows, HostileExpectation, ParityAnchor, RequiredAssertionClass,
};

#[test]
fn milestone_nine_one_certification_adapter_emits_named_matrix() {
    let artifact = MilestoneNineOneCertificationAdapter::
        query_subscription_declaration_and_lowering_certification_artifact();

    assert_eq!(
        artifact.suite_name,
        "Query Subscription Declaration And Lowering Parity Test"
    );
    assert!(!artifact.certification_bundle_digest.is_empty());
    assert!(!artifact.coverage_matrix_digest.is_empty());
}

#[test]
fn milestone_nine_one_certification_matrix_meets_required_rows() {
    let matrix = MilestoneNineOneCertificationAdapter::
        query_subscription_declaration_and_lowering_parity_test();
    let requirements = milestone_nine_one_requirements();
    let missing = unmet_required_rows(
        &matrix,
        requirements.required_canonical_rows,
        requirements.required_rejection_rows,
    );

    assert!(
        missing.is_empty(),
        "missing milestone 9.1 certification rows: {missing:?}"
    );
}

#[test]
fn milestone_nine_one_rows_have_required_outputs() {
    let matrix = MilestoneNineOneCertificationAdapter::
        query_subscription_declaration_and_lowering_parity_test();

    for row in &matrix.rows {
        assert!(
            row.control_lane.has_required_outputs(),
            "control lane '{}' should emit required proof outputs",
            row.row_name
        );
        assert!(
            row.hostile_lane.has_required_outputs(),
            "hostile lane '{}' should emit required proof outputs",
            row.row_name
        );
        assert!(
            row.parity_lane.has_required_outputs(),
            "parity lane '{}' should emit required proof outputs",
            row.row_name
        );
    }

    for row in &matrix.rejection_rows {
        assert!(
            row.control_lane.has_required_outputs(),
            "rejection row '{}' needs a successful control lane",
            row.row_name
        );
        assert!(
            row.parity_lane.has_required_outputs(),
            "rejection row '{}' needs a successful parity lane",
            row.row_name
        );
        assert!(!row.hostile_lane.failure_kind.is_empty());
        assert!(!row.hostile_lane.diagnostic_stage.is_empty());
        assert!(!row.hostile_lane.diagnostic_digest.is_empty());
        assert!(!row.hostile_lane.failure_digest.is_empty());
        assert!(!row.hostile_lane.counter_snapshot_digest.is_empty());
    }
}

#[test]
fn milestone_nine_one_rows_enforce_declared_lane_semantics() {
    let matrix = MilestoneNineOneCertificationAdapter::
        query_subscription_declaration_and_lowering_parity_test();
    let mut covered = Vec::new();

    for row in &matrix.rows {
        let control = row.control_lane.subscription_semantic_signature();
        let hostile = row.hostile_lane.subscription_semantic_signature();
        let parity = row.parity_lane.subscription_semantic_signature();

        match row.hostile_expectation {
            HostileExpectation::EquivalentToControl => assert_eq!(
                control, hostile,
                "row '{}' declares hostile equivalence but emits different subscription evidence",
                row.row_name
            ),
            HostileExpectation::DistinctFromControl => assert_ne!(
                control, hostile,
                "row '{}' declares hostile distinction but emits identical subscription evidence",
                row.row_name
            ),
        }
        match row.hostile_expectation {
            HostileExpectation::EquivalentToControl => {
                covered.push(RequiredAssertionClass::Equality)
            }
            HostileExpectation::DistinctFromControl => {
                covered.push(RequiredAssertionClass::Inequality)
            }
        }

        match row.parity_anchor {
            ParityAnchor::Control => assert_eq!(
                parity, control,
                "row '{}' parity lane must independently match the control anchor",
                row.row_name
            ),
            ParityAnchor::Hostile => assert_eq!(
                parity, hostile,
                "row '{}' parity lane must independently match the hostile anchor",
                row.row_name
            ),
        }
    }

    for row in &matrix.rejection_rows {
        assert!(!row.hostile_lane.failure_digest.is_empty());
        covered.push(RequiredAssertionClass::TypedFailure);
        if row.hostile_lane.counter_snapshot_digest != row.control_lane.counter_snapshot_digest {
            covered.push(RequiredAssertionClass::ZeroResidue);
        }
    }

    covered.sort();
    covered.dedup();
    let missing = unmet_required_assertion_classes(
        &covered,
        milestone_nine_one_requirements().required_assertion_classes,
    );
    assert!(
        missing.is_empty(),
        "missing milestone 9.1 assertion classes: {missing:?}"
    );
}

#[test]
fn milestone_nine_one_covers_named_subscription_proof_rows() {
    let matrix = MilestoneNineOneCertificationAdapter::
        query_subscription_declaration_and_lowering_parity_test();

    assert!(contains_row(
        &matrix,
        "detail-direct-scope-template-saved-facade-parity"
    ));
    assert!(contains_row(
        &matrix,
        "direct-scope-template-saved-subscription-parity"
    ));
    assert!(contains_row(&matrix, "facade-helper-subscription-parity"));
    assert!(contains_row(
        &matrix,
        "grouped-query-meaning-shares-collection-bridge-family"
    ));
    assert!(contains_row(
        &matrix,
        "inspector-query-meaning-shares-detail-bridge-family"
    ));
    assert!(contains_row(
        &matrix,
        "activation-certification-source-binding"
    ));
    assert!(contains_row(
        &matrix,
        "basis-request-binds-policy-tenant-meaning"
    ));
    assert!(contains_row(
        &matrix,
        "relationship-proof-binds-subscription-meaning"
    ));
    assert!(contains_row(&matrix, "scale-slope-row-count-only-honesty"));
}

#[test]
fn milestone_nine_one_bundles_expose_spec_required_verification_outputs() {
    let matrix = MilestoneNineOneCertificationAdapter::
        query_subscription_declaration_and_lowering_parity_test();

    for row in &matrix.rows {
        for (lane_name, lane) in [
            ("control", &row.control_lane),
            ("hostile", &row.hostile_lane),
            ("parity", &row.parity_lane),
        ] {
            assert!(
                !lane.query_digest.is_empty(),
                "row '{}' {lane_name} lane must expose query digest",
                row.row_name
            );
            assert!(
                !lane.live_family_digest.is_empty(),
                "row '{}' {lane_name} lane must expose live family digest",
                row.row_name
            );
            assert!(
                !lane.subscription_family_digest.is_empty(),
                "row '{}' {lane_name} lane must expose subscription family digest",
                row.row_name
            );
            assert!(
                !lane.subscription_equivalence_digest.is_empty(),
                "row '{}' {lane_name} lane must expose subscription equivalence digest",
                row.row_name
            );
            assert!(
                !lane.policy_digest.is_empty(),
                "row '{}' {lane_name} lane must expose policy digest",
                row.row_name
            );
            assert!(
                !lane.tenant_basis_digest.is_empty(),
                "row '{}' {lane_name} lane must expose tenant basis digest",
                row.row_name
            );
            assert!(
                !lane.relationship_proof_digest.is_empty(),
                "row '{}' {lane_name} lane must expose relationship proof digest",
                row.row_name
            );
            assert!(
                !lane.view_shape_digest.is_empty(),
                "row '{}' {lane_name} lane must expose view shape digest",
                row.row_name
            );
            assert!(
                !lane.basis_digest.is_empty(),
                "row '{}' {lane_name} lane must expose basis digest",
                row.row_name
            );
            assert!(
                !lane.fixture_digest.is_empty(),
                "row '{}' {lane_name} lane must expose fixture digest",
                row.row_name
            );
            assert!(
                !lane.compile_fail_boundary_digest.is_empty(),
                "row '{}' {lane_name} lane must expose compile-fail boundary digest",
                row.row_name
            );
            assert!(
                !lane.support_matrix_digest.is_empty(),
                "row '{}' {lane_name} lane must expose support matrix digest",
                row.row_name
            );
        }
    }
}

#[test]
fn milestone_nine_one_required_compile_fail_targets_are_present() {
    let ui_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/ui");
    let missing = MILESTONE_NINE_ONE_REQUIRED_COMPILE_FAIL_TARGETS
        .iter()
        .filter(|target| !ui_root.join(target).is_file())
        .copied()
        .collect::<Vec<_>>();
    let missing_stderr = MILESTONE_NINE_ONE_REQUIRED_COMPILE_FAIL_TARGETS
        .iter()
        .map(|target| target.trim_end_matches(".rs").to_string() + ".stderr")
        .filter(|target| !ui_root.join(target).is_file())
        .collect::<Vec<_>>();

    assert!(
        missing.is_empty(),
        "missing milestone 9.1 compile-fail targets: {missing:?}"
    );
    assert!(
        missing_stderr.is_empty(),
        "missing milestone 9.1 compile-fail baselines: {missing_stderr:?}"
    );
}

#[test]
fn milestone_nine_one_shared_bridge_rows_still_preserve_query_meaning() {
    let matrix = MilestoneNineOneCertificationAdapter::
        query_subscription_declaration_and_lowering_parity_test();

    let grouped = matrix
        .rows
        .iter()
        .find(|row| row.row_name == "grouped-query-meaning-shares-collection-bridge-family")
        .expect("grouped query meaning row should exist");
    assert_eq!(
        grouped.control_lane.bridge_family,
        grouped.hostile_lane.bridge_family
    );
    assert_ne!(
        grouped.control_lane.query_family,
        grouped.hostile_lane.query_family
    );
    assert_ne!(
        grouped.control_lane.declaration_digest,
        grouped.hostile_lane.declaration_digest
    );

    let inspector = matrix
        .rows
        .iter()
        .find(|row| row.row_name == "inspector-query-meaning-shares-detail-bridge-family")
        .expect("inspector query meaning row should exist");
    assert_eq!(
        inspector.control_lane.bridge_family,
        inspector.hostile_lane.bridge_family
    );
    assert_ne!(
        inspector.control_lane.query_family,
        inspector.hostile_lane.query_family
    );
    assert_ne!(
        inspector.control_lane.declaration_digest,
        inspector.hostile_lane.declaration_digest
    );
}

#[test]
fn milestone_nine_one_activation_certification_binds_scale_source_identity() {
    let matrix = MilestoneNineOneCertificationAdapter::
        query_subscription_declaration_and_lowering_parity_test();

    for row in &matrix.rows {
        assert_eq!(
            row.control_lane.scale_activation_digest, row.control_lane.activation_digest,
            "row '{}' control lane scale proof should bind activation",
            row.row_name
        );
        assert_eq!(
            row.control_lane.scale_admission_digest, row.control_lane.admission_digest,
            "row '{}' control lane scale proof should bind admission",
            row.row_name
        );
    }
}

#[test]
fn milestone_nine_one_basis_request_binds_policy_and_tenant_meaning() {
    let matrix = MilestoneNineOneCertificationAdapter::
        query_subscription_declaration_and_lowering_parity_test();
    let row = matrix
        .rows
        .iter()
        .find(|row| row.row_name == "basis-request-binds-policy-tenant-meaning")
        .expect("policy/tenant basis binding row should exist");

    assert_ne!(
        row.control_lane.declaration_digest,
        row.hostile_lane.declaration_digest
    );
    assert_ne!(
        row.control_lane.basis_request_digest,
        row.hostile_lane.basis_request_digest
    );
    assert_eq!(
        row.hostile_lane.basis_request_digest,
        row.parity_lane.basis_request_digest
    );
}

#[test]
fn milestone_nine_one_relationship_proof_binds_subscription_meaning() {
    let matrix = MilestoneNineOneCertificationAdapter::
        query_subscription_declaration_and_lowering_parity_test();
    let row = matrix
        .rows
        .iter()
        .find(|row| row.row_name == "relationship-proof-binds-subscription-meaning")
        .expect("relationship proof binding row should exist");

    assert_ne!(
        row.control_lane.declaration_digest,
        row.hostile_lane.declaration_digest
    );
    assert_ne!(
        row.control_lane.basis_request_digest,
        row.hostile_lane.basis_request_digest
    );
    assert_eq!(
        row.hostile_lane.basis_request_digest,
        row.parity_lane.basis_request_digest
    );
}

#[test]
fn milestone_nine_one_rejection_rows_are_typed_and_non_cosmetic() {
    let matrix = MilestoneNineOneCertificationAdapter::
        query_subscription_declaration_and_lowering_parity_test();

    let view = matrix
        .rejection_rows
        .iter()
        .find(|row| row.row_name == "view-family-mismatch-denies-before-declaration")
        .expect("view mismatch row should exist");
    assert_eq!(
        view.hostile_lane.failure_class,
        MilestoneNineOneFailureClass::FamilySelectionDenied
    );
    assert_eq!(
        view.hostile_lane.failure_kind,
        "view_family_live_family_mismatch"
    );
    assert_eq!(view.hostile_lane.diagnostic_stage, "view_mismatch");
    assert_ne!(
        view.hostile_lane.diagnostic_digest, view.control_lane.diagnostics_digest,
        "view mismatch row must carry hostile diagnostic evidence"
    );

    let bridge = matrix
        .rejection_rows
        .iter()
        .find(|row| row.row_name == "bridge-family-unsupported-denies-before-admission")
        .expect("bridge family row should exist");
    assert_eq!(
        bridge.hostile_lane.failure_class,
        MilestoneNineOneFailureClass::BridgeLoweringDenied
    );
    assert_eq!(
        bridge.hostile_lane.failure_kind,
        "bridge_family_unsupported"
    );
    assert_eq!(
        bridge.hostile_lane.diagnostic_stage,
        "bridge_family_lowering"
    );
    assert_ne!(
        bridge.hostile_lane.diagnostic_digest, bridge.control_lane.diagnostics_digest,
        "bridge family rejection must carry hostile diagnostic evidence"
    );

    for row_name in [
        "masked-detail-slice-denies-before-bridge-lowering",
        "masked-table-ordering-denies-before-bridge-lowering",
        "masked-grouped-membership-denies-before-bridge-lowering",
    ] {
        let masked = matrix
            .rejection_rows
            .iter()
            .find(|row| row.row_name == row_name)
            .expect("masked-slice row should exist");
        assert_eq!(
            masked.hostile_lane.failure_class,
            MilestoneNineOneFailureClass::DeclarationDenied
        );
        assert_eq!(masked.hostile_lane.failure_kind, "unsupported_masked_slice");
        assert_eq!(masked.hostile_lane.diagnostic_stage, "declaration");
        assert_ne!(
            masked.hostile_lane.diagnostic_digest, masked.control_lane.diagnostics_digest,
            "masked slice row '{}' must carry hostile diagnostic evidence",
            row_name
        );
    }

    let broken_proof = matrix
        .rejection_rows
        .iter()
        .find(|row| row.row_name == "broken-relationship-proof-denies-before-bridge-lowering")
        .expect("broken relationship proof row should exist");
    assert_eq!(
        broken_proof.hostile_lane.failure_class,
        MilestoneNineOneFailureClass::FamilySelectionDenied
    );
    assert_eq!(
        broken_proof.hostile_lane.failure_kind,
        "relationship_proof_admission_drift"
    );
    assert_eq!(
        broken_proof.hostile_lane.diagnostic_stage,
        "relationship_proof_drift"
    );
    assert_ne!(
        broken_proof.hostile_lane.diagnostic_digest, broken_proof.control_lane.diagnostics_digest,
        "relationship proof drift must carry hostile diagnostic evidence"
    );
    assert_ne!(
        broken_proof.hostile_lane.counter_snapshot_digest,
        bridge.hostile_lane.counter_snapshot_digest,
        "relationship proof drift must not reuse bridge-family counter evidence"
    );

    let durable = matrix
        .rejection_rows
        .iter()
        .find(|row| row.row_name == "durable-reload-overclaim-denies-before-activation")
        .expect("durable reload row should exist");
    assert_eq!(
        durable.hostile_lane.failure_class,
        MilestoneNineOneFailureClass::AdmissionDenied
    );
    assert_eq!(
        durable.hostile_lane.failure_kind,
        "durable_reload_overclaim"
    );
    assert_eq!(
        durable.hostile_lane.diagnostic_stage,
        "durable_reload_overclaim"
    );
    assert!(!durable.hostile_lane.support_profile_digest.is_empty());
    assert_ne!(
        durable.hostile_lane.diagnostic_digest, durable.control_lane.diagnostics_digest,
        "durable reload row must carry hostile diagnostic evidence"
    );

    let scale_source = matrix
        .rejection_rows
        .iter()
        .find(|row| row.row_name == "scale-report-source-mismatch-denies-certification")
        .expect("scale source mismatch row should exist");
    assert_eq!(
        scale_source.hostile_lane.failure_class,
        MilestoneNineOneFailureClass::CertificationDenied
    );
    assert_eq!(
        scale_source.hostile_lane.failure_kind,
        "scale_slope_source_mismatch"
    );
    assert_eq!(scale_source.hostile_lane.diagnostic_stage, "certification");

    let scale_zero_row = matrix
        .rejection_rows
        .iter()
        .find(|row| row.row_name == "scale-zero-row-baseline-denied")
        .expect("scale zero-row rejection should exist");
    assert_eq!(
        scale_zero_row.hostile_lane.failure_class,
        MilestoneNineOneFailureClass::CertificationDenied
    );
    assert_eq!(
        scale_zero_row.hostile_lane.failure_kind,
        "scale_slope_drift"
    );
    assert_eq!(
        scale_zero_row.hostile_lane.diagnostic_stage,
        "certification"
    );

    for row in &matrix.rejection_rows {
        assert!(!row.hostile_lane.diagnostic_digest.is_empty());
        assert_ne!(
            row.hostile_lane.failure_digest, row.hostile_lane.diagnostic_digest,
            "rejection row '{}' failure digest must bind more than the diagnostic alone",
            row.row_name
        );
        assert_ne!(
            row.hostile_lane.counter_snapshot_digest, row.control_lane.counter_snapshot_digest,
            "rejection row '{}' must not reuse control counter evidence",
            row.row_name
        );
    }
}
