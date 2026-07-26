use super::{
    basis_lifecycle_support_matrix, discover_basis_lifecycle_support, BasisSupportPosture,
};
use crate::domain_computation::{
    evaluate_basis_certification_eligibility, evaluate_basis_effect_authoring_deferred_eligibility,
    evaluate_basis_inspection_advisory_eligibility, evaluate_basis_inspection_eligibility,
    evaluate_basis_materialization_eligibility, evaluate_basis_mutation_preparation_eligibility,
    evaluate_basis_observation_eligibility, evaluate_basis_preview_closeout_eligibility,
    evaluate_basis_replay_eligibility, evaluate_basis_subscription_activation_eligibility,
    evaluate_basis_subscription_declaration_eligibility, normalize_raw_basis_intent, BasisFamily,
    DeniedBasisCapabilityKind, RawBasisIntent,
};

#[test]
fn every_advertised_support_row_matches_executable_admission_behavior() {
    let matrix = basis_lifecycle_support_matrix();

    for row in matrix.rows() {
        let executable = executable_posture_for(row.family(), row.operation_lane());

        assert_eq!(
            executable,
            row.posture(),
            "{} / {} support row drifted from executable admission",
            row.family().as_str(),
            row.operation_lane()
        );
    }
}

#[test]
fn unsupported_neighbors_have_no_fake_matrix_row_and_fail_closed() {
    let unsupported = discover_basis_lifecycle_support(BasisFamily::BranchHead, "materialization");
    let normalized = normalize_raw_basis_intent(
        raw_admitted_basis_for(BasisFamily::BranchHead),
        "materialization",
    )
    .expect("branch materialization normalization should succeed before lane denial");
    let denial = evaluate_basis_materialization_eligibility(normalized)
        .expect_err("unsupported branch materialization must deny");

    assert_eq!(unsupported.posture(), BasisSupportPosture::Unsupported);
    assert!(unsupported.matched_row_digest().is_none());
    assert_eq!(
        denial.denial_kind(),
        DeniedBasisCapabilityKind::OperationIneligible
    );
    assert_eq!(denial.counters().denied_residue_count(), 0);
}

#[test]
fn support_lookup_width_is_exact_for_control_hostile_and_absent_lanes() {
    let matrix = basis_lifecycle_support_matrix();
    let control = discover_basis_lifecycle_support(BasisFamily::CurrentHead, "observation");
    let hostile = discover_basis_lifecycle_support(BasisFamily::DurableReload, "certification");
    let absent = discover_basis_lifecycle_support(BasisFamily::BranchHead, "materialization");

    assert_eq!(control.counters().basis_support_lookup_width(), 1);
    assert_eq!(
        hostile.counters().basis_support_lookup_width(),
        matrix.rows().len()
    );
    assert_eq!(
        absent.counters().basis_support_lookup_width(),
        matrix.rows().len()
    );
}

fn executable_posture_for(family: BasisFamily, lane: &'static str) -> BasisSupportPosture {
    let normalized = normalize_raw_basis_intent(raw_admitted_basis_for(family), lane)
        .expect("advertised support rows must normalize");

    match lane {
        "observation" => posture_from_result(evaluate_basis_observation_eligibility(normalized)),
        "mutation_preparation" => {
            posture_from_result(evaluate_basis_mutation_preparation_eligibility(normalized))
        }
        "replay" => posture_from_result(evaluate_basis_replay_eligibility(normalized)),
        "inspection" if family == BasisFamily::PreviewDerived => {
            evaluate_basis_inspection_advisory_eligibility(normalized)
                .map(|_| BasisSupportPosture::Advisory)
                .unwrap_or(BasisSupportPosture::Denied)
        }
        "inspection" => posture_from_result(evaluate_basis_inspection_eligibility(normalized)),
        "materialization" => {
            posture_from_result(evaluate_basis_materialization_eligibility(normalized))
        }
        "subscription_declaration" => posture_from_result(
            evaluate_basis_subscription_declaration_eligibility(normalized),
        ),
        "subscription_activation" => posture_from_result(
            evaluate_basis_subscription_activation_eligibility(normalized),
        ),
        "preview_closeout" => {
            posture_from_result(evaluate_basis_preview_closeout_eligibility(normalized))
        }
        "certification" => {
            posture_from_result(evaluate_basis_certification_eligibility(normalized))
        }
        "effect_authoring" => evaluate_basis_effect_authoring_deferred_eligibility(normalized)
            .map(|_| BasisSupportPosture::Deferred)
            .unwrap_or(BasisSupportPosture::Denied),
        other => panic!("unsupported test lane {other}"),
    }
}

#[test]
fn deferred_effect_authoring_lane_produces_real_deferred_basis_proof() {
    let store_backed = normalize_raw_basis_intent(
        RawBasisIntent::StoreBacked {
            store_basis_identity: "store-effect-authoring".to_string(),
        },
        "effect_authoring",
    )
    .expect("store-backed effect authoring basis should normalize");
    let deferred = evaluate_basis_effect_authoring_deferred_eligibility(store_backed)
        .expect("store-backed effect authoring should return deferred proof");

    assert_eq!(deferred.normalized().family(), BasisFamily::StoreBacked);
    assert_eq!(
        deferred.denial_kind(),
        DeniedBasisCapabilityKind::StoreBackedDeferred
    );
    assert_eq!(deferred.counters().denied_residue_count(), 0);
}

fn posture_from_result<T>(
    result: Result<T, crate::domain_computation::DeniedBasisCapability>,
) -> BasisSupportPosture {
    match result {
        Ok(_) => BasisSupportPosture::Admitted,
        Err(denial)
            if matches!(
                denial.denial_kind(),
                DeniedBasisCapabilityKind::DurableOverclaim
                    | DeniedBasisCapabilityKind::StoreBackedDeferred
            ) =>
        {
            assert_eq!(denial.counters().denied_residue_count(), 0);
            BasisSupportPosture::Deferred
        }
        Err(_) => BasisSupportPosture::Denied,
    }
}

fn raw_admitted_basis_for(family: BasisFamily) -> RawBasisIntent {
    match family {
        BasisFamily::CurrentHead => RawBasisIntent::CurrentHead,
        BasisFamily::BranchHead => RawBasisIntent::BranchHead {
            branch_identity: "branch-support".to_string(),
            accessible: true,
        },
        BasisFamily::BranchSnapshot => RawBasisIntent::BranchSnapshot {
            branch_identity: "branch-support".to_string(),
            snapshot_identity: "snapshot-support".to_string(),
        },
        BasisFamily::Preview => RawBasisIntent::Preview {
            preview_identity: "preview-support".to_string(),
            stale: false,
        },
        BasisFamily::PreviewDerived => RawBasisIntent::PreviewDerived {
            preview_identity: "preview-support".to_string(),
            source_basis_identity: "branch-support".to_string(),
        },
        BasisFamily::RuntimeSnapshot => RawBasisIntent::RuntimeSnapshot {
            snapshot_identity: "snapshot-support".to_string(),
            lower_runtime_binding_digest: Some("bridge-runtime-snapshot:support".to_string()),
        },
        BasisFamily::HistoricalSnapshot => RawBasisIntent::HistoricalSnapshot {
            snapshot_identity: "history-support".to_string(),
            replay_supported: true,
        },
        BasisFamily::HistoricalCommit => RawBasisIntent::HistoricalCommit {
            commit_identity: "commit-support".to_string(),
            replay_supported: true,
        },
        BasisFamily::TenantScoped => RawBasisIntent::TenantScoped {
            tenant_identity: "tenant-support".to_string(),
            branch_identity: "branch-support".to_string(),
            schema_identity: "schema-support".to_string(),
            tenant_schema_matches: true,
        },
        BasisFamily::PolicyScoped => RawBasisIntent::PolicyScoped {
            policy_digest: "policy-support".to_string(),
            tenant_identity: "tenant-support".to_string(),
            branch_identity: "branch-support".to_string(),
            schema_identity: "schema-support".to_string(),
            tenant_schema_matches: true,
            policy_masks_operation: false,
            advisory_visibility: false,
        },
        BasisFamily::StoreBacked => RawBasisIntent::StoreBacked {
            store_basis_identity: "store-support".to_string(),
        },
        BasisFamily::DurableReload => RawBasisIntent::DurableReload {
            reload_identity: "reload-support".to_string(),
        },
    }
}
