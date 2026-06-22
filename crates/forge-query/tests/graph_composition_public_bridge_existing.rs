use forge_foundational::facade::{AspectKey, AspectValue, CanonicalFieldPath, FieldKey};
use forge_query::facade::{
    ForgeQueryAspectTouch, ForgeQueryContinuityMutationFamily,
    ForgeQueryContinuityPriorAuthorityLabel, ForgeQueryContinuitySuccessorAuthorityLabel,
    ForgeQueryExistingRelationTarget, ForgeQueryExistingTruthAssertionMode,
    ForgeQueryExistingTruthBindingAuthorityLabel, ForgeQueryExistingTruthTargetBinding,
    ForgeQueryGraphCompositionDenialKind, ForgeQueryGraphCompositionLifecycleOutcomeKind,
    ForgeQueryGraphCompositionProgramStepKind, ForgeQueryInspection, ForgeQueryLiveView,
    ForgeQueryMutationAuthorityIdentity, ForgeQueryNamingAttachmentAuthorityLabel,
    ForgeQueryNamingMutationFamily, ForgeQueryNamingPriorAuthorityLabel,
    ForgeQueryNamingTargetAuthorityLabel, ForgeQueryNativeRow, ForgeQueryRuntimeError,
};
mod support;

use support::public_bridge_runtime::{public_graph_support_profile, PublicBridgeRuntimeHarness};

fn public_multi_verified_relation_profile() -> forge_query::facade::ForgeQueryRuntimeSupportProfile
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

fn existing_authority(label: &str) -> ForgeQueryMutationAuthorityIdentity {
    ForgeQueryMutationAuthorityIdentity::existing_truth_binding_authority(
        ForgeQueryExistingTruthBindingAuthorityLabel::new(label)
            .expect("existing-truth authority label"),
    )
    .expect("existing-truth authority identity")
}

fn naming_attachment(label: &str) -> ForgeQueryMutationAuthorityIdentity {
    ForgeQueryMutationAuthorityIdentity::naming_attachment(
        ForgeQueryNamingAttachmentAuthorityLabel::new(label).expect("naming attachment label"),
    )
    .expect("naming attachment identity")
}

fn naming_prior(label: &str) -> ForgeQueryMutationAuthorityIdentity {
    ForgeQueryMutationAuthorityIdentity::naming_prior_authority(
        ForgeQueryNamingPriorAuthorityLabel::new(label).expect("naming prior label"),
    )
    .expect("naming prior identity")
}

fn naming_target(label: &str) -> ForgeQueryMutationAuthorityIdentity {
    ForgeQueryMutationAuthorityIdentity::naming_target_authority(
        ForgeQueryNamingTargetAuthorityLabel::new(label).expect("naming target label"),
    )
    .expect("naming target identity")
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
fn graph_composition_public_bridge_supports_existing_target_retarget_lifecycle() {
    let harness = PublicBridgeRuntimeHarness::new();
    let runtime = harness.bridge_backed_runtime();
    let mut workspace = runtime
        .workspace("public.graph-composition-existing-retarget")
        .expect("runtime should open a named workspace");
    let relations: ForgeQueryLiveView<ForgeQueryNativeRow> = workspace
        .live_view(
            "public.graph-composition-existing-retarget-relations",
            |q| {
                q.from("TaskRelation")
                    .select([
                        forge_query::facade::AspectFieldKey::new("identity", "id").unwrap(),
                        forge_query::facade::AspectFieldKey::new("kind", "value").unwrap(),
                        forge_query::facade::AspectFieldKey::new("source", "id").unwrap(),
                        forge_query::facade::AspectFieldKey::new("target", "id").unwrap(),
                    ])
                    .order_by(forge_query::facade::AspectFieldKey::new("identity", "id").unwrap())
                    .schema_basis("public-graph-composition-existing-retarget-relations")
            },
        )
        .expect("relation live view should declare");
    let seed = workspace
        .insert("TaskRelation", |relation| {
            relation
                .aspect(touch("identity.id"), text("rel-next"))
                .aspect(touch("kind.value"), text("loop_successor"))
                .aspect(touch("source.id"), text("loop-a"))
                .aspect(touch("target.id"), text("loop-b"))
        })
        .expect("seed insert should execute");
    let binding = ForgeQueryExistingTruthTargetBinding::from_relation_target(
        ForgeQueryExistingRelationTarget::new(
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
                task.aspect(touch("identity.id"), text("task-loop-c"))
                    .aspect(touch("title.value"), text("Loop successor target"))
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
                    .aspect(touch("target.id"), text("loop-c"))
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
        ForgeQueryGraphCompositionProgramStepKind::ExistingTargetRetarget
    );
    assert_eq!(
        receipt
            .graph_composition_lifecycle_outcomes()
            .expect("graph composition receipt should expose lifecycle")
            .entries()[1]
            .outcome_kind(),
        ForgeQueryGraphCompositionLifecycleOutcomeKind::RetargetedIdentityPreserved
    );
    assert_eq!(
        workspace.read(&relations)[0].scalar_value_at(&field_path("target.id")),
        Some(&text("loop-c"))
    );

    match workspace.inspect(&receipt).expect("receipt should inspect") {
        ForgeQueryInspection::BatchWriteReceipt(inspection) => {
            let component = &inspection.component_operations()[1];
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
                .aspect(touch("identity.id"), text("rel-1"))
                .aspect(touch("status.value"), text("active"))
        })
        .expect("update seed should execute");
    let delete_seed = workspace
        .insert("TaskRelation", |relation| {
            relation
                .aspect(touch("identity.id"), text("rel-2"))
                .aspect(touch("kind.value"), text("depends_on"))
        })
        .expect("delete seed should execute");
    let update_binding = ForgeQueryExistingTruthTargetBinding::from_relation_target(
        ForgeQueryExistingRelationTarget::new(
            existing_authority("authority:rel-1"),
            update_seed.deltas()[0].entity_identity().clone(),
        )
        .expect("existing relation target should build")
        .in_target_collection("TaskRelation")
        .expect("existing relation target collection should build"),
    )
    .expect("update relation binding should build");
    let delete_binding = ForgeQueryExistingTruthTargetBinding::from_relation_target(
        ForgeQueryExistingRelationTarget::new(
            existing_authority("authority:rel-2"),
            delete_seed.deltas()[0].entity_identity().clone(),
        )
        .expect("existing relation target should build")
        .in_target_collection("TaskRelation")
        .expect("existing relation target collection should build"),
    )
    .expect("delete relation binding should build");
    harness.seed_backend_authoritative_truth(&update_binding, "status.value", text("active"));
    harness.seed_backend_authoritative_truth(&delete_binding, "kind.value", text("depends_on"));

    let receipt = workspace
        .compose_graph(|graph| {
            let _ = graph.insert_entity("draft-task", "Task", |task| {
                task.aspect(touch("identity.id"), text("task-verified-existing"))
                    .aspect(touch("title.value"), text("Verified existing task"))
            })?;
            graph.update_existing_verified(
                update_binding,
                |verify| verify.aspect(touch("status.value"), text("active")),
                |update| update.aspect(touch("status.value"), text("retired")),
            )?;
            graph.delete_existing_verified(
                delete_binding,
                |verify| verify.aspect(touch("kind.value"), text("depends_on")),
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
        ForgeQueryGraphCompositionProgramStepKind::ExistingTargetVerifiedFollowupMutation
    );
    assert_eq!(
        receipt
            .graph_composition_program()
            .expect("graph composition receipt should expose composition program")
            .steps()[2]
            .kind(),
        ForgeQueryGraphCompositionProgramStepKind::ExistingTargetVerifiedRetirement
    );
    assert_eq!(receipt.write_receipts().len(), 3);

    match workspace.inspect(&receipt).expect("receipt should inspect") {
        ForgeQueryInspection::BatchWriteReceipt(inspection) => {
            let update = &inspection.component_operations()[1];
            let delete = &inspection.component_operations()[2];
            assert_eq!(
                update
                    .existing_truth_assertion_evidence()
                    .expect("verified update should retain assertion evidence")
                    .mode(),
                ForgeQueryExistingTruthAssertionMode::BackendVerifiedAssertion
            );
            assert_eq!(
                delete
                    .existing_truth_assertion_evidence()
                    .expect("verified delete should retain assertion evidence")
                    .mode(),
                ForgeQueryExistingTruthAssertionMode::BackendVerifiedAssertion
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
                .aspect(touch("identity.id"), text("rel-mismatch"))
                .aspect(touch("status.value"), text("active"))
        })
        .expect("update seed should execute");
    let update_binding = ForgeQueryExistingTruthTargetBinding::from_relation_target(
        ForgeQueryExistingRelationTarget::new(
            existing_authority("authority:rel-mismatch"),
            update_seed.deltas()[0].entity_identity().clone(),
        )
        .expect("existing relation target should build")
        .in_target_collection("TaskRelation")
        .expect("existing relation target collection should build"),
    )
    .expect("update relation binding should build");
    let seed =
        harness.seed_backend_authoritative_truth(&update_binding, "status.value", text("active"));

    let error = workspace
        .compose_graph(|graph| {
            graph.update_existing_verified(
                update_binding,
                |verify| verify.aspect(touch("status.value"), text("stale")),
                |update| update.aspect(touch("status.value"), text("retired")),
            )?;
            Ok(())
        })
        .expect_err("stale backend assertion should deny the graph program");

    assert_eq!(seed.target_collection(), "TaskRelation");
    match error {
        ForgeQueryRuntimeError::GraphCompositionDenied(denial) => {
            assert_eq!(
                denial.kind(),
                ForgeQueryGraphCompositionDenialKind::ExistingTargetAssertedValueMismatch
            );
            assert_eq!(denial.target_collection(), Some("TaskRelation"));
        }
        other => panic!("expected graph composition denial, got {other:?}"),
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
