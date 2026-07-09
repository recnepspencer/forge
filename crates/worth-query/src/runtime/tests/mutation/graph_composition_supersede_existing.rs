use super::super::support::*;

fn task_relation_runtime() -> WorthQueryRuntime {
    stateful_bridge_task_relation_runtime()
}

fn verified_profile() -> WorthQueryRuntimeSupportProfile {
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
fn compose_graph_supports_existing_target_supersession_lifecycle() {
    let mut workspace = task_relation_runtime()
        .workspace("tasks.graph-composition-supersede-existing")
        .expect("runtime should open a named workspace");
    let _: WorthQueryLiveView<WorthQueryNativeRow> = workspace
        .live_view("tasks.graph-composition-supersede-existing-tasks", |q| {
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
                .schema_basis("tasks-graph-composition-supersede-existing-tasks")
        })
        .expect("task live view should declare");
    let _: WorthQueryLiveView<WorthQueryNativeRow> = workspace
        .live_view(
            "tasks.graph-composition-supersede-existing-relations",
            |q| {
                q.from("TaskRelation")
                    .select([
                        crate::authoring::AspectFieldKey::from_authoring_parts("identity", "id")
                            .unwrap(),
                        crate::authoring::AspectFieldKey::from_authoring_parts("kind", "value")
                            .unwrap(),
                        crate::authoring::AspectFieldKey::from_authoring_parts("source", "id")
                            .unwrap(),
                        crate::authoring::AspectFieldKey::from_authoring_parts("target", "id")
                            .unwrap(),
                    ])
                    .order_by(
                        crate::authoring::AspectFieldKey::from_authoring_parts("identity", "id")
                            .unwrap(),
                    )
                    .schema_basis("tasks-graph-composition-supersede-existing-relations")
            },
        )
        .expect("relation live view should declare");

    let seed = workspace
        .insert("TaskRelation", |relation| {
            relation
                .set_aspect(
                    test_aspect_touch("identity.id"),
                    test_authored_string_aspect_value("rel-edge"),
                )
                .set_aspect(
                    test_aspect_touch("kind.value"),
                    test_authored_string_aspect_value("edge"),
                )
                .set_aspect(
                    test_aspect_touch("source.id"),
                    test_authored_string_aspect_value("vertex-a"),
                )
                .set_aspect(
                    test_aspect_touch("target.id"),
                    test_authored_string_aspect_value("vertex-b"),
                )
        })
        .expect("seed insert should execute");
    let binding = workspace
        .bind_existing_relation(
            WorthQueryExistingRelationTarget::new(
                crate::runtime::WorthQueryMutationAuthorityIdentity::existing_truth_binding_authority(crate::runtime::WorthQueryExistingTruthBindingAuthorityLabel::new("authority:rel-edge").expect("existing-truth authority label")).expect("existing-truth authority identity"),
                seed.deltas()[0].entity_identity.clone(),
            )
            .expect("existing relation target should build")
            .in_target_collection("TaskRelation")
            .expect("existing relation target collection should build"),
        )
        .expect("relation binding should build");

    let receipt = workspace
        .compose_graph(|graph| {
            let _ = graph.insert_entity("draft-vertex", "Task", |task| {
                task.set_aspect(test_aspect_touch("identity.id"), test_authored_string_aspect_value("vertex-split"))
                    .set_aspect(test_aspect_touch("title.value"), test_authored_string_aspect_value("Split vertex"))
            })?;
            graph.supersede_existing(binding, |relation| {
                relation
                    .continuity_split_successors(
                        crate::runtime::WorthQueryMutationAuthorityIdentity::continuity_prior_authority(crate::runtime::WorthQueryContinuityPriorAuthorityLabel::new(
                            "authority:rel-edge",
                        )
                        .expect("continuity prior authority label")).expect("continuity prior authority identity"),
                        [
                            crate::runtime::WorthQueryMutationAuthorityIdentity::continuity_successor_authority(crate::runtime::WorthQueryContinuitySuccessorAuthorityLabel::new(
                                "authority:rel-edge-left",
                            )
                            .expect("continuity successor authority label")).expect("continuity successor authority identity"),
                            crate::runtime::WorthQueryMutationAuthorityIdentity::continuity_successor_authority(crate::runtime::WorthQueryContinuitySuccessorAuthorityLabel::new(
                                "authority:rel-edge-right",
                            )
                            .expect("continuity successor authority label")).expect("continuity successor authority identity"),
                        ],
                    )
                    .set_aspect(test_aspect_touch("target.id"), test_authored_string_aspect_value("vertex-split"))
            })?;
            Ok(())
        })
        .expect("supersession program should execute");

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
        WorthQueryGraphCompositionProgramStepKind::ExistingTargetSupersession
    );
    assert_eq!(
        lifecycle.entries()[1].outcome_kind(),
        WorthQueryGraphCompositionLifecycleOutcomeKind::SupersededWithLineage
    );
    assert_eq!(
        lifecycle.counter_snapshot(),
        "created=1;updated_identity_preserved=0;retargeted_identity_preserved=0;retired_current_truth=0;superseded_with_lineage=1;deleted_if_uncommitted=0;denied_before_execution=0"
    );
    assert_eq!(
        evidence.counter_snapshot(),
        "components=2;symbolic_entities=1;symbolic_relations=0;symbolic_resolutions=0;affected_live_views=2;affected_derived_views=0;considered_computed_views=0;created=1;updated_identity_preserved=0;retargeted_identity_preserved=0;retired_current_truth=0;superseded_with_lineage=1;deleted_if_uncommitted=0;denied_before_execution=0"
    );

    match workspace.inspect(&receipt).expect("receipt should inspect") {
        WorthQueryInspection::BatchWriteReceipt(inspection) => {
            let component = &inspection.component_operations()[1];
            assert_eq!(
                inspection
                    .graph_composition_program()
                    .expect("inspection should expose program")
                    .steps()[1]
                    .kind(),
                WorthQueryGraphCompositionProgramStepKind::ExistingTargetSupersession
            );
            assert_eq!(
                inspection
                    .graph_composition_lifecycle_outcomes()
                    .expect("inspection should expose lifecycle")
                    .entries()[1]
                    .outcome_kind(),
                WorthQueryGraphCompositionLifecycleOutcomeKind::SupersededWithLineage
            );
            assert_eq!(
                component
                    .continuity_mutation_evidence()
                    .expect("supersession component should retain continuity evidence")
                    .outcome_class(),
                WorthQueryContinuityOutcomeClass::ContinuesAsSplitSuccessors
            );
        }
        other => panic!("expected batch receipt inspection, got {other:?}"),
    }
}

#[test]
fn compose_graph_supports_verified_existing_target_supersession_lifecycle() {
    let binding = WorthQueryExistingRelationTarget::new(
        crate::runtime::WorthQueryMutationAuthorityIdentity::existing_truth_binding_authority(
            crate::runtime::WorthQueryExistingTruthBindingAuthorityLabel::new("authority:rel-edge")
                .expect("existing-truth authority label"),
        )
        .expect("existing-truth authority identity"),
        test_entity_identity("TaskRelation:1"),
    )
    .expect("existing relation target should build")
    .in_target_collection("TaskRelation")
    .expect("existing relation target collection should build");
    let binding = WorthQueryExistingTruthTargetBinding::from_relation_target(binding)
        .expect("relation binding should build");
    let runtime = bridge_runtime_with_support_and_existing_truth_verification(
        verified_profile(),
        TestExistingTruthVerificationAdapter::default()
            .with_value(&binding, "source.id", test_string_aspect_value("vertex-a"))
            .with_value(&binding, "target.id", test_string_aspect_value("vertex-b")),
    );
    let mut workspace = runtime
        .workspace("tasks.graph-composition-verified-supersede")
        .expect("workspace should open");

    let receipt = workspace
        .compose_graph(|graph| {
            graph.supersede_existing_verified(
                binding,
                |verify| {
                    verify
                        .set_aspect(test_aspect_touch("source.id"), test_authored_string_aspect_value("vertex-a"))
                        .set_aspect(test_aspect_touch("target.id"), test_authored_string_aspect_value("vertex-b"))
                },
                |update| {
                    update
                        .continuity_rebind_merge_successor(crate::runtime::WorthQueryMutationAuthorityIdentity::continuity_prior_authority(crate::runtime::WorthQueryContinuityPriorAuthorityLabel::new("authority:rel-edge").expect("continuity prior authority label")).expect("continuity prior authority identity"), crate::runtime::WorthQueryMutationAuthorityIdentity::continuity_successor_authority(crate::runtime::WorthQueryContinuitySuccessorAuthorityLabel::new("authority:rel-edge-merged").expect("continuity successor authority label")).expect("continuity successor authority identity"),
                        )
                        .set_aspect(test_aspect_touch("target.id"), test_authored_string_aspect_value("vertex-merged"))
                },
            )?;
            Ok(())
        })
        .expect("verified supersession program should execute");

    let program = receipt
        .graph_composition_program()
        .expect("graph composition receipt should expose program");
    let lifecycle = receipt
        .graph_composition_lifecycle_outcomes()
        .expect("graph composition receipt should expose lifecycle");

    assert_eq!(
        program.steps()[0].kind(),
        WorthQueryGraphCompositionProgramStepKind::ExistingTargetVerifiedSupersession
    );
    assert_eq!(
        lifecycle.entries()[0].outcome_kind(),
        WorthQueryGraphCompositionLifecycleOutcomeKind::SupersededWithLineage
    );
    assert_eq!(
        lifecycle.counter_snapshot(),
        "created=0;updated_identity_preserved=0;retargeted_identity_preserved=0;retired_current_truth=0;superseded_with_lineage=1;deleted_if_uncommitted=0;denied_before_execution=0"
    );
    assert_eq!(
        receipt
            .graph_composition_evidence()
            .expect("graph composition receipt should expose evidence")
            .counter_snapshot(),
        "components=1;symbolic_entities=0;symbolic_relations=0;symbolic_resolutions=0;affected_live_views=0;affected_derived_views=0;considered_computed_views=0;created=0;updated_identity_preserved=0;retargeted_identity_preserved=0;retired_current_truth=0;superseded_with_lineage=1;deleted_if_uncommitted=0;denied_before_execution=0"
    );

    match workspace.inspect(&receipt).expect("receipt should inspect") {
        WorthQueryInspection::BatchWriteReceipt(inspection) => {
            let component = &inspection.component_operations()[0];
            assert_eq!(
                component
                    .existing_truth_assertion_evidence()
                    .expect("verified supersession should retain assertion evidence")
                    .mode(),
                WorthQueryExistingTruthAssertionMode::BackendVerifiedAssertion
            );
            assert_eq!(
                component
                    .continuity_mutation_evidence()
                    .expect("verified supersession should retain continuity evidence")
                    .outcome_class(),
                WorthQueryContinuityOutcomeClass::ContinuesViaTruthLoweredCanonicalMergeSuccessor
            );
        }
        other => panic!("expected batch receipt inspection, got {other:?}"),
    }
}

#[test]
fn compose_graph_denies_existing_target_supersession_without_lineage_intent() {
    let mut workspace = task_relation_runtime()
        .workspace("tasks.graph-composition-supersede-denial")
        .expect("runtime should open a named workspace");
    let _: WorthQueryLiveView<WorthQueryNativeRow> = workspace
        .live_view("tasks.graph-composition-supersede-denial-relations", |q| {
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
                .schema_basis("tasks-graph-composition-supersede-denial-relations")
        })
        .expect("relation live view should declare");
    let seed = workspace
        .insert("TaskRelation", |relation| {
            relation
                .set_aspect(
                    test_aspect_touch("identity.id"),
                    test_authored_string_aspect_value("rel-edge"),
                )
                .set_aspect(
                    test_aspect_touch("kind.value"),
                    test_authored_string_aspect_value("edge"),
                )
                .set_aspect(
                    test_aspect_touch("source.id"),
                    test_authored_string_aspect_value("vertex-a"),
                )
                .set_aspect(
                    test_aspect_touch("target.id"),
                    test_authored_string_aspect_value("vertex-b"),
                )
        })
        .expect("seed insert should execute");
    let binding = workspace
        .bind_existing_relation(
            WorthQueryExistingRelationTarget::new(
                crate::runtime::WorthQueryMutationAuthorityIdentity::existing_truth_binding_authority(crate::runtime::WorthQueryExistingTruthBindingAuthorityLabel::new("authority:rel-edge").expect("existing-truth authority label")).expect("existing-truth authority identity"),
                seed.deltas()[0].entity_identity.clone(),
            )
            .expect("existing relation target should build")
            .in_target_collection("TaskRelation")
            .expect("existing relation target collection should build"),
        )
        .expect("relation binding should build");

    let error = workspace
        .compose_graph(|graph| {
            graph.supersede_existing(binding, |relation| {
                relation
                    .continuity_rebind_existing_target(crate::runtime::WorthQueryMutationAuthorityIdentity::continuity_prior_authority(crate::runtime::WorthQueryContinuityPriorAuthorityLabel::new("authority:rel-edge").expect("continuity prior authority label")).expect("continuity prior authority identity"), crate::runtime::WorthQueryMutationAuthorityIdentity::continuity_successor_authority(crate::runtime::WorthQueryContinuitySuccessorAuthorityLabel::new("authority:rel-edge-successor").expect("continuity successor authority label")).expect("continuity successor authority identity"),
                    )
                    .set_aspect(test_aspect_touch("target.id"), test_authored_string_aspect_value("vertex-c"))
            })?;
            Ok(())
        })
        .expect_err("single-successor continuity should not impersonate supersession");

    match error {
        WorthQueryRuntimeError::GraphCompositionDenied(denial) => {
            assert_eq!(
                denial.kind(),
                WorthQueryGraphCompositionDenialKind::ExistingTargetSupersessionUnsupported
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
