use super::super::support::*;

fn task_edge_runtime() -> WorthQueryRuntime {
    stateful_bridge_task_edge_runtime()
}

fn loop_successor_verified_profile() -> WorthQueryRuntimeSupportProfile {
    WorthQueryRuntimeSupportProfile::bridge_backed(
        "test-subscription-activation",
        "test-preview-basis",
        "test-inspector-evidence",
    )
    .with_bridge_backed_verification_support(
        "update_existing_verified",
        "direct_relation_identity",
        true,
        true,
        None,
    )
}

#[test]
fn compose_graph_with_invariant_pack_executes_when_pack_admits_program() {
    let mut workspace = task_edge_runtime()
        .workspace("tasks.graph-composition-invariant-pack")
        .expect("runtime should open a named workspace");
    let _: WorthQueryLiveView<WorthQueryNativeRow> = workspace
        .live_view("tasks.graph-composition-invariant-pack-tasks", |q| {
            q.from("Task")
                .select([
                    crate::authoring::AspectFieldKey::from_authoring_parts("identity", "id")
                        .unwrap(),
                    crate::authoring::AspectFieldKey::from_authoring_parts("title", "value")
                        .unwrap(),
                ])
                .order_by(
                    crate::authoring::AspectFieldKey::from_authoring_parts("title", "value")
                        .unwrap(),
                )
                .schema_basis("tasks-graph-composition-invariant-pack-tasks")
        })
        .expect("task live view should declare");
    let _: WorthQueryLiveView<WorthQueryNativeRow> = workspace
        .live_view("tasks.graph-composition-invariant-pack-edges", |q| {
            q.from("TaskEdge")
                .select([
                    crate::authoring::AspectFieldKey::from_authoring_parts("edge", "kind").unwrap(),
                    crate::authoring::AspectFieldKey::from_authoring_parts(
                        "edge",
                        "source_identity",
                    )
                    .unwrap(),
                    crate::authoring::AspectFieldKey::from_authoring_parts(
                        "edge",
                        "target_identity",
                    )
                    .unwrap(),
                ])
                .order_by(
                    crate::authoring::AspectFieldKey::from_authoring_parts("edge", "kind").unwrap(),
                )
                .schema_basis("tasks-graph-composition-invariant-pack-edges")
        })
        .expect("edge live view should declare");

    let mut seen_command_count = 0usize;
    let mut seen_component_count = 0usize;
    let receipt = workspace
        .compose_graph_with_invariant_pack(
            |graph| {
                let task = graph.insert_entity("draft-task", "Task", |task| {
                    task.set_aspect(
                        test_aspect_touch("identity.id"),
                        test_authored_string_aspect_value("task-draft"),
                    )
                    .set_aspect(
                        test_aspect_touch("title.value"),
                        test_authored_string_aspect_value("Draft task"),
                    )
                })?;
                graph.insert_relation("TaskEdge", |edge| {
                    edge.set_aspect(
                        test_aspect_touch("edge.kind"),
                        test_authored_string_aspect_value("depends_on"),
                    )
                    .symbolic_entity_identity(test_aspect_touch("edge.source_identity"), &task)
                    .existing_entity_identity(
                        test_aspect_touch("edge.target_identity"),
                        test_entity_identity("task-existing"),
                    )
                })?;
                Ok(())
            },
            |context: &WorthQueryGraphCompositionInvariantPackContext<'_>| {
                seen_command_count = context.commands().len();
                seen_component_count = context.graph_composition_program().component_count();
                assert_eq!(context.graph_composition_breadth().component_count(), 2);
                Ok(())
            },
        )
        .expect("invariant pack should admit graph composition");

    assert_eq!(seen_command_count, 2);
    assert_eq!(seen_component_count, 2);
    assert_eq!(
        receipt
            .graph_composition_program()
            .expect("graph composition receipt should expose composition program")
            .component_count(),
        2
    );
}

#[test]
fn compose_graph_with_invariant_pack_does_not_overclaim_symbolic_relation_target_family() {
    let mut workspace = task_edge_runtime()
        .workspace("tasks.graph-composition-invariant-pack-unrelated-relation")
        .expect("runtime should open a named workspace");
    let _: WorthQueryLiveView<WorthQueryNativeRow> = workspace
        .live_view(
            "tasks.graph-composition-invariant-pack-unrelated-tasks",
            |q| {
                q.from("Task")
                    .select([
                        crate::authoring::AspectFieldKey::from_authoring_parts("identity", "id")
                            .unwrap(),
                        crate::authoring::AspectFieldKey::from_authoring_parts("title", "value")
                            .unwrap(),
                    ])
                    .order_by(
                        crate::authoring::AspectFieldKey::from_authoring_parts("title", "value")
                            .unwrap(),
                    )
                    .schema_basis("tasks-graph-composition-invariant-pack-unrelated-tasks")
            },
        )
        .expect("task live view should declare");
    let _: WorthQueryLiveView<WorthQueryNativeRow> = workspace
        .live_view(
            "tasks.graph-composition-invariant-pack-unrelated-edges",
            |q| {
                q.from("TaskEdge")
                    .select([
                        crate::authoring::AspectFieldKey::from_authoring_parts("edge", "kind")
                            .unwrap(),
                        crate::authoring::AspectFieldKey::from_authoring_parts(
                            "edge",
                            "source_identity",
                        )
                        .unwrap(),
                        crate::authoring::AspectFieldKey::from_authoring_parts(
                            "edge",
                            "target_identity",
                        )
                        .unwrap(),
                    ])
                    .order_by(
                        crate::authoring::AspectFieldKey::from_authoring_parts("edge", "kind")
                            .unwrap(),
                    )
                    .schema_basis("tasks-graph-composition-invariant-pack-unrelated-edges")
            },
        )
        .expect("edge live view should declare");

    workspace
        .compose_graph_with_invariant_pack(
            |graph| {
                graph.insert_entity("draft-task", "Task", |task| {
                    task.set_aspect(test_aspect_touch("identity.id"), test_authored_string_aspect_value("task-draft"))
                        .set_aspect(test_aspect_touch("title.value"), test_authored_string_aspect_value("Draft task"))
                })?;
                graph.insert_relation("TaskEdge", |edge| {
                    edge.set_aspect(test_aspect_touch("edge.kind"), test_authored_string_aspect_value("depends_on"))
                        .existing_entity_identity(test_aspect_touch("edge.source_identity"),
                            test_entity_identity("task-existing-left"),
                        )
                        .existing_entity_identity(test_aspect_touch("edge.target_identity"),
                            test_entity_identity("task-existing-right"),
                        )
                })?;
                Ok(())
            },
            |context| {
                let summary = context.graph_composition_domain_invariant_summary();
                assert!(
                    summary.target_combination_families().is_empty(),
                    "unrelated symbolic entity creation plus existing-only relation must not claim a same-batch identity edge family"
                );
                Ok(())
            },
        )
        .expect("unrelated relation program should still admit");
}

#[test]
fn compose_graph_with_invariant_pack_denies_domain_invalid_program_distinctly() {
    let binding = WorthQueryExistingRelationTarget::new(
        crate::runtime::WorthQueryMutationAuthorityIdentity::existing_truth_binding_authority(
            crate::runtime::WorthQueryExistingTruthBindingAuthorityLabel::new(
                "authority:loop-next-rel",
            )
            .expect("existing-truth authority label"),
        )
        .expect("existing-truth authority identity"),
        test_entity_identity("HalfEdgeNextRelation:1"),
    )
    .expect("existing relation target should build")
    .in_target_collection("HalfEdgeNextRelation")
    .expect("existing relation target collection should build");
    let binding = WorthQueryExistingTruthTargetBinding::from_relation_target(binding)
        .expect("relation binding should build");
    let runtime = bridge_runtime_with_support_and_existing_truth_verification(
        loop_successor_verified_profile(),
        TestExistingTruthVerificationAdapter::default()
            .with_value(&binding, "source.id", test_string_aspect_value("he-1"))
            .with_value(&binding, "target.id", test_string_aspect_value("he-2")),
    );
    let mut workspace = runtime
        .workspace("topology.graph-composition-failed-non-manifold-admission")
        .expect("runtime should open a named workspace");
    let mut seen_summary_digest = None::<String>;

    let error = workspace
        .compose_graph_with_invariant_pack(
            |graph| {
                let successor = graph.insert_entity("draft-half-edge", "HalfEdge", |half_edge| {
                    half_edge
                        .set_aspect(test_aspect_touch("identity.id"), test_authored_string_aspect_value("he-3"))
                        .set_aspect(test_aspect_touch("kind.value"), test_authored_string_aspect_value("half_edge"))
                })?;
                graph.retarget_existing_verified(
                    binding,
                    |verify| {
                        verify
                            .set_aspect(test_aspect_touch("source.id"), test_authored_string_aspect_value("he-1"))
                            .set_aspect(test_aspect_touch("target.id"), test_authored_string_aspect_value("he-2"))
                    },
                    |update| {
                        update
                            .set_aspect(test_aspect_touch("source.id"), test_authored_string_aspect_value("he-1"))
                            .continuity_rebind_existing_target(crate::runtime::WorthQueryMutationAuthorityIdentity::continuity_prior_authority(crate::runtime::WorthQueryContinuityPriorAuthorityLabel::new("authority:loop-next-rel").expect("continuity prior authority label")).expect("continuity prior authority identity"), crate::runtime::WorthQueryMutationAuthorityIdentity::continuity_successor_authority(crate::runtime::WorthQueryContinuitySuccessorAuthorityLabel::new("authority:loop-next-rel-successor").expect("continuity successor authority label")).expect("continuity successor authority identity"),
                            )
                            .symbolic_entity_identity(test_aspect_touch("target.id"), successor.reference().clone())
                    },
                )?;
                Ok(())
            },
            |context| {
                let summary = context.graph_composition_domain_invariant_summary();
                seen_summary_digest = Some(summary.summary_digest().to_string());
                assert_eq!(
                    summary.declared_collections(),
                    &[
                        "HalfEdge".to_string(),
                        "HalfEdgeNextRelation".to_string()
                    ]
                );
                assert_eq!(summary.declared_symbols(), &["draft-half-edge".to_string()]);
                assert_eq!(
                    summary.target_combination_families(),
                    &["mixed_existing_and_symbolic_entity_identity_edges".to_string()]
                );
                assert_eq!(
                    summary.lifecycle_families(),
                    &["mixed_existing_target_verified_retarget".to_string()]
                );
                assert_eq!(
                    summary.counter_snapshot(),
                    "components=2;symbolic_entities=1;symbolic_relations=0;declared_collections=2;declared_symbols=1;target_combinations=1;lifecycle_families=1"
                );
                Err(WorthQueryGraphCompositionInvariantPackViolation::new(
                    "non_manifold_topology",
                    "loop successor rewire would create a non-manifold adjacency fanout",
                ))
            },
        )
        .expect_err("domain-invalid program should deny before execution");

    match error {
        WorthQueryRuntimeError::GraphCompositionDomainInvariantDenied(denial) => {
            assert_eq!(denial.hook_family(), "domain_invariant_pack_hook");
            assert_eq!(denial.invariant_family(), "non_manifold_topology");
            assert_eq!(
                denial.failure_stage(),
                WorthQueryGraphCompositionAdmissionTraceStage::DomainInvariantEvaluated
            );
            assert!(denial.message().contains("non-manifold adjacency fanout"));
            assert_eq!(
                denial.domain_invariant_summary().declared_collections(),
                &["HalfEdge".to_string(), "HalfEdgeNextRelation".to_string()]
            );
            assert_eq!(
                denial.domain_invariant_summary().declared_symbols(),
                &["draft-half-edge".to_string()]
            );
            assert_eq!(
                denial
                    .domain_invariant_summary()
                    .target_combination_families(),
                &["mixed_existing_and_symbolic_entity_identity_edges".to_string()]
            );
            assert_eq!(
                denial.domain_invariant_summary().lifecycle_families(),
                &["mixed_existing_target_verified_retarget".to_string()]
            );
            assert_eq!(
                denial.domain_invariant_summary().counter_snapshot(),
                "components=2;symbolic_entities=1;symbolic_relations=0;declared_collections=2;declared_symbols=1;target_combinations=1;lifecycle_families=1"
            );
            assert_eq!(
                denial.domain_invariant_summary().summary_digest(),
                seen_summary_digest
                    .as_deref()
                    .expect("invariant pack should observe denial summary"),
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
            assert!(!denial
                .domain_invariant_summary()
                .program_digest()
                .is_empty());
            assert!(!denial
                .domain_invariant_summary()
                .breadth_digest()
                .is_empty());
            assert!(!denial.denial_digest().is_empty());
        }
        other => panic!("expected domain invariant denial, got {other:?}"),
    }
}
