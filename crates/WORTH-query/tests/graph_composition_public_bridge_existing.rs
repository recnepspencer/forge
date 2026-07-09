use worth_foundational::facade::{CanonicalFieldPath, FieldKey};
use worth_query::facade::{
    WorthQueryContinuityMutationFamily, WorthQueryContinuityPriorAuthorityLabel,
    WorthQueryContinuitySuccessorAuthorityLabel, WorthQueryExistingRelationTarget,
    WorthQueryExistingTruthAssertionMode, WorthQueryExistingTruthBindingAuthorityLabel,
    WorthQueryExistingTruthTargetBinding, WorthQueryGraphCompositionDenialKind,
    WorthQueryGraphCompositionLifecycleOutcomeKind, WorthQueryGraphCompositionProgramStepKind,
    WorthQueryInspection, WorthQueryLiveView, WorthQueryMutationAuthorityIdentity,
    WorthQueryNamingAttachmentAuthorityLabel, WorthQueryNamingMutationFamily,
    WorthQueryNamingPriorAuthorityLabel, WorthQueryNamingTargetAuthorityLabel, WorthQueryNativeRow,
    WorthQueryRuntimeError,
};
mod support;

use support::aspect_touch as touch;
use support::public_bridge_runtime::{public_graph_support_profile, PublicBridgeRuntimeHarness};

fn public_multi_verified_relation_profile() -> worth_query::facade::WorthQueryRuntimeSupportProfile
{
    ["update_existing_verified", "delete_existing_verified"]
        .into_iter()
        .fold(
            public_graph_support_profile(),
            |profile, operation_family| {
                profile.with_bridge_backed_verification_support(
                    operation_family,
                    "direct_relation_identity",
                    true,
                    true,
                    None,
                )
            },
        )
}

fn existing_authority(label: &str) -> WorthQueryMutationAuthorityIdentity {
    WorthQueryMutationAuthorityIdentity::existing_truth_binding_authority(
        WorthQueryExistingTruthBindingAuthorityLabel::new(label)
            .expect("existing-truth authority label"),
    )
    .expect("existing-truth authority identity")
}

fn naming_attachment(label: &str) -> WorthQueryMutationAuthorityIdentity {
    WorthQueryMutationAuthorityIdentity::naming_attachment(
        WorthQueryNamingAttachmentAuthorityLabel::new(label).expect("naming attachment label"),
    )
    .expect("naming attachment identity")
}

fn naming_prior(label: &str) -> WorthQueryMutationAuthorityIdentity {
    WorthQueryMutationAuthorityIdentity::naming_prior_authority(
        WorthQueryNamingPriorAuthorityLabel::new(label).expect("naming prior label"),
    )
    .expect("naming prior identity")
}

fn naming_target(label: &str) -> WorthQueryMutationAuthorityIdentity {
    WorthQueryMutationAuthorityIdentity::naming_target_authority(
        WorthQueryNamingTargetAuthorityLabel::new(label).expect("naming target label"),
    )
    .expect("naming target identity")
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
fn graph_composition_public_bridge_supports_existing_target_retarget_lifecycle() {
    let harness = PublicBridgeRuntimeHarness::new();
    let runtime = harness.bridge_backed_runtime();
    let mut workspace = runtime
        .workspace("public.graph-composition-existing-retarget")
        .expect("runtime should open a named workspace");
    let relations: WorthQueryLiveView<WorthQueryNativeRow> = workspace
        .live_view(
            "public.graph-composition-existing-retarget-relations",
            |q| {
                q.from("TaskRelation")
                    .select([
                        worth_query::facade::AspectFieldKey::from_authoring_parts("identity", "id")
                            .unwrap(),
                        worth_query::facade::AspectFieldKey::from_authoring_parts("kind", "value")
                            .unwrap(),
                        worth_query::facade::AspectFieldKey::from_authoring_parts("source", "id")
                            .unwrap(),
                        worth_query::facade::AspectFieldKey::from_authoring_parts("target", "id")
                            .unwrap(),
                    ])
                    .order_by(
                        worth_query::facade::AspectFieldKey::from_authoring_parts("identity", "id")
                            .unwrap(),
                    )
                    .schema_basis("public-graph-composition-existing-retarget-relations")
            },
        )
        .expect("relation live view should declare");
    let seed = workspace
        .insert("TaskRelation", |relation| {
            relation
                .set_aspect(touch("identity.id"), authored_text("rel-next"))
                .set_aspect(touch("kind.value"), authored_text("loop_successor"))
                .set_aspect(touch("source.id"), authored_text("loop-a"))
                .set_aspect(touch("target.id"), authored_text("loop-b"))
        })
        .expect("seed insert should execute");
    let binding = WorthQueryExistingTruthTargetBinding::from_relation_target(
        WorthQueryExistingRelationTarget::new(
            existing_authority("authority:rel-next"),
            seed.deltas()[0].entity_identity().clone(),
        )
        .expect("existing relation target should build")
        .in_target_collection("TaskRelation")
        .expect("existing relation target collection should build"),
    )
    .expect("relation binding should build");

    let receipt = workspace
        .compose_graph(|graph| {
            let _ = graph.insert_entity("draft-task", "Task", |task| {
                task.set_aspect(touch("identity.id"), authored_text("task-loop-c"))
                    .set_aspect(touch("title.value"), authored_text("Loop successor target"))
            })?;
            graph.retarget_existing(binding, |relation| {
                relation
                    .naming_rebind_target(
                        naming_attachment("attachment:loop-next"),
                        naming_prior("authority:loop-b"),
                        naming_target("authority:loop-c"),
                    )
                    .continuity_rebind_existing_target(
                        continuity_prior("authority:rel-next"),
                        continuity_successor("authority:rel-next-successor"),
                    )
                    .set_aspect(touch("target.id"), authored_text("loop-c"))
            })?;
            Ok(())
        })
        .expect("retarget program should execute");

    assert_eq!(
        receipt
            .graph_composition_program()
            .expect("graph composition receipt should expose program")
            .steps()[1]
            .kind(),
        WorthQueryGraphCompositionProgramStepKind::ExistingTargetRetarget
    );
    assert_eq!(
        receipt
            .graph_composition_lifecycle_outcomes()
            .expect("graph composition receipt should expose lifecycle")
            .entries()[1]
            .outcome_kind(),
        WorthQueryGraphCompositionLifecycleOutcomeKind::RetargetedIdentityPreserved
    );
    assert_eq!(
        workspace.read(&relations)[0].scalar_value_at(&field_path("target.id")),
        Some(&text("loop-c"))
    );

    match workspace.inspect(&receipt).expect("receipt should inspect") {
        WorthQueryInspection::BatchWriteReceipt(inspection) => {
            let component = &inspection.component_operations()[1];
            assert_eq!(
                component
                    .naming_mutation_evidence()
                    .expect("retarget component should retain naming evidence")
                    .family(),
                WorthQueryNamingMutationFamily::RebindTarget
            );
            assert_eq!(
                component
                    .continuity_mutation_evidence()
                    .expect("retarget component should retain continuity evidence")
                    .family(),
                WorthQueryContinuityMutationFamily::RebindExistingTarget
            );
        }
        other => panic!("expected batch receipt inspection, got {other:?}"),
    }
}

#[test]
fn graph_composition_public_bridge_supports_verified_existing_followup_and_retirement() {
    let harness = PublicBridgeRuntimeHarness::new();
    let runtime = harness
        .bridge_backed_runtime_builder()
        .support_profile(public_multi_verified_relation_profile())
        .build();
    let mut workspace = runtime
        .workspace("public.graph-composition-verified-existing")
        .expect("workspace should open");
    let update_seed = workspace
        .insert("TaskRelation", |relation| {
            relation
                .set_aspect(touch("identity.id"), authored_text("rel-1"))
                .set_aspect(touch("status.value"), authored_text("active"))
        })
        .expect("update seed should execute");
    let delete_seed = workspace
        .insert("TaskRelation", |relation| {
            relation
                .set_aspect(touch("identity.id"), authored_text("rel-2"))
                .set_aspect(touch("kind.value"), authored_text("depends_on"))
        })
        .expect("delete seed should execute");
    let update_binding = WorthQueryExistingTruthTargetBinding::from_relation_target(
        WorthQueryExistingRelationTarget::new(
            existing_authority("authority:rel-1"),
            update_seed.deltas()[0].entity_identity().clone(),
        )
        .expect("existing relation target should build")
        .in_target_collection("TaskRelation")
        .expect("existing relation target collection should build"),
    )
    .expect("update relation binding should build");
    let delete_binding = WorthQueryExistingTruthTargetBinding::from_relation_target(
        WorthQueryExistingRelationTarget::new(
            existing_authority("authority:rel-2"),
            delete_seed.deltas()[0].entity_identity().clone(),
        )
        .expect("existing relation target should build")
        .in_target_collection("TaskRelation")
        .expect("existing relation target collection should build"),
    )
    .expect("delete relation binding should build");
    harness.seed_backend_authoritative_truth(
        &update_binding,
        touch("status.value"),
        text("active"),
    );
    harness.seed_backend_authoritative_truth(
        &delete_binding,
        touch("kind.value"),
        text("depends_on"),
    );

    let receipt = workspace
        .compose_graph(|graph| {
            let _ = graph.insert_entity("draft-task", "Task", |task| {
                task.set_aspect(
                    touch("identity.id"),
                    authored_text("task-verified-existing"),
                )
                .set_aspect(
                    touch("title.value"),
                    authored_text("Verified existing task"),
                )
            })?;
            graph.update_existing_verified(
                update_binding,
                |verify| verify.set_aspect(touch("status.value"), authored_text("active")),
                |update| update.set_aspect(touch("status.value"), authored_text("retired")),
            )?;
            graph.delete_existing_verified(
                delete_binding,
                |verify| verify.set_aspect(touch("kind.value"), authored_text("depends_on")),
                |delete| delete.touch(touch("kind.value")),
            )?;
            Ok(())
        })
        .expect("verified existing-target lifecycle should execute");

    assert_eq!(
        receipt
            .graph_composition_program()
            .expect("graph composition receipt should expose composition program")
            .steps()[1]
            .kind(),
        WorthQueryGraphCompositionProgramStepKind::ExistingTargetVerifiedFollowupMutation
    );
    assert_eq!(
        receipt
            .graph_composition_program()
            .expect("graph composition receipt should expose composition program")
            .steps()[2]
            .kind(),
        WorthQueryGraphCompositionProgramStepKind::ExistingTargetVerifiedRetirement
    );
    assert_eq!(receipt.write_receipts().len(), 3);

    match workspace.inspect(&receipt).expect("receipt should inspect") {
        WorthQueryInspection::BatchWriteReceipt(inspection) => {
            let update = &inspection.component_operations()[1];
            let delete = &inspection.component_operations()[2];
            assert_eq!(
                update
                    .existing_truth_assertion_evidence()
                    .expect("verified update should retain assertion evidence")
                    .mode(),
                WorthQueryExistingTruthAssertionMode::BackendVerifiedAssertion
            );
            assert_eq!(
                delete
                    .existing_truth_assertion_evidence()
                    .expect("verified delete should retain assertion evidence")
                    .mode(),
                WorthQueryExistingTruthAssertionMode::BackendVerifiedAssertion
            );
        }
        other => panic!("expected batch receipt inspection, got {other:?}"),
    }
}

#[test]
fn graph_composition_public_bridge_denies_verified_existing_mismatch() {
    let harness = PublicBridgeRuntimeHarness::new();
    let runtime = harness
        .bridge_backed_runtime_builder()
        .support_profile(public_multi_verified_relation_profile())
        .build();
    let mut workspace = runtime
        .workspace("public.graph-composition-verified-existing-mismatch")
        .expect("workspace should open");
    let update_seed = workspace
        .insert("TaskRelation", |relation| {
            relation
                .set_aspect(touch("identity.id"), authored_text("rel-mismatch"))
                .set_aspect(touch("status.value"), authored_text("active"))
        })
        .expect("update seed should execute");
    let update_binding = WorthQueryExistingTruthTargetBinding::from_relation_target(
        WorthQueryExistingRelationTarget::new(
            existing_authority("authority:rel-mismatch"),
            update_seed.deltas()[0].entity_identity().clone(),
        )
        .expect("existing relation target should build")
        .in_target_collection("TaskRelation")
        .expect("existing relation target collection should build"),
    )
    .expect("update relation binding should build");
    let seed = harness.seed_backend_authoritative_truth(
        &update_binding,
        touch("status.value"),
        text("active"),
    );

    let error = workspace
        .compose_graph(|graph| {
            graph.update_existing_verified(
                update_binding,
                |verify| verify.set_aspect(touch("status.value"), authored_text("stale")),
                |update| update.set_aspect(touch("status.value"), authored_text("retired")),
            )?;
            Ok(())
        })
        .expect_err("stale backend assertion should deny the graph program");

    assert_eq!(seed.target_collection(), "TaskRelation");
    match error {
        WorthQueryRuntimeError::GraphCompositionDenied(denial) => {
            assert_eq!(
                denial.kind(),
                WorthQueryGraphCompositionDenialKind::ExistingTargetAssertedValueMismatch
            );
            assert_eq!(denial.target_collection(), Some("TaskRelation"));
        }
        other => panic!("expected graph composition denial, got {other:?}"),
    }
}

fn authored_text(value: impl Into<String>) -> worth_query::facade::WorthQueryAuthoredAspectValue {
    worth_query::facade::WorthQueryAuthoredAspectValue::string(value)
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
