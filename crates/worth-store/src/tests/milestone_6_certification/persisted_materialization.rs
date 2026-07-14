use super::*;

#[test]
fn milestone_6_certification_bundle_prefers_persisted_layout_materialization_when_present() {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let root = latest_envelope(&runtime);
    let request = request_for_scope(
        &root,
        AspectScopeClass::EntitySetUniform(EntitySetUniformAspectScope::new(vec![
            "entity-a".to_string(),
            "entity-b".to_string(),
        ])),
        &["profile", "status"],
    );
    let path = unique_test_store_path("worth-store-m6-persisted-certification");

    let mut store = WORTHStoreBuilder::new()
        .local_file(path.clone())
        .build()
        .unwrap();
    store.append_canonical_commit(root).unwrap();
    let materialized = store
        .materialize_milestone_6_layout_support(request.clone())
        .unwrap();
    let direct_bundle = store
        .milestone_6_certification_bundle(request.clone())
        .unwrap();
    assert_eq!(
        direct_bundle.requested_layout_support_lane,
        Milestone6LayoutSupportLane::OnDemandMaterialized
    );
    assert_eq!(
        direct_bundle.resolved_layout_support_lane,
        Milestone6ResolvedLayoutSupportLane::OnDemandMaterialized
    );
    assert_eq!(
        direct_bundle.layout_support_publication_disposition,
        crate::Milestone6LayoutSupportPublicationDisposition::ReusedExisting
    );
    assert_eq!(
        direct_bundle.certification_origin,
        crate::Milestone6CertificationOrigin::PersistedMaterialization
    );
    assert_eq!(
        direct_bundle
            .layout_materialization_report
            .as_ref()
            .map(|report| report.artifact_id.as_str()),
        Some(materialized.artifact_id())
    );
    drop(store);

    let reopened = WORTHStoreBuilder::new().local_file(path).build().unwrap();
    let fetched = reopened
        .fetch_milestone_6_layout_support(request.clone())
        .unwrap();
    let reopened_bundle = reopened.milestone_6_certification_bundle(request).unwrap();
    assert_eq!(
        reopened_bundle.requested_layout_support_lane,
        Milestone6LayoutSupportLane::OnDemandMaterialized
    );
    assert_eq!(
        reopened_bundle.resolved_layout_support_lane,
        Milestone6ResolvedLayoutSupportLane::OnDemandMaterialized
    );
    assert_eq!(
        reopened_bundle.layout_support_publication_disposition,
        crate::Milestone6LayoutSupportPublicationDisposition::ReusedExisting
    );
    assert_eq!(
        reopened_bundle.certification_origin,
        crate::Milestone6CertificationOrigin::PersistedMaterialization
    );
    assert_eq!(
        reopened_bundle
            .layout_materialization_report
            .as_ref()
            .map(|report| report.artifact_id.as_str()),
        Some(fetched.artifact_id())
    );

    assert_eq!(materialized, fetched);
    assert_eq!(direct_bundle.truth_digest, reopened_bundle.truth_digest);
    assert_eq!(
        direct_bundle.artifact_digest,
        reopened_bundle.artifact_digest
    );
    assert_ne!(
        direct_bundle.diagnostics_digest,
        reopened_bundle.diagnostics_digest
    );
    assert_eq!(
        direct_bundle.complexity_status.aspect_layout_read.status,
        crate::ComplexityStatus::Verified
    );
    assert_eq!(
        direct_bundle
            .complexity_status
            .structural_block_reuse
            .status,
        crate::ComplexityStatus::Verified
    );
    assert_eq!(
        direct_bundle.complexity_status.chunk_model_freeze.status,
        crate::ComplexityStatus::Verified
    );
    assert_eq!(
        direct_bundle
            .access_structure_verification
            .structural_block_reuse
            .verified_at_open,
        true
    );
    assert_eq!(
        direct_bundle
            .access_structure_verification
            .aspect_layout_read
            .verified_at_open,
        true
    );
    assert_eq!(
        direct_bundle
            .access_structure_verification
            .chunk_model_freeze
            .verified_at_open,
        true
    );
    assert_eq!(
        direct_bundle
            .access_structure_verification
            .milestone_9_physical_chunk_reference
            .verified_at_open,
        true
    );
    assert_eq!(direct_bundle.certification_summary.verified_path_count, 5);
    assert_eq!(direct_bundle.certification_summary.debt_path_count, 0);
    assert_eq!(
        reopened_bundle.complexity_status.aspect_layout_read.status,
        crate::ComplexityStatus::Verified
    );
    assert_eq!(
        reopened_bundle
            .complexity_status
            .structural_block_reuse
            .status,
        crate::ComplexityStatus::Verified
    );
    assert_eq!(
        reopened_bundle.complexity_status.chunk_model_freeze.status,
        crate::ComplexityStatus::Verified
    );
    assert_eq!(reopened_bundle.certification_summary.verified_path_count, 5);
    assert_eq!(reopened_bundle.certification_summary.debt_path_count, 0);
    assert_eq!(
        reopened_bundle.physical_layout_report.structural_block_id,
        fetched
            .block_reuse()
            .structural_block_id()
            .as_str()
            .to_string()
    );
    assert_eq!(
        reopened_bundle.physical_layout_report.physical_chunk_id,
        fetched
            .milestone_9_reference()
            .physical_chunk_id()
            .as_str()
            .to_string()
    );
}
