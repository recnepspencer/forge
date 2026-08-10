use super::*;

#[test]
fn observation_window_normalizes_duplicate_keys() {
    let window = WorkingSetObservationWindow::new(
        PlacementObservationScopeClass::Branch,
        "branch:main",
        vec![
            "artifact:b".to_string(),
            "artifact:a".to_string(),
            "artifact:b".to_string(),
        ],
    );

    assert_eq!(
        window.observed_artifact_keys(),
        &["artifact:a".to_string(), "artifact:b".to_string()]
    );
    assert_eq!(window.scope_class(), PlacementObservationScopeClass::Branch);
    assert_eq!(window.scope_key(), "branch:main");
}

#[test]
fn residency_manifest_normalizes_lists() {
    let manifest = CanonicalResidencyManifest::new(
        vec!["b".to_string(), "a".to_string(), "a".to_string()],
        vec!["x".to_string(), "x".to_string()],
    );

    assert_eq!(
        manifest.resident_artifact_keys(),
        &["a".to_string(), "b".to_string()]
    );
    assert_eq!(manifest.in_flight_transfer_keys(), &["x".to_string()]);
}

#[test]
fn proof_accessors_preserve_construction() {
    let intent = TierTransferIntent::new(
        "artifact:1",
        TierResidenceClass::Warm,
        TierResidenceClass::Cold,
        PlacementExecutionOrigin::Background,
    );
    let replica = TransferredTierReplica::new(intent.clone(), "cold://artifact:1");
    let verified = VerifiedTierReplica::new(replica, "digest-ok");

    assert_eq!(verified.transferred_replica().intent(), &intent);
    assert_eq!(verified.verification_label(), "digest-ok");
}

#[test]
fn recall_coalescing_key_preserves_scope_shape() {
    let key = RecallCoalescingKey::new(
        PlacementArtifactFamily::SnapshotFamily,
        PlacementObservationScopeClass::ArtifactFamily,
        "family:snapshot",
    );

    assert_eq!(
        key.artifact_family(),
        PlacementArtifactFamily::SnapshotFamily
    );
    assert_eq!(
        key.scope_class(),
        PlacementObservationScopeClass::ArtifactFamily
    );
    assert_eq!(key.scope_key(), "family:snapshot");
}
