use super::{
    classify, verify_paths, verify_paths_at_stage, verify_recovery_obligation_count,
    DurabilityArtifactManifestStage, DurableArtifactKind,
};

const ALLOWED: [&str; 14] = [
    "namespace/identity",
    "namespace/mutation.lock",
    "families/records/bootstrap.catalog",
    "families/records/roots/root-0000000000000001.manifest",
    "families/records/roots/root-0000000000000001-block-0000000000000002.manifest",
    "families/records/segments/segment-0000000000000001-0000000000000002.pages",
    "families/records/segment-manifests/segment-0000000000000001-0000000000000002.manifest",
    "families/records/segment-manifests/segments-0000000000000001-block-0000000000000002.manifest",
    "families/records/extents/extent-0000000000000001-0000000000000002.data",
    "families/records/extent-manifests/extent-0000000000000001-0000000000000002.manifest",
    "families/records/free-space/free-space-0000000000000001.manifest",
    "families/records/free-space/free-space-0000000000000001-block-0000000000000002.manifest",
    "families/wal/segment-1-generation-2.wal",
    "families/checkpoint.current",
];

#[test]
fn closed_grammar_accepts_every_production_artifact_family() {
    assert_eq!(
        classify(ALLOWED[0]),
        Some(DurableArtifactKind::NamespaceIdentity)
    );
    assert_eq!(
        classify(ALLOWED[1]),
        Some(DurableArtifactKind::MutationOwnerDiagnostic)
    );
    assert_eq!(
        classify(ALLOWED[2]),
        Some(DurableArtifactKind::BootstrapCatalog)
    );
    assert_eq!(
        classify(ALLOWED[3]),
        Some(DurableArtifactKind::RootManifest)
    );
    assert_eq!(
        classify(ALLOWED[4]),
        Some(DurableArtifactKind::RootRoutingBlock)
    );
    assert_eq!(classify(ALLOWED[5]), Some(DurableArtifactKind::Segment));
    assert_eq!(
        classify(ALLOWED[6]),
        Some(DurableArtifactKind::SegmentManifest)
    );
    assert_eq!(
        classify(ALLOWED[7]),
        Some(DurableArtifactKind::SegmentMembershipBlock)
    );
    assert_eq!(classify(ALLOWED[8]), Some(DurableArtifactKind::Extent));
    assert_eq!(
        classify(ALLOWED[9]),
        Some(DurableArtifactKind::ExtentManifest)
    );
    assert_eq!(
        classify(ALLOWED[10]),
        Some(DurableArtifactKind::FreeSpaceManifest)
    );
    assert_eq!(
        classify(ALLOWED[11]),
        Some(DurableArtifactKind::FreeSpaceMembershipBlock)
    );
    assert_eq!(classify(ALLOWED[12]), Some(DurableArtifactKind::WalSegment));
    assert_eq!(classify(ALLOWED[13]), Some(DurableArtifactKind::Checkpoint));
    assert!(verify_paths(ALLOWED).is_ok());
}

#[test]
fn only_post_boundary_manifests_admit_exact_staged_candidates() {
    let checkpoint = "staging/checkpoint-0000000000000001.candidate";
    let catalog = "staging/records/bootstrap-0000000000000002.candidate";
    let recovery =
        "families/physical-work/effect-0000000000000003-0000000000000004-0000000000000005.pending";
    let paths = ALLOWED.into_iter().chain([checkpoint, catalog, recovery]);
    assert!(verify_paths_at_stage(paths.clone(), true).is_ok());
    assert!(verify_paths_at_stage(paths, false).is_err());
    assert_eq!(
        classify(recovery),
        Some(DurableArtifactKind::PhysicalWorkRecoveryObligation)
    );
}

#[test]
fn recovery_obligation_count_is_exact_at_each_stage() {
    assert!(
        verify_recovery_obligation_count(0, 0, DurabilityArtifactManifestStage::CleanBaseline,)
            .is_ok()
    );
    assert!(
        verify_recovery_obligation_count(1, 1, DurabilityArtifactManifestStage::PostBoundary,)
            .is_ok()
    );
    assert!(
        verify_recovery_obligation_count(0, 1, DurabilityArtifactManifestStage::PostBoundary,)
            .is_err()
    );
    assert!(
        verify_recovery_obligation_count(2, 1, DurabilityArtifactManifestStage::PostBoundary,)
            .is_err()
    );
}

#[test]
fn closed_grammar_rejects_forbidden_state_artifacts() {
    for forbidden in [
        "replay/records.bin",
        "oracle/decoded-expected-records.json",
        "families/records/pool.snapshot",
        "families/records/serving.heap-image",
        "fixtures/s2/frame-snapshot.bin",
    ] {
        assert_forbidden(forbidden);
    }
}

#[test]
fn closed_grammar_rejects_malformed_names_inside_allowed_directories() {
    for forbidden in [
        "families/records/roots/root-current.manifest",
        "families/records/roots/root-000000000000000G.manifest",
        "families/records/segments/segment-0000000000000001.pages",
        "families/records/extents/extent-0000000000000001-0000000000000002.bin",
        "families/records/free-space/free-space-0000000000000001.tmp",
        "families/records/bootstrap-0000000000000001.candidate",
        "families/physical-work/effect-0000000000000001-0000000000000002.pending",
        "families/physical-work/effect-0000000000000001-0000000000000002-xyz.pending",
    ] {
        assert_forbidden(forbidden);
    }
}

#[test]
fn manifest_requires_nonempty_unique_paths() {
    assert!(verify_paths(std::iter::empty()).is_err());
    let duplicate = [
        "namespace/identity",
        "families/records/bootstrap.catalog",
        "namespace/identity",
    ];
    let denial = verify_paths(duplicate).unwrap_err();
    assert!(denial.contains("duplicated artifact `namespace/identity`"));
}

fn assert_forbidden(forbidden: &str) {
    let mut paths = ALLOWED.to_vec();
    paths.push(forbidden);
    let denial = verify_paths(paths).unwrap_err();
    assert!(
        denial.contains(forbidden),
        "wrong forbidden-artifact denial: {denial}"
    );
}
