use super::super::{
    MilestoneNineCertificationAdapter, MilestoneNineFailureClass,
    MilestoneNinePhaseFourSupportStatus, MilestoneNinePhaseFourSupportSurface,
};
use super::is_sha256_digest;
use crate::harness::certification::digest_parts;

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
    let narrowed = super::super::phase_three_test_narrowed_artifact();
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
