use super::super::support::*;

fn task_relation_runtime() -> WorthQueryRuntime {
    stateful_bridge_runtime_with_collections(&["TaskRelation"])
}

fn seed_relation_binding(
    workspace: &mut WorthQueryWorkspace,
    workspace_name: &str,
) -> WorthQueryExistingTruthTargetBinding {
    let _: WorthQueryLiveView<WorthQueryNativeRow> = workspace
        .live_view(workspace_name, |q| {
            q.from("TaskRelation")
                .select([
                    crate::authoring::AspectFieldKey::from_authoring_parts("identity", "id")
                        .unwrap(),
                    crate::authoring::AspectFieldKey::from_authoring_parts("kind", "value")
                        .unwrap(),
                    crate::authoring::AspectFieldKey::from_authoring_parts("source", "id").unwrap(),
                    crate::authoring::AspectFieldKey::from_authoring_parts("target", "id").unwrap(),
                ])
                .order_by(
                    crate::authoring::AspectFieldKey::from_authoring_parts("identity", "id")
                        .unwrap(),
                )
                .schema_basis(workspace_name)
        })
        .expect("relation live view should declare");
    let seed = workspace
        .insert("TaskRelation", |relation| {
            relation
                .set_aspect(
                    test_aspect_touch("identity.id"),
                    test_authored_string_aspect_value("rel-next"),
                )
                .set_aspect(
                    test_aspect_touch("kind.value"),
                    test_authored_string_aspect_value("loop_successor"),
                )
                .set_aspect(
                    test_aspect_touch("source.id"),
                    test_authored_string_aspect_value("loop-a"),
                )
                .set_aspect(
                    test_aspect_touch("target.id"),
                    test_authored_string_aspect_value("loop-b"),
                )
        })
        .expect("seed insert should execute");
    workspace
        .bind_existing_relation(
            WorthQueryExistingRelationTarget::new(
                crate::runtime::WorthQueryMutationAuthorityIdentity::existing_truth_binding_authority(crate::runtime::WorthQueryExistingTruthBindingAuthorityLabel::new("authority:rel-next").expect("existing-truth authority label")).expect("existing-truth authority identity"),
                seed.deltas()[0].entity_identity.clone(),
            )
            .expect("existing relation target should build")
            .in_target_collection("TaskRelation")
            .expect("existing relation target collection should build"),
        )
        .expect("relation binding should build")
}

#[test]
fn compose_graph_denies_existing_target_retarget_with_split_successor_continuity() {
    let mut workspace = task_relation_runtime()
        .workspace("tasks.graph-composition-retarget-split-denial")
        .expect("runtime should open a named workspace");
    let binding = seed_relation_binding(
        &mut workspace,
        "tasks-graph-composition-retarget-split-denial-relations",
    );

    let error = workspace
        .compose_graph(|graph| {
            graph.retarget_existing(binding, |relation| {
                relation
                    .continuity_split_successors(
                        crate::runtime::WorthQueryMutationAuthorityIdentity::continuity_prior_authority(crate::runtime::WorthQueryContinuityPriorAuthorityLabel::new(
                            "authority:rel-next",
                        )
                        .expect("continuity prior authority label")).expect("continuity prior authority identity"),
                        [
                            crate::runtime::WorthQueryMutationAuthorityIdentity::continuity_successor_authority(crate::runtime::WorthQueryContinuitySuccessorAuthorityLabel::new(
                                "authority:rel-next-left",
                            )
                            .expect("continuity successor authority label")).expect("continuity successor authority identity"),
                            crate::runtime::WorthQueryMutationAuthorityIdentity::continuity_successor_authority(crate::runtime::WorthQueryContinuitySuccessorAuthorityLabel::new(
                                "authority:rel-next-right",
                            )
                            .expect("continuity successor authority label")).expect("continuity successor authority identity"),
                        ],
                    )
                    .set_aspect(test_aspect_touch("target.id"), test_authored_string_aspect_value("loop-c"))
            })?;
            Ok(())
        })
        .expect_err("split successor continuity should not impersonate retarget");

    match error {
        WorthQueryRuntimeError::GraphCompositionDenied(denial) => {
            assert_eq!(
                denial.kind(),
                WorthQueryGraphCompositionDenialKind::ExistingTargetIdentityPreservationUnavailable
            );
            assert_eq!(
                denial.failure_stage(),
                WorthQueryGraphCompositionAdmissionTraceStage::LoweringValidated
            );
            assert_eq!(denial.target_collection(), Some("TaskRelation"));
        }
        other => panic!("expected graph composition denial, got {other:?}"),
    }
}

#[test]
fn compose_graph_denies_existing_target_retarget_without_rebind_intent() {
    let mut workspace = task_relation_runtime()
        .workspace("tasks.graph-composition-retarget-denial")
        .expect("runtime should open a named workspace");
    let binding = seed_relation_binding(
        &mut workspace,
        "tasks-graph-composition-retarget-denial-relations",
    );

    let error = workspace
        .compose_graph(|graph| {
            graph.retarget_existing(binding, |relation| {
                relation.set_aspect(
                    test_aspect_touch("target.id"),
                    test_authored_string_aspect_value("loop-c"),
                )
            })?;
            Ok(())
        })
        .expect_err("retarget lanes should deny without rebind intent");

    match error {
        WorthQueryRuntimeError::GraphCompositionDenied(denial) => {
            assert_eq!(
                denial.kind(),
                WorthQueryGraphCompositionDenialKind::ExistingTargetRetargetUnsupported
            );
            assert_eq!(
                denial.failure_stage(),
                WorthQueryGraphCompositionAdmissionTraceStage::LoweringValidated
            );
            assert_eq!(denial.target_collection(), Some("TaskRelation"));
        }
        other => panic!("expected graph composition denial, got {other:?}"),
    }
}
