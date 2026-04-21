use super::{
    MilestoneNineCertificationAdapter, MilestoneNineCertificationBundle, MilestoneNineFailureClass,
    MilestoneNinePhaseFourSupportStatus, MilestoneNinePhaseFourSupportSurface,
};
use crate::harness::certification::{
    contains_row, digest_parts, milestone_nine_requirements, unmet_required_assertion_classes,
    unmet_required_rows, HostileExpectation, ParityAnchor, RequiredAssertionClass,
};

fn semantic_signature(bundle: &MilestoneNineCertificationBundle) -> String {
    digest_parts(&[
        format!("query:{}", bundle.canonical_query_digest),
        format!("policy:{}", bundle.policy_digest),
        format!("result:{}", bundle.result_digest),
        format!("tenant_truth:{}", bundle.tenant_truth_basis_digest),
        format!("tenant_schema:{}", bundle.tenant_schema_basis_digest),
        format!("branch:{}", bundle.branch_access_digest),
        format!("schema:{}", bundle.schema_variant_digest),
        format!("authorized:{}", bundle.authorized_projection_digest),
        format!("shape:{}", bundle.narrowed_result_shape_digest),
        format!("proof:{}", bundle.relationship_proof_digest),
        format!("plan:{}", bundle.policy_plan_digest),
        format!("seam:{}", bundle.policy_execution_seam_digest),
        format!("delivery:{}", bundle.delivery_digest),
    ])
}

fn is_sha256_digest(value: &str) -> bool {
    value.len() == 64 && value.chars().all(|ch| ch.is_ascii_hexdigit())
}

#[test]
fn milestone_nine_certification_adapter_emits_named_matrix() {
    let artifact =
        MilestoneNineCertificationAdapter::policy_tenant_context_admission_certification_artifact();

    assert_eq!(
        artifact.suite_name,
        "Policy And Tenant Context Admission Test"
    );
    assert!(!artifact.certification_bundle_digest.is_empty());
    assert!(!artifact.coverage_matrix_digest.is_empty());
}

#[test]
fn milestone_nine_certification_matrix_meets_required_rows() {
    let matrix = MilestoneNineCertificationAdapter::policy_tenant_context_admission_test();
    let requirements = milestone_nine_requirements();
    let missing = unmet_required_rows(
        &matrix,
        requirements.required_canonical_rows,
        requirements.required_rejection_rows,
    );

    assert!(
        missing.is_empty(),
        "missing milestone nine certification rows: {missing:?}"
    );
}

#[test]
fn milestone_nine_certification_rows_have_required_outputs() {
    let matrix = MilestoneNineCertificationAdapter::policy_tenant_context_admission_test();

    for row in &matrix.rows {
        assert!(
            row.control_lane.has_required_outputs(),
            "control lane '{}' should have required outputs",
            row.row_name
        );
        assert!(
            row.hostile_lane.has_required_outputs(),
            "hostile lane '{}' should have required outputs",
            row.row_name
        );
        assert!(
            row.parity_lane.has_required_outputs(),
            "parity lane '{}' should have required outputs",
            row.row_name
        );
    }
}

#[test]
fn milestone_nine_certification_rows_enforce_declared_lane_semantics() {
    let matrix = MilestoneNineCertificationAdapter::policy_tenant_context_admission_test();
    let mut covered = Vec::new();

    for row in &matrix.rows {
        let control = semantic_signature(&row.control_lane);
        let hostile = semantic_signature(&row.hostile_lane);
        let parity = semantic_signature(&row.parity_lane);
        match row.hostile_expectation {
            HostileExpectation::EquivalentToControl => {
                assert_eq!(
                    control, hostile,
                    "row '{}' declares hostile equivalence but emits different semantic evidence",
                    row.row_name
                );
                covered.push(RequiredAssertionClass::Equality);
            }
            HostileExpectation::DistinctFromControl => {
                assert_ne!(
                    control, hostile,
                    "row '{}' declares hostile distinction but emits identical semantic evidence",
                    row.row_name
                );
                covered.push(RequiredAssertionClass::Inequality);
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
        assert!(
            row.control_lane.has_required_outputs(),
            "rejection row '{}' needs a successful control basis",
            row.row_name
        );
        assert!(
            row.parity_lane.has_required_outputs(),
            "rejection row '{}' needs a successful parity basis",
            row.row_name
        );
        assert!(
            !row.hostile_lane.failure_digest.is_empty()
                && !row.hostile_lane.counter_snapshot_digest.is_empty(),
            "rejection row '{}' must emit failure and counter evidence",
            row.row_name
        );
        assert_ne!(
            row.hostile_lane.counter_snapshot_digest, row.control_lane.counter_snapshot_digest,
            "rejection row '{}' must not reuse the control counter snapshot",
            row.row_name
        );
        covered.push(RequiredAssertionClass::TypedFailure);
    }

    let zero_residue = matrix.rejection_rows.iter().any(|row| {
        matches!(
            row.row_name,
            "relationship-proof-host-callback-forbidden"
                | "phase-two-no-truth-touch"
                | "phase-three-no-truth-touch-before-plan-admission"
        ) && row.hostile_lane.counter_snapshot_digest != row.control_lane.counter_snapshot_digest
    });
    if zero_residue {
        covered.push(RequiredAssertionClass::ZeroResidue);
    }
    covered.sort();
    covered.dedup();

    let missing = unmet_required_assertion_classes(
        &covered,
        milestone_nine_requirements().required_assertion_classes,
    );
    assert!(
        missing.is_empty(),
        "missing milestone nine assertion classes: {missing:?}"
    );
}

#[test]
fn milestone_nine_execution_seam_denials_are_typed() {
    let matrix = MilestoneNineCertificationAdapter::policy_tenant_context_admission_test();
    let live = matrix
        .rejection_rows
        .iter()
        .find(|row| row.row_name == "live-subscription-deferred-before-truth")
        .expect("live seam denial row should exist");
    let diff = matrix
        .rejection_rows
        .iter()
        .find(|row| row.row_name == "historical-diff-deferred-before-truth")
        .expect("diff seam denial row should exist");

    assert_eq!(
        live.hostile_lane.failure_class,
        MilestoneNineFailureClass::UnsupportedExecutionMode
    );
    assert_eq!(
        diff.hostile_lane.failure_class,
        MilestoneNineFailureClass::UnsupportedExecutionMode
    );
    assert_ne!(
        live.hostile_lane.counter_snapshot_digest,
        live.control_lane.counter_snapshot_digest
    );
}

#[test]
fn milestone_nine_hidden_and_branch_rows_are_present() {
    let matrix = MilestoneNineCertificationAdapter::policy_tenant_context_admission_test();

    assert!(contains_row(&matrix, "branch-denial-before-tenant-truth"));
    assert!(contains_row(&matrix, "hidden-tenant-filter-denied"));
    assert!(contains_row(&matrix, "global-schema-fallback-denied"));
    assert!(contains_row(
        &matrix,
        "unknown-policy-cost-denied-before-truth"
    ));
    assert!(contains_row(&matrix, "saved-query-policy-tenant-drift"));
}

#[test]
fn milestone_nine_policy_narrowing_changes_only_the_admission_disposition() {
    let matrix = MilestoneNineCertificationAdapter::policy_tenant_context_admission_test();
    let row = matrix
        .rows
        .iter()
        .find(|row| row.row_name == "policy-narrowing-disposition")
        .expect("policy narrowing row should exist");

    assert_eq!(
        row.control_lane.canonical_query_digest,
        row.hostile_lane.canonical_query_digest
    );
    assert_eq!(
        row.control_lane.tenant_truth_basis_digest,
        row.hostile_lane.tenant_truth_basis_digest
    );
    assert_ne!(
        row.control_lane.admission_disposition,
        row.hostile_lane.admission_disposition
    );
}

#[test]
fn milestone_nine_work_budget_is_part_of_admission_evidence() {
    let matrix = MilestoneNineCertificationAdapter::policy_tenant_context_admission_test();
    let row = matrix
        .rows
        .iter()
        .find(|row| row.row_name == "policy-work-budget-explicitness")
        .expect("policy work budget row should exist");
    let unknown = matrix
        .rejection_rows
        .iter()
        .find(|row| row.row_name == "unknown-policy-cost-denied-before-truth")
        .expect("unknown cost denial row should exist");

    assert_eq!(row.control_lane.policy_cost_posture, "constant_proof");
    assert!(!row.control_lane.policy_work_budget_digest.is_empty());
    assert_ne!(
        unknown.hostile_lane.counter_snapshot_digest,
        row.control_lane.counter_snapshot_digest
    );
}

#[test]
fn milestone_nine_runtime_store_and_durable_handoff_is_explicit() {
    let matrix = MilestoneNineCertificationAdapter::policy_tenant_context_admission_test();
    let handoff = matrix
        .rows
        .iter()
        .find(|row| row.row_name == "policy-execution-handoff-honesty")
        .expect("handoff honesty row should exist");
    let durable_cursor = matrix
        .rejection_rows
        .iter()
        .find(|row| row.row_name == "durable-policy-cursor-deferred")
        .expect("durable cursor deferred row should exist");
    let durable_reload = matrix
        .rejection_rows
        .iter()
        .find(|row| row.row_name == "durable-policy-artifact-reload-deferred")
        .expect("durable artifact reload deferred row should exist");
    let durable_delivery = matrix
        .rejection_rows
        .iter()
        .find(|row| row.row_name == "durable-policy-delivery-metadata-deferred")
        .expect("durable delivery metadata deferred row should exist");

    assert_ne!(
        handoff.control_lane.policy_execution_seam_digest,
        handoff.control_lane.support_profile_digest
    );
    assert_eq!(
        durable_cursor.hostile_lane.failure_class,
        MilestoneNineFailureClass::PolicyExecutionSeamDenied
    );
    assert_ne!(
        durable_cursor.hostile_lane.counter_snapshot_digest,
        durable_reload.hostile_lane.counter_snapshot_digest
    );
    assert_ne!(
        durable_reload.hostile_lane.counter_snapshot_digest,
        durable_delivery.hostile_lane.counter_snapshot_digest
    );
}

#[test]
fn milestone_nine_phase_four_closeout_rows_are_concrete() {
    let matrix = MilestoneNineCertificationAdapter::policy_tenant_context_admission_test();
    let employee = matrix
        .rows
        .iter()
        .find(|row| row.row_name == "employee-record-fixture-policy-basis")
        .expect("employee fixture row should exist");
    let scale = matrix
        .rows
        .iter()
        .find(|row| row.row_name == "policy-scale-slope-honesty")
        .expect("scale slope row should exist");
    let delivery = matrix
        .rows
        .iter()
        .find(|row| row.row_name == "delivery-width-class-honesty")
        .expect("delivery width row should exist");
    let live_drift = matrix
        .rows
        .iter()
        .find(|row| row.row_name == "live-policy-epoch-drift-readmission")
        .expect("live drift row should exist");
    let live_density = matrix
        .rows
        .iter()
        .find(|row| row.row_name == "live-policy-density-posture-honesty")
        .expect("live density row should exist");
    let aggregation = matrix
        .rejection_rows
        .iter()
        .find(|row| row.row_name == "masked-aggregation-without-witness-forbidden")
        .expect("masked aggregation rejection row should exist");
    let placeholder = matrix
        .rejection_rows
        .iter()
        .find(|row| row.row_name == "masked-placeholder-shape-forbidden")
        .expect("masked placeholder rejection row should exist");
    let cursor = matrix
        .rejection_rows
        .iter()
        .find(|row| row.row_name == "masked-cursor-without-witness-forbidden")
        .expect("masked cursor rejection row should exist");
    let view_membership = matrix
        .rejection_rows
        .iter()
        .find(|row| row.row_name == "masked-view-membership-without-witness-forbidden")
        .expect("masked view membership rejection row should exist");
    let allocation = matrix
        .rejection_rows
        .iter()
        .find(|row| row.row_name == "policy-per-row-allocation-forbidden")
        .expect("per-row allocation rejection row should exist");
    let fanout = matrix
        .rejection_rows
        .iter()
        .find(|row| row.row_name == "policy-cross-tenant-fanout-forbidden")
        .expect("cross-tenant fanout rejection row should exist");
    let saved_bypass = matrix
        .rejection_rows
        .iter()
        .find(|row| row.row_name == "saved-query-policy-bypass-forbidden")
        .expect("saved bypass rejection row should exist");
    let workflow = matrix
        .rejection_rows
        .iter()
        .find(|row| row.row_name == "unsupported-policy-workflow-composition-forbidden")
        .expect("unsupported workflow composition rejection row should exist");

    assert!(!employee.control_lane.employee_fixture_digest.is_empty());
    assert!(!scale
        .control_lane
        .policy_scale_counter_slope_digest
        .is_empty());
    assert!(!delivery.control_lane.delivery_width_class_digest.is_empty());
    assert!(!live_drift
        .hostile_lane
        .live_drift_evidence_digest
        .is_empty());
    assert!(!live_density
        .control_lane
        .live_drift_evidence_digest
        .is_empty());
    assert_ne!(
        live_drift.control_lane.live_drift_evidence_digest,
        live_drift.hostile_lane.live_drift_evidence_digest
    );
    assert_ne!(
        live_drift.control_lane.counter_snapshot_digest,
        live_drift.hostile_lane.counter_snapshot_digest
    );
    assert_ne!(
        live_density.control_lane.counter_snapshot_digest,
        delivery.control_lane.counter_snapshot_digest
    );
    assert_eq!(
        aggregation.hostile_lane.failure_class,
        MilestoneNineFailureClass::PolicyNarrowingDenied
    );
    assert_eq!(
        placeholder.hostile_lane.failure_class,
        MilestoneNineFailureClass::PolicyExecutionSeamDenied
    );
    assert_ne!(
        aggregation.hostile_lane.counter_snapshot_digest,
        cursor.hostile_lane.counter_snapshot_digest
    );
    assert_ne!(
        cursor.hostile_lane.counter_snapshot_digest,
        view_membership.hostile_lane.counter_snapshot_digest
    );
    assert_eq!(
        allocation.hostile_lane.failure_class,
        MilestoneNineFailureClass::PolicyExecutionSeamDenied
    );
    assert_ne!(
        allocation.hostile_lane.counter_snapshot_digest,
        fanout.hostile_lane.counter_snapshot_digest
    );
    assert_ne!(
        saved_bypass.hostile_lane.counter_snapshot_digest,
        workflow.hostile_lane.counter_snapshot_digest
    );
}

#[test]
fn milestone_nine_phase_four_rows_use_machine_digests_not_certified_labels() {
    let matrix = MilestoneNineCertificationAdapter::policy_tenant_context_admission_test();
    let phase_four_rows = [
        "employee-record-fixture-policy-basis",
        "tenant-alpha-versus-tenant-beta-schema",
        "masked-versus-unmasked-policy-parity",
        "delivery-width-class-honesty",
        "live-policy-epoch-drift-readmission",
        "live-policy-density-posture-honesty",
        "policy-scale-slope-honesty",
        "policy-direct-scope-template-saved-parity",
        "policy-view-shape-delivery-parity",
        "policy-identity-aware-inspector-parity",
    ];

    for row_name in phase_four_rows {
        let row = matrix
            .rows
            .iter()
            .find(|row| row.row_name == row_name)
            .expect("phase four row should exist");
        for (lane_name, lane) in [
            ("control", &row.control_lane),
            ("hostile", &row.hostile_lane),
            ("parity", &row.parity_lane),
        ] {
            assert!(
                is_sha256_digest(&lane.result_digest),
                "{row_name}/{lane_name} result evidence must be a machine digest"
            );
            assert!(
                is_sha256_digest(&lane.policy_plan_digest),
                "{row_name}/{lane_name} plan evidence must be a machine digest"
            );
            assert!(
                is_sha256_digest(&lane.policy_execution_seam_digest),
                "{row_name}/{lane_name} seam evidence must be a machine digest"
            );
            assert!(
                is_sha256_digest(&lane.delivery_digest),
                "{row_name}/{lane_name} delivery evidence must be a machine digest"
            );
            assert!(
                is_sha256_digest(&lane.placeholder_denial_digest),
                "{row_name}/{lane_name} placeholder denial evidence must be a machine digest"
            );
            assert!(
                is_sha256_digest(&lane.employee_fixture_digest),
                "{row_name}/{lane_name} employee fixture evidence must be a machine digest"
            );
            assert!(
                is_sha256_digest(&lane.policy_scale_counter_slope_digest),
                "{row_name}/{lane_name} scale slope evidence must be a machine digest"
            );
            assert!(
                is_sha256_digest(&lane.live_drift_evidence_digest),
                "{row_name}/{lane_name} live drift evidence must be a machine digest"
            );
            assert!(
                is_sha256_digest(&lane.delivery_width_class_digest),
                "{row_name}/{lane_name} delivery width evidence must be a machine digest"
            );
            assert!(
                is_sha256_digest(&lane.composition_policy_parity_digest),
                "{row_name}/{lane_name} composition parity evidence must be a machine digest"
            );
            assert!(
                is_sha256_digest(&lane.view_shape_policy_parity_digest),
                "{row_name}/{lane_name} view shape parity evidence must be a machine digest"
            );
        }
    }
}

#[test]
fn milestone_nine_delivery_width_evidence_is_recomputed_from_all_admitted_width_classes() {
    let matrix = MilestoneNineCertificationAdapter::policy_tenant_context_admission_test();
    let row = matrix
        .rows
        .iter()
        .find(|row| row.row_name == "delivery-width-class-honesty")
        .expect("delivery width row should exist");
    let narrowed = super::phase_three_test_narrowed_artifact();
    let scalar = crate::policy_delivery::lower_policy_aware_delivery_shape(
        &narrowed,
        crate::policy_delivery::DeliveryWidthClass::ScalarDetail,
    )
    .unwrap();
    let narrow = crate::policy_delivery::lower_policy_aware_delivery_shape(
        &narrowed,
        crate::policy_delivery::DeliveryWidthClass::NarrowCollection,
    )
    .unwrap();
    let grouped = crate::policy_delivery::lower_policy_aware_delivery_shape(
        &narrowed,
        crate::policy_delivery::DeliveryWidthClass::GroupedDelta,
    )
    .unwrap();
    let diff = crate::policy_delivery::lower_policy_aware_delivery_shape(
        &narrowed,
        crate::policy_delivery::DeliveryWidthClass::DiffDelta,
    )
    .unwrap();
    let expected = digest_parts(&[
        format!("scalar:{}", scalar.report().digest()),
        format!("narrow:{}", narrow.report().digest()),
        format!("grouped:{}", grouped.report().digest()),
        format!("diff:{}", diff.report().digest()),
    ]);

    assert_eq!(row.control_lane.delivery_width_class_digest, expected);
    assert_ne!(
        row.control_lane.delivery_width_class_digest,
        scalar.report().digest(),
        "delivery-width evidence must not collapse to a single scalar lane"
    );
}

#[test]
fn milestone_nine_phase_four_support_report_is_row_derived_and_honest_about_debt() {
    let matrix = MilestoneNineCertificationAdapter::policy_tenant_context_admission_test();
    let report = matrix.phase_four_support_report();

    assert_eq!(report.verified_surface_count(), 7);
    assert_eq!(report.deferred_surface_count(), 2);
    assert!(!report.report_digest().is_empty());
    assert!(report.diagnostics().iter().any(|diagnostic| {
        diagnostic.surface() == MilestoneNinePhaseFourSupportSurface::StoreBackedPolicyExecution
            && diagnostic.status() == MilestoneNinePhaseFourSupportStatus::Deferred
    }));
    assert!(report.diagnostics().iter().any(|diagnostic| {
        diagnostic.surface() == MilestoneNinePhaseFourSupportSurface::PolicyCompositionParity
            && diagnostic.row_name() == "policy-direct-scope-template-saved-parity"
    }));
}

#[test]
fn milestone_nine_phase_four_support_report_requires_executable_evidence_not_only_row_names() {
    let mut matrix = MilestoneNineCertificationAdapter::policy_tenant_context_admission_test();
    let row = matrix
        .rows
        .iter_mut()
        .find(|row| row.row_name == "live-policy-epoch-drift-readmission")
        .expect("live drift row should exist");
    row.control_lane.live_drift_evidence_digest = "phase4-live-drift-deferred".to_string();

    let report = matrix.phase_four_support_report();

    assert!(!report.diagnostics().iter().any(|diagnostic| {
        diagnostic.surface() == MilestoneNinePhaseFourSupportSurface::LiveDriftReadmission
    }));
}
