use super::super::support::*;

fn task_edge_runtime() -> ForgeQueryRuntime {
    stateful_bridge_task_edge_runtime()
}

#[test]
fn compose_graph_denies_empty_composition_typed_and_early() {
    let mut workspace = stateful_bridge_task_runtime()
        .workspace("tasks.graph-composition-empty")
        .expect("task runtime should open a named workspace");

    let error = workspace
        .compose_graph(|_graph| Ok(()))
        .expect_err("empty graph compositions should deny");

    match error {
        ForgeQueryRuntimeError::GraphCompositionDenied(denial) => {
            assert_eq!(
                denial.kind(),
                ForgeQueryGraphCompositionDenialKind::EmptyComposition
            );
            assert_eq!(
                denial.failure_stage(),
                ForgeQueryGraphCompositionAdmissionTraceStage::ProgramParsed
            );
            assert_eq!(denial.symbol(), None);
            assert!(denial.message().contains("at least one operation"));
            assert_eq!(
                denial.admission_trace().stages(),
                &[
                    ForgeQueryGraphCompositionAdmissionTraceStage::ProgramParsed,
                    ForgeQueryGraphCompositionAdmissionTraceStage::DeniedBeforeExecution,
                ]
            );
            assert!(!denial.admission_trace().admission_trace_digest().is_empty());
        }
        other => panic!("expected graph composition denial, got {other:?}"),
    }
}

#[test]
fn compose_graph_denies_duplicate_symbol_declarations_typed_and_early() {
    let mut workspace = stateful_bridge_task_runtime()
        .workspace("tasks.graph-composition-duplicate")
        .expect("task runtime should open a named workspace");

    let error = workspace
        .compose_graph(|graph| {
            let _ = graph.insert_entity("draft-task", "Task", |task| {
                task.set_aspect(
                    test_aspect_touch("identity.id"),
                    test_authored_string_aspect_value("task-draft-one"),
                )
                .set_aspect(
                    test_aspect_touch("title.value"),
                    test_authored_string_aspect_value("Draft one"),
                )
            })?;
            let _ = graph.insert_entity("draft-task", "Task", |task| {
                task.set_aspect(
                    test_aspect_touch("identity.id"),
                    test_authored_string_aspect_value("task-draft-two"),
                )
                .set_aspect(
                    test_aspect_touch("title.value"),
                    test_authored_string_aspect_value("Draft two"),
                )
            })?;
            Ok(())
        })
        .expect_err("duplicate graph symbols should deny");

    match error {
        ForgeQueryRuntimeError::GraphCompositionDenied(denial) => {
            assert_eq!(
                denial.kind(),
                ForgeQueryGraphCompositionDenialKind::DuplicateSymbolDeclaration
            );
            assert_eq!(
                denial.failure_stage(),
                ForgeQueryGraphCompositionAdmissionTraceStage::SymbolsValidated
            );
            assert_eq!(denial.symbol(), Some("draft-task"));
            assert_eq!(denial.target_collection(), Some("Task"));
            assert!(denial.message().contains("declared more than once"));
            assert_eq!(
                denial.admission_trace().stages(),
                &[
                    ForgeQueryGraphCompositionAdmissionTraceStage::ProgramParsed,
                    ForgeQueryGraphCompositionAdmissionTraceStage::SymbolsValidated,
                    ForgeQueryGraphCompositionAdmissionTraceStage::DeniedBeforeExecution,
                ]
            );
        }
        other => panic!("expected graph composition denial, got {other:?}"),
    }
}

#[test]
fn compose_graph_denies_relation_symbol_reuse_across_compositions_typed_and_early() {
    let mut workspace = task_edge_runtime()
        .workspace("tasks.graph-composition-relation-leak")
        .expect("runtime should open a named workspace");
    let _: ForgeQueryLiveView<ForgeQueryNativeRow> = workspace
        .live_view("tasks.graph-composition-relation-leak-tasks", |q| {
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
                .schema_basis("tasks-graph-composition-relation-leak-tasks")
        })
        .expect("task live view should declare");
    let _: ForgeQueryLiveView<ForgeQueryNativeRow> = workspace
        .live_view("tasks.graph-composition-relation-leak-edges", |q| {
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
                .schema_basis("tasks-graph-composition-relation-leak-edges")
        })
        .expect("edge live view should declare");
    let mut saved_relation = None;
    let _receipt = workspace
        .compose_graph(|graph| {
            let draft = graph.insert_entity("draft-task", "Task", |task| {
                task.set_aspect(
                    test_aspect_touch("identity.id"),
                    test_authored_string_aspect_value("task-draft"),
                )
                .set_aspect(
                    test_aspect_touch("title.value"),
                    test_authored_string_aspect_value("Draft task"),
                )
            })?;
            saved_relation = Some(graph.insert_symbolic_relation(
                "draft-edge",
                "TaskEdge",
                |relation| {
                    relation
                        .set_aspect(
                            test_aspect_touch("edge.kind"),
                            test_authored_string_aspect_value("depends_on"),
                        )
                        .symbolic_entity_identity(test_aspect_touch("edge.source_identity"), &draft)
                        .existing_entity_identity(
                            test_aspect_touch("edge.target_identity"),
                            test_entity_identity("task-existing"),
                        )
                },
            )?);
            Ok(())
        })
        .expect("initial composition should execute");
    let saved_relation = saved_relation.expect("composition should expose a saved relation symbol");

    let error = workspace
        .compose_graph(|graph| {
            graph.update_relation(&saved_relation, |relation| {
                relation
                    .set_aspect(
                        test_aspect_touch("edge.kind"),
                        test_authored_string_aspect_value("blocks"),
                    )
                    .set_aspect(
                        test_aspect_touch("edge.target_identity"),
                        test_authored_string_aspect_value("task-second-existing"),
                    )
            })?;
            Ok(())
        })
        .expect_err("relation symbol reuse across compositions should deny");

    match error {
        ForgeQueryRuntimeError::GraphCompositionDenied(denial) => {
            assert_eq!(
                denial.kind(),
                ForgeQueryGraphCompositionDenialKind::UnresolvedSymbolicReference
            );
            assert_eq!(
                denial.failure_stage(),
                ForgeQueryGraphCompositionAdmissionTraceStage::LoweringValidated
            );
            assert_eq!(denial.symbol(), Some("draft-edge"));
            assert_eq!(denial.target_collection(), Some("TaskEdge"));
            assert!(denial
                .message()
                .contains("same-batch symbolic target `draft-edge` was not declared earlier"));
            assert_eq!(
                denial.admission_trace().stages(),
                &[
                    ForgeQueryGraphCompositionAdmissionTraceStage::ProgramParsed,
                    ForgeQueryGraphCompositionAdmissionTraceStage::SymbolsValidated,
                    ForgeQueryGraphCompositionAdmissionTraceStage::LoweringValidated,
                    ForgeQueryGraphCompositionAdmissionTraceStage::DeniedBeforeExecution,
                ]
            );
        }
        other => panic!("expected graph composition denial, got {other:?}"),
    }
}

#[test]
fn compose_graph_denies_entity_symbol_reuse_across_compositions_typed_and_early() {
    let mut workspace = stateful_bridge_task_runtime()
        .workspace("tasks.graph-composition-entity-leak")
        .expect("task runtime should open a named workspace");
    let _: ForgeQueryLiveView<ForgeQueryNativeRow> = workspace
        .live_view("tasks.graph-composition-entity-leak-tasks", |q| {
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
                .schema_basis("tasks-graph-composition-entity-leak-tasks")
        })
        .expect("task live view should declare");
    let mut saved_entity = None;
    let _receipt = workspace
        .compose_graph(|graph| {
            saved_entity = Some(graph.insert_entity("draft-task", "Task", |task| {
                task.set_aspect(
                    test_aspect_touch("identity.id"),
                    test_authored_string_aspect_value("task-draft"),
                )
                .set_aspect(
                    test_aspect_touch("title.value"),
                    test_authored_string_aspect_value("Draft task"),
                )
            })?);
            Ok(())
        })
        .expect("initial composition should execute");
    let saved_entity = saved_entity.expect("composition should expose a saved entity symbol");

    let error = workspace
        .compose_graph(|graph| {
            graph.update_entity(&saved_entity, |task| {
                task.set_aspect(
                    test_aspect_touch("title.value"),
                    test_authored_string_aspect_value("Leaked title"),
                )
            })?;
            Ok(())
        })
        .expect_err("entity symbol reuse across compositions should deny");

    match error {
        ForgeQueryRuntimeError::GraphCompositionDenied(denial) => {
            assert_eq!(
                denial.kind(),
                ForgeQueryGraphCompositionDenialKind::UnresolvedSymbolicReference
            );
            assert_eq!(
                denial.failure_stage(),
                ForgeQueryGraphCompositionAdmissionTraceStage::LoweringValidated
            );
            assert_eq!(denial.symbol(), Some("draft-task"));
            assert_eq!(denial.target_collection(), Some("Task"));
            assert!(denial
                .message()
                .contains("same-batch symbolic target `draft-task` was not declared earlier"));
        }
        other => panic!("expected graph composition denial, got {other:?}"),
    }
}

#[test]
fn compose_graph_denies_symbolic_collection_mismatch_typed_and_early() {
    let mut workspace = task_edge_runtime()
        .workspace("tasks.graph-composition-collection-mismatch")
        .expect("runtime should open a named workspace");
    let _: ForgeQueryLiveView<ForgeQueryNativeRow> = workspace
        .live_view("tasks.graph-composition-collection-mismatch-tasks", |q| {
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
                .schema_basis("tasks-graph-composition-collection-mismatch-tasks")
        })
        .expect("task live view should declare");
    let _: ForgeQueryLiveView<ForgeQueryNativeRow> = workspace
        .live_view("tasks.graph-composition-collection-mismatch-edges", |q| {
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
                .schema_basis("tasks-graph-composition-collection-mismatch-edges")
        })
        .expect("edge live view should declare");

    let error = workspace
        .compose_graph(|graph| {
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
            let _ = graph.insert_symbolic_relation("draft-edge", "TaskEdge", |edge| {
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
            let leaked_entity = ForgeQueryGraphEntitySymbol::new(
                ForgeQuerySymbolicTargetReference::new("draft-edge")?
                    .in_target_collection("Task")?,
            );
            graph.update_entity(&leaked_entity, |entity| {
                entity.set_aspect(
                    test_aspect_touch("title.value"),
                    test_authored_string_aspect_value("Should never lower"),
                )
            })?;
            Ok(())
        })
        .expect_err("collection-mismatched symbolic entity should deny");

    match error {
        ForgeQueryRuntimeError::GraphCompositionDenied(denial) => {
            assert_eq!(
                denial.kind(),
                ForgeQueryGraphCompositionDenialKind::SymbolicCollectionMismatch
            );
            assert_eq!(
                denial.failure_stage(),
                ForgeQueryGraphCompositionAdmissionTraceStage::LoweringValidated
            );
            assert_eq!(denial.symbol(), Some("draft-edge"));
            assert_eq!(denial.target_collection(), Some("Task"));
            assert!(denial
                .message()
                .contains("resolved to collection `TaskEdge`, not `Task`"));
            assert_eq!(
                denial.admission_trace().stages(),
                &[
                    ForgeQueryGraphCompositionAdmissionTraceStage::ProgramParsed,
                    ForgeQueryGraphCompositionAdmissionTraceStage::SymbolsValidated,
                    ForgeQueryGraphCompositionAdmissionTraceStage::LoweringValidated,
                    ForgeQueryGraphCompositionAdmissionTraceStage::DeniedBeforeExecution,
                ]
            );
        }
        other => panic!("expected graph composition denial, got {other:?}"),
    }
}

#[test]
fn graph_composition_denial_traces_distinguish_symbol_validation_from_lowering_failures() {
    let mut workspace = task_edge_runtime()
        .workspace("tasks.graph-composition-denial-trace-classes")
        .expect("runtime should open a named workspace");

    let duplicate = workspace
        .compose_graph(|graph| {
            let _ = graph.insert_entity("draft-task", "Task", |task| {
                task.set_aspect(
                    test_aspect_touch("identity.id"),
                    test_authored_string_aspect_value("task-draft-one"),
                )
                .set_aspect(
                    test_aspect_touch("title.value"),
                    test_authored_string_aspect_value("Draft one"),
                )
            })?;
            let _ = graph.insert_entity("draft-task", "Task", |task| {
                task.set_aspect(
                    test_aspect_touch("identity.id"),
                    test_authored_string_aspect_value("task-draft-two"),
                )
                .set_aspect(
                    test_aspect_touch("title.value"),
                    test_authored_string_aspect_value("Draft two"),
                )
            })?;
            Ok(())
        })
        .expect_err("duplicate symbol declaration should deny");

    let unresolved = workspace
        .compose_graph(|graph| {
            let relation = ForgeQueryGraphRelationSymbol::new(
                ForgeQuerySymbolicTargetReference::new("leaked-edge")?
                    .in_target_collection("TaskEdge")?,
                None,
            );
            graph.update_relation(&relation, |edge| {
                edge.set_aspect(
                    test_aspect_touch("edge.kind"),
                    test_authored_string_aspect_value("blocks"),
                )
            })?;
            Ok(())
        })
        .expect_err("unresolved symbolic relation should deny");

    let duplicate = match duplicate {
        ForgeQueryRuntimeError::GraphCompositionDenied(denial) => denial,
        other => panic!("expected graph composition denial, got {other:?}"),
    };
    let unresolved = match unresolved {
        ForgeQueryRuntimeError::GraphCompositionDenied(denial) => denial,
        other => panic!("expected graph composition denial, got {other:?}"),
    };

    assert_eq!(
        duplicate.failure_stage(),
        ForgeQueryGraphCompositionAdmissionTraceStage::SymbolsValidated
    );
    assert_eq!(
        unresolved.failure_stage(),
        ForgeQueryGraphCompositionAdmissionTraceStage::LoweringValidated
    );
    assert_ne!(
        duplicate.admission_trace().admission_trace_digest(),
        unresolved.admission_trace().admission_trace_digest()
    );
}
