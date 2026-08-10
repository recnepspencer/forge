use super::*;

#[test]
fn runtime_publishes_merge_continuity_and_explanation_bundle() {
    let declaration = registered_merge(
        MergeHistoryDeclarationIdentity::admit_bridge_owned("merge:publish"),
        BridgeMergeConsumptionClass::AspectReconciliationMerge,
    );
    let runtime = runtime_with_merge(declaration.clone());
    let contract = runtime
        .admit_merge_history(declaration)
        .expect("merge declaration should be admitted");
    let lowered = runtime
        .lower_merge_history(&contract)
        .expect("merge contract should lower");
    let reduced = runtime
        .reduce_merge_routing(&lowered)
        .expect("reduced merge routing should succeed");
    let continuity = runtime
        .publish_merge_continuity_artifact(&reduced)
        .expect("continuity publication should succeed");
    let explanation =
        runtime.publish_merge_explanation_artifact(&lowered, &reduced, Some(&continuity), None);

    assert_eq!(continuity.reduced_routing_artifact(), &reduced);
    assert_eq!(explanation.continuity_digest(), Some(continuity.digest()));
    assert_eq!(
        explanation.outcome_class(),
        BridgeMergeRoutingOutcomeClass::ContinuityCandidate
    );
}

#[test]
fn runtime_publishes_merge_remap_only_when_structural_advisory_is_consistent() {
    let declaration = registered_merge(
        MergeHistoryDeclarationIdentity::admit_bridge_owned("merge:publish-remap"),
        BridgeMergeConsumptionClass::AspectReconciliationMerge,
    )
    .with_structural_advisory(BridgeMergeStructuralAdvisoryDisposition::AdvisoryConsistent);
    let runtime = runtime_with_merge(declaration.clone());
    let contract = runtime
        .admit_merge_history(declaration)
        .expect("merge declaration should be admitted");
    let lowered = runtime
        .lower_merge_history(&contract)
        .expect("merge contract should lower");
    let reduced = runtime
        .reduce_merge_routing(&lowered)
        .expect("reduced merge routing should succeed");
    let remap = runtime
        .publish_merge_remap_artifact(&reduced)
        .expect("remap publication should succeed");

    assert_eq!(remap.reduced_routing_artifact(), &reduced);
    assert_eq!(
        remap
            .reduced_routing_artifact()
            .lowered_packet_set()
            .contract()
            .validated_declaration()
            .declaration()
            .structural_advisory(),
        BridgeMergeStructuralAdvisoryDisposition::AdvisoryConsistent
    );
}

#[test]
fn runtime_rejects_merge_publication_with_typed_denial_kind() {
    let declaration = registered_merge(
        MergeHistoryDeclarationIdentity::admit_bridge_owned("merge:typed-denial"),
        BridgeMergeConsumptionClass::DeletionMerge,
    );
    let runtime = runtime_with_merge(declaration.clone());
    let contract = runtime
        .admit_merge_history(declaration)
        .expect("merge declaration should be admitted");
    let lowered = runtime
        .lower_merge_history(&contract)
        .expect("merge contract should lower");
    let reduced = runtime
        .reduce_merge_routing(&lowered)
        .expect("reduced merge routing should succeed");

    let error = runtime
        .publish_merge_continuity_artifact(&reduced)
        .expect_err("deletion-gated merge should not publish continuity");

    assert_eq!(
        error.kind(),
        crate::error::BridgeMergeErrorKind::MergeDeletionDenied
    );
}
