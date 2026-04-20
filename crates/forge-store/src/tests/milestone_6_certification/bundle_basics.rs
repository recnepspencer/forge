use super::*;

#[test]
fn milestone_6_certification_bundle_is_backend_stable() {
    let mut truth_digests = Vec::new();
    let mut artifact_digests = Vec::new();
    let mut diagnostics_digests = Vec::new();
    let mut chunk_digests = Vec::new();

    for lane in [StoreLane::InMemory, StoreLane::LocalFile, StoreLane::Sqlite] {
        let bundle = entity_set_bundle_for_lane(lane);
        assert_eq!(
            bundle.requested_layout_support_lane,
            Milestone6LayoutSupportLane::ProofOnly
        );
        assert_eq!(
            bundle.resolved_layout_support_lane,
            Milestone6ResolvedLayoutSupportLane::ProofOnly
        );
        truth_digests.push((lane.label(), bundle.truth_digest.clone()));
        artifact_digests.push((lane.label(), bundle.artifact_digest.clone()));
        diagnostics_digests.push((lane.label(), bundle.diagnostics_digest.clone()));
        chunk_digests.push((
            lane.label(),
            bundle.physical_layout_report.determinism_digest.clone(),
        ));
        assert_eq!(bundle.layout_read_report.scope_class, "entity_set_uniform");
        assert_eq!(
            bundle.physical_layout_report.branch_id,
            BranchId("main".to_string())
        );
        assert_eq!(
            bundle.physical_layout_report.milestone_9_chunk_member_count,
            2
        );
        assert!(bundle
            .access_structure_contract
            .aspect_layout_read
            .access_structure
            .contains("scope-to-slice membership records"));
        assert!(
            !bundle
                .access_structure_verification
                .aspect_layout_read
                .verified_at_open
        );
        assert_eq!(
            bundle.complexity_status.aspect_layout_read.status,
            crate::ComplexityStatus::Debt
        );
        assert_eq!(
            bundle.complexity_status.structural_block_reuse.status,
            crate::ComplexityStatus::Debt
        );
        assert_eq!(
            bundle.complexity_status.chunk_model_freeze.status,
            crate::ComplexityStatus::Debt
        );
        assert_eq!(
            bundle.complexity_status.milestone_7_layout_reference.status,
            crate::ComplexityStatus::Verified
        );
        assert_eq!(
            bundle
                .complexity_status
                .milestone_9_physical_chunk_reference
                .status,
            crate::ComplexityStatus::Verified
        );
        assert!(bundle
            .complexity_status
            .aspect_layout_read
            .debt_reason
            .as_deref()
            .unwrap_or_default()
            .contains("proof-only"));
        assert_eq!(bundle.counter_contract.aspect_layout_plan_count, 1);
        assert_eq!(bundle.counter_contract.aspect_layout_admitted_count, 1);
        assert_eq!(bundle.counter_contract.aspect_layout_fallback_count, 0);
        assert_eq!(
            bundle
                .counter_contract
                .structural_block_reuse_admission_count,
            1
        );
        assert_eq!(bundle.counter_contract.chunk_model_freeze_count, 1);
        assert_eq!(
            bundle
                .counter_contract
                .milestone_7_layout_reference_admission_count,
            1
        );
        assert_eq!(
            bundle
                .counter_contract
                .milestone_9_physical_chunk_reference_admission_count,
            1
        );
        assert_eq!(bundle.certification_summary.verified_path_count, 2);
        assert_eq!(bundle.certification_summary.debt_path_count, 3);
        assert!(bundle.certification_summary.fallback_free_admission);
        assert!(bundle.certification_summary.deterministic_chunk_freeze);
        assert!(bundle.certification_summary.milestone_7_boundary_isolated);
        assert!(bundle.certification_summary.milestone_9_boundary_isolated);
    }

    let first_truth = &truth_digests[0].1;
    assert!(truth_digests
        .iter()
        .all(|(_, digest)| digest == first_truth));
    let first_artifact = &artifact_digests[0].1;
    assert!(artifact_digests
        .iter()
        .all(|(_, digest)| digest == first_artifact));
    let first_diagnostics = &diagnostics_digests[0].1;
    assert!(diagnostics_digests
        .iter()
        .any(|(_, digest)| digest != first_diagnostics));
    let first_chunk = &chunk_digests[0].1;
    assert!(chunk_digests
        .iter()
        .all(|(_, digest)| digest == first_chunk));
}

#[test]
fn milestone_6_certification_bundle_proves_scope_shape_changes_layout_truth() {
    let single = single_entity_bundle_for_lane(StoreLane::InMemory);
    let entity_set = entity_set_bundle_for_lane(StoreLane::InMemory);

    assert_ne!(single.truth_digest, entity_set.truth_digest);
    assert_ne!(single.artifact_digest, entity_set.artifact_digest);
    assert_eq!(single.layout_read_report.scope_class, "single_entity");
    assert_eq!(
        entity_set.layout_read_report.scope_class,
        "entity_set_uniform"
    );
    assert_eq!(
        single.physical_layout_report.milestone_9_chunk_member_count,
        1
    );
    assert_eq!(
        entity_set
            .physical_layout_report
            .milestone_9_chunk_member_count,
        2
    );
    assert_ne!(
        single.physical_layout_report.structural_block_id,
        entity_set.physical_layout_report.structural_block_id
    );
}

#[test]
fn milestone_6_certification_rejects_non_admitted_generalized_scope() {
    let (store, root) = store_for_lane_with_root(StoreLane::InMemory, "rejection");
    let error = store
        .milestone_6_certification_bundle(request_for_scope(
            &root,
            AspectScopeClass::Generalized {
                descriptor: "wildcard-join".to_string(),
            },
            &["profile"],
        ))
        .unwrap_err();

    assert_eq!(error.kind(), &StoreErrorKind::AspectLayoutFallbackRequired);
    assert_eq!(store.counters().aspect_layout_plan_count, 1);
    assert_eq!(store.counters().aspect_layout_admitted_count, 0);
    assert_eq!(store.counters().aspect_layout_fallback_count, 1);
    assert_eq!(store.counters().aspect_layout_rejected_count, 0);
}
