use super::super::batch_digest_helpers::{
    batch_continuity_mutation_digest, batch_existing_truth_binding_digest,
    batch_naming_mutation_digest, batch_symbolic_target_reference_digest,
};
use crate::runtime::{
    ForgeQueryContinuityMutationEvidence, ForgeQueryContinuityMutationFamily,
    ForgeQueryExistingTruthAssertionEvidence, ForgeQueryExistingTruthBindingEvidence,
    ForgeQueryMutationFamily, ForgeQueryNamingMutationEvidence,
    ForgeQuerySymbolicTargetReferenceEvidence,
};
use forge_runtime_bridge::facade::{
    BridgeContinuityAuthoritativeIdentity, BridgeContinuityMutationBundle,
    BridgeContinuityOutcomeClass, BridgeContinuityResolvedTargetIdentity,
    BridgeContinuityTargetCollection, BridgeExistingTruthBindingBundle, BridgeNamingMutationBundle,
    BridgeSymbolicTargetReferenceBundle,
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
            continuity_identity("authority:task-1"),
            [
                continuity_identity("authority:task-1:a"),
                continuity_identity("authority:task-1:b"),
            ],
            Some(resolved_target("entity:task-1")),
            Some(target_collection("Task")),
        )
        .expect("left split continuity should build"),
    );
    let right = ForgeQueryContinuityMutationEvidence::from_bridge(
        &BridgeContinuityMutationBundle::split_existing_target(
            BridgeContinuityOutcomeClass::ContinuesAsSplitSuccessors,
            continuity_identity("authority:task-1"),
            [
                continuity_identity("authority:task-1:a"),
                continuity_identity("authority:task-1:c"),
            ],
            Some(resolved_target("entity:task-1")),
            Some(target_collection("Task")),
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
fn split_successor_batch_digest_changes_with_resolved_target_identity() {
    let left = ForgeQueryContinuityMutationEvidence::from_bridge(
        &BridgeContinuityMutationBundle::split_existing_target(
            BridgeContinuityOutcomeClass::ContinuesAsSplitSuccessors,
            continuity_identity("authority:task-1"),
            [
                continuity_identity("authority:task-1:a"),
                continuity_identity("authority:task-1:b"),
            ],
            Some(resolved_target("entity:task-1:left")),
            Some(target_collection("Task")),
        )
        .expect("left split continuity should build"),
    );
    let right = ForgeQueryContinuityMutationEvidence::from_bridge(
        &BridgeContinuityMutationBundle::split_existing_target(
            BridgeContinuityOutcomeClass::ContinuesAsSplitSuccessors,
            continuity_identity("authority:task-1"),
            [
                continuity_identity("authority:task-1:a"),
                continuity_identity("authority:task-1:b"),
            ],
            Some(resolved_target("entity:task-1:right")),
            Some(target_collection("Task")),
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
            continuity_identity("authority:task-1"),
            Some(continuity_identity("authority:task-1:successor")),
            Some(resolved_target("entity:task-1")),
            Some(target_collection("Task")),
        )
        .expect("rebind continuity should build"),
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

fn continuity_identity(value: &str) -> BridgeContinuityAuthoritativeIdentity {
    BridgeContinuityAuthoritativeIdentity::new(value)
        .expect("test continuity identity should be native")
}

fn resolved_target(value: &str) -> BridgeContinuityResolvedTargetIdentity {
    BridgeContinuityResolvedTargetIdentity::new(value)
        .expect("test resolved target should be native")
}

fn target_collection(value: &str) -> BridgeContinuityTargetCollection {
    BridgeContinuityTargetCollection::new(value).expect("test target collection should be native")
}

#[test]
fn existing_truth_mode_summary_digest_changes_with_mutation_family() {
    let backend_verified = ForgeQueryExistingTruthAssertionEvidence::backend_verified(
        &crate::runtime::ForgeQueryVerifiedExistingTruthAssertion::new(
            &crate::runtime::ForgeQueryExistingTruthTargetBinding::direct_entity(
                "authority:task-1",
                "Task:1",
            )
            .expect("binding should build"),
            &[crate::runtime::ForgeQueryAspectValue::new(
                "title.value",
                serde_json::json!("Seed title"),
            )
            .expect("aspect should build")],
            "snapshot:test",
        )
        .expect("verified assertion should build"),
    );

    let update = super::summarize_existing_truth_modes(
        &[ForgeQueryMutationFamily::Update],
        &[Some(backend_verified.clone())],
    );
    let delete = super::summarize_existing_truth_modes(
        &[ForgeQueryMutationFamily::Delete],
        &[Some(backend_verified)],
    );

    assert_ne!(update.4, delete.4);
}

#[test]
fn existing_truth_mode_summary_digest_changes_with_assertion_mode() {
    let retained = ForgeQueryExistingTruthAssertionEvidence::retained_assertion(
        1,
        "retained-assertion-digest",
    );
    let backend_verified = ForgeQueryExistingTruthAssertionEvidence::backend_verified(
        &crate::runtime::ForgeQueryVerifiedExistingTruthAssertion::new(
            &crate::runtime::ForgeQueryExistingTruthTargetBinding::direct_entity(
                "authority:task-1",
                "Task:1",
            )
            .expect("binding should build"),
            &[crate::runtime::ForgeQueryAspectValue::new(
                "title.value",
                serde_json::json!("Seed title"),
            )
            .expect("aspect should build")],
            "snapshot:test",
        )
        .expect("verified assertion should build"),
    );

    let retained_summary = super::summarize_existing_truth_modes(
        &[ForgeQueryMutationFamily::Assertion],
        &[Some(retained)],
    );
    let verified_summary = super::summarize_existing_truth_modes(
        &[ForgeQueryMutationFamily::Assertion],
        &[Some(backend_verified)],
    );

    assert_ne!(retained_summary.4, verified_summary.4);
}

#[test]
#[should_panic(expected = "invalid existing-truth assertion mode")]
fn existing_truth_mode_summary_panics_on_invalid_family_mode_pair() {
    let retained = ForgeQueryExistingTruthAssertionEvidence::retained_assertion(
        1,
        "retained-assertion-digest",
    );

    let _ = super::summarize_existing_truth_modes(
        &[ForgeQueryMutationFamily::Update],
        &[Some(retained)],
    );
}
