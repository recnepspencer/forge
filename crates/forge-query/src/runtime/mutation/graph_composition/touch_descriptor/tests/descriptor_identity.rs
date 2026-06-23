use crate::intent_admission::ForgeQueryAuthoritativeMutationBatchIntentSeed;
use crate::runtime::{
    ForgeQueryGraphCompositionProgramStepKind, ForgeQueryGraphTouchDescriptor,
    ForgeQueryGraphTouchDescriptorKind, ForgeQueryGraphTouchLifecycleFamily,
    ForgeQueryGraphTouchReadVerb,
};
use forge_relational::facade::identity::KindId;

use super::fixtures::{
    descriptor_for_collection, descriptor_for_relation_kind_id, descriptor_for_step_kind,
    descriptor_for_touched_paths, one_step_delete_program, one_step_update_program,
};

#[test]
fn direct_and_seed_descriptor_derivation_match() {
    let (commands, breadth, program) = one_step_delete_program(
        ForgeQueryGraphCompositionProgramStepKind::SymbolicRelationRetirement,
        "topology.edge",
        "edge",
        vec!["weight"],
    );
    let direct = ForgeQueryGraphTouchDescriptor::from_authoritative_mutation_batch(
        &program, &breadth, &commands,
    )
    .unwrap();
    let seed = ForgeQueryAuthoritativeMutationBatchIntentSeed::new(commands, breadth, program);

    let from_seed = seed.graph_touch_descriptor().unwrap();

    assert_eq!(direct.descriptor_digest(), from_seed.descriptor_digest());
    assert_eq!(
        from_seed.kind(),
        ForgeQueryGraphTouchDescriptorKind::AuthoritativeMutationBatch
    );
    assert!(from_seed.touches_collection("topology.edge"));
    assert!(from_seed.touches_aspect_path("weight"));
}

#[test]
fn lifecycle_changes_alter_descriptor_identity() {
    let update = descriptor_for_step_kind(
        ForgeQueryGraphCompositionProgramStepKind::SymbolicRelationFollowupMutation,
    );
    let retirement = descriptor_for_step_kind(
        ForgeQueryGraphCompositionProgramStepKind::SymbolicRelationRetirement,
    );

    assert_ne!(update.descriptor_digest(), retirement.descriptor_digest());
    assert_eq!(
        retirement.rows()[0].lifecycle_family(),
        Some(ForgeQueryGraphTouchLifecycleFamily::SameBatchSymbolicRelationRetirement)
    );
}

#[test]
fn relation_kind_changes_alter_descriptor_identity() {
    let relation = descriptor_for_collection("topology.edge");
    let containment = descriptor_for_collection("topology.containment");

    assert_ne!(
        relation.descriptor_digest(),
        containment.descriptor_digest()
    );
    assert!(relation.touches_collection("topology.edge"));
    assert!(!relation.touches_collection("topology.containment"));
}

#[test]
fn relation_kind_ids_are_retained_as_descriptor_identity_lanes() {
    let edge = descriptor_for_relation_kind_id(KindId(77));
    let containment = descriptor_for_relation_kind_id(KindId(88));

    assert_ne!(edge.descriptor_digest(), containment.descriptor_digest());
    assert_eq!(edge.relation_kind_count(), 1);
    assert!(edge.touches_relation_kind_id(KindId(77)));
    assert!(!edge.touches_relation_kind_id(KindId(88)));
    assert!(edge.touches_collection("topology.edge"));
    assert!(!edge.touches_collection("77"));
}

#[test]
fn explicit_touched_paths_alter_descriptor_identity() {
    let weight = descriptor_for_touched_paths(vec!["weight"]);
    let capacity = descriptor_for_touched_paths(vec!["capacity"]);

    assert_ne!(weight.descriptor_digest(), capacity.descriptor_digest());
    assert!(weight.touches_aspect_path("weight"));
    assert!(!weight.touches_aspect_path("capacity"));
}

#[test]
fn declared_aspect_operations_are_retained_separately_from_touches() {
    let (commands, breadth, program) = one_step_update_program(
        ForgeQueryGraphCompositionProgramStepKind::SymbolicRelationFollowupMutation,
        "topology.edge",
        "edge",
        "weight",
    );

    let descriptor = ForgeQueryGraphTouchDescriptor::from_authoritative_mutation_batch(
        &program, &breadth, &commands,
    )
    .unwrap();

    assert_eq!(descriptor.declared_aspect_path_count(), 1);
    assert_eq!(descriptor.declared_aspect_operation_count(), 1);
    assert_eq!(descriptor.touched_aspect_count(), 0);
    assert_eq!(descriptor.insert_command_count(), 0);
    assert_eq!(descriptor.update_command_count(), 1);
    assert_eq!(descriptor.assertion_command_count(), 0);
    assert_eq!(descriptor.delete_command_count(), 0);
    assert!(descriptor.touches_declared_aspect_operation("set:weight"));
    assert!(descriptor.touches_aspect_path("weight"));
    assert!(!descriptor.touches_aspect_path("eight"));
}

#[test]
fn delete_command_breadth_is_retained_as_descriptor_shape() {
    let descriptor = descriptor_for_touched_paths(vec!["weight"]);

    assert_eq!(descriptor.insert_command_count(), 0);
    assert_eq!(descriptor.update_command_count(), 0);
    assert_eq!(descriptor.assertion_command_count(), 0);
    assert_eq!(descriptor.delete_command_count(), 1);
}

#[test]
fn read_descriptors_use_read_vocabulary_without_mutation_lifecycle_overclaim() {
    let read = ForgeQueryGraphTouchDescriptor::read_family(
        "TaskEdge",
        [
            ForgeQueryGraphTouchReadVerb::ObservesCollection,
            ForgeQueryGraphTouchReadVerb::ExposesDerivedTopology,
        ],
    )
    .unwrap();
    let live = ForgeQueryGraphTouchDescriptor::live_read("TaskEdge").unwrap();

    assert_eq!(read.kind(), ForgeQueryGraphTouchDescriptorKind::ReadFamily);
    assert_eq!(live.kind(), ForgeQueryGraphTouchDescriptorKind::LiveRead);
    assert!(read.touches_collection("TaskEdge"));
    assert_eq!(
        read.rows()[0].read_verb(),
        Some(ForgeQueryGraphTouchReadVerb::ObservesCollection)
    );
    assert_eq!(read.rows()[0].lifecycle_family(), None);
    assert_ne!(read.descriptor_digest(), live.descriptor_digest());
}
