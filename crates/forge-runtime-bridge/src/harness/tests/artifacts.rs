use crate::facade::BridgeRouteRequest;
use crate::facade::TruthSnapshotIdentity;

use super::support::{
    build_runtime, committed_patch_items, registration, snapshot, surface_widening_registration,
};
use crate::harness::fixtures::{InMemoryRelationalBridgeSource, RecordingSignalBridgeSink};

#[test]
fn bridge_artifact_identities_are_bounded_and_stable_for_identical_patchsets() {
    let left_source = InMemoryRelationalBridgeSource::default();
    left_source.insert_committed_patch(committed_patch_items(
        crate::facade::TruthCommitIdentity::new("commit-a"),
        crate::facade::TruthPatchIdentity::new("patch-a"),
        TruthSnapshotIdentity::new("snapshot-a"),
        vec![
            crate::facade::BridgeCommittedPatchItem::with_target(
                "user",
                crate::facade::BridgeCommittedPatchTarget::entity_field_path(
                    forge_foundational::facade::AspectLocator::new(
                        forge_foundational::facade::LocatorAuthority::Authoritative,
                        forge_foundational::facade::AspectKey::new("profile")
                            .expect("valid bridge patch aspect key"),
                    ),
                    forge_foundational::facade::CanonicalFieldPath::single(
                        forge_foundational::facade::FieldKey::new("name".to_owned())
                            .expect("valid foundational field key"),
                    ),
                ),
            ),
            crate::facade::BridgeCommittedPatchItem::with_target(
                "user",
                crate::facade::BridgeCommittedPatchTarget::entity_field_path(
                    forge_foundational::facade::AspectLocator::new(
                        forge_foundational::facade::LocatorAuthority::Authoritative,
                        forge_foundational::facade::AspectKey::new("profile")
                            .expect("valid bridge patch aspect key"),
                    ),
                    forge_foundational::facade::CanonicalFieldPath::single(
                        forge_foundational::facade::FieldKey::new("avatar".to_owned())
                            .expect("valid foundational field key"),
                    ),
                ),
            ),
            crate::facade::BridgeCommittedPatchItem::with_target(
                "user",
                crate::facade::BridgeCommittedPatchTarget::entity_field_path(
                    forge_foundational::facade::AspectLocator::new(
                        forge_foundational::facade::LocatorAuthority::Authoritative,
                        forge_foundational::facade::AspectKey::new("profile")
                            .expect("valid bridge patch aspect key"),
                    ),
                    forge_foundational::facade::CanonicalFieldPath::single(
                        forge_foundational::facade::FieldKey::new("name".to_owned())
                            .expect("valid foundational field key"),
                    ),
                ),
            ),
        ],
    ));
    left_source.insert_snapshot(snapshot(TruthSnapshotIdentity::new("snapshot-a"), "alice"));
    let left_runtime = build_runtime(
        left_source,
        RecordingSignalBridgeSink::default(),
        vec![registration(), surface_widening_registration()],
    );

    let right_source = InMemoryRelationalBridgeSource::default();
    right_source.insert_committed_patch(committed_patch_items(
        crate::facade::TruthCommitIdentity::new("commit-a"),
        crate::facade::TruthPatchIdentity::new("patch-a"),
        TruthSnapshotIdentity::new("snapshot-a"),
        vec![
            crate::facade::BridgeCommittedPatchItem::with_target(
                "user",
                crate::facade::BridgeCommittedPatchTarget::entity_field_path(
                    forge_foundational::facade::AspectLocator::new(
                        forge_foundational::facade::LocatorAuthority::Authoritative,
                        forge_foundational::facade::AspectKey::new("profile")
                            .expect("valid bridge patch aspect key"),
                    ),
                    forge_foundational::facade::CanonicalFieldPath::single(
                        forge_foundational::facade::FieldKey::new("avatar".to_owned())
                            .expect("valid foundational field key"),
                    ),
                ),
            ),
            crate::facade::BridgeCommittedPatchItem::with_target(
                "user",
                crate::facade::BridgeCommittedPatchTarget::entity_field_path(
                    forge_foundational::facade::AspectLocator::new(
                        forge_foundational::facade::LocatorAuthority::Authoritative,
                        forge_foundational::facade::AspectKey::new("profile")
                            .expect("valid bridge patch aspect key"),
                    ),
                    forge_foundational::facade::CanonicalFieldPath::single(
                        forge_foundational::facade::FieldKey::new("name".to_owned())
                            .expect("valid foundational field key"),
                    ),
                ),
            ),
        ],
    ));
    right_source.insert_snapshot(snapshot(TruthSnapshotIdentity::new("snapshot-a"), "alice"));
    let right_runtime = build_runtime(
        right_source,
        RecordingSignalBridgeSink::default(),
        vec![registration(), surface_widening_registration()],
    );

    let left_route = left_runtime
        .plan_committed_patch(BridgeRouteRequest::for_commit(
            crate::facade::TruthCommitIdentity::new("commit-a"),
        ))
        .expect("bridge should plan canonical route identity");
    let right_route = right_runtime
        .plan_committed_patch(BridgeRouteRequest::for_commit(
            crate::facade::TruthCommitIdentity::new("commit-a"),
        ))
        .expect("bridge should plan canonical route identity");

    let left_result = left_runtime
        .deliver_invalidation(left_route)
        .expect("bridge should lower and deliver canonical invalidation artifact");
    let right_result = right_runtime
        .deliver_invalidation(right_route)
        .expect("bridge should lower and deliver canonical invalidation artifact");

    assert_eq!(
        left_result.routing_summary().route_identity(),
        right_result.routing_summary().route_identity()
    );
    assert_eq!(
        left_result.artifact().invalidation_identity(),
        right_result.artifact().invalidation_identity()
    );
    assert_eq!(
        left_result.artifact().snapshot_token().token_value(),
        right_result.artifact().snapshot_token().token_value()
    );

    let route_identity = left_result.routing_summary().route_identity().as_str();
    let invalidation_identity = left_result.artifact().invalidation_identity().as_str();
    let snapshot_token = left_result.artifact().snapshot_token().token_value();
    assert!(route_identity.len() < 90);
    assert!(invalidation_identity.len() < 100);
    assert!(snapshot_token.len() < 100);
}
