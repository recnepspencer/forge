use super::super::support::*;
use crate::authoring::{AspectFieldSelector, AuthoredResultShapeField};
use crate::facade::TraversalSelector;
use crate::schema_view::{QuerySchemaView, SchemaRelationView};

fn expanded_manager_schema() -> QuerySchemaView {
    QuerySchemaView::new(
        "runtime-read-composition-expanded",
        [
            SchemaFieldView::new(
                crate::authoring::AspectName::new("identity")
                    .expect("schema aspect literal must be valid"),
                crate::authoring::FieldName::new("id").expect("schema field literal must be valid"),
                SchemaFieldKind::String,
            ),
            SchemaFieldView::new(
                crate::authoring::AspectName::new("profile")
                    .expect("schema aspect literal must be valid"),
                crate::authoring::FieldName::new("display_name")
                    .expect("schema field literal must be valid"),
                SchemaFieldKind::String,
            ),
        ],
        [SchemaRelationView::new(
            crate::authoring::RelationName::new("manager")
                .expect("schema relation literal must be valid"),
            2,
        )],
    )
}

#[test]
fn intent_execution_routing_stop_class_preserves_stage_evidence_and_source() {
    let mut runtime = bridge_runtime_with_support_and_intent_authority(
        intent_support_profile(),
        TestIntentAuthority,
    );
    let declaration = WorthQueryIntentDeclaration::strategy_commit(
        "phase-three-routing-stop-class",
        "strategy.intent.reconcile",
        "1.0",
        "intent.reconcile.input.v1",
        test_intent_input([("entity", "task-1")]),
    );
    let handoff = runtime
        .admit_authoritative_intent_for_execution(declaration.clone())
        .expect("authoritative handoff should admit");
    let binding = runtime.prepare_authoritative_intent_execution_binding(handoff.clone());
    let execution = runtime
        .backend
        .execute_intent(binding.declaration())
        .expect("backend execution should succeed");
    let admitted_handoff = WorthQueryAdmittedIntentExecutionHandoff::from(handoff);
    let snapshot_evidence_identity = execution
        .mutation_receipt()
        .snapshot_identity
        .evidence_identity();
    let execution_provenance =
        WorthQueryIntentExecutionProvenance::for_shared_execution_typed_parts(
            binding.family(),
            binding.entrypoint(),
            binding.execution_seam(),
            binding.handoff().decision_digest(),
            binding.handoff().handoff_digest(),
            binding.binding_digest(),
            execution.outcome_digest(),
            &snapshot_evidence_identity,
        );
    let decision_trace_envelope = WorthQueryIntentDecisionTraceEnvelope::for_admitted_execution(
        &admitted_handoff,
        &execution,
    );
    let error = runtime.intent_execution_routing_error(
        &declaration,
        &execution,
        execution_provenance.clone(),
        decision_trace_envelope.clone(),
        WorthQueryRuntimeError::LiveSubscriptionInstallation {
            view_name: "tasks.phase-three-routing-stop-class".to_string(),
            stage: "delivery-window",
            message: "simulated route failure".to_string(),
        },
    );

    match error.stop_class() {
        WorthQueryStopClass::IntentExecutionRoutingFailed {
            intent_name,
            stage,
            evidence,
            source,
            ..
        } => {
            assert_eq!(intent_name, "phase-three-routing-stop-class");
            assert_eq!(stage, "post-execution-routing");
            assert_eq!(
                evidence
                    .execution_provenance()
                    .execution_provenance_chain_digest(),
                execution_provenance.execution_provenance_chain_digest()
            );
            assert_eq!(
                evidence.decision_trace_envelope().trace_digest(),
                decision_trace_envelope.trace_digest()
            );
            match source.stop_class() {
                WorthQueryStopClass::RuntimeDeclarationFailed {
                    kind,
                    name,
                    stage,
                    message,
                } => {
                    assert_eq!(
                        kind,
                        WorthQueryRuntimeDeclarationFailureKind::LiveSubscriptionInstallation
                    );
                    assert_eq!(name, "tasks.phase-three-routing-stop-class");
                    assert_eq!(stage, "delivery-window");
                    assert_eq!(message, "simulated route failure");
                }
                other => panic!("expected routed source stop class, got {other:?}"),
            }
        }
        other => panic!("expected intent execution routing stop class, got {other:?}"),
    }
}

#[test]
fn read_domain_invariant_stop_class_preserves_summary_and_invariant_identity() {
    let mut workspace =
        bridge_runtime_with_support(WorthQueryRuntimeSupportProfile::bridge_backed(
            "test-subscription-activation",
            "test-preview-basis",
            "test-inspector-evidence",
        ))
        .workspace("runtime.read-composition.stop-class-domain-invariant")
        .expect("read-backed runtime should open a workspace");

    let error = workspace
        .define_read_family_with_invariant_pack(
            "neighbors-stop-class",
            "no_traversal_reads",
            |read| {
                read.anchored_detail(
                    "user",
                    expanded_manager_schema(),
                    |query| {
                        query
                            .project(
                                AspectFieldSelector::new("identity", "id")
                                    .expect("identity projection should build"),
                            )
                            .traverse(
                                TraversalSelector::bounded("manager", 2)
                                    .expect("bounded traversal should build"),
                            )
                    },
                    |shape| {
                        shape.field(
                            AuthoredResultShapeField::new("identity", "id", "id")
                                .expect("identity result-shape field should build"),
                        )
                    },
                )
            },
            |context| {
                let summary = context.read_domain_invariant_summary();
                if summary.declared_traversal_clause_count() > 0 {
                    Err(WorthQueryReadInvariantPackViolation::new(
                        "no_traversal_reads",
                        "this domain hook denies traversal-bearing reads",
                    ))
                } else {
                    Ok(())
                }
            },
        )
        .expect_err("denied invariant packs should reject before execution");

    match error.stop_class() {
        WorthQueryStopClass::ReadCompositionDomainInvariantDenied { denial } => {
            assert_eq!(denial.hook_family(), "domain_invariant_pack_hook");
            assert_eq!(denial.invariant_family(), "no_traversal_reads");
            assert_eq!(
                denial.domain_invariant_summary().graph_family(),
                &WorthQueryReadGraphFamily::Detail
            );
            assert_eq!(
                denial.domain_invariant_summary().scope_class(),
                "anchored_expansion"
            );
            assert_eq!(
                denial
                    .domain_invariant_summary()
                    .declared_traversal_clause_count(),
                1
            );
            assert_eq!(
                denial
                    .domain_invariant_summary()
                    .declared_traversal_depth_limit(),
                2
            );
        }
        other => panic!("expected read-domain-invariant stop class, got {other:?}"),
    }
}
