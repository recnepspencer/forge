use super::super::support::*;

fn task_relation_runtime() -> ForgeQueryRuntime {
    stateful_bridge_runtime_with_collections(&["TaskRelation"])
}

fn seed_relation_binding(
    workspace: &mut ForgeQueryWorkspace,
    workspace_name: &str,
) -> ForgeQueryExistingTruthTargetBinding {
    let _: ForgeQueryLiveView<Value> = workspace
        .live_view(workspace_name, |q| {
            q.from("TaskRelation")
                .select(["identity.id", "kind.value", "source.id", "target.id"])
                .order_by("identity.id")
                .schema_basis(workspace_name)
        })
        .expect("relation live view should declare");
    let seed = workspace
        .insert("TaskRelation", |relation| {
            relation
                .aspect("identity.id", "rel-next")
                .aspect("kind.value", "loop_successor")
                .aspect("source.id", "loop-a")
                .aspect("target.id", "loop-b")
        })
        .expect("seed insert should execute");
    workspace
        .bind_existing_relation(
            ForgeQueryExistingRelationTarget::new(
                crate::runtime::ForgeQueryMutationAuthorityIdentity::existing_truth_binding_authority(crate::runtime::ForgeQueryExistingTruthBindingAuthorityLabel::new("authority:rel-next").expect("existing-truth authority label")).expect("existing-truth authority identity"),
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
                        crate::runtime::ForgeQueryMutationAuthorityIdentity::continuity_prior_authority(crate::runtime::ForgeQueryContinuityPriorAuthorityLabel::new(
                            "authority:rel-next",
                        )
                        .expect("continuity prior authority label")).expect("continuity prior authority identity"),
                        [
                            crate::runtime::ForgeQueryMutationAuthorityIdentity::continuity_successor_authority(crate::runtime::ForgeQueryContinuitySuccessorAuthorityLabel::new(
                                "authority:rel-next-left",
                            )
                            .expect("continuity successor authority label")).expect("continuity successor authority identity"),
                            crate::runtime::ForgeQueryMutationAuthorityIdentity::continuity_successor_authority(crate::runtime::ForgeQueryContinuitySuccessorAuthorityLabel::new(
                                "authority:rel-next-right",
                            )
                            .expect("continuity successor authority label")).expect("continuity successor authority identity"),
                        ],
                    )
                    .aspect("target.id", "loop-c")
            })?;
            Ok(())
        })
        .expect_err("split successor continuity should not impersonate retarget");

    match error {
        ForgeQueryRuntimeError::GraphCompositionDenied(denial) => {
            assert_eq!(
                denial.kind(),
                ForgeQueryGraphCompositionDenialKind::ExistingTargetIdentityPreservationUnavailable
            );
            assert_eq!(
                denial.failure_stage(),
                ForgeQueryGraphCompositionAdmissionTraceStage::LoweringValidated
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
            graph.retarget_existing(binding, |relation| relation.aspect("target.id", "loop-c"))?;
            Ok(())
        })
        .expect_err("retarget lanes should deny without rebind intent");

    match error {
        ForgeQueryRuntimeError::GraphCompositionDenied(denial) => {
            assert_eq!(
                denial.kind(),
                ForgeQueryGraphCompositionDenialKind::ExistingTargetRetargetUnsupported
            );
            assert_eq!(
                denial.failure_stage(),
                ForgeQueryGraphCompositionAdmissionTraceStage::LoweringValidated
            );
            assert_eq!(denial.target_collection(), Some("TaskRelation"));
        }
        other => panic!("expected graph composition denial, got {other:?}"),
    }
}
