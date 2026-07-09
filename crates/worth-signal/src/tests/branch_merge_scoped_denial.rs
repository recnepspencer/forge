use crate::facade::*;
use crate::logic::transaction::{
    BranchMergeScopedDenialKind, BranchMergeScopedDeniedLocus,
    BranchMergeScopedUnavailableOutcomeKind, BranchMergeScopedUnavailableReason,
};
use crate::tests::branch_merge_scoped_denial_support::{
    assert_scoped_denial_is_side_effect_free, build_ambiguous_scoped_denial_runtime,
    build_scoped_denial_runtime, selected_aspect_scope_digest, selected_node_scope_digest,
};
use crate::tests::support::ASPECT_A;

#[test]
fn scoped_merge_denials_preserve_distinct_selected_node_aspect_and_ambiguous_posture() {
    let (mut missing_node_runtime, feature, main, _primary) = build_scoped_denial_runtime();
    let before_missing_node = missing_node_runtime
        .current_branch_basis_artifact()
        .payload()
        .basis_digest()
        .to_owned();
    let missing_node_error = match missing_node_runtime
        .merge()
        .from(feature.clone())
        .into(main.clone())
        .selected_nodes([NodeId::new(999, 1)])
        .plan()
    {
        Err(error) => error,
        Ok(_) => panic!("missing selected node should deny before scoped merge planning"),
    };
    match missing_node_error {
        SignalError::BranchMergeFailed {
            kind: BranchMergeFailureKind::ScopedMergeDenied,
            evidence: Some(BranchMergeFailureEvidence::ScopedDenial(evidence)),
            ..
        } => {
            assert_eq!(
                evidence.denial_kind,
                BranchMergeScopedDenialKind::UnknownSelectedNode
            );
            assert_eq!(
                evidence.scope_digest,
                selected_node_scope_digest(&feature, &main, NodeId::new(999, 1))
            );
            assert_eq!(
                evidence.denied_locus,
                BranchMergeScopedDeniedLocus::Node(NodeId::new(999, 1))
            );
        }
        other => panic!("expected typed scoped selected-node denial, got {other:?}"),
    }
    assert_scoped_denial_is_side_effect_free(
        &mut missing_node_runtime,
        &main,
        &before_missing_node,
    );
    assert_eq!(
        missing_node_runtime
            .telemetry()
            .transaction
            .scoped_merge_denial_count,
        1
    );

    let (mut missing_aspect_runtime, feature, main, _primary) = build_scoped_denial_runtime();
    let before_missing_aspect = missing_aspect_runtime
        .current_branch_basis_artifact()
        .payload()
        .basis_digest()
        .to_owned();
    let missing_aspect_error = match missing_aspect_runtime
        .merge()
        .from(feature.clone())
        .into(main.clone())
        .selected_aspects([SignalSelectedAspectRequestEntry::new(
            NodeId::new(777, 2),
            ASPECT_A,
        )])
        .plan()
    {
        Err(error) => error,
        Ok(_) => panic!("missing selected aspect should deny before scoped merge planning"),
    };
    match missing_aspect_error {
        SignalError::BranchMergeFailed {
            kind: BranchMergeFailureKind::ScopedMergeDenied,
            evidence: Some(BranchMergeFailureEvidence::ScopedDenial(evidence)),
            ..
        } => {
            let denied_aspect =
                SignalSelectedAspectRequestEntry::new(NodeId::new(777, 2), ASPECT_A);
            assert_eq!(
                evidence.denial_kind,
                BranchMergeScopedDenialKind::UnknownSelectedAspect
            );
            assert_eq!(
                evidence.scope_digest,
                selected_aspect_scope_digest(&feature, &main, denied_aspect.clone())
            );
            assert_eq!(
                evidence.denied_locus,
                BranchMergeScopedDeniedLocus::Aspect(denied_aspect)
            );
        }
        other => panic!("expected typed scoped selected-aspect denial, got {other:?}"),
    }
    assert_scoped_denial_is_side_effect_free(
        &mut missing_aspect_runtime,
        &main,
        &before_missing_aspect,
    );
    assert_eq!(
        missing_aspect_runtime
            .telemetry()
            .transaction
            .scoped_merge_denial_count,
        1
    );

    let (mut ambiguous_runtime, feature, main, source) = build_ambiguous_scoped_denial_runtime();
    let before_ambiguous = ambiguous_runtime
        .current_branch_basis_artifact()
        .payload()
        .basis_digest()
        .to_owned();
    let ambiguous_error = match ambiguous_runtime
        .merge()
        .from(feature.clone())
        .into(main.clone())
        .selected_nodes([source])
        .identity_matcher_named("signal.identity.output-identity-in-target-journal")
        .plan()
    {
        Err(error) => error,
        Ok(_) => {
            panic!("ambiguous selected correspondence should deny with localized scope evidence")
        }
    };
    match ambiguous_error {
        SignalError::BranchMergeFailed {
            kind: BranchMergeFailureKind::ScopedMergeDenied,
            evidence: Some(BranchMergeFailureEvidence::ScopedDenial(evidence)),
            ..
        } => {
            assert_eq!(
                evidence.denial_kind,
                BranchMergeScopedDenialKind::SelectedTargetCorrespondenceAmbiguous
            );
            assert_eq!(
                evidence.scope_digest,
                selected_node_scope_digest(&feature, &main, source)
            );
            assert_eq!(
                evidence.denied_locus,
                BranchMergeScopedDeniedLocus::Node(source)
            );
        }
        other => panic!("expected typed scoped ambiguity denial, got {other:?}"),
    }
    assert_scoped_denial_is_side_effect_free(&mut ambiguous_runtime, &main, &before_ambiguous);
    assert_eq!(
        ambiguous_runtime
            .telemetry()
            .transaction
            .scoped_merge_denial_count,
        1
    );
}

#[test]
fn unsupported_scoped_strategy_fails_as_unavailable_without_branch_mutation_or_delivery() {
    let (mut runtime, feature, main, primary) = build_scoped_denial_runtime();
    let before_current_digest = runtime
        .current_branch_basis_artifact()
        .payload()
        .basis_digest()
        .to_owned();
    let before_feature_digest = match runtime.branch_basis_artifact(feature.clone()) {
        worth_proof::TransitionOutcome::Success(artifact) => {
            artifact.payload().basis_digest().to_owned()
        }
        other => panic!("expected feature branch basis artifact, got {other:?}"),
    };
    let unavailable = match runtime
        .merge()
        .from(feature.clone())
        .into(main.clone())
        .selected_nodes([primary])
        .strategy_hint(BranchMergeStrategy::RebaseSourceOntoTarget)
        .plan()
    {
        Err(error) => error,
        Ok(_) => panic!("unsupported scoped strategy should stay unavailable and side-effect free"),
    };

    match unavailable {
        SignalError::BranchMergeFailed {
            kind: BranchMergeFailureKind::ScopedMergeUnavailable,
            evidence: Some(BranchMergeFailureEvidence::ScopedUnavailable(evidence)),
            ..
        } => {
            assert_eq!(
                evidence.reason,
                BranchMergeScopedUnavailableReason::RuntimeDoesNotSupportSelectedNodes
            );
            assert_eq!(
                evidence.scope_digest,
                selected_node_scope_digest(&feature, &main, primary)
            );
            assert_eq!(
                evidence.outcome_kind,
                BranchMergeScopedUnavailableOutcomeKind::Deferred
            );
        }
        other => panic!("expected typed scoped unavailable posture, got {other:?}"),
    }
    assert_eq!(runtime.current_branch(), main);
    assert_eq!(
        runtime
            .current_branch_basis_artifact()
            .payload()
            .basis_digest(),
        before_current_digest,
        "scoped unavailable posture must not mutate the active branch"
    );
    assert_eq!(
        match runtime.branch_basis_artifact(feature) {
            worth_proof::TransitionOutcome::Success(artifact) =>
                artifact.payload().basis_digest().to_owned(),
            other => panic!(
                "expected feature branch basis artifact after unavailable posture, got {other:?}"
            ),
        },
        before_feature_digest,
        "scoped unavailable posture must not mutate the source branch basis either"
    );
    assert_eq!(
        runtime.telemetry().transaction.delivered_observation_count,
        0
    );
    assert_eq!(runtime.telemetry().transaction.scoped_merge_denial_count, 0);
    assert_eq!(
        runtime
            .telemetry()
            .transaction
            .scoped_merge_unavailable_count,
        1
    );
}
