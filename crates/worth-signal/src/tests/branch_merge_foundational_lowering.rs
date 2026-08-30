use worth_foundational::facade::FoundationalMergeScopeFamily;

use crate::facade::*;
use crate::logic::transaction::{BranchMergeScopedDenialKind, BranchMergeScopedDeniedLocus};
use crate::tests::support::{ASPECT_A, ASPECT_B};

#[test]
fn guided_and_explicit_full_branch_requests_lower_to_identical_foundational_scope() {
    let graph = SignalGraph::new();
    let mut runtime = SignalRuntime::builder(graph).with_kernel_defaults().build();

    let feature = runtime
        .create_branch("feature")
        .expect("feature branch creation should succeed");
    let main = runtime.current_branch();

    let default_scope = runtime
        .merge()
        .from(feature.clone())
        .into(main.clone())
        .build_lowered_foundational_request()
        .expect("default guided merge should lower to foundational full-branch scope");
    let explicit_scope = runtime
        .merge()
        .from(feature)
        .into(main)
        .full_branch()
        .build_lowered_foundational_request()
        .expect("explicit full-branch merge should lower to foundational scope");

    assert_eq!(
        default_scope.foundational_scope().family(),
        FoundationalMergeScopeFamily::FullBranch
    );
    assert_eq!(
        default_scope.foundational_scope(),
        explicit_scope.foundational_scope()
    );
    assert_eq!(
        runtime
            .telemetry()
            .transaction
            .foundational_scope_lowering_count,
        2
    );
    assert_eq!(
        runtime
            .telemetry()
            .transaction
            .foundational_full_branch_lowering_count,
        2
    );
    assert_eq!(
        runtime
            .telemetry()
            .transaction
            .foundational_scope_lowering_denial_count,
        0
    );
}

#[test]
fn selected_node_scope_lowers_identically_across_guided_and_proof_visible_lanes() {
    let graph = SignalGraph::new();
    let mut runtime = SignalRuntime::builder(graph).with_kernel_defaults().build();

    let feature = runtime
        .create_branch("feature")
        .expect("feature branch creation should succeed");
    let main = runtime.current_branch();
    let node_a = NodeId::new(7, 1);
    let node_b = NodeId::new(3, 2);

    let guided_scope = runtime
        .merge()
        .from(feature.clone())
        .into(main.clone())
        .selected_nodes([node_a, node_b, node_a])
        .build_lowered_foundational_request()
        .expect("guided selected-node request should lower to foundational scope");
    let proof_visible_scope = BranchMergeRequest::selected_nodes(feature, main, [node_b, node_a])
        .normalize()
        .expect("proof-visible selected-node request should normalize")
        .lower_to_foundational_scope()
        .expect("proof-visible selected-node request should lower to foundational scope");

    assert_eq!(
        guided_scope.foundational_scope().family(),
        FoundationalMergeScopeFamily::SelectedNodes
    );
    assert_eq!(
        guided_scope.foundational_scope().selected_nodes_loci(),
        proof_visible_scope
            .foundational_scope()
            .selected_nodes_loci()
    );
    assert_eq!(
        guided_scope.foundational_scope(),
        proof_visible_scope.foundational_scope()
    );
    assert_eq!(
        runtime
            .telemetry()
            .transaction
            .foundational_scope_lowering_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .transaction
            .foundational_selected_node_lowering_count,
        1
    );
}

#[test]
fn selected_aspect_scope_lowers_identically_across_guided_and_proof_visible_lanes() {
    let graph = SignalGraph::new();
    let mut runtime = SignalRuntime::builder(graph).with_kernel_defaults().build();

    let feature = runtime
        .create_branch("feature")
        .expect("feature branch creation should succeed");
    let main = runtime.current_branch();
    let node = NodeId::new(7, 1);

    let guided_scope = runtime
        .merge()
        .from(feature.clone())
        .into(main.clone())
        .selected_aspects([
            SignalSelectedAspectRequestEntry::new(node, ASPECT_B),
            SignalSelectedAspectRequestEntry::new(node, ASPECT_A),
            SignalSelectedAspectRequestEntry::new(node, ASPECT_B),
        ])
        .build_lowered_foundational_request()
        .expect("guided selected-aspect request should lower to foundational scope");
    let proof_visible_scope = BranchMergeRequest::selected_aspects(
        feature,
        main,
        [
            SignalSelectedAspectRequestEntry::new(node, ASPECT_A),
            SignalSelectedAspectRequestEntry::new(node, ASPECT_B),
        ],
    )
    .normalize()
    .expect("proof-visible selected-aspect request should normalize")
    .lower_to_foundational_scope()
    .expect("proof-visible selected-aspect request should lower to foundational scope");

    assert_eq!(
        guided_scope.foundational_scope().family(),
        FoundationalMergeScopeFamily::SelectedAspects
    );
    assert_eq!(
        guided_scope.foundational_scope().selected_aspect_loci(),
        proof_visible_scope
            .foundational_scope()
            .selected_aspect_loci()
    );
    assert_eq!(
        guided_scope.foundational_scope(),
        proof_visible_scope.foundational_scope()
    );
    assert_eq!(
        runtime
            .telemetry()
            .transaction
            .foundational_scope_lowering_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .transaction
            .foundational_selected_aspect_lowering_count,
        1
    );
}

#[test]
fn scoped_requests_keep_foundational_family_visible_when_planning_narrows_candidates() {
    let graph = SignalGraph::new();
    let mut runtime = SignalRuntime::builder(graph).with_kernel_defaults().build();
    let scoped_node = runtime.graph_mut().node().build();
    let feature = runtime
        .create_branch("feature")
        .expect("feature branch creation should succeed");
    let main = runtime.current_branch();

    let lowered_request = runtime
        .merge()
        .from(feature.clone())
        .into(main.clone())
        .selected_nodes([scoped_node])
        .build_lowered_foundational_request()
        .expect("selected-node request should lower before candidate planning exists");
    assert_eq!(
        lowered_request.foundational_scope().family(),
        FoundationalMergeScopeFamily::SelectedNodes
    );

    let err = match runtime
        .merge()
        .from(feature)
        .into(main)
        .selected_nodes([scoped_node])
        .plan()
    {
        Err(err) => err,
        Ok(_) => panic!("phase-6 scoped request should deny once foundational lowering reaches a missing selected-node boundary"),
    };
    match err {
        SignalError::BranchMergeFailed {
            kind: BranchMergeFailureKind::ScopedMergeDenied,
            evidence: Some(evidence),
            ..
        } => {
            let BranchMergeFailureEvidence::ScopedDenial(evidence) = *evidence else {
                panic!("expected scoped denial evidence")
            };
            assert_eq!(
                evidence.denial_kind,
                BranchMergeScopedDenialKind::SelectedNodeMissingFromSourceScope
            );
            assert_eq!(
                evidence.denied_locus,
                BranchMergeScopedDeniedLocus::Node(scoped_node)
            );
        }
        other => panic!("expected typed scoped denial after foundational lowering, got {other:?}"),
    }
    assert_eq!(
        runtime
            .telemetry()
            .transaction
            .foundational_scope_lowering_count,
        2
    );
    assert_eq!(
        runtime
            .telemetry()
            .transaction
            .foundational_selected_node_lowering_count,
        2
    );
}

#[test]
fn compatibility_merge_branch_uses_the_same_foundational_full_branch_lowering_boundary() {
    let graph = SignalGraph::new();
    let mut runtime = SignalRuntime::builder(graph).with_kernel_defaults().build();

    let feature = runtime
        .create_branch("feature")
        .expect("feature branch creation should succeed");
    let main = runtime.current_branch();

    runtime
        .merge_branch(feature, main)
        .expect("compatibility merge should still lower through foundational full-branch scope");

    assert_eq!(
        runtime
            .telemetry()
            .transaction
            .foundational_scope_lowering_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .transaction
            .foundational_full_branch_lowering_count,
        1
    );
}

#[test]
fn malformed_scoped_requests_fail_before_foundational_lowering_and_distinct_families_stay_distinct()
{
    let graph = SignalGraph::new();
    let mut runtime = SignalRuntime::builder(graph).with_kernel_defaults().build();
    let node = runtime.graph_mut().node().build();
    let feature = runtime
        .create_branch("feature")
        .expect("feature branch creation should succeed");
    let main = runtime.current_branch();

    let lowering_count_before = runtime
        .telemetry()
        .transaction
        .foundational_scope_lowering_count;
    let err = runtime
        .merge()
        .from(feature.clone())
        .into(main.clone())
        .selected_nodes(Vec::<NodeId>::new())
        .build_lowered_foundational_request()
        .expect_err("empty scoped request should deny before foundational lowering");
    assert!(
        err.to_string()
            .contains("selected-node merge requests must name at least one source node"),
        "expected boundary denial before lowering, got {err:?}"
    );
    assert_eq!(
        runtime
            .telemetry()
            .transaction
            .foundational_scope_lowering_count,
        lowering_count_before,
        "malformed scoped requests must not cross the foundational lowering seam"
    );

    let selected_nodes = runtime
        .merge()
        .from(feature.clone())
        .into(main.clone())
        .selected_nodes([node])
        .build_lowered_foundational_request()
        .expect("selected-node request should lower");
    let selected_aspects = runtime
        .merge()
        .from(feature)
        .into(main)
        .selected_aspects([SignalSelectedAspectRequestEntry::new(node, ASPECT_A)])
        .build_lowered_foundational_request()
        .expect("selected-aspect request should lower");

    assert_ne!(
        selected_nodes.foundational_scope(),
        selected_aspects.foundational_scope(),
        "family-distinct scoped requests must stay distinct after foundational lowering"
    );
}
