#[test]
fn admitted_shared_dedupe_reference_edges_localize_all_affected_edges() {
    let scope_case = "phase11.shared.scope";
    let bytes = b"aaaabbbb";
    let (published, visible) =
        publish_shared_scope_generation(scope_case, "phase11.shared.source", 1, bytes, 4);
    let (affected_published, _) =
        publish_shared_scope_generation(scope_case, "phase11.shared.affected", 2, bytes, 4);
    let source_frontier = frontier_for(scope_case, bytes, 4);
    let affected_frontier = frontier_for(scope_case, bytes, 4);
    let registered_reference = registered_share_reference_for_first_chunk(scope_case, b"aaaa");

    let localized_edge =
        BlobCorruptionReferenceEdge::from_reachability_staging_identity(published.staging_identity());
    let affected_edge = BlobCorruptionReferenceEdge::from_admitted_shared_dedupe_reference(
        published.staging_identity(),
        affected_published.staging_identity(),
        &affected_frontier,
        BlobChunkOrdinal::first(),
        &registered_reference,
    )
    .expect("admitted dedupe sharing should mint affected edge");
    let edges = BlobCorruptionReferenceEdges::from_admitted_edges(&[localized_edge, affected_edge])
        .expect("localized and affected shared edges should bind");

    let localized = BlobCorruptedChunkLocalization::from_read_corruption(
        visible,
        source_frontier,
        BlobChunkOrdinal::first(),
        BlobCorruptionPlacementClass::LocalPhysical,
        edges,
    )
    .expect("shared dedupe corruption should localize all affected edges");

    assert_eq!(
        localized.sharing_scope(),
        BlobCorruptionReferenceSharingScope::SharedReferenceEdges
    );
    assert_eq!(localized.reference_edges().edge_count(), 2);
    assert_eq!(localized.counters().affected_reference_edges(), 2);
}

#[test]
fn shared_dedupe_edge_requires_affected_frontier_to_contain_the_shared_chunk() {
    let scope_case = "phase11.shared.mismatch";
    let (published, _) =
        publish_shared_scope_generation(scope_case, "phase11.shared.mismatch.source", 1, b"aaaabbbb", 4);
    let (affected_published, _) =
        publish_shared_scope_generation(scope_case, "phase11.shared.mismatch.affected", 2, b"ccccdddd", 4);
    let affected_frontier = frontier_for(scope_case, b"ccccdddd", 4);
    let registered_reference = registered_share_reference_for_first_chunk(scope_case, b"aaaa");

    let denied = BlobCorruptionReferenceEdge::from_admitted_shared_dedupe_reference(
        published.staging_identity(),
        affected_published.staging_identity(),
        &affected_frontier,
        BlobChunkOrdinal::first(),
        &registered_reference,
    )
    .expect_err("copied dedupe claim must not bind an unrelated affected frontier");

    assert!(matches!(
        denied,
        BlobCorruptionDenial::AffectedReferenceEdgeMismatch { .. }
    ));
}

fn registered_share_reference_for_first_chunk(
    scope_case: &str,
    chunk: &[u8],
) -> crate::BlobChunkRegisteredDedupeReference {
    let existing = candidate_for_bytes_and_scope(
        chunk,
        blob_scope(scope_case, StoreTenantScope::TenantPhysicalBoundary),
    );
    let candidate = candidate_for_bytes_and_scope(
        chunk,
        blob_scope(scope_case, StoreTenantScope::TenantPhysicalBoundary),
    );
    let equivalence = canonical_equivalence(&existing, &candidate);
    let share_claim = match BlobChunkDedupeAdmission::compare_candidates(existing, candidate)
        .with_foundational_canonical_equivalence(equivalence)
        .admit()
    {
        TransitionOutcome::Success(claim) => claim,
        other => panic!("expected admitted share claim: {other:?}"),
    };
    let mut registry = crate::BlobChunkDedupeReferenceRegistry::new_store_owned();
    share_claim
        .admit_into_reference_registry(&mut registry)
        .expect("registered dedupe reference should admit")
}

fn publish_shared_scope_generation(
    scope_case: &str,
    object_case: &str,
    generation_sequence: u64,
    bytes: &[u8],
    chunk_size: u64,
) -> (crate::BlobGenerationPublished, crate::BlobVisibleGeneration) {
    let (root, stored_digest) = crate::blob_generation_registry_test_support::
        root_publication_with_bytes_and_chunk_size(scope_case, bytes, chunk_size);
    let receipt = crate::blob_generation_registry_test_support::
        lifecycle_receipt_for_publication_with_identity(
            scope_case,
            object_case,
            generation_sequence,
            root.chunk_tree_root().clone(),
            root.logical_content_digest().clone(),
            stored_digest,
            crate::BlobAuthorityClassification::StoreOwnedPhysicalBlob,
            bytes,
        );
    let object_id = receipt.declaration().object_id().clone();
    let generation = receipt.declaration().generation();
    let classification = crate::BlobObjectClassificationAdmission::from_executed_lifecycle(&receipt);
    let mut registry = crate::BlobGenerationRegistry::new();
    crate::BlobGenerationRegistryAdmission::from_executed_lifecycle(
        root.clone(),
        receipt,
        classification,
    )
    .publish(
        &mut registry,
        crate::blob_generation_registry_test_support::registry_authority(scope_case),
    )
    .expect("registry publication should admit");
    let observation = registry
        .observe_registered_generation(&object_id, generation)
        .expect("registered generation should observe");
    let reachability = observation.lifecycle_receipt().reachability().clone();
    let resumability = observation.lifecycle_receipt().resumability_receipt();
    let candidate = crate::BlobRootCandidateForPublication::from_registry_observation(
        observation,
        root,
    )
    .expect("root candidate should bind registry observation");
    let staged = crate::BlobReachabilityStaging::stage(candidate, reachability)
        .expect("reachability should stage");
    let payload = crate::BlobPublicationWalPayload::from_staged_reachability(&staged);
    let wal_commit = crate::BlobPublicationWalCommit::from_replayable_wal_record(
        staged,
        payload.clone(),
        crate::blob_publication_commit_test_support::durable_wal_publication(
            payload.frame_digest(),
        ),
        &crate::blob_publication_commit_test_support::replayable_wal_classification(
            payload.frame_digest(),
        ),
    )
    .expect("wal publication commit should admit");
    let wal_record = crate::BlobPublicationWalRecord::append(wal_commit);
    let closeout = crate::BlobPublicationSessionCloseout::close(wal_record, resumability)
        .expect("session closeout should admit");
    let published = crate::BlobGenerationPublished::commit_visible(
        closeout,
        crate::BlobPublicationAuthority::from_current_store_authority(current_authority(
            &format!("{scope_case}.{object_case}.publication"),
            "publication",
        )),
    );
    let visible = crate::BlobVisibleGeneration::from_published(&published);
    (published, visible)
}
