use super::{
    batch_continuity_mutation_digest, batch_existing_truth_binding_digest,
    batch_naming_mutation_digest, batch_symbolic_target_reference_digest,
};
use crate::runtime::{
    ForgeQueryContinuityMutationEvidence, ForgeQueryContinuityMutationFamily,
    ForgeQueryExistingTruthBindingEvidence, ForgeQueryNamingMutationEvidence,
    ForgeQuerySymbolicTargetReferenceEvidence,
};
use forge_runtime_bridge::facade::{
    BridgeContinuityMutationBundle, BridgeContinuityOutcomeClass, BridgeExistingTruthBindingBundle,
    BridgeNamingMutationBundle, BridgeSymbolicTargetReferenceBundle,
};

#[test]
fn existing_truth_binding_batch_digest_changes_with_authoritative_identity() {
    let left = ForgeQueryExistingTruthBindingEvidence::from_bridge(
        &BridgeExistingTruthBindingBundle::direct_entity(
            "authority:left",
            "entity:task",
            Some("Task"),
        ),
    );
    let right = ForgeQueryExistingTruthBindingEvidence::from_bridge(
        &BridgeExistingTruthBindingBundle::direct_entity(
            "authority:right",
            "entity:task",
            Some("Task"),
        ),
    );

    let left_digest =
        batch_existing_truth_binding_digest(&[Some(left)]).expect("left digest should exist");
    let right_digest =
        batch_existing_truth_binding_digest(&[Some(right)]).expect("right digest should exist");

    assert_ne!(left_digest, right_digest);
}

#[test]
fn symbolic_target_batch_digest_changes_with_symbol_identity() {
    let left = ForgeQuerySymbolicTargetReferenceEvidence::from_bridge(
        &BridgeSymbolicTargetReferenceBundle::same_batch_target(
            "draft:left",
            "entity:task",
            Some("Task"),
        ),
    );
    let right = ForgeQuerySymbolicTargetReferenceEvidence::from_bridge(
        &BridgeSymbolicTargetReferenceBundle::same_batch_target(
            "draft:right",
            "entity:task",
            Some("Task"),
        ),
    );

    let left_digest =
        batch_symbolic_target_reference_digest(&[Some(left)]).expect("left digest should exist");
    let right_digest =
        batch_symbolic_target_reference_digest(&[Some(right)]).expect("right digest should exist");

    assert_ne!(left_digest, right_digest);
}

#[test]
fn naming_batch_digest_changes_with_attachment_identity() {
    let left = ForgeQueryNamingMutationEvidence::from_bridge(
        &BridgeNamingMutationBundle::attach_new_target(
            "persistent-name:left",
            "entity:task",
            Some("Task"),
        ),
    );
    let right = ForgeQueryNamingMutationEvidence::from_bridge(
        &BridgeNamingMutationBundle::attach_new_target(
            "persistent-name:right",
            "entity:task",
            Some("Task"),
        ),
    );

    let left_digest =
        batch_naming_mutation_digest(&[Some(left)]).expect("left digest should exist");
    let right_digest =
        batch_naming_mutation_digest(&[Some(right)]).expect("right digest should exist");

    assert_ne!(left_digest, right_digest);
}

#[test]
fn split_successor_batch_digest_changes_with_successor_set() {
    let left = ForgeQueryContinuityMutationEvidence::from_bridge(
        &BridgeContinuityMutationBundle::split_existing_target(
            BridgeContinuityOutcomeClass::ContinuesAsSplitSuccessors,
            "authority:task-1",
            ["authority:task-1:a", "authority:task-1:b"],
            Some("binding:sha256:task-1"),
            Some("entity:task-1"),
            Some("Task"),
            "lineage:sha256:task-1",
            "continuity:sha256:task-1:left",
        )
        .expect("left split continuity should build"),
    );
    let right = ForgeQueryContinuityMutationEvidence::from_bridge(
        &BridgeContinuityMutationBundle::split_existing_target(
            BridgeContinuityOutcomeClass::ContinuesAsSplitSuccessors,
            "authority:task-1",
            ["authority:task-1:a", "authority:task-1:c"],
            Some("binding:sha256:task-1"),
            Some("entity:task-1"),
            Some("Task"),
            "lineage:sha256:task-1",
            "continuity:sha256:task-1:left",
        )
        .expect("right split continuity should build"),
    );

    let left_digest =
        batch_continuity_mutation_digest(&[Some(left)]).expect("left digest should exist");
    let right_digest =
        batch_continuity_mutation_digest(&[Some(right)]).expect("right digest should exist");

    assert_ne!(left_digest, right_digest);
}

#[test]
fn split_successor_batch_digest_changes_with_binding_basis() {
    let left = ForgeQueryContinuityMutationEvidence::from_bridge(
        &BridgeContinuityMutationBundle::split_existing_target(
            BridgeContinuityOutcomeClass::ContinuesAsSplitSuccessors,
            "authority:task-1",
            ["authority:task-1:a", "authority:task-1:b"],
            Some("binding:sha256:left"),
            Some("entity:task-1"),
            Some("Task"),
            "lineage:sha256:task-1",
            "continuity:sha256:task-1:left",
        )
        .expect("left split continuity should build"),
    );
    let right = ForgeQueryContinuityMutationEvidence::from_bridge(
        &BridgeContinuityMutationBundle::split_existing_target(
            BridgeContinuityOutcomeClass::ContinuesAsSplitSuccessors,
            "authority:task-1",
            ["authority:task-1:a", "authority:task-1:b"],
            Some("binding:sha256:right"),
            Some("entity:task-1"),
            Some("Task"),
            "lineage:sha256:task-1",
            "continuity:sha256:task-1:left",
        )
        .expect("right split continuity should build"),
    );

    let left_digest =
        batch_continuity_mutation_digest(&[Some(left)]).expect("left digest should exist");
    let right_digest =
        batch_continuity_mutation_digest(&[Some(right)]).expect("right digest should exist");

    assert_ne!(left_digest, right_digest);
}

#[test]
fn continuity_batch_digest_changes_with_family() {
    let rebind = ForgeQueryContinuityMutationEvidence::from_bridge(
        &BridgeContinuityMutationBundle::rebind_existing_target(
            BridgeContinuityOutcomeClass::ContinuesAsSingleSuccessor,
            "authority:task-1",
            Some("authority:task-1:successor"),
            Some("binding:sha256:task-1"),
            Some("entity:task-1"),
            Some("Task"),
            "lineage:sha256:task-1",
            "continuity:sha256:task-1",
        ),
    );
    let split = rebind
        .clone()
        .with_test_family(ForgeQueryContinuityMutationFamily::SplitExistingTarget);

    let rebind_digest =
        batch_continuity_mutation_digest(&[Some(rebind)]).expect("rebind digest should exist");
    let split_digest =
        batch_continuity_mutation_digest(&[Some(split)]).expect("split digest should exist");

    assert_ne!(rebind_digest, split_digest);
}
