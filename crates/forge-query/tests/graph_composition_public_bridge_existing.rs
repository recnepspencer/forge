use forge_query::facade::{
    ForgeQueryContinuityMutationFamily, ForgeQueryExistingRelationTarget,
    ForgeQueryExistingTruthAssertionMode, ForgeQueryGraphCompositionLifecycleOutcomeKind,
    ForgeQueryGraphCompositionProgramStepKind, ForgeQueryInspection, ForgeQueryLiveView,
    ForgeQueryNamingMutationFamily,
};
use serde_json::{json, Value};

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

#[test]
fn graph_composition_public_bridge_supports_existing_target_retarget_lifecycle() {
    let harness = PublicBridgeRuntimeHarness::new();
    let runtime = harness.runtime(public_graph_support_profile());
    let mut workspace = runtime
        .workspace("public.graph-composition-existing-retarget")
        .expect("runtime should open a named workspace");
    let relations: ForgeQueryLiveView<Value> = workspace
        .live_view(
            "public.graph-composition-existing-retarget-relations",
            |q| {
                q.from("TaskRelation")
                    .select(["identity.id", "kind.value", "source.id", "target.id"])
                    .order_by("identity.id")
                    .schema_basis("public-graph-composition-existing-retarget-relations")
            },
        )
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
    let binding = workspace
        .bind_existing_relation(
            ForgeQueryExistingRelationTarget::new(
                "authority:rel-next",
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
                task.aspect("identity.id", "task-loop-c")
                    .aspect("title.value", "Loop successor target")
            })?;
            graph.retarget_existing(binding, |relation| {
                relation
                    .naming_rebind_target(
                        "attachment:loop-next",
                        "authority:loop-b",
                        "authority:loop-c",
                    )
                    .continuity_rebind_existing_target(
                        "authority:rel-next",
                        "authority:rel-next-successor",
                    )
                    .aspect("target.id", "loop-c")
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
        workspace.read(&relations)[0].external_row()["target"]["id"].as_str(),
        Some("loop-c")
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
    let runtime = harness.runtime(public_multi_verified_relation_profile());
    let mut workspace = runtime
        .workspace("public.graph-composition-verified-existing")
        .expect("workspace should open");
    let update_seed = workspace
        .insert("TaskRelation", |relation| {
            relation
                .aspect("identity.id", "rel-1")
                .aspect("status.value", "active")
        })
        .expect("update seed should execute");
    let delete_seed = workspace
        .insert("TaskRelation", |relation| {
            relation
                .aspect("identity.id", "rel-2")
                .aspect("kind.value", "depends_on")
        })
        .expect("delete seed should execute");
    let update_binding = workspace
        .bind_existing_relation(
            ForgeQueryExistingRelationTarget::new(
                "authority:rel-1",
                update_seed.deltas()[0].entity_identity.clone(),
            )
            .expect("existing relation target should build")
            .in_target_collection("TaskRelation")
            .expect("existing relation target collection should build"),
        )
        .expect("update relation binding should build");
    let delete_binding = workspace
        .bind_existing_relation(
            ForgeQueryExistingRelationTarget::new(
                "authority:rel-2",
                delete_seed.deltas()[0].entity_identity.clone(),
            )
            .expect("existing relation target should build")
            .in_target_collection("TaskRelation")
            .expect("existing relation target collection should build"),
        )
        .expect("delete relation binding should build");
    harness.seed_existing_truth_value(&update_binding, "status.value", json!("active"));
    harness.seed_existing_truth_value(&delete_binding, "kind.value", json!("depends_on"));

    let receipt = workspace
        .compose_graph(|graph| {
            let _ = graph.insert_entity("draft-task", "Task", |task| {
                task.aspect("identity.id", "task-verified-existing")
                    .aspect("title.value", "Verified existing task")
            })?;
            graph.update_existing_verified(
                update_binding,
                |verify| verify.aspect("status.value", "active"),
                |update| update.aspect("status.value", "retired"),
            )?;
            graph.delete_existing_verified(
                delete_binding,
                |verify| verify.aspect("kind.value", "depends_on"),
                |delete| delete.touch("kind.value"),
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
