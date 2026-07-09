use super::{worth_query_domain, WorthQueryDomainCapabilityOutcomeKind};
use crate::domain_capabilities::{
    admit_eligible_domain_capability_contribution,
    evaluate_requested_domain_capability_contribution, materialize_projection_consumption_contract,
    materialize_projection_consumption_review,
    prepare_admitted_domain_capability_contribution_for_materialization,
    WorthQueryAftermathContributionAuthoring, WorthQueryProjectionContractRequest,
};
use crate::projection_consumption::{
    ProjectMaterializedFacts, ProjectionConsumptionBindingContext,
    ProjectionConsumptionEligibility, ProjectionConsumptionSource,
    ProjectionConsumptionSupportPosture, ProjectionSourceFamily,
};
use crate::runtime::WorthQueryAdmittedIntentPlan;

#[test]
fn common_aftermath_contract_lane_matches_proof_lane_materialization() {
    let plan = admitted_projection_consumption_plan();

    let common = worth_query_domain("worth.spatial")
        .for_admitted_intent_plan(&plan)
        .consumes_projection_contract(
            "projection.contract",
            WorthQueryProjectionContractRequest::new(
                admitted_projection_source(),
                admitted_projection_binding(),
                ProjectMaterializedFacts::declare().display_field_path(
                    crate::projection_consumption::projection_fact_field_path_from_segments([
                        worth_foundational::facade::FieldKey::new("field")
                            .expect("projection fact field segment should admit"),
                        worth_foundational::facade::FieldKey::new("visible")
                            .expect("projection fact field segment should admit"),
                    ]),
                ),
            ),
        )
        .because("admitted plan aftermath should bind a stable projection contract")
        .materialize()
        .expect("aftermath common lane should materialize");

    let proof = proof_contract(&plan);

    assert_eq!(common, proof);
}

#[test]
fn common_aftermath_review_lane_matches_proof_lane_materialization() {
    let plan = admitted_projection_consumption_plan();

    let common = worth_query_domain("worth.spatial")
        .for_admitted_intent_plan(&plan)
        .establishes_projection_contract(
            "projection.review",
            WorthQueryProjectionContractRequest::new(
                deferred_projection_source(),
                deferred_projection_binding(),
                ProjectMaterializedFacts::declare().target_identity(),
            ),
        )
        .because("write-receipt projection aftermath should stay inspectable before admission")
        .review()
        .expect("aftermath review lane should materialize");

    let proof = proof_review(&plan);

    assert_eq!(common, proof);
}

#[test]
fn common_aftermath_support_and_eligibility_lanes_preserve_warning_truth() {
    let plan = admitted_projection_consumption_plan();

    let support = worth_query_domain("worth.spatial")
        .for_admitted_intent_plan(&plan)
        .establishes_projection_contract(
            "projection.warning",
            WorthQueryProjectionContractRequest::new(
                warning_projection_source(),
                warning_projection_binding(),
                ProjectMaterializedFacts::declare().display_field_path(
                    crate::projection_consumption::projection_fact_field_path_from_segments([
                        worth_foundational::facade::FieldKey::new("field")
                            .expect("projection fact field segment should admit"),
                        worth_foundational::facade::FieldKey::new("visible")
                            .expect("projection fact field segment should admit"),
                    ]),
                ),
            ),
        )
        .because("query-context display aftermath should keep warning-bearing support truth")
        .materialize_support_report()
        .expect("aftermath support lane should materialize");

    let eligibility = worth_query_domain("worth.spatial")
        .for_admitted_intent_plan(&plan)
        .establishes_projection_contract(
            "projection.warning",
            WorthQueryProjectionContractRequest::new(
                warning_projection_source(),
                warning_projection_binding(),
                ProjectMaterializedFacts::declare().display_field_path(
                    crate::projection_consumption::projection_fact_field_path_from_segments([
                        worth_foundational::facade::FieldKey::new("field")
                            .expect("projection fact field segment should admit"),
                        worth_foundational::facade::FieldKey::new("visible")
                            .expect("projection fact field segment should admit"),
                    ]),
                ),
            ),
        )
        .because("query-context display aftermath should keep warning-bearing support truth")
        .materialize_eligibility()
        .expect("aftermath eligibility lane should materialize");

    assert_eq!(
        support.source_family(),
        ProjectionSourceFamily::QueryContextExecution
    );
    assert!(support.rows().iter().any(|row| {
        row.fact_kind() == crate::projection_consumption::ProjectionFactKind::DisplayField
            && matches!(
                row.posture(),
                ProjectionConsumptionSupportPosture::AdmittedWithWarnings(_)
            )
    }));
    assert!(matches!(
        eligibility,
        ProjectionConsumptionEligibility::AdmittedWithWarnings(_, _)
    ));
}

#[test]
fn checked_aftermath_lane_preserves_denied_metadata() {
    let plan = admitted_projection_consumption_plan();

    let checked = worth_query_domain("worth.spatial")
        .for_admitted_intent_plan(&plan)
        .consumes_projection_contract(
            "projection.contract",
            WorthQueryProjectionContractRequest::new(
                admitted_projection_source(),
                admitted_projection_binding(),
                ProjectMaterializedFacts::declare().display_field_path(
                    crate::projection_consumption::projection_fact_field_path_from_segments([
                        worth_foundational::facade::FieldKey::new("field")
                            .expect("projection fact field segment should admit"),
                        worth_foundational::facade::FieldKey::new("visible")
                            .expect("projection fact field segment should admit"),
                    ]),
                ),
            ),
        )
        .because("")
        .try_materialize();

    assert_eq!(
        checked.kind(),
        WorthQueryDomainCapabilityOutcomeKind::Denied
    );
    assert_eq!(checked.category(), "consequence-aftermath");
    assert_eq!(checked.semantic_posture(), "consumes-fact");
    assert_eq!(
        checked.target_kind(),
        crate::domain_capabilities::WorthQueryDomainCapabilityTargetKind::AdmittedIntentPlan
    );
    assert!(checked.denial().is_some());
}

fn proof_contract(
    plan: &WorthQueryAdmittedIntentPlan,
) -> crate::projection_consumption::MaterializedProjectionContract {
    let proof_requested = WorthQueryAftermathContributionAuthoring::consumes_projection_contract(
        "worth.spatial.projection.contract",
        "admitted plan aftermath should bind a stable projection contract",
        admitted_projection_source(),
        admitted_projection_binding(),
        ProjectMaterializedFacts::declare().display_field_path(
            crate::projection_consumption::projection_fact_field_path_from_segments([
                worth_foundational::facade::FieldKey::new("field")
                    .expect("projection fact field segment should admit"),
                worth_foundational::facade::FieldKey::new("visible")
                    .expect("projection fact field segment should admit"),
            ]),
        ),
    )
    .for_admitted_intent_plan(plan);
    let proof_admitted = success(admit_eligible_domain_capability_contribution(success(
        evaluate_requested_domain_capability_contribution(proof_requested),
    )));
    let proof_target = proof_admitted.payload().target().clone();

    success(materialize_projection_consumption_contract(success(
        prepare_admitted_domain_capability_contribution_for_materialization(
            proof_admitted,
            proof_target,
        ),
    )))
}

fn proof_review(
    plan: &WorthQueryAdmittedIntentPlan,
) -> crate::domain_capabilities::WorthQueryAftermathProjectionConsumptionReview {
    let proof_requested =
        WorthQueryAftermathContributionAuthoring::establishes_projection_contract(
            "worth.spatial.projection.review",
            "write-receipt projection aftermath should stay inspectable before admission",
            deferred_projection_source(),
            deferred_projection_binding(),
            ProjectMaterializedFacts::declare().target_identity(),
        )
        .for_admitted_intent_plan(plan);
    let proof_admitted = success(admit_eligible_domain_capability_contribution(success(
        evaluate_requested_domain_capability_contribution(proof_requested),
    )));
    let proof_target = proof_admitted.payload().target().clone();

    success(materialize_projection_consumption_review(success(
        prepare_admitted_domain_capability_contribution_for_materialization(
            proof_admitted,
            proof_target,
        ),
    )))
}

fn success<T>(
    outcome: crate::domain_capabilities::WorthQueryDomainCapabilityTransitionOutcome<T>,
) -> T {
    match outcome {
        worth_proof::TransitionOutcome::Success(value) => value,
        _ => panic!("expected success"),
    }
}

fn admitted_projection_consumption_plan() -> WorthQueryAdmittedIntentPlan {
    let declaration = crate::projection_consumption::declare_projection_consumption(
        admitted_projection_source(),
        admitted_projection_binding(),
        ProjectMaterializedFacts::declare().display_field_path(
            crate::projection_consumption::projection_fact_field_path_from_segments([
                worth_foundational::facade::FieldKey::new("field")
                    .expect("projection fact field segment should admit"),
                worth_foundational::facade::FieldKey::new("visible")
                    .expect("projection fact field segment should admit"),
            ]),
        ),
    )
    .expect("projection declaration should build");
    let request =
        crate::intent_admission::WorthQueryRawIntentAdmissionRequest::projection_consumption(
            declaration,
        )
        .expect("projection request should build");

    match crate::intent_admission::admit_runtime_intent_request(request) {
        crate::intent_admission::WorthQueryIntentAdmissionDecision::Admitted(plan) => plan,
        other => panic!("expected admitted projection-consumption plan, got {other:?}"),
    }
}

fn admitted_projection_source() -> ProjectionConsumptionSource {
    ProjectionConsumptionSource::test_only(
        ProjectionSourceFamily::QueryReadReceipt,
        Some("query-digest:domain-capability"),
        Some("basis-digest:domain-capability"),
        Some("result-digest:domain-capability"),
        Some("shape-digest:domain-capability"),
        "query-read:domain-capability",
    )
}

fn warning_projection_source() -> ProjectionConsumptionSource {
    ProjectionConsumptionSource::test_only(
        ProjectionSourceFamily::QueryContextExecution,
        Some("query-digest:domain-capability"),
        Some("basis-digest:domain-capability"),
        Some("result-digest:domain-capability"),
        Some("shape-digest:domain-capability"),
        "query-context:domain-capability",
    )
}

fn deferred_projection_source() -> ProjectionConsumptionSource {
    ProjectionConsumptionSource::test_only(
        ProjectionSourceFamily::QueryWriteReceipt,
        None,
        Some("basis-digest:domain-capability"),
        None,
        None,
        "query-write:domain-capability",
    )
}

fn admitted_projection_binding() -> ProjectionConsumptionBindingContext {
    ProjectionConsumptionBindingContext::intent_admission_certification_binding(
        "shape-digest:domain-capability",
        "query-digest:domain-capability",
        "shape-digest:domain-capability",
        "authorized-projection:domain-capability",
        "narrowed-shape-digest:domain-capability",
        "policy-digest:domain-capability",
        "tenant-schema-digest:domain-capability",
        crate::projection_consumption::test_authorized_field_paths(&["field.visible"]),
    )
}

fn warning_projection_binding() -> ProjectionConsumptionBindingContext {
    admitted_projection_binding()
}

fn deferred_projection_binding() -> ProjectionConsumptionBindingContext {
    ProjectionConsumptionBindingContext::test_only(
        "result-shape:test",
        "authorized-projection:domain-capability",
        crate::projection_consumption::test_authorized_field_paths(&["identity.id"]),
    )
}
