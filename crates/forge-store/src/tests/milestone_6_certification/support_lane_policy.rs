use super::*;

fn assert_complexity_debt(
    path: &crate::Milestone6ComplexityPathStatus,
    verification: &crate::Milestone6AccessStructureVerificationPath,
    expected_fragment: &str,
) {
    assert!(!verification.verified_at_open);
    assert!(verification
        .verification_gap
        .as_deref()
        .unwrap_or_default()
        .contains(expected_fragment));
    assert_eq!(path.status, crate::ComplexityStatus::Debt);
    assert!(path.proof_basis.is_none());
    assert!(path
        .debt_reason
        .as_deref()
        .unwrap_or_default()
        .contains(expected_fragment));
}

#[test]
fn milestone_6_live_certification_marks_unpublished_layout_paths_as_debt() {
    let bundle = entity_set_bundle_for_lane(StoreLane::InMemory);
    assert_eq!(
        bundle.requested_layout_support_lane,
        Milestone6LayoutSupportLane::ProofOnly
    );
    assert_eq!(
        bundle.resolved_layout_support_lane,
        Milestone6ResolvedLayoutSupportLane::ProofOnly
    );
    assert_eq!(
        bundle.layout_support_publication_disposition,
        crate::Milestone6LayoutSupportPublicationDisposition::None
    );
    assert_complexity_debt(
        &bundle.complexity_status.aspect_layout_read,
        &bundle.access_structure_verification.aspect_layout_read,
        "proof-only",
    );
    assert_complexity_debt(
        &bundle.complexity_status.chunk_model_freeze,
        &bundle.access_structure_verification.chunk_model_freeze,
        "proof-only",
    );
    assert_complexity_debt(
        &bundle.complexity_status.structural_block_reuse,
        &bundle.access_structure_verification.structural_block_reuse,
        "proof-only",
    );
    assert_eq!(bundle.certification_summary.verified_path_count, 2);
    assert_eq!(bundle.certification_summary.debt_path_count, 3);
}

#[test]
fn milestone_6_explicit_proof_only_certification_stays_debt_even_when_materialized_support_exists()
{
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
    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    store.append_canonical_commit(root).unwrap();
    store
        .materialize_milestone_6_layout_support(request.clone())
        .unwrap();

    let bundle = store
        .milestone_6_certification_bundle_in_lane(request, Milestone6LayoutSupportLane::ProofOnly)
        .unwrap();

    assert_eq!(
        bundle.requested_layout_support_lane,
        Milestone6LayoutSupportLane::ProofOnly
    );
    assert_eq!(
        bundle.resolved_layout_support_lane,
        Milestone6ResolvedLayoutSupportLane::ProofOnly
    );
    assert_eq!(
        bundle.layout_support_publication_disposition,
        crate::Milestone6LayoutSupportPublicationDisposition::None
    );
    assert_eq!(
        bundle.certification_origin,
        crate::Milestone6CertificationOrigin::ReconstructedWitness
    );
    assert_eq!(
        bundle.complexity_status.aspect_layout_read.status,
        crate::ComplexityStatus::Debt
    );
    assert_eq!(bundle.certification_summary.debt_path_count, 3);
}

#[test]
fn milestone_6_explicit_on_demand_certification_materializes_support_when_missing() {
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
    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    store.append_canonical_commit(root).unwrap();

    let bundle = store
        .milestone_6_certification_bundle_in_lane(
            request.clone(),
            Milestone6LayoutSupportLane::OnDemandMaterialized,
        )
        .unwrap();

    assert_eq!(
        bundle.requested_layout_support_lane,
        Milestone6LayoutSupportLane::OnDemandMaterialized
    );
    assert_eq!(
        bundle.resolved_layout_support_lane,
        Milestone6ResolvedLayoutSupportLane::OnDemandMaterialized
    );
    assert_eq!(
        bundle.layout_support_publication_disposition,
        crate::Milestone6LayoutSupportPublicationDisposition::PublishedThisOperation
    );
    assert_eq!(bundle.certification_summary.debt_path_count, 0);
    assert_eq!(bundle.certification_summary.verified_path_count, 5);
}

#[test]
fn milestone_6_policy_eager_certification_resolving_to_proof_only_stays_debt() {
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
    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    store.append_canonical_commit(root).unwrap();

    let bundle = store
        .milestone_6_certification_bundle_in_lane_with_policy(
            request,
            Milestone6LayoutSupportLane::PolicyEagerMaterialized,
            Milestone6LayoutSupportPolicy::new(true, true, 2),
        )
        .unwrap();

    assert_eq!(
        bundle.resolved_layout_support_lane,
        Milestone6ResolvedLayoutSupportLane::ProofOnly
    );
    assert_eq!(bundle.certification_summary.debt_path_count, 3);
    assert_eq!(bundle.certification_summary.verified_path_count, 2);
}

#[test]
fn milestone_6_policy_eager_certification_resolving_to_materialized_is_verified() {
    let mut runtime = runtime_with_demo_schema();
    let entity_id = create_entity(&mut runtime, "alpha");
    let root = latest_envelope(&runtime);
    let request = request_for_scope(
        &root,
        AspectScopeClass::EntitySetUniform(EntitySetUniformAspectScope::new(vec![
            "entity-a".to_string(),
            "entity-b".to_string(),
        ])),
        &["profile", "status"],
    );
    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    store.append_canonical_commit(root.clone()).unwrap();

    let branch_id = root.branch_context.clone();
    let commit_id = root.commit.commit_id;
    for _ in 0..2 {
        let _ = store
            .prepare_milestone_6_layout_support_with_policy(
                request.clone(),
                Milestone6LayoutSupportLane::PolicyEagerMaterialized,
                Milestone6LayoutSupportPolicy::new(false, true, 2),
            )
            .unwrap();
    }

    let bundle = store
        .milestone_6_certification_bundle_in_lane_with_policy(
            AspectLayoutReadRequest::new(
                AspectLayoutTarget::new(branch_id, commit_id),
                AspectScopeClass::EntitySetUniform(EntitySetUniformAspectScope::new(vec![
                    "entity-a".to_string(),
                    "entity-b".to_string(),
                ])),
                AspectProjectionSet::new(vec!["profile".to_string(), "status".to_string()]),
            ),
            Milestone6LayoutSupportLane::PolicyEagerMaterialized,
            Milestone6LayoutSupportPolicy::new(false, true, 2),
        )
        .unwrap();

    assert_eq!(
        bundle.resolved_layout_support_lane,
        Milestone6ResolvedLayoutSupportLane::PolicyEagerMaterializedReuseExisting
    );
    assert_eq!(bundle.certification_summary.debt_path_count, 0);
    assert_eq!(bundle.certification_summary.verified_path_count, 5);
    let _ = entity_id;
}
