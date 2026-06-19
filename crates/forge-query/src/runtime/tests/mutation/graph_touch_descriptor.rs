use super::super::support::*;
use crate::intent_admission::{
    ForgeQueryAdmittedIntentPlan, ForgeQueryAuthoritativeMutationBatchExecutionPlan,
};

#[test]
fn graph_touch_descriptor_is_derived_from_real_graph_builder_and_admission() {
    let runtime = stateful_bridge_task_edge_runtime();
    let (commands, breadth, program) = mixed_graph_program();

    let review = runtime
        .review_authoritative_runtime_write_batch_with_graph_artifacts(commands, breadth, program)
        .expect("graph batch review should admit real builder artifacts");
    let plan = admitted_batch_plan(&review);
    let descriptor = plan
        .graph_touch_descriptor()
        .expect("real builder artifacts should derive graph touch descriptor");
    let handoff = runtime
        .resolve_reviewed_admitted_authoritative_write_batch_handoff(review)
        .expect("admitted graph batch should resolve to handoff");
    let handoff_descriptor = handoff
        .graph_touch_descriptor()
        .expect("handoff should preserve graph touch descriptor derivation");

    assert_eq!(
        descriptor.descriptor_digest(),
        handoff_descriptor.descriptor_digest()
    );
    assert_eq!(descriptor.component_count(), 4);
    assert_eq!(descriptor.insert_command_count(), 2);
    assert_eq!(descriptor.update_command_count(), 1);
    assert_eq!(descriptor.delete_command_count(), 1);
    assert_eq!(descriptor.declared_collection_count(), 2);
    assert_eq!(descriptor.declared_aspect_path_count(), 5);
    assert_eq!(descriptor.touched_aspect_count(), 3);
    assert!(descriptor.touches_collection("Task"));
    assert!(descriptor.touches_collection("TaskEdge"));
    assert!(descriptor.touches_aspect_path("edge.kind"));
    assert!(descriptor.touches_aspect_path("edge.source_identity"));
    assert!(descriptor.touches_aspect_path("edge.target_identity"));
    assert_eq!(
        descriptor.rows()[3].lifecycle_family(),
        Some(ForgeQueryGraphTouchLifecycleFamily::SameBatchSymbolicRelationRetirement)
    );
}

#[test]
fn graph_touch_descriptor_replay_is_stable_for_equivalent_real_graph_programs() {
    let mut runtime = stateful_bridge_task_edge_runtime();
    let first = review_descriptor(&mut runtime, mixed_graph_program());
    let second = review_descriptor(&mut runtime, mixed_graph_program());

    assert_eq!(first.descriptor_digest(), second.descriptor_digest());
}

#[test]
fn ordinary_batch_reuses_touch_vocabulary_without_graph_lifecycle_overclaim() {
    let runtime = stateful_bridge_task_runtime();
    let command = ForgeQueryWriteCommand::InsertAspects {
        collection: "Task".to_string(),
        aspects: vec![
            ForgeQueryAspectValue::new("identity.id", "task-ordinary").unwrap(),
            ForgeQueryAspectValue::new("title.value", "Ordinary task").unwrap(),
        ],
        symbolic_aspect_references: Vec::new(),
        metadata: ForgeQueryMutationMetadata::new(),
        naming_intent: None,
        continuity_intent: None,
        symbolic_target_reference: None,
    };
    let review = runtime
        .review_authoritative_runtime_write_batch(vec![command])
        .expect("ordinary batch review should admit");
    let plan = admitted_batch_plan(&review);
    let descriptor = plan
        .graph_touch_descriptor()
        .expect("ordinary batch should still derive shared touch vocabulary");

    assert_eq!(descriptor.component_count(), 0);
    assert_eq!(descriptor.rows().len(), 1);
    assert_eq!(descriptor.insert_command_count(), 1);
    assert_eq!(descriptor.update_command_count(), 0);
    assert_eq!(descriptor.delete_command_count(), 0);
    assert_eq!(descriptor.rows()[0].program_step_kind(), None);
    assert_eq!(descriptor.rows()[0].lifecycle_family(), None);
    assert!(descriptor.touches_collection("Task"));
    assert!(descriptor.touches_aspect_path("identity.id"));
    assert!(descriptor.touches_aspect_path("title.value"));
}

fn review_descriptor(
    runtime: &mut ForgeQueryRuntime,
    graph_program: (
        Vec<ForgeQueryWriteCommand>,
        ForgeQueryGraphCompositionBreadth,
        ForgeQueryGraphCompositionProgram,
    ),
) -> ForgeQueryGraphTouchDescriptor {
    let (commands, breadth, program) = graph_program;
    let review = runtime
        .review_authoritative_runtime_write_batch_with_graph_artifacts(commands, breadth, program)
        .expect("graph batch review should admit");
    admitted_batch_plan(&review)
        .graph_touch_descriptor()
        .expect("real graph program should derive descriptor")
}

fn admitted_batch_plan(
    review: &crate::intent_admission::dx::ForgeQueryRuntimeIntentAdmissionReviewData,
) -> ForgeQueryAuthoritativeMutationBatchExecutionPlan {
    match review.admitted_plan() {
        Some(ForgeQueryAdmittedIntentPlan::AuthoritativeMutationBatch(plan)) => plan.clone(),
        _ => panic!("review should produce authoritative mutation batch plan"),
    }
}

fn mixed_graph_program() -> (
    Vec<ForgeQueryWriteCommand>,
    ForgeQueryGraphCompositionBreadth,
    ForgeQueryGraphCompositionProgram,
) {
    let mut graph = ForgeQueryGraphCompositionBuilder::new();
    let task = graph
        .insert_entity("task", "Task", |entity| {
            entity
                .aspect("identity.id", "task-touch")
                .aspect("title.value", "Draft")
        })
        .unwrap();
    let edge = graph
        .insert_symbolic_relation("edge", "TaskEdge", |relation| {
            relation
                .aspect("edge.kind", "depends_on")
                .symbolic_entity_identity("edge.source_identity", &task)
                .existing_entity_identity(
                    "edge.target_identity",
                    test_entity_identity("task-existing"),
                )
        })
        .unwrap();
    graph
        .update_entity(&task, |entity| entity.aspect("title.value", "Published"))
        .unwrap();
    graph
        .delete_relation(&edge, |delete| {
            delete.touches(["edge.kind", "edge.source_identity", "edge.target_identity"])
        })
        .unwrap();
    graph.finish().unwrap()
}
