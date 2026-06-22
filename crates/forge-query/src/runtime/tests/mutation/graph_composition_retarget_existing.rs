use super::super::support::*;

fn task_relation_runtime() -> ForgeQueryRuntime {
    stateful_bridge_task_relation_runtime()
}

fn verified_profile() -> ForgeQueryRuntimeSupportProfile {
    ForgeQueryRuntimeSupportProfile::bridge_backed(
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
fn compose_graph_supports_existing_target_retarget_lifecycle() {
    let mut workspace = task_relation_runtime()
        .workspace("tasks.graph-composition-retarget-existing")
        .expect("runtime should open a named workspace");
    let _: ForgeQueryLiveView<ForgeQueryNativeRow> = workspace
        .live_view("tasks.graph-composition-retarget-existing-tasks", |q| {
            q.from("Task")
                .select([
                    crate::authoring::AspectFieldKey::new("identity", "id").unwrap(),
                    crate::authoring::AspectFieldKey::new("title", "value").unwrap(),
                ])
                .order_by(crate::authoring::AspectFieldKey::new("title", "value").unwrap())
                .schema_basis("tasks-graph-composition-retarget-existing-tasks")
        })
        .expect("task live view should declare");
    let _: ForgeQueryLiveView<ForgeQueryNativeRow> = workspace
        .live_view("tasks.graph-composition-retarget-existing-relations", |q| {
            q.from("TaskRelation")
                .select([
                    crate::authoring::AspectFieldKey::new("identity", "id").unwrap(),
                    crate::authoring::AspectFieldKey::new("kind", "value").unwrap(),
                    crate::authoring::AspectFieldKey::new("source", "id").unwrap(),
                    crate::authoring::AspectFieldKey::new("target", "id").unwrap(),
                ])
                .order_by(crate::authoring::AspectFieldKey::new("identity", "id").unwrap())
                .schema_basis("tasks-graph-composition-retarget-existing-relations")
        })
        .expect("relation live view should declare");

    let seed = workspace
        .insert("TaskRelation", |relation| {
            relation
                .aspect(
                    test_aspect_touch("identity.id"),
                    test_string_aspect_value("rel-next"),
                )
                .aspect(
                    test_aspect_touch("kind.value"),
                    test_string_aspect_value("loop_successor"),
                )
                .aspect(
                    test_aspect_touch("source.id"),
                    test_string_aspect_value("loop-a"),
                )
                .aspect(
                    test_aspect_touch("target.id"),
                    test_string_aspect_value("loop-b"),
                )
        })
        .expect("seed insert should execute");
    let binding = workspace
        .bind_existing_relation(
            ForgeQueryExistingRelationTarget::new(
                crate::runtime::ForgeQueryMutationAuthorityIdentity::existing_truth_binding_authority(crate::runtime::ForgeQueryExistingTruthBindingAuthorityLabel::new("authority:rel-next").expect("existing-truth authority label")).expect("existing-truth authority identity"),
                seed.deltas()[0].entity_identity.clone(),
            )
            .expect("existing relation target should build")
            .in_target_collection("TaskRelation")
            .expect("existing relation target collection should build"),
        )
        .expect("relation binding should build");

    let receipt = workspace
        .compose_graph(|graph| {
            let _ = graph.insert_entity("draft-task", "Task", |task| {
                task.aspect(test_aspect_touch("identity.id"), test_string_aspect_value("task-loop-c"))
                    .aspect(test_aspect_touch("title.value"), test_string_aspect_value("Loop successor target"))
            })?;
            graph.retarget_existing(binding, |relation| {
                relation
                    .naming_rebind_target(crate::runtime::ForgeQueryMutationAuthorityIdentity::naming_attachment(crate::runtime::ForgeQueryNamingAttachmentAuthorityLabel::new("attachment:loop-next").expect("naming attachment authority label")).expect("naming attachment identity"), crate::runtime::ForgeQueryMutationAuthorityIdentity::naming_prior_authority(crate::runtime::ForgeQueryNamingPriorAuthorityLabel::new("authority:loop-b").expect("naming prior authority label")).expect("naming prior authority identity"), crate::runtime::ForgeQueryMutationAuthorityIdentity::naming_target_authority(crate::runtime::ForgeQueryNamingTargetAuthorityLabel::new("authority:loop-c").expect("naming target authority label")).expect("naming target authority identity"),
                    )
                    .continuity_rebind_existing_target(crate::runtime::ForgeQueryMutationAuthorityIdentity::continuity_prior_authority(crate::runtime::ForgeQueryContinuityPriorAuthorityLabel::new("authority:rel-next").expect("continuity prior authority label")).expect("continuity prior authority identity"), crate::runtime::ForgeQueryMutationAuthorityIdentity::continuity_successor_authority(crate::runtime::ForgeQueryContinuitySuccessorAuthorityLabel::new("authority:rel-next-successor").expect("continuity successor authority label")).expect("continuity successor authority identity"),
                    )
                    .aspect(test_aspect_touch("target.id"), test_string_aspect_value("loop-c"))
            })?;
            Ok(())
        })
        .expect("retarget program should execute");

    let program = receipt
        .graph_composition_program()
        .expect("graph composition receipt should expose program");
    let lifecycle = receipt
        .graph_composition_lifecycle_outcomes()
        .expect("graph composition receipt should expose lifecycle");
    let evidence = receipt
        .graph_composition_evidence()
        .expect("graph composition receipt should expose evidence");

    assert_eq!(
        program.steps()[1].kind(),
        ForgeQueryGraphCompositionProgramStepKind::ExistingTargetRetarget
    );
    assert_eq!(
        lifecycle.entries()[1].outcome_kind(),
        ForgeQueryGraphCompositionLifecycleOutcomeKind::RetargetedIdentityPreserved
    );
    assert_eq!(
        lifecycle.counter_snapshot(),
        "created=1;updated_identity_preserved=0;retargeted_identity_preserved=1;retired_current_truth=0;superseded_with_lineage=0;deleted_if_uncommitted=0;denied_before_execution=0"
    );
    assert_eq!(
        evidence.counter_snapshot(),
        "components=2;symbolic_entities=1;symbolic_relations=0;symbolic_resolutions=0;affected_live_views=2;affected_derived_views=0;considered_computed_views=0;created=1;updated_identity_preserved=0;retargeted_identity_preserved=1;retired_current_truth=0;superseded_with_lineage=0;deleted_if_uncommitted=0;denied_before_execution=0"
    );

    match workspace.inspect(&receipt).expect("receipt should inspect") {
        ForgeQueryInspection::BatchWriteReceipt(inspection) => {
            let component = &inspection.component_operations()[1];
            assert_eq!(
                inspection
                    .graph_composition_program()
                    .expect("inspection should expose program")
                    .steps()[1]
                    .kind(),
                ForgeQueryGraphCompositionProgramStepKind::ExistingTargetRetarget
            );
            assert_eq!(
                inspection
                    .graph_composition_lifecycle_outcomes()
                    .expect("inspection should expose lifecycle")
                    .entries()[1]
                    .outcome_kind(),
                ForgeQueryGraphCompositionLifecycleOutcomeKind::RetargetedIdentityPreserved
            );
            assert_eq!(component.family(), "update");
            assert_eq!(
                component
                    .naming_mutation_evidence()
                    .expect("retarget component should retain naming evidence")
                    .family(),
                ForgeQueryNamingMutationFamily::RebindTarget
            );
            assert_eq!(
                component
                    .continuity_mutation_evidence()
                    .expect("retarget component should retain continuity evidence")
                    .family(),
                ForgeQueryContinuityMutationFamily::RebindExistingTarget
            );
        }
        other => panic!("expected batch receipt inspection, got {other:?}"),
    }
}

#[test]
fn compose_graph_supports_verified_existing_target_retarget_lifecycle() {
    let binding = ForgeQueryExistingRelationTarget::new(
        crate::runtime::ForgeQueryMutationAuthorityIdentity::existing_truth_binding_authority(
            crate::runtime::ForgeQueryExistingTruthBindingAuthorityLabel::new("authority:rel-next")
                .expect("existing-truth authority label"),
        )
        .expect("existing-truth authority identity"),
        test_entity_identity("TaskRelation:1"),
    )
    .expect("existing relation target should build")
    .in_target_collection("TaskRelation")
    .expect("existing relation target collection should build");
    let binding = ForgeQueryExistingTruthTargetBinding::from_relation_target(binding)
        .expect("relation binding should build");
    let runtime = bridge_runtime_with_support_and_existing_truth_verification(
        verified_profile(),
        TestExistingTruthVerificationAdapter::default()
            .with_value(&binding, "source.id", test_string_aspect_value("loop-a"))
            .with_value(&binding, "target.id", test_string_aspect_value("loop-b")),
    );
    let mut workspace = runtime
        .workspace("tasks.graph-composition-verified-retarget")
        .expect("workspace should open");

    let receipt = workspace
        .compose_graph(|graph| {
            graph.retarget_existing_verified(
                binding,
                |verify| {
                    verify
                        .aspect(test_aspect_touch("source.id"), test_string_aspect_value("loop-a"))
                        .aspect(test_aspect_touch("target.id"), test_string_aspect_value("loop-b"))
                },
                |update| {
                    update
                        .naming_rebind_target(crate::runtime::ForgeQueryMutationAuthorityIdentity::naming_attachment(crate::runtime::ForgeQueryNamingAttachmentAuthorityLabel::new("attachment:loop-next").expect("naming attachment authority label")).expect("naming attachment identity"), crate::runtime::ForgeQueryMutationAuthorityIdentity::naming_prior_authority(crate::runtime::ForgeQueryNamingPriorAuthorityLabel::new("authority:loop-b").expect("naming prior authority label")).expect("naming prior authority identity"), crate::runtime::ForgeQueryMutationAuthorityIdentity::naming_target_authority(crate::runtime::ForgeQueryNamingTargetAuthorityLabel::new("authority:loop-c").expect("naming target authority label")).expect("naming target authority identity"),
                        )
                        .continuity_rebind_existing_target(crate::runtime::ForgeQueryMutationAuthorityIdentity::continuity_prior_authority(crate::runtime::ForgeQueryContinuityPriorAuthorityLabel::new("authority:rel-next").expect("continuity prior authority label")).expect("continuity prior authority identity"), crate::runtime::ForgeQueryMutationAuthorityIdentity::continuity_successor_authority(crate::runtime::ForgeQueryContinuitySuccessorAuthorityLabel::new("authority:rel-next-successor").expect("continuity successor authority label")).expect("continuity successor authority identity"),
                        )
                        .aspect(test_aspect_touch("target.id"), test_string_aspect_value("loop-c"))
                },
            )?;
            Ok(())
        })
        .expect("verified retarget program should execute");

    let program = receipt
        .graph_composition_program()
        .expect("graph composition receipt should expose program");
    let lifecycle = receipt
        .graph_composition_lifecycle_outcomes()
        .expect("graph composition receipt should expose lifecycle");

    assert_eq!(program.component_count(), 1);
    assert_eq!(
        program.steps()[0].kind(),
        ForgeQueryGraphCompositionProgramStepKind::ExistingTargetVerifiedRetarget
    );
    assert_eq!(
        lifecycle.entries()[0].outcome_kind(),
        ForgeQueryGraphCompositionLifecycleOutcomeKind::RetargetedIdentityPreserved
    );
    assert_eq!(
        lifecycle.counter_snapshot(),
        "created=0;updated_identity_preserved=0;retargeted_identity_preserved=1;retired_current_truth=0;superseded_with_lineage=0;deleted_if_uncommitted=0;denied_before_execution=0"
    );
    assert_eq!(
        receipt
            .graph_composition_evidence()
            .expect("graph composition receipt should expose evidence")
            .counter_snapshot(),
        "components=1;symbolic_entities=0;symbolic_relations=0;symbolic_resolutions=0;affected_live_views=0;affected_derived_views=0;considered_computed_views=0;created=0;updated_identity_preserved=0;retargeted_identity_preserved=1;retired_current_truth=0;superseded_with_lineage=0;deleted_if_uncommitted=0;denied_before_execution=0"
    );

    match workspace.inspect(&receipt).expect("receipt should inspect") {
        ForgeQueryInspection::BatchWriteReceipt(inspection) => {
            let component = &inspection.component_operations()[0];
            assert_eq!(
                inspection
                    .graph_composition_program()
                    .expect("inspection should expose program")
                    .steps()[0]
                    .kind(),
                ForgeQueryGraphCompositionProgramStepKind::ExistingTargetVerifiedRetarget
            );
            assert_eq!(
                inspection
                    .graph_composition_lifecycle_outcomes()
                    .expect("inspection should expose lifecycle")
                    .entries()[0]
                    .outcome_kind(),
                ForgeQueryGraphCompositionLifecycleOutcomeKind::RetargetedIdentityPreserved
            );
            assert_eq!(
                component
                    .existing_truth_assertion_evidence()
                    .expect("verified retarget should retain assertion evidence")
                    .mode(),
                ForgeQueryExistingTruthAssertionMode::BackendVerifiedAssertion
            );
            assert_eq!(
                component
                    .existing_truth_assertion_evidence()
                    .expect("verified retarget should retain assertion evidence")
                    .verification_read_set_breadth()
                    .expect("verified retarget should retain read-set breadth")
                    .counter_snapshot(),
                "target_bindings=1;asserted_aspects=2;distinct_asserted_aspect_touches=2;cleared_assertions=0"
            );
            assert_eq!(
                component
                    .naming_mutation_evidence()
                    .expect("verified retarget should retain naming evidence")
                    .family(),
                ForgeQueryNamingMutationFamily::RebindTarget
            );
        }
        other => panic!("expected batch receipt inspection, got {other:?}"),
    }
}
