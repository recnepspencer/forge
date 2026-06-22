use forge_foundational::facade::{AspectKey, AspectValue, CanonicalFieldPath, FieldKey};
use forge_query::facade::{
    ForgeQueryAspectTouch, ForgeQueryContinuityPriorAuthorityLabel,
    ForgeQueryContinuitySuccessorAuthorityLabel, ForgeQueryExistingRelationTarget,
    ForgeQueryExistingTruthBindingAuthorityLabel, ForgeQueryExistingTruthTargetBinding,
    ForgeQueryGraphCompositionAdmissionTraceStage, ForgeQueryGraphCompositionLifecycleOutcomeKind,
    ForgeQueryGraphCompositionProgramStepKind, ForgeQueryInspection, ForgeQueryLiveView,
    ForgeQueryMutationAuthorityIdentity, ForgeQueryNativeRow, ForgeQueryRuntimeError,
    ForgeQuerySymbolicTargetReference,
};
mod support;

use support::public_bridge_runtime::{public_graph_support_profile, PublicBridgeRuntimeHarness};
use support::test_entity_identities::relational_test_entity_identity;

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
    let tasks: ForgeQueryLiveView<ForgeQueryNativeRow> = workspace
        .live_view("public.graph-composition-lifecycle-tasks", |q| {
            q.from("Task")
                .select([
                    forge_query::facade::AspectFieldKey::new("identity", "id").unwrap(),
                    forge_query::facade::AspectFieldKey::new("title", "value").unwrap(),
                ])
                .order_by(forge_query::facade::AspectFieldKey::new("title", "value").unwrap())
                .schema_basis("public-graph-composition-lifecycle-tasks")
        })
        .expect("task live view should declare");
    let edges: ForgeQueryLiveView<ForgeQueryNativeRow> = workspace
        .live_view("public.graph-composition-lifecycle-edges", |q| {
            q.from("TaskEdge")
                .select([
                    forge_query::facade::AspectFieldKey::new("edge", "kind").unwrap(),
                    forge_query::facade::AspectFieldKey::new("edge", "source_identity").unwrap(),
                    forge_query::facade::AspectFieldKey::new("edge", "target_identity").unwrap(),
                ])
                .order_by(forge_query::facade::AspectFieldKey::new("edge", "kind").unwrap())
                .schema_basis("public-graph-composition-lifecycle-edges")
        })
        .expect("edge live view should declare");

    let receipt = workspace
        .compose_graph(|graph| {
            let draft = graph.insert_entity("draft-task", "Task", |task| {
                task.aspect(touch("identity.id"), text("task-lifecycle"))
                    .aspect(touch("title.value"), text("Draft task"))
            })?;
            let edge = graph.insert_symbolic_relation("draft-edge", "TaskEdge", |relation| {
                relation
                    .aspect(touch("edge.kind"), text("depends_on"))
                    .symbolic_entity_identity(touch("edge.source_identity"), &draft)
                    .existing_entity_identity(
                        touch("edge.target_identity"),
                        relational_test_entity_identity("task-existing"),
                    )
            })?;
            graph.update_entity(&draft, |task| {
                task.aspect(touch("title.value"), text("Published task"))
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
        task_rows[0].scalar_value_at(&field_path("title.value")),
        Some(&text("Published task"))
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
        relational_test_entity_identity("HalfEdgeNextRelation:1"),
    )
    .expect("existing relation target should build")
    .in_target_collection("HalfEdgeNextRelation")
    .expect("existing relation target collection should build");
    let binding = ForgeQueryExistingTruthTargetBinding::from_relation_target(binding)
        .expect("relation binding should build");
    harness.seed_backend_authoritative_truth(&binding, "source.id", text("he-1"));
    harness.seed_backend_authoritative_truth(&binding, "target.id", text("he-2"));
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
                            .aspect(touch("identity.id"), text("he-3"))
                            .aspect(touch("kind.value"), text("half_edge"))
                    })?;
                graph.retarget_existing_verified(
                    binding,
                    |verify| {
                        verify
                            .aspect(touch("source.id"), text("he-1"))
                            .aspect(touch("target.id"), text("he-2"))
                    },
                    |update| {
                        update
                            .aspect(touch("source.id"), text("he-1"))
                            .continuity_rebind_existing_target(
                                continuity_prior("authority:loop-next-rel"),
                                continuity_successor("authority:loop-next-rel-successor"),
                            )
                            .symbolic_entity_identity(
                                touch("target.id"),
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

fn touch(aspect_path: &str) -> ForgeQueryAspectTouch {
    let mut segments = aspect_path.split('.');
    let aspect = segments
        .next()
        .and_then(|segment| AspectKey::new(segment.to_string()))
        .expect("test aspect path aspect should admit");
    let fields = segments
        .map(|segment| {
            FieldKey::new(segment.to_string()).expect("test aspect path field should admit")
        })
        .collect::<Vec<_>>();
    if fields.is_empty() {
        ForgeQueryAspectTouch::aspect(aspect)
    } else {
        ForgeQueryAspectTouch::field_path(
            aspect,
            CanonicalFieldPath::new(fields).expect("test aspect path should have fields"),
        )
    }
}

fn text(value: impl Into<String>) -> AspectValue {
    AspectValue::String(value.into().into())
}

fn field_path(path: &str) -> CanonicalFieldPath {
    CanonicalFieldPath::new(
        path.split('.').map(|segment| {
            FieldKey::new(segment).expect("test field path segment should be valid")
        }),
    )
    .expect("test field path should be non-empty")
}
