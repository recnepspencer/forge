use super::*;

#[test]
fn runtime_replay_merge_history_certifies_full_bundle() {
    let declaration = registered_merge(
        MergeHistoryDeclarationIdentity::admit_bridge_owned("merge:replay-bundle"),
        BridgeMergeConsumptionClass::AspectReconciliationMerge,
    )
    .with_structural_advisory(BridgeMergeStructuralAdvisoryDisposition::AdvisoryConsistent);
    let runtime = runtime_with_merge(declaration.clone());
    let contract = runtime
        .admit_merge_history(declaration)
        .expect("merge declaration should be admitted");

    let bundle = runtime
        .replay_merge_history(&contract)
        .expect("merge replay bundle should reconstruct");

    assert_eq!(bundle.contract().digest(), contract.digest());
    assert!(bundle.continuity_artifact().is_some());
    assert!(bundle.remap_artifact().is_some());
    assert_eq!(
        bundle.explanation_artifact().remap_digest(),
        bundle.remap_artifact().map(|artifact| artifact.digest())
    );
    assert_eq!(
        bundle
            .reduced_routing_artifact()
            .counters()
            .merge_remap_publication_count(),
        1
    );
    assert_eq!(
        bundle
            .explanation_artifact()
            .counters()
            .merge_explanation_request_count(),
        1
    );
}

#[test]
fn runtime_canonicalizes_and_explains_merge_record() {
    let declaration = registered_merge(
        MergeHistoryDeclarationIdentity::admit_bridge_owned("merge:diagnostics"),
        BridgeMergeConsumptionClass::AspectReconciliationMerge,
    )
    .with_structural_advisory(BridgeMergeStructuralAdvisoryDisposition::AdvisoryConsistent);
    let runtime = runtime_with_merge(declaration.clone());
    let contract = runtime
        .admit_merge_history(declaration)
        .expect("merge declaration should be admitted");
    let bundle = runtime
        .replay_merge_history(&contract)
        .expect("merge bundle should reconstruct");
    let record = runtime.canonicalize_merge_record(&bundle);
    let explanation = runtime
        .diagnostics()
        .explain_last_merge_record()
        .expect("merge record should be retained");

    assert_eq!(record.bundle().digest(), bundle.digest());
    assert_eq!(
        runtime
            .diagnostics()
            .last_merge_record()
            .expect("merge record should be present")
            .record_identity(),
        record.record_identity()
    );
    assert_eq!(
        explanation.outcome_class(),
        BridgeMergeRoutingOutcomeClass::ContinuityCandidate
    );
    assert!(explanation.continuity_digest().is_some());
    assert!(explanation.remap_digest().is_some());
}

#[test]
fn runtime_replays_canonical_merge_record() {
    let declaration = registered_merge(
        MergeHistoryDeclarationIdentity::admit_bridge_owned("merge:canonical-replay"),
        BridgeMergeConsumptionClass::AspectReconciliationMerge,
    )
    .with_structural_advisory(BridgeMergeStructuralAdvisoryDisposition::AdvisoryConsistent);
    let runtime = runtime_with_merge(declaration.clone());
    let contract = runtime
        .admit_merge_history(declaration)
        .expect("merge declaration should be admitted");
    let bundle = runtime
        .replay_merge_history(&contract)
        .expect("merge bundle should reconstruct");
    let record = runtime.canonicalize_merge_record(&bundle);

    let replayed = runtime
        .replay_canonical_merge_record(&record)
        .expect("canonical merge record should replay");

    assert_eq!(replayed.digest(), bundle.digest());
    assert_eq!(
        replayed
            .reduced_routing_artifact()
            .counters()
            .merge_replay_request_count(),
        1
    );
    assert_eq!(
        replayed
            .explanation_artifact()
            .counters()
            .merge_explanation_request_count(),
        1
    );
}
