use super::super::support::*;

fn task_relation_runtime() -> ForgeQueryRuntime {
    stateful_bridge_task_relation_runtime()
}

#[test]
fn compose_graph_denies_existing_target_collection_mismatch_typed_and_early() {
    let mut workspace = task_relation_runtime()
        .workspace("tasks.graph-composition-existing-target-collection-mismatch")
        .expect("runtime should open a named workspace");
    let _: ForgeQueryLiveView<ForgeQueryNativeRow> = workspace
        .live_view(
            "tasks.graph-composition-existing-target-collection-mismatch-relations",
            |q| {
                q.from("TaskRelation")
                    .select([
                        crate::authoring::AspectFieldKey::new("identity", "id").unwrap(),
                        crate::authoring::AspectFieldKey::new("kind", "value").unwrap(),
                        crate::authoring::AspectFieldKey::new("status", "value").unwrap(),
                    ])
                    .order_by(crate::authoring::AspectFieldKey::new("identity", "id").unwrap())
                    .schema_basis(
                        "tasks-graph-composition-existing-target-collection-mismatch-relations",
                    )
            },
        )
        .expect("relation live view should declare");
    let relation_seed = workspace
        .insert("TaskRelation", |relation| {
            relation
                .aspect(
                    test_aspect_touch("identity.id"),
                    test_string_aspect_value("rel-update"),
                )
                .aspect(
                    test_aspect_touch("kind.value"),
                    test_string_aspect_value("depends_on"),
                )
                .aspect(
                    test_aspect_touch("status.value"),
                    test_string_aspect_value("open"),
                )
        })
        .expect("relation seed should execute");
    let binding = workspace
        .bind_existing_relation(
            ForgeQueryExistingRelationTarget::new(
                crate::runtime::ForgeQueryMutationAuthorityIdentity::existing_truth_binding_authority(crate::runtime::ForgeQueryExistingTruthBindingAuthorityLabel::new("authority:rel-update").expect("existing-truth authority label")).expect("existing-truth authority identity"),
                relation_seed.deltas()[0].entity_identity.clone(),
            )
            .expect("existing relation target should build")
            .in_target_collection("Task")
            .expect("mismatched collection should still build as a declared binding"),
        )
        .expect("binding should build");

    let error = workspace
        .compose_graph(|graph| {
            graph.update_existing(binding, |relation| {
                relation.aspect(
                    test_aspect_touch("status.value"),
                    test_string_aspect_value("closed"),
                )
            })?;
            Ok(())
        })
        .expect_err("collection-mismatched existing target should deny");

    match error {
        ForgeQueryRuntimeError::GraphCompositionDenied(denial) => {
            assert_eq!(
                denial.kind(),
                ForgeQueryGraphCompositionDenialKind::ExistingTargetCollectionMismatch
            );
            assert_eq!(
                denial.failure_stage(),
                ForgeQueryGraphCompositionAdmissionTraceStage::LoweringValidated
            );
            assert_eq!(denial.symbol(), None);
            assert_eq!(denial.target_collection(), Some("Task"));
            assert!(denial
                .message()
                .contains("belongs to collection `TaskRelation`, not `Task`"));
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
fn compose_graph_denies_existing_target_missing_row_typed_and_early() {
    let mut workspace = task_relation_runtime()
        .workspace("tasks.graph-composition-existing-target-missing")
        .expect("runtime should open a named workspace");
    let binding = workspace
        .bind_existing_relation(
            ForgeQueryExistingRelationTarget::new(
                crate::runtime::ForgeQueryMutationAuthorityIdentity::existing_truth_binding_authority(crate::runtime::ForgeQueryExistingTruthBindingAuthorityLabel::new("authority:rel-update").expect("existing-truth authority label")).expect("existing-truth authority identity"),
                test_entity_identity("TaskRelation:999"),
            )
            .expect("existing relation target should build")
            .in_target_collection("TaskRelation")
            .expect("target collection should build"),
        )
        .expect("binding should build");

    let error = workspace
        .compose_graph(|graph| {
            graph.delete_existing(binding, |delete| {
                delete.touches(test_aspect_touches(["kind.value", "status.value"]))
            })?;
            Ok(())
        })
        .expect_err("missing existing target should deny");

    match error {
        ForgeQueryRuntimeError::GraphCompositionDenied(denial) => {
            assert_eq!(
                denial.kind(),
                ForgeQueryGraphCompositionDenialKind::ExistingTargetResolvedTargetMissing
            );
            assert_eq!(
                denial.failure_stage(),
                ForgeQueryGraphCompositionAdmissionTraceStage::LoweringValidated
            );
            assert_eq!(denial.symbol(), None);
            assert_eq!(denial.target_collection(), Some("TaskRelation"));
            assert!(denial.message().contains("resolved target"));
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
