use crate::support;
use worth_foundational::facade::{CanonicalFieldPath, FieldKey};
use worth_query::facade::runtime::{
    WorthQueryContinuityPriorAuthorityLabel, WorthQueryContinuitySuccessorAuthorityLabel,
    WorthQueryExistingRelationTarget, WorthQueryExistingTruthBindingAuthorityLabel,
    WorthQueryExistingTruthTargetBinding, WorthQueryGraphCompositionAdmissionTraceStage,
    WorthQueryGraphCompositionLifecycleOutcomeKind, WorthQueryGraphCompositionProgramStepKind,
    WorthQueryInspection, WorthQueryLiveView, WorthQueryMutationAuthorityIdentity,
    WorthQueryRuntimeError, WorthQuerySymbolicTargetReference, WorthQueryUnrefinedLiveShape,
};

use support::aspect_touch as touch;
use support::public_bridge_runtime::{public_graph_support_profile, PublicBridgeRuntimeHarness};
use support::test_entity_identities::relational_test_entity_identity;

fn public_verified_relation_profile(
    operation_family: &str,
) -> worth_query::facade::runtime::WorthQueryRuntimeSupportProfile {
    public_graph_support_profile().with_bridge_backed_verification_support(
        operation_family,
        "direct_relation_identity",
        true,
        true,
        None,
    )
}

fn existing_authority(label: &str) -> WorthQueryMutationAuthorityIdentity {
    WorthQueryMutationAuthorityIdentity::existing_truth_binding_authority(
        WorthQueryExistingTruthBindingAuthorityLabel::new(label)
            .expect("existing-truth authority label"),
    )
    .expect("existing-truth authority identity")
}

fn continuity_prior(label: &str) -> WorthQueryMutationAuthorityIdentity {
    WorthQueryMutationAuthorityIdentity::continuity_prior_authority(
        WorthQueryContinuityPriorAuthorityLabel::new(label).expect("continuity prior label"),
    )
    .expect("continuity prior identity")
}

fn continuity_successor(label: &str) -> WorthQueryMutationAuthorityIdentity {
    WorthQueryMutationAuthorityIdentity::continuity_successor_authority(
        WorthQueryContinuitySuccessorAuthorityLabel::new(label)
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
    let tasks: WorthQueryLiveView<WorthQueryUnrefinedLiveShape> = workspace
        .live_view("public.graph-composition-lifecycle-tasks", |q| {
            q.from("Task")
                .select([
                    worth_query::facade::foundation::AspectFieldKey::from_authoring_parts(
                        "identity", "id",
                    )
                    .unwrap(),
                    worth_query::facade::foundation::AspectFieldKey::from_authoring_parts(
                        "title", "value",
                    )
                    .unwrap(),
                ])
                .order_by(
                    worth_query::facade::foundation::AspectFieldKey::from_authoring_parts(
                        "title", "value",
                    )
                    .unwrap(),
                )
                .schema_basis("public-graph-composition-lifecycle-tasks")
        })
        .expect("task live view should declare");
    let edges: WorthQueryLiveView<WorthQueryUnrefinedLiveShape> = workspace
        .live_view("public.graph-composition-lifecycle-edges", |q| {
            q.from("TaskEdge")
                .select([
                    worth_query::facade::foundation::AspectFieldKey::from_authoring_parts(
                        "edge", "kind",
                    )
                    .unwrap(),
                    worth_query::facade::foundation::AspectFieldKey::from_authoring_parts(
                        "edge",
                        "source_identity",
                    )
                    .unwrap(),
                    worth_query::facade::foundation::AspectFieldKey::from_authoring_parts(
                        "edge",
                        "target_identity",
                    )
                    .unwrap(),
                ])
                .order_by(
                    worth_query::facade::foundation::AspectFieldKey::from_authoring_parts(
                        "edge", "kind",
                    )
                    .unwrap(),
                )
                .schema_basis("public-graph-composition-lifecycle-edges")
        })
        .expect("edge live view should declare");

    let receipt = workspace
        .compose_graph(|graph| {
            let draft = graph.insert_entity("draft-task", "Task", |task| {
                task.set_aspect(touch("identity.id"), authored_text("task-lifecycle"))
                    .set_aspect(touch("title.value"), authored_text("Draft task"))
            })?;
            let edge = graph.insert_symbolic_relation("draft-edge", "TaskEdge", |relation| {
                relation
                    .set_aspect(touch("edge.kind"), authored_text("depends_on"))
                    .symbolic_entity_identity(touch("edge.source_identity"), &draft)
                    .existing_entity_identity(
                        touch("edge.target_identity"),
                        relational_test_entity_identity("task-existing"),
                    )
            })?;
            graph.update_entity(&draft, |task| {
                task.set_aspect(touch("title.value"), authored_text("Published task"))
            })?;
            graph.delete_relation(&edge, |delete| {
                delete.touches([
                    touch("edge.kind"),
                    touch("edge.source_identity"),
                    touch("edge.target_identity"),
                ])
            })?;
            Ok(())
        })
        .expect("graph composition lifecycle should execute");
    let task_rows = workspace.read(&tasks);
    let edge_rows = workspace.read(&edges);
    let inspection = workspace
        .inspections()
        .expect("inspection lane should admit")
        .inspect(&receipt)
        .expect("receipt should inspect");

    assert_eq!(receipt.write_receipts().len(), 4);
    assert_eq!(
        receipt
            .graph_composition_program()
            .expect("graph composition receipt should expose composition program")
            .steps()[2]
            .kind(),
        WorthQueryGraphCompositionProgramStepKind::SymbolicEntityFollowupMutation
    );
    assert_eq!(
        receipt
            .graph_composition_program()
            .expect("graph composition receipt should expose composition program")
            .steps()[3]
            .kind(),
        WorthQueryGraphCompositionProgramStepKind::SymbolicRelationRetirement
    );
    assert_eq!(
        receipt
            .graph_composition_lifecycle_outcomes()
            .expect("graph composition receipt should expose lifecycle outcomes")
            .entries()[2]
            .outcome_kind(),
        WorthQueryGraphCompositionLifecycleOutcomeKind::UpdatedIdentityPreserved
    );
    assert_eq!(
        receipt
            .graph_composition_lifecycle_outcomes()
            .expect("graph composition receipt should expose lifecycle outcomes")
            .entries()[3]
            .outcome_kind(),
        WorthQueryGraphCompositionLifecycleOutcomeKind::RetiredCurrentTruth
    );
    assert_eq!(receipt.graph_composition_resolution_map().len(), 3);
    assert_eq!(task_rows.len(), 1);
    assert_eq!(edge_rows.len(), 0);
    assert_eq!(
        task_rows[0].scalar_value_at(&field_path("title.value")),
        Some(&text("Published task"))
    );

    match inspection {
        WorthQueryInspection::BatchWriteReceipt(inspection) => {
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
    let binding = WorthQueryExistingRelationTarget::new(
        existing_authority("authority:loop-next-rel"),
        relational_test_entity_identity("HalfEdgeNextRelation:1"),
    )
    .expect("existing relation target should build")
    .in_target_collection("HalfEdgeNextRelation")
    .expect("existing relation target collection should build");
    let binding = WorthQueryExistingTruthTargetBinding::from_relation_target(binding)
        .expect("relation binding should build");
    harness.seed_backend_authoritative_truth(&binding, touch("source.id"), text("he-1"));
    harness.seed_backend_authoritative_truth(&binding, touch("target.id"), text("he-2"));
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
                            .set_aspect(touch("identity.id"), authored_text("he-3"))
                            .set_aspect(touch("kind.value"), authored_text("half_edge"))
                    })?;
                graph.retarget_existing_verified(
                    binding,
                    |verify| {
                        verify
                            .set_aspect(touch("source.id"), authored_text("he-1"))
                            .set_aspect(touch("target.id"), authored_text("he-2"))
                    },
                    |update| {
                        update
                            .set_aspect(touch("source.id"), authored_text("he-1"))
                            .continuity_rebind_existing_target(
                                continuity_prior("authority:loop-next-rel"),
                                continuity_successor("authority:loop-next-rel-successor"),
                            )
                            .symbolic_entity_identity(
                                touch("target.id"),
                                WorthQuerySymbolicTargetReference::new(successor.symbol())
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
                    worth_query::facade::runtime::WorthQueryGraphCompositionInvariantPackViolation::new(
                        "non_manifold_topology",
                        "loop successor rewire would create a non-manifold adjacency fanout",
                    ),
                )
            },
        )
        .expect_err("domain-invalid program should deny before execution");

    match error {
        WorthQueryRuntimeError::GraphCompositionDomainInvariantDenied(denial) => {
            assert_eq!(denial.invariant_family(), "non_manifold_topology");
            assert_eq!(
                denial.failure_stage(),
                WorthQueryGraphCompositionAdmissionTraceStage::DomainInvariantEvaluated
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
                    WorthQueryGraphCompositionAdmissionTraceStage::ProgramParsed,
                    WorthQueryGraphCompositionAdmissionTraceStage::SymbolsValidated,
                    WorthQueryGraphCompositionAdmissionTraceStage::LoweringValidated,
                    WorthQueryGraphCompositionAdmissionTraceStage::DomainInvariantEvaluated,
                    WorthQueryGraphCompositionAdmissionTraceStage::DeniedBeforeExecution,
                ]
            );
        }
        other => panic!("expected domain invariant denial, got {other:?}"),
    }
}

fn authored_text(
    value: impl Into<String>,
) -> worth_query::facade::runtime::WorthQueryAuthoredAspectValue {
    worth_query::facade::runtime::WorthQueryAuthoredAspectValue::string(value)
}

fn text(value: impl Into<String>) -> worth_foundational::facade::AspectValue {
    worth_foundational::facade::AspectValue::String(value.into().into())
}

fn field_path(path: &str) -> CanonicalFieldPath {
    CanonicalFieldPath::new(
        path.split('.').map(|segment| {
            FieldKey::new(segment).expect("test field path segment should be valid")
        }),
    )
    .expect("test field path should be non-empty")
}
