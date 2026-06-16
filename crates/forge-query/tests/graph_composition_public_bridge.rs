use forge_query::facade::{
    ForgeQueryContinuityPriorAuthorityLabel, ForgeQueryContinuitySuccessorAuthorityLabel,
    ForgeQueryEntityIdentity, ForgeQueryExistingRelationTarget,
    ForgeQueryExistingTruthBindingAuthorityLabel, ForgeQueryExistingTruthTargetBinding,
    ForgeQueryGraphCompositionAdmissionTraceStage, ForgeQueryGraphCompositionLifecycleOutcomeKind,
    ForgeQueryGraphCompositionProgramStepKind, ForgeQueryInspection, ForgeQueryLiveView,
    ForgeQueryMutationAuthorityIdentity, ForgeQueryRuntimeError, ForgeQuerySymbolicTargetReference,
};
use serde_json::{json, Value};

mod support;

use support::public_bridge_runtime::{public_graph_support_profile, PublicBridgeRuntimeHarness};

fn public_verified_relation_profile(
    operation_family: &str,
) -> forge_query::facade::ForgeQueryRuntimeSupportProfile {
    public_graph_support_profile().with_bridge_backed_verification_support(
        operation_family,
        "direct_relation_identity",
        true,
        true,
        None,
    )
}

fn existing_authority(label: &str) -> ForgeQueryMutationAuthorityIdentity {
    ForgeQueryMutationAuthorityIdentity::existing_truth_binding_authority(
        ForgeQueryExistingTruthBindingAuthorityLabel::new(label)
            .expect("existing-truth authority label"),
    )
    .expect("existing-truth authority identity")
}

fn continuity_prior(label: &str) -> ForgeQueryMutationAuthorityIdentity {
    ForgeQueryMutationAuthorityIdentity::continuity_prior_authority(
        ForgeQueryContinuityPriorAuthorityLabel::new(label).expect("continuity prior label"),
    )
    .expect("continuity prior identity")
}

fn continuity_successor(label: &str) -> ForgeQueryMutationAuthorityIdentity {
    ForgeQueryMutationAuthorityIdentity::continuity_successor_authority(
        ForgeQueryContinuitySuccessorAuthorityLabel::new(label)
            .expect("continuity successor label"),
    )
    .expect("continuity successor identity")
}

#[test]
fn graph_composition_public_bridge_executes_symbolic_followup_and_relation_retirement() {
    let harness = PublicBridgeRuntimeHarness::new();
    let runtime = harness.bridge_backed_runtime();
    let mut workspace = runtime
        .workspace("public.graph-composition-lifecycle")
        .expect("runtime should open a named workspace");
    let tasks: ForgeQueryLiveView<Value> = workspace
        .live_view("public.graph-composition-lifecycle-tasks", |q| {
            q.from("Task")
                .select(["identity.id", "title.value"])
                .order_by("title.value")
                .schema_basis("public-graph-composition-lifecycle-tasks")
        })
        .expect("task live view should declare");
    let edges: ForgeQueryLiveView<Value> = workspace
        .live_view("public.graph-composition-lifecycle-edges", |q| {
            q.from("TaskEdge")
                .select(["edge.kind", "edge.source_identity", "edge.target_identity"])
                .order_by("edge.kind")
                .schema_basis("public-graph-composition-lifecycle-edges")
        })
        .expect("edge live view should declare");

    let receipt = workspace
        .compose_graph(|graph| {
            let draft = graph.insert_entity("draft-task", "Task", |task| {
                task.aspect("identity.id", "task-lifecycle")
                    .aspect("title.value", "Draft task")
            })?;
            let edge = graph.insert_symbolic_relation("draft-edge", "TaskEdge", |relation| {
                relation
                    .aspect("edge.kind", "depends_on")
                    .symbolic_entity_identity("edge.source_identity", &draft)
                    .existing_entity_identity(
                        "edge.target_identity",
                        ForgeQueryEntityIdentity::admit_authored_entity_token(
                            forge_query::facade::QueryExternalIdentityToken::new(
                                std::sync::Arc::from("task-existing"),
                            ),
                        ),
                    )
            })?;
            graph.update_entity(&draft, |task| task.aspect("title.value", "Published task"))?;
            graph.delete_relation(&edge, |delete| {
                delete.touches(["edge.kind", "edge.source_identity", "edge.target_identity"])
            })?;
            Ok(())
        })
        .expect("graph composition lifecycle should execute");
    let task_rows = workspace.read(&tasks);
    let edge_rows = workspace.read(&edges);
    let inspection = workspace.inspect(&receipt).expect("receipt should inspect");

    assert_eq!(receipt.write_receipts().len(), 4);
    assert_eq!(
        receipt
            .graph_composition_program()
            .expect("graph composition receipt should expose composition program")
            .steps()[2]
            .kind(),
        ForgeQueryGraphCompositionProgramStepKind::SymbolicEntityFollowupMutation
    );
    assert_eq!(
        receipt
            .graph_composition_program()
            .expect("graph composition receipt should expose composition program")
            .steps()[3]
            .kind(),
        ForgeQueryGraphCompositionProgramStepKind::SymbolicRelationRetirement
    );
    assert_eq!(
        receipt
            .graph_composition_lifecycle_outcomes()
            .expect("graph composition receipt should expose lifecycle outcomes")
            .entries()[2]
            .outcome_kind(),
        ForgeQueryGraphCompositionLifecycleOutcomeKind::UpdatedIdentityPreserved
    );
    assert_eq!(
        receipt
            .graph_composition_lifecycle_outcomes()
            .expect("graph composition receipt should expose lifecycle outcomes")
            .entries()[3]
            .outcome_kind(),
        ForgeQueryGraphCompositionLifecycleOutcomeKind::RetiredCurrentTruth
    );
    assert_eq!(receipt.graph_composition_resolution_map().len(), 3);
    assert_eq!(task_rows.len(), 1);
    assert_eq!(edge_rows.len(), 0);
    assert_eq!(
        task_rows[0].external_row()["title"]["value"].as_str(),
        Some("Published task")
    );

    match inspection {
        ForgeQueryInspection::BatchWriteReceipt(inspection) => {
            assert_eq!(inspection.component_operations().len(), 4);
            assert_eq!(
                inspection
                    .graph_composition_evidence()
                    .expect("inspection should expose graph composition evidence")
                    .affected_live_view_count(),
                2
            );
            assert_eq!(
                inspection
                    .graph_composition_resolution_map()
                    .entries()
                    .len(),
                3
            );
        }
        other => panic!("expected batch receipt inspection, got {other:?}"),
    }
}

#[test]
fn graph_composition_public_bridge_preserves_domain_invariant_denial_lane() {
    let harness = PublicBridgeRuntimeHarness::new();
    let binding = ForgeQueryExistingRelationTarget::new(
        existing_authority("authority:loop-next-rel"),
        ForgeQueryEntityIdentity::admit_authored_entity_token(
            forge_query::facade::QueryExternalIdentityToken::new(std::sync::Arc::from(
                "HalfEdgeNextRelation:1",
            )),
        ),
    )
    .expect("existing relation target should build")
    .in_target_collection("HalfEdgeNextRelation")
    .expect("existing relation target collection should build");
    let binding = ForgeQueryExistingTruthTargetBinding::from_relation_target(binding)
        .expect("relation binding should build");
    harness.seed_existing_truth_value(&binding, "source.id", json!("he-1"));
    harness.seed_existing_truth_value(&binding, "target.id", json!("he-2"));
    let runtime = harness
        .bridge_backed_runtime_builder()
        .support_profile(public_verified_relation_profile("update_existing_verified"))
        .build();
    let mut workspace = runtime
        .workspace("public.graph-composition-domain-invariant-denial")
        .expect("runtime should open a named workspace");

    let error = workspace
        .compose_graph_with_invariant_pack(
            |graph| {
                let successor =
                    graph.insert_entity("draft-half-edge", "HalfEdge", |half_edge| {
                        half_edge
                            .aspect("identity.id", "he-3")
                            .aspect("kind.value", "half_edge")
                    })?;
                graph.retarget_existing_verified(
                    binding,
                    |verify| {
                        verify
                            .aspect("source.id", "he-1")
                            .aspect("target.id", "he-2")
                    },
                    |update| {
                        update
                            .aspect("source.id", "he-1")
                            .continuity_rebind_existing_target(
                                continuity_prior("authority:loop-next-rel"),
                                continuity_successor("authority:loop-next-rel-successor"),
                            )
                            .symbolic_entity_identity(
                                "target.id",
                                ForgeQuerySymbolicTargetReference::new(successor.symbol())
                                    .expect("symbolic successor reference should build")
                                    .in_target_collection("HalfEdge")
                                    .expect("symbolic successor collection should build"),
                            )
                    },
                )?;
                Ok(())
            },
            |_context| {
                Err(
                    forge_query::facade::ForgeQueryGraphCompositionInvariantPackViolation::new(
                        "non_manifold_topology",
                        "loop successor rewire would create a non-manifold adjacency fanout",
                    ),
                )
            },
        )
        .expect_err("domain-invalid program should deny before execution");

    match error {
        ForgeQueryRuntimeError::GraphCompositionDomainInvariantDenied(denial) => {
            assert_eq!(denial.invariant_family(), "non_manifold_topology");
            assert_eq!(
                denial.failure_stage(),
                ForgeQueryGraphCompositionAdmissionTraceStage::DomainInvariantEvaluated
            );
            assert_eq!(
                denial.domain_invariant_summary().declared_collections(),
                &["HalfEdge".to_string(), "HalfEdgeNextRelation".to_string()]
            );
            assert_eq!(
                denial.domain_invariant_summary().declared_symbols(),
                &["draft-half-edge".to_string()]
            );
            assert_eq!(
                denial.admission_trace().stages(),
                &[
                    ForgeQueryGraphCompositionAdmissionTraceStage::ProgramParsed,
                    ForgeQueryGraphCompositionAdmissionTraceStage::SymbolsValidated,
                    ForgeQueryGraphCompositionAdmissionTraceStage::LoweringValidated,
                    ForgeQueryGraphCompositionAdmissionTraceStage::DomainInvariantEvaluated,
                    ForgeQueryGraphCompositionAdmissionTraceStage::DeniedBeforeExecution,
                ]
            );
        }
        other => panic!("expected domain invariant denial, got {other:?}"),
    }
}
