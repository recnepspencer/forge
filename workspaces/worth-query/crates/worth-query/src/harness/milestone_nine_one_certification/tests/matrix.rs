use super::*;

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
