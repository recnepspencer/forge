use super::super::support::*;
use crate::identity::hash_parts;

fn admitted_primary_profile(target_binding_family: &str) -> ForgeQueryRuntimeSupportProfile {
    ["update_existing_verified", "delete_existing_verified"]
        .into_iter()
        .fold(
            ForgeQueryRuntimeSupportProfile::bridge_backed(
                "test-subscription-activation",
                "test-preview-basis",
                "test-inspector-evidence",
            ),
            |profile, operation_family| {
                profile.with_bridge_backed_verification_support(
                    operation_family,
                    target_binding_family,
                    true,
                    true,
                    None,
                )
            },
        )
}

fn expected_snapshot_digest(binding_digest: &str, snapshot_token: &str) -> String {
    hash_parts(&[
        "forge_query_existing_truth_assumption_snapshot_v1".to_string(),
        format!("binding:{binding_digest}"),
        format!("snapshot:{snapshot_token}"),
    ])
}

#[test]
fn compose_graph_supports_verified_existing_target_lifecycle() {
    let binding = ForgeQueryExistingRelationTarget::new("authority:rel-1", "TaskRelation:1")
        .expect("existing relation target should build")
        .in_target_collection("TaskRelation")
        .expect("existing relation target collection should build");
    let binding = ForgeQueryExistingTruthTargetBinding::from_relation_target(binding)
        .expect("relation binding should build");
    let runtime = bridge_runtime_with_support_and_existing_truth_verification(
        admitted_primary_profile("direct_relation_identity"),
        TestExistingTruthVerificationAdapter::default()
            .with_value(&binding, "status.value", json!("active"))
            .with_value(&binding, "kind.value", json!("depends_on")),
    );
    let mut workspace = runtime
        .workspace("tasks.graph-composition-verified-existing")
        .expect("workspace should open");

    let receipt = workspace
        .compose_graph(|graph| {
            let _ = graph.insert_entity("draft-task", "Task", |task| {
                task.aspect("identity.id", "task-verified-existing")
                    .aspect("title.value", "Verified existing task")
            })?;
            graph.update_existing_verified(
                binding.clone(),
                |verify| verify.aspect("status.value", "active"),
                |update| update.aspect("status.value", "retired"),
            )?;
            graph.delete_existing_verified(
                binding,
                |verify| verify.aspect("kind.value", "depends_on"),
                |delete| delete.touch("kind.value"),
            )?;
            Ok(())
        })
        .expect("verified existing-target lifecycle should execute");

    let program = receipt
        .graph_composition_program()
        .expect("graph composition receipt should expose composition program");
    let lifecycle = receipt
        .graph_composition_lifecycle_outcomes()
        .expect("graph composition receipt should expose lifecycle outcomes");
    let evidence = receipt
        .graph_composition_evidence()
        .expect("graph composition receipt should expose composition evidence");
    let assumptions = receipt
        .graph_composition_assumption_summary()
        .expect("verified graph composition should expose assumption summary");

    assert_eq!(program.component_count(), 3);
    assert_eq!(
        program.steps()[1].kind(),
        ForgeQueryGraphCompositionProgramStepKind::ExistingTargetVerifiedFollowupMutation
    );
    assert_eq!(
        program.steps()[2].kind(),
        ForgeQueryGraphCompositionProgramStepKind::ExistingTargetVerifiedRetirement
    );
    assert_eq!(
        lifecycle.counter_snapshot(),
        "created=1;updated_identity_preserved=1;retargeted_identity_preserved=0;retired_current_truth=1;superseded_with_lineage=0;deleted_if_uncommitted=0;denied_before_execution=0"
    );
    assert_eq!(
        evidence.counter_snapshot(),
        "components=3;symbolic_entities=1;symbolic_relations=0;symbolic_resolutions=0;affected_live_views=0;affected_derived_views=0;considered_computed_views=0;created=1;updated_identity_preserved=1;retargeted_identity_preserved=0;retired_current_truth=1;superseded_with_lineage=0;deleted_if_uncommitted=0;denied_before_execution=0"
    );
    assert_eq!(
        assumptions.counter_snapshot(),
        "verified_steps=2;target_bindings=2;asserted_aspects=2;distinct_asserted_aspect_paths=2;cleared_assertions=0"
    );
    assert_eq!(assumptions.verified_step_count(), 2);
    assert_eq!(assumptions.assumption_snapshot_digests().len(), 2);
    assert_eq!(assumptions.verified_precondition_digests().len(), 2);
    assert_eq!(
        evidence
            .assumption_summary()
            .expect("graph composition evidence should retain assumption summary")
            .assumption_summary_digest(),
        assumptions.assumption_summary_digest()
    );
    let support = workspace.public_authoritative_mutation_evidence_support();
    assert!(support
        .graph_composition_families()
        .iter()
        .any(|family| family == "mixed_existing_target_verified_followup_mutation"));
    assert!(support
        .graph_composition_families()
        .iter()
        .any(|family| family == "mixed_existing_target_verified_retirement"));

    match workspace.inspect(&receipt).expect("receipt should inspect") {
        ForgeQueryInspection::BatchWriteReceipt(inspection) => {
            let update = &inspection.component_operations()[1];
            let delete = &inspection.component_operations()[2];
            let inspection_assumptions = inspection
                .graph_composition_assumption_summary()
                .expect("inspection should expose assumption summary");
            assert_eq!(update.family(), "update");
            assert_eq!(delete.family(), "delete");
            assert_eq!(
                inspection_assumptions.assumption_summary_digest(),
                assumptions.assumption_summary_digest()
            );
            assert_eq!(
                update
                    .existing_truth_assertion_evidence()
                    .expect("verified update should retain assertion evidence")
                    .mode(),
                ForgeQueryExistingTruthAssertionMode::BackendVerifiedAssertion
            );
            let update_assertion = update
                .existing_truth_assertion_evidence()
                .expect("verified update should retain assertion evidence");
            let update_assumptions = update_assertion
                .verified_assumption_set()
                .expect("verified update should retain assumption set");
            assert_eq!(
                update_assumptions.assumption_snapshot_digest(),
                expected_snapshot_digest(
                    update_assumptions.binding_digest(),
                    update_assumptions.assumption_snapshot_token()
                )
            );
            assert_eq!(
                update_assertion
                    .verification_read_set_breadth()
                    .expect("verified update should retain read-set breadth")
                    .counter_snapshot(),
                "target_bindings=1;asserted_aspects=1;distinct_asserted_aspect_paths=1;cleared_assertions=0"
            );
            assert_eq!(
                delete
                    .existing_truth_assertion_evidence()
                    .expect("verified delete should retain assertion evidence")
                    .mode(),
                ForgeQueryExistingTruthAssertionMode::BackendVerifiedAssertion
            );
            assert_eq!(
                delete
                    .existing_truth_assertion_evidence()
                    .expect("verified delete should retain assertion evidence")
                    .verification_read_set_breadth()
                    .expect("verified delete should retain read-set breadth")
                    .counter_snapshot(),
                "target_bindings=1;asserted_aspects=1;distinct_asserted_aspect_paths=1;cleared_assertions=0"
            );
        }
        other => panic!("expected batch receipt inspection, got {other:?}"),
    }
}

#[test]
fn compose_graph_denies_verified_existing_target_when_backend_verification_is_unsupported() {
    let mut workspace =
        bridge_runtime_with_support(ForgeQueryRuntimeSupportProfile::bridge_backed(
            "test-subscription-activation",
            "test-preview-basis",
            "test-inspector-evidence",
        ))
        .workspace("tasks.graph-composition-verified-existing-unsupported")
        .expect("workspace should open");
    let binding = workspace
        .bind_existing_relation(
            ForgeQueryExistingRelationTarget::new("authority:rel-1", "TaskRelation:1")
                .expect("existing relation target should build")
                .in_target_collection("TaskRelation")
                .expect("existing relation target collection should build"),
        )
        .expect("relation binding should build");

    let error = workspace
        .compose_graph(|graph| {
            graph.update_existing_verified(
                binding,
                |verify| verify.aspect("status.value", "active"),
                |update| update.aspect("status.value", "retired"),
            )?;
            Ok(())
        })
        .expect_err("unsupported backend verification should deny");

    match error {
        ForgeQueryRuntimeError::GraphCompositionDenied(denial) => {
            assert_eq!(
                denial.kind(),
                ForgeQueryGraphCompositionDenialKind::ExistingTargetBackendVerificationUnsupported
            );
            assert_eq!(
                denial.failure_stage(),
                ForgeQueryGraphCompositionAdmissionTraceStage::VerificationSubstrateEvaluated
            );
            assert_eq!(denial.target_collection(), Some("TaskRelation"));
            assert_eq!(
                denial.admission_trace().stages(),
                &[
                    ForgeQueryGraphCompositionAdmissionTraceStage::ProgramParsed,
                    ForgeQueryGraphCompositionAdmissionTraceStage::SymbolsValidated,
                    ForgeQueryGraphCompositionAdmissionTraceStage::LoweringValidated,
                    ForgeQueryGraphCompositionAdmissionTraceStage::VerificationSubstrateEvaluated,
                    ForgeQueryGraphCompositionAdmissionTraceStage::DeniedBeforeExecution,
                ]
            );
        }
        other => panic!("expected graph composition denial, got {other:?}"),
    }
}
