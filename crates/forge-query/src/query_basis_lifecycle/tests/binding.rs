use super::binding_support::{
    continuity_runtime, delivered_continuity, detail_subscription, observation_runtime,
    subscription_runtime, MAIN_BRANCH, OTHER_BRANCH,
};
use super::{
    admit_inspection_basis, admit_observation_basis, admit_subscription_activation_basis,
    admit_subscription_declaration_basis, evaluate_basis_eligibility, normalize_raw_basis,
    readmit_bridge_continuity_evidence, readmit_bridge_subscription_activation_evidence,
    readmit_bridge_subscription_declaration_evidence, readmit_bridge_truth_view_evidence,
    BasisCapabilityAdmission, BasisOperationLaneRequest, BridgeLowerRuntimeEvidenceKind,
    DeniedBasisCapabilityKind, RawBasisIntent,
};
use forge_runtime_bridge::facade::BridgeTruthViewEvaluationRequest;

#[test]
fn truth_view_readmission_binds_branch_head_observation_evidence() {
    let runtime = observation_runtime();
    let evaluation = runtime
        .evaluate(BridgeTruthViewEvaluationRequest::for_branch_head(
            super::test_branch_truth_identity(MAIN_BRANCH),
        ))
        .expect("branch-head truth view should evaluate");
    let capability = branch_head_observation(MAIN_BRANCH);

    let bound = readmit_bridge_truth_view_evidence(capability, &evaluation)
        .expect("matching branch-head truth view should readmit");

    assert_eq!(bound.authority_name(), "forge_runtime_bridge");
    assert_eq!(
        bound.evidence().kind(),
        BridgeLowerRuntimeEvidenceKind::TruthViewEvaluation
    );
    let record_identity = evaluation
        .record()
        .record_identity()
        .bridge_admission_evidence();
    assert_eq!(
        bound.evidence().record_identity(),
        Some(record_identity.terminal_projection_for_reporting())
    );
    let selector_identity = evaluation
        .record()
        .declaration()
        .selector()
        .selector_identity()
        .bridge_admission_evidence();
    assert_eq!(
        bound.evidence().selector_identity(),
        Some(selector_identity.terminal_projection_for_reporting())
    );
    assert_eq!(
        bound.evidence().authority_digest(),
        Some(evaluation.record().decision_log().authority_digest())
    );
    assert_eq!(
        bound.evidence().snapshot_identity(),
        Some(
            evaluation
                .snapshot_identity()
                .bridge_admission_evidence()
                .terminal_projection_for_reporting()
        )
    );
    assert_eq!(bound.counters().lower_runtime_check_count(), 1);
    assert_eq!(bound.counters().denied_residue_count(), 0);
    assert!(!bound.binding_for_reporting().is_empty());
}

#[test]
fn truth_view_readmission_denies_mismatched_branch_head_evidence() {
    let runtime = observation_runtime();
    let evaluation = runtime
        .evaluate(BridgeTruthViewEvaluationRequest::for_branch_head(
            super::test_branch_truth_identity(MAIN_BRANCH),
        ))
        .expect("branch-head truth view should evaluate");
    let capability = branch_head_observation(OTHER_BRANCH);

    let denial = readmit_bridge_truth_view_evidence(capability, &evaluation)
        .expect_err("mismatched branch-head evidence should deny");

    match denial.kind() {
        DeniedBasisCapabilityKind::LowerRuntimeBindingMismatch {
            authority,
            expected,
            observed,
        } => {
            assert_eq!(authority, &"forge_runtime_bridge");
            assert_eq!(
                expected,
                &format!(
                    "branch_head:{}",
                    super::test_branch_identity(OTHER_BRANCH).terminal_projection_for_reporting()
                )
            );
            assert_eq!(
                observed,
                &format!(
                    "branch_head:{}",
                    super::test_branch_identity(MAIN_BRANCH).terminal_projection_for_reporting()
                )
            );
        }
        other => panic!("unexpected denial kind: {other:?}"),
    }
    assert_eq!(denial.counters().lower_runtime_check_count(), 1);
    assert_eq!(denial.counters().denied_residue_count(), 1);
}

#[test]
fn continuity_readmission_binds_branch_head_inspection_evidence() {
    let runtime = continuity_runtime();
    let continuity = delivered_continuity(&runtime);
    let capability = branch_head_inspection(MAIN_BRANCH);

    let bound = readmit_bridge_continuity_evidence(capability, &continuity)
        .expect("matching continuity evidence should readmit");

    assert_eq!(
        bound.evidence().kind(),
        BridgeLowerRuntimeEvidenceKind::ContinuityDelivery
    );
    let route_identity = continuity
        .canonical_record()
        .route_identity()
        .bridge_admission_evidence();
    assert_eq!(
        bound.evidence().route_identity(),
        Some(route_identity.terminal_projection_for_reporting())
    );
    let continuity_identity = continuity.continuity_identity().bridge_admission_evidence();
    assert_eq!(
        bound.evidence().continuity_identity(),
        Some(continuity_identity.terminal_projection_for_reporting())
    );
    assert_eq!(
        bound.evidence().continuity_resolution_digest(),
        Some(continuity.canonical_record().continuity_resolution_digest())
    );
    assert_eq!(
        bound.evidence().source_snapshot_identity(),
        Some(
            continuity
                .canonical_record()
                .source_snapshot()
                .bridge_admission_evidence()
                .terminal_projection_for_reporting()
        )
    );
    assert_eq!(bound.counters().lower_runtime_check_count(), 1);
}

#[test]
fn continuity_readmission_denies_preview_advisory_capability() {
    let runtime = continuity_runtime();
    let continuity = delivered_continuity(&runtime);
    let capability = preview_inspection();

    let denial = readmit_bridge_continuity_evidence(capability, &continuity)
        .expect_err("advisory preview capability cannot bind continuity evidence");

    match denial.kind() {
        DeniedBasisCapabilityKind::LowerRuntimeCapabilityUnsupported {
            authority,
            family,
            operation_lane,
        } => {
            assert_eq!(authority, &"forge_runtime_bridge");
            assert_eq!(family.as_str(), "preview");
            assert_eq!(operation_lane, &BasisOperationLaneRequest::Inspection);
        }
        other => panic!("unexpected denial kind: {other:?}"),
    }
    assert_eq!(denial.counters().lower_runtime_check_count(), 1);
    assert_eq!(denial.counters().denied_residue_count(), 1);
}

#[test]
fn subscription_readmission_binds_branch_head_declaration_and_activation_evidence() {
    let runtime = subscription_runtime();
    let admitted = detail_subscription(&runtime);
    let declaration = branch_head_subscription_declaration(MAIN_BRANCH);
    let activation = branch_head_subscription_activation(MAIN_BRANCH);

    let declaration_bound =
        readmit_bridge_subscription_declaration_evidence(declaration, &admitted)
            .expect("subscription declaration evidence should readmit");
    let activation_bound = readmit_bridge_subscription_activation_evidence(activation, &admitted)
        .expect("subscription activation evidence should readmit");

    for bound in [
        declaration_bound.evidence().clone(),
        activation_bound.evidence().clone(),
    ] {
        assert_eq!(
            bound.kind(),
            BridgeLowerRuntimeEvidenceKind::SubscriptionAdmission
        );
        let admitted_subscription_identity = admitted
            .admitted_subscription_identity()
            .bridge_admission_evidence();
        assert_eq!(
            bound.admitted_subscription_identity(),
            Some(admitted_subscription_identity.terminal_projection_for_reporting())
        );
        let basis_identity = admitted
            .basis_binding()
            .basis_identity()
            .bridge_admission_evidence();
        assert_eq!(
            bound.basis_identity(),
            Some(basis_identity.terminal_projection_for_reporting())
        );
        let strategy_identity = admitted
            .signal_strategy()
            .strategy_identity()
            .bridge_admission_evidence();
        assert_eq!(
            bound.strategy_identity(),
            Some(strategy_identity.terminal_projection_for_reporting())
        );
    }
}

fn branch_head_observation(branch_identity: &str) -> super::ObservationBasisCapability {
    admit_observation_basis(
        evaluate_basis_eligibility(
            normalize_raw_basis(RawBasisIntent::branch_head(
                super::test_branch_identity(branch_identity),
                BasisOperationLaneRequest::Observation,
            ))
            .expect("branch-head observation should normalize"),
        )
        .expect("branch-head observation should be eligible"),
    )
    .expect("branch-head observation capability should admit")
}

fn branch_head_inspection(branch_identity: &str) -> super::InspectionBasisCapability {
    admit_inspection_basis(
        evaluate_basis_eligibility(
            normalize_raw_basis(RawBasisIntent::branch_head(
                super::test_branch_identity(branch_identity),
                BasisOperationLaneRequest::Inspection,
            ))
            .expect("branch-head inspection should normalize"),
        )
        .expect("branch-head inspection should be eligible"),
    )
    .expect("branch-head inspection capability should admit")
}

fn preview_inspection() -> super::InspectionBasisCapability {
    let capability = admit_inspection_basis(
        evaluate_basis_eligibility(
            normalize_raw_basis(RawBasisIntent::preview(
                super::test_preview_identity("preview:session-1"),
                BasisOperationLaneRequest::Inspection,
            ))
            .expect("preview inspection should normalize"),
        )
        .expect("preview inspection should remain advisory-eligible"),
    )
    .expect("preview inspection capability wrapper should admit");
    assert!(matches!(
        capability.admission(),
        BasisCapabilityAdmission::Advisory(_)
    ));
    capability
}

fn branch_head_subscription_declaration(
    branch_identity: &str,
) -> super::SubscriptionDeclarationBasisCapability {
    admit_subscription_declaration_basis(
        evaluate_basis_eligibility(
            normalize_raw_basis(RawBasisIntent::branch_head(
                super::test_branch_identity(branch_identity),
                BasisOperationLaneRequest::SubscriptionDeclaration,
            ))
            .expect("branch-head declaration should normalize"),
        )
        .expect("branch-head declaration should be eligible"),
    )
    .expect("branch-head declaration capability should admit")
}

fn branch_head_subscription_activation(
    branch_identity: &str,
) -> super::SubscriptionActivationBasisCapability {
    admit_subscription_activation_basis(
        evaluate_basis_eligibility(
            normalize_raw_basis(RawBasisIntent::branch_head(
                super::test_branch_identity(branch_identity),
                BasisOperationLaneRequest::SubscriptionActivation,
            ))
            .expect("branch-head activation should normalize"),
        )
        .expect("branch-head activation should be eligible"),
    )
    .expect("branch-head activation capability should admit")
}
