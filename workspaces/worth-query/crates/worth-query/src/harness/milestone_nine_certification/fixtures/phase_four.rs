use crate::harness::certification::digest_parts;
use crate::harness::milestone_nine_certification::bundles::MilestoneNineCertificationBundle;
use crate::harness::milestone_nine_certification::fixtures::phase_three_test_narrowed_artifact;
use crate::harness::milestone_nine_certification::fixtures::policy_placeholder_request;

pub(in crate::harness::milestone_nine_certification) fn policy_execution_handoff_bundle(
) -> MilestoneNineCertificationBundle {
    let narrowed = phase_three_test_narrowed_artifact();
    let support_profile =
        crate::policy_execution_seam::runtime_backed_policy_execution_seam_support_profile();
    let handoff =
        crate::policy_execution_seam::runtime_backed_policy_execution_seam_handoff_report();
    MilestoneNineCertificationBundle {
        canonical_query_digest: narrowed.canonical_query_digest().to_string(),
        policy_digest: narrowed.policy_digest().to_string(),
        result_digest: digest_parts(&[
            format!("handoff:{}", handoff.handoff_digest()),
            format!("narrowed:{}", narrowed.digest()),
            format!("shape:{}", narrowed.narrowed_result_shape_digest()),
        ]),
        tenant_truth_basis_digest: narrowed.tenant_truth_basis_digest().to_string(),
        tenant_schema_basis_digest: narrowed.tenant_schema_basis_digest().to_string(),
        branch_access_digest: narrowed.branch_access_digest().to_string(),
        schema_variant_digest: "phase3-schema-variant-bound-in-phase1".to_string(),
        execution_mode: "policy-execution-handoff".to_string(),
        admission_disposition: "runtime-backed-verified-store-and-durable-deferred".to_string(),
        policy_cost_posture: narrowed.cost_posture().as_str().to_string(),
        policy_work_budget_digest: narrowed.work_budget().digest_part(),
        authorized_projection_digest: narrowed
            .authorized_projection()
            .identity()
            .as_str()
            .to_string(),
        narrowed_result_shape_digest: narrowed.narrowed_result_shape_digest().to_string(),
        relationship_proof_digest: narrowed
            .relationship_proof()
            .identity()
            .as_str()
            .to_string(),
        validation_report_digest: narrowed.validation_report().digest().to_string(),
        policy_plan_digest: digest_parts(&[
            format!(
                "m10_handoff:{}",
                handoff.milestone_ten_store_backed_handoff().join("|")
            ),
            format!(
                "m11_handoff:{}",
                handoff.milestone_eleven_durable_handoff().join("|")
            ),
        ]),
        policy_execution_seam_digest: handoff.handoff_digest().to_string(),
        delivery_digest: "durable-delivery-metadata-deferred-to-m11".to_string(),
        employee_fixture_digest: "phase3-handoff-employee-fixture-deferred".to_string(),
        policy_scale_counter_slope_digest: "phase3-handoff-policy-scale-deferred".to_string(),
        live_drift_evidence_digest: "phase3-handoff-live-drift-deferred".to_string(),
        delivery_width_class_digest: "phase3-handoff-delivery-width-deferred".to_string(),
        composition_policy_parity_digest: "phase3-handoff-composition-parity-deferred".to_string(),
        view_shape_policy_parity_digest: "phase3-handoff-view-shape-parity-deferred".to_string(),
        placeholder_denial_digest: "phase3-handoff-placeholder-denial-deferred".to_string(),
        counter_snapshot_digest: digest_parts(&[
            format!(
                "runtime_verified:{}",
                handoff.runtime_backed_verified_surface_count()
            ),
            format!(
                "blocked_or_deferred:{}",
                handoff.blocked_or_deferred_surface_count()
            ),
        ]),
        support_profile_digest: support_profile.profile_digest().to_string(),
    }
}

pub(in crate::harness::milestone_nine_certification) fn phase_four_bundle(
    row_label: &str,
    employee_fixture_digest: impl Into<String>,
    policy_scale_counter_slope_digest: impl Into<String>,
    live_drift_evidence_digest: impl Into<String>,
    delivery_width_class_digest: impl Into<String>,
    composition_policy_parity_digest: impl Into<String>,
    view_shape_policy_parity_digest: impl Into<String>,
    extra_counter_parts: &[String],
) -> MilestoneNineCertificationBundle {
    let employee_fixture_digest = employee_fixture_digest.into();
    let narrowed = phase_three_test_narrowed_artifact();
    phase_four_bundle_from_narrowed(
        row_label,
        narrowed,
        employee_fixture_digest,
        policy_scale_counter_slope_digest,
        live_drift_evidence_digest,
        delivery_width_class_digest,
        composition_policy_parity_digest,
        view_shape_policy_parity_digest,
        extra_counter_parts,
    )
}

pub(in crate::harness::milestone_nine_certification) fn phase_four_bundle_from_narrowed(
    row_label: &str,
    narrowed: crate::policy_narrowing::NarrowedPolicyQueryArtifact,
    employee_fixture_digest: impl Into<String>,
    policy_scale_counter_slope_digest: impl Into<String>,
    live_drift_evidence_digest: impl Into<String>,
    delivery_width_class_digest: impl Into<String>,
    composition_policy_parity_digest: impl Into<String>,
    view_shape_policy_parity_digest: impl Into<String>,
    extra_counter_parts: &[String],
) -> MilestoneNineCertificationBundle {
    let employee_fixture_digest = employee_fixture_digest.into();
    let support_profile =
        crate::policy_execution_seam::runtime_backed_policy_execution_seam_support_profile();
    let current_plan = crate::policy_plan::lower_policy_aware_current_plan(&narrowed);
    let scalar_delivery = crate::policy_delivery::lower_policy_aware_delivery_shape(
        &narrowed,
        crate::policy_delivery::DeliveryWidthClass::ScalarDetail,
    )
    .unwrap();
    let placeholder_denial = crate::policy_delivery::deny_policy_placeholder_masking(
        &narrowed,
        policy_placeholder_request([("secret", "salary")]),
    );
    let placeholder_denial_digest = match placeholder_denial {
        Ok(admitted_no_denial) => admitted_no_denial.failure_digest().to_string(),
        Err(error) => digest_parts(&[
            error.failure_class().as_str().to_string(),
            digest_parts(&error.counters().digest_parts()),
        ]),
    };
    MilestoneNineCertificationBundle {
        canonical_query_digest: narrowed.canonical_query_digest().to_string(),
        policy_digest: narrowed.policy_digest().to_string(),
        result_digest: digest_parts(&[
            format!("row:{row_label}"),
            format!("narrowed:{}", narrowed.digest()),
            format!("delivery:{}", scalar_delivery.digest().as_str()),
            format!("employee_fixture:{}", employee_fixture_digest),
        ]),
        tenant_truth_basis_digest: narrowed.tenant_truth_basis_digest().to_string(),
        tenant_schema_basis_digest: narrowed.tenant_schema_basis_digest().to_string(),
        branch_access_digest: narrowed.branch_access_digest().to_string(),
        schema_variant_digest: "phase4-schema-variant-bound-in-fixture".to_string(),
        execution_mode: row_label.to_string(),
        admission_disposition: "phase4-runtime-backed-certified".to_string(),
        policy_cost_posture: narrowed.cost_posture().as_str().to_string(),
        policy_work_budget_digest: narrowed.work_budget().digest_part(),
        authorized_projection_digest: narrowed
            .authorized_projection()
            .identity()
            .as_str()
            .to_string(),
        narrowed_result_shape_digest: narrowed.narrowed_result_shape_digest().to_string(),
        relationship_proof_digest: narrowed
            .relationship_proof()
            .identity()
            .as_str()
            .to_string(),
        validation_report_digest: narrowed.validation_report().digest().to_string(),
        policy_plan_digest: current_plan.core().digest().as_str().to_string(),
        policy_execution_seam_digest: current_plan.core().seam().identity().as_str().to_string(),
        delivery_digest: scalar_delivery.digest().as_str().to_string(),
        employee_fixture_digest,
        policy_scale_counter_slope_digest: policy_scale_counter_slope_digest.into(),
        live_drift_evidence_digest: live_drift_evidence_digest.into(),
        delivery_width_class_digest: delivery_width_class_digest.into(),
        composition_policy_parity_digest: composition_policy_parity_digest.into(),
        view_shape_policy_parity_digest: view_shape_policy_parity_digest.into(),
        placeholder_denial_digest,
        counter_snapshot_digest: {
            let mut counter_parts = vec![
                "phase4_employee_fixture:1".to_string(),
                "phase4_scale_slope:1".to_string(),
                "phase4_live_drift_evidence:1".to_string(),
                "phase4_delivery_width:1".to_string(),
                "phase4_composition_parity:1".to_string(),
            ];
            counter_parts.extend(extra_counter_parts.iter().cloned());
            digest_parts(&counter_parts)
        },
        support_profile_digest: support_profile.profile_digest().to_string(),
    }
}
