use super::test_support::{
    export_lane, export_readiness, offline_declaration_digest, offline_declarations,
    offline_digest_evidence, ordered_multichunk_exported_chunks,
};
use super::{
    reject_copied_export_row_as_blob_export_bundle,
    reject_placement_only_evidence_as_blob_export_bundle,
    reject_terminal_projection_row_as_blob_export_bundle, BlobExportAuthority,
    BlobExportBundleDenial, BlobExportIntent,
};
use crate::test_support::current_authority;
use crate::BlobChunkByteWindow;
use worth_foundational::{compare_canonical_exports, CanonicalExportComparisonOutcome};
use worth_store_offline_verifier::{
    inspect_offline_export_bundle, OfflineExportBundleObservationDenial,
    OfflineExportChunkDeclaration, OfflineExportDigestEvidence,
};

#[test]
fn equivalent_current_evidence_produces_same_export_bundle_and_digest() {
    let authority = BlobExportAuthority::from_current_store_authority(current_authority(
        "phase19.export",
        "export-authority",
    ));
    let lane = export_lane(&authority, "phase19.export", b"aaaabbbbcccc", 12);
    let left_observation = lane.observe();
    let left_bundle = authority
        .publish_export_bundle(
            BlobExportIntent::for_current_lifecycle(
                left_observation.lifecycle_receipt(),
                &lane.publication,
                &lane.reachability,
                &lane.placement,
                &export_readiness("phase19.export"),
            )
            .with_export_name("tenant/blob/gen")
            .with_exported_chunks(lane.exported.clone()),
        )
        .expect("left export should publish");
    let right_observation = lane.observe();
    let right_bundle = authority
        .publish_export_bundle(
            BlobExportIntent::for_current_lifecycle(
                right_observation.lifecycle_receipt(),
                &lane.publication,
                &lane.reachability,
                &lane.placement,
                &export_readiness("phase19.export"),
            )
            .with_export_name("tenant/blob/gen")
            .with_exported_chunks(lane.exported.clone()),
        )
        .expect("right export should publish");

    assert_eq!(
        left_bundle.digest_evidence().logical_content_digest(),
        right_bundle.digest_evidence().logical_content_digest()
    );
    assert_eq!(
        left_bundle.digest_evidence().export_bundle_digest(),
        right_bundle.digest_evidence().export_bundle_digest()
    );
    assert_eq!(
        left_bundle.digest_evidence().declaration_digest(),
        right_bundle.digest_evidence().declaration_digest()
    );
    assert!(matches!(
        compare_canonical_exports(
            left_bundle.canonical_export(),
            right_bundle.canonical_export()
        ),
        CanonicalExportComparisonOutcome::Equivalent
    ));
    assert_eq!(
        left_bundle.manifest().rows(),
        right_bundle.manifest().rows()
    );
    assert_eq!(
        left_bundle.offline_declarations(),
        right_bundle.offline_declarations()
    );
    assert_eq!(left_bundle.counters().exported_chunks(), 1);
    assert_eq!(left_bundle.counters().exported_bytes(), 12);
    assert_eq!(left_bundle.counters().manifest_rows(), 1);
    assert_eq!(left_bundle.counters().skipped_chunks(), 0);

    let observation = inspect_offline_export_bundle(
        offline_declarations(left_bundle.offline_declarations()),
        offline_digest_evidence(&left_bundle),
    )
    .expect("offline inspection should observe exported bundle");
    assert_eq!(
        observation.digest_evidence(),
        &offline_digest_evidence(&left_bundle)
    );
    assert_eq!(
        observation.digest_evidence_count(),
        left_bundle.digest_evidence().evidence_item_count()
    );
    assert_eq!(observation.total_bytes(), 12);
}

#[test]
fn canonical_classification_reorders_equivalent_exported_chunks_to_same_manifest_and_digest() {
    let authority = BlobExportAuthority::from_current_store_authority(current_authority(
        "phase19.export.classification",
        "export-authority",
    ));
    let ordered = ordered_multichunk_exported_chunks(
        &authority,
        "phase19.export.classification",
        b"aaaabbbbcccc",
        4,
    );
    let left =
        super::classification::BlobExportCanonicalClassification::from_exported_chunks(&ordered);
    let mut reversed = ordered.clone();
    reversed.reverse();
    let right =
        super::classification::BlobExportCanonicalClassification::from_exported_chunks(&reversed);

    assert_eq!(left.manifest_rows(), right.manifest_rows());
    assert_eq!(left.offline_declarations(), right.offline_declarations());
    assert_eq!(left.counts(), right.counts());
    assert_eq!(
        super::evidence_bundle::declaration_digest(left.offline_declarations()),
        super::evidence_bundle::declaration_digest(right.offline_declarations())
    );
}

#[test]
fn stale_reachability_missing_chunk_and_wrong_sources_deny_before_export() {
    let authority = BlobExportAuthority::from_current_store_authority(current_authority(
        "phase19.export.denial",
        "export-authority",
    ));
    let denial_lane = export_lane(&authority, "phase19.export.denial", b"aaaabbbbcccc", 12);
    let stale_lane = export_lane(
        &authority,
        "phase19.export.denial.stale",
        b"other-bytes",
        12,
    );
    let stale_observation = denial_lane.observe();

    let stale = authority.publish_export_bundle(
        BlobExportIntent::for_current_lifecycle(
            stale_observation.lifecycle_receipt(),
            &denial_lane.publication,
            &stale_lane.reachability,
            &denial_lane.placement,
            &export_readiness("phase19.export.denial"),
        )
        .with_export_name("tenant/blob/gen")
        .with_exported_chunks(denial_lane.exported.clone()),
    );
    assert!(matches!(
        stale,
        Err(BlobExportBundleDenial::StaleReachability { .. })
    ));
    let stale_counters = match stale {
        Err(denial) => *denial.counters(),
        Ok(_) => panic!("stale reachability should deny"),
    };
    assert_eq!(stale_counters.stale_reachability_denials(), 1);

    let export_observation = denial_lane.observe();
    let missing = authority.publish_export_bundle(
        BlobExportIntent::for_current_lifecycle(
            export_observation.lifecycle_receipt(),
            &denial_lane.publication,
            &denial_lane.reachability,
            &denial_lane.placement,
            &export_readiness("phase19.export.denial"),
        )
        .with_export_name("tenant/blob/gen")
        .with_exported_chunks(Vec::<super::BlobExportedChunkBytes<'static>>::new()),
    );
    assert!(matches!(
        missing,
        Err(BlobExportBundleDenial::MissingChunk { .. })
    ));
    let missing_counters = match missing {
        Err(denial) => *denial.counters(),
        Ok(_) => panic!("missing chunk should deny"),
    };
    assert_eq!(missing_counters.missing_chunk_denials(), 1);

    let terminal = reject_terminal_projection_row_as_blob_export_bundle("row");
    assert!(matches!(
        terminal,
        BlobExportBundleDenial::TerminalProjectionRejected { .. }
    ));
    assert_eq!(terminal.counters().terminal_projection_denials(), 1);

    let copied = reject_copied_export_row_as_blob_export_bundle("row");
    assert!(matches!(
        copied,
        BlobExportBundleDenial::CopiedExportRowRejected { .. }
    ));
    assert_eq!(copied.counters().copied_row_denials(), 1);

    let placement_only =
        reject_placement_only_evidence_as_blob_export_bundle(&denial_lane.placement);
    assert!(matches!(
        placement_only,
        BlobExportBundleDenial::PlacementOnlyEvidenceRejected { .. }
    ));
    assert_eq!(placement_only.counters().placement_only_denials(), 1);
}

#[test]
fn wrong_bytes_and_wrong_offline_digest_evidence_are_denied() {
    let authority = BlobExportAuthority::from_current_store_authority(current_authority(
        "phase19.export.hostile",
        "export-authority",
    ));
    let lane = export_lane(&authority, "phase19.export.hostile", b"aaaabbbbcccc", 12);
    let wrong_bytes = authority.collect_exported_chunk_bytes(
        &lane.ordered_leaves[0],
        BlobChunkByteWindow::borrowed(0, b"zzzz").expect("window"),
    );
    assert!(matches!(
        wrong_bytes,
        Err(BlobExportBundleDenial::ChunkEvidenceMismatch { .. })
    ));

    let observation = lane.observe();
    let published = authority
        .publish_export_bundle(
            BlobExportIntent::for_current_lifecycle(
                observation.lifecycle_receipt(),
                &lane.publication,
                &lane.reachability,
                &lane.placement,
                &export_readiness("phase19.export.hostile"),
            )
            .with_export_name("tenant/blob/gen")
            .with_exported_chunks(lane.exported.clone()),
        )
        .expect("export should publish");

    let mut digest_evidence = offline_digest_evidence(&published);
    digest_evidence.declaration_digest = "s7:export-declarations:deadbeefdeadbeef".to_owned();
    let offline = inspect_offline_export_bundle(
        offline_declarations(published.offline_declarations()),
        digest_evidence,
    );
    assert!(matches!(
        offline,
        Err(OfflineExportBundleObservationDenial::DigestEvidenceMismatch)
    ));

    let malformed_logical = inspect_offline_export_bundle(
        offline_declarations(published.offline_declarations()),
        OfflineExportDigestEvidence {
            logical_content_digest: "not-a-digest".to_owned(),
            ..offline_digest_evidence(&published)
        },
    );
    assert!(matches!(
        malformed_logical,
        Err(OfflineExportBundleObservationDenial::EmptyDigestField)
    ));

    let malformed_export_digest = inspect_offline_export_bundle(
        offline_declarations(published.offline_declarations()),
        OfflineExportDigestEvidence {
            export_bundle_digest: "xyz".to_owned(),
            ..offline_digest_evidence(&published)
        },
    );
    assert!(matches!(
        malformed_export_digest,
        Err(OfflineExportBundleObservationDenial::EmptyDigestField)
    ));
}

#[test]
fn offline_observer_inspects_multichunk_declarations_with_real_digest_evidence() {
    let declarations = vec![
        OfflineExportChunkDeclaration {
            ordinal: 1,
            chunk_identity: "s7:chunk:1111111111111111".to_owned(),
            stored_digest: "s7:stored:2222222222222222".to_owned(),
            checksum_digest: "fnv64:3333333333333333".to_owned(),
            bytes: 4,
        },
        OfflineExportChunkDeclaration {
            ordinal: 2,
            chunk_identity: "s7:chunk:4444444444444444".to_owned(),
            stored_digest: "s7:stored:5555555555555555".to_owned(),
            checksum_digest: "fnv64:6666666666666666".to_owned(),
            bytes: 8,
        },
    ];
    let observation = inspect_offline_export_bundle(
        declarations.clone(),
        OfflineExportDigestEvidence {
            logical_content_digest: "s7:logical:7777777777777777".to_owned(),
            export_bundle_digest:
                "8888888888888888888888888888888888888888888888888888888888888888".to_owned(),
            declaration_digest: offline_declaration_digest(&declarations),
            declared_chunk_count: declarations.len() as u64,
            declared_total_bytes: 12,
        },
    )
    .expect("multi-chunk offline inspection should admit");
    assert_eq!(observation.declarations().len(), 2);
    assert_eq!(observation.total_bytes(), 12);
    assert_eq!(
        observation.digest_evidence(),
        &OfflineExportDigestEvidence {
            logical_content_digest: "s7:logical:7777777777777777".to_owned(),
            export_bundle_digest:
                "8888888888888888888888888888888888888888888888888888888888888888".to_owned(),
            declaration_digest: offline_declaration_digest(&declarations),
            declared_chunk_count: declarations.len() as u64,
            declared_total_bytes: 12,
        }
    );
    assert_eq!(observation.digest_evidence_count(), 5);
}
