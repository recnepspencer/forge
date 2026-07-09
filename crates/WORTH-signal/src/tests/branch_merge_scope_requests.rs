use crate::facade::*;
use crate::logic::transaction::{BranchMergeScopedDenialKind, BranchMergeScopedDeniedLocus};
use crate::tests::support::{ASPECT_A, ASPECT_B};

#[test]
fn guided_merge_defaults_and_explicit_full_branch_requests_normalize_identically() {
    let graph = SignalGraph::new();
    let mut runtime = SignalRuntime::builder(graph).with_kernel_defaults().build();

    let feature = runtime
        .create_branch("feature")
        .expect("feature branch creation should succeed");
    let main = runtime.current_branch();

    let default_request = runtime
        .merge()
        .from(feature.clone())
        .into(main.clone())
        .build_normalized_request()
        .expect("default guided merge should build a normalized full-branch request");
    let explicit_request = runtime
        .merge()
        .from(feature)
        .into(main)
        .full_branch()
        .build_normalized_request()
        .expect("explicit full-branch guided merge should build a normalized request");

    let default_scope = default_request.normalized_scope();
    let explicit_scope = explicit_request.normalized_scope();

    assert_eq!(
        default_request.request().scope,
        BranchMergeRequestScope::FullBranch
    );
    assert_eq!(
        explicit_request.request().scope,
        BranchMergeRequestScope::FullBranch
    );
    assert_eq!(
        default_scope.family(),
        BranchMergeRequestScopeFamily::FullBranch
    );
    assert_eq!(default_scope.scope_digest(), explicit_scope.scope_digest());
}

#[test]
fn selected_node_requests_normalize_duplicates_and_order_canonically() {
    let source = SignalBranchHandle {
        id: SignalBranchId(7),
        name: "source".to_string(),
        parent_branch_id: Some(SignalBranchId(0)),
        head_snapshot_id: None,
    };
    let target = SignalBranchHandle {
        id: SignalBranchId(8),
        name: "target".to_string(),
        parent_branch_id: Some(SignalBranchId(0)),
        head_snapshot_id: None,
    };
    let node_a = NodeId::new(7, 1);
    let node_b = NodeId::new(3, 2);

    let unordered = BranchMergeRequest::selected_nodes(
        source.clone(),
        target.clone(),
        [node_a, node_b, node_a],
    );
    let canonical = BranchMergeRequest::selected_nodes(source, target, [node_b, node_a]);

    let unordered_scope = unordered
        .normalize_scope()
        .expect("selected-node scope should normalize");
    let canonical_scope = canonical
        .normalize_scope()
        .expect("selected-node scope should normalize");

    assert_eq!(
        unordered_scope.family(),
        BranchMergeRequestScopeFamily::SelectedNodes
    );
    assert_eq!(
        unordered_scope.scope_digest(),
        canonical_scope.scope_digest()
    );
    assert_eq!(unordered_scope.selected_nodes(), &[node_b, node_a]);
}

#[test]
fn guided_and_proof_visible_selected_node_requests_normalize_identically() {
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
        .build_normalized_request()
        .expect("guided selected-node request should normalize");
    let proof_visible_scope = BranchMergeRequest::selected_nodes(feature, main, [node_b, node_a])
        .normalize_scope()
        .expect("proof-visible selected-node request should normalize");

    assert_eq!(
        guided_scope.normalized_scope().family(),
        BranchMergeRequestScopeFamily::SelectedNodes
    );
    assert_eq!(
        guided_scope.normalized_scope().scope_digest(),
        proof_visible_scope.scope_digest()
    );
    assert_eq!(
        guided_scope.normalized_scope().selected_nodes(),
        proof_visible_scope.selected_nodes()
    );
}

#[test]
fn selected_aspect_requests_normalize_duplicates_and_order_canonically() {
    let source = SignalBranchHandle {
        id: SignalBranchId(7),
        name: "source".to_string(),
        parent_branch_id: Some(SignalBranchId(0)),
        head_snapshot_id: None,
    };
    let target = SignalBranchHandle {
        id: SignalBranchId(8),
        name: "target".to_string(),
        parent_branch_id: Some(SignalBranchId(0)),
        head_snapshot_id: None,
    };

    let node_a = NodeId::new(7, 1);
    let node_b = NodeId::new(3, 2);
    let unordered = BranchMergeRequest::selected_aspects(
        source.clone(),
        target.clone(),
        [
            SignalSelectedAspectRequestEntry::new(node_a, ASPECT_B),
            SignalSelectedAspectRequestEntry::new(node_a, ASPECT_A),
            SignalSelectedAspectRequestEntry::new(node_b, ASPECT_B),
            SignalSelectedAspectRequestEntry::new(node_a, ASPECT_B),
        ],
    );
    let canonical = BranchMergeRequest::selected_aspects(
        source,
        target,
        [
            SignalSelectedAspectRequestEntry::new(node_a, ASPECT_A),
            SignalSelectedAspectRequestEntry::new(node_a, ASPECT_B),
            SignalSelectedAspectRequestEntry::new(node_b, ASPECT_B),
        ],
    );

    let unordered_scope = unordered
        .normalize_scope()
        .expect("selected-aspect scope should normalize");
    let canonical_scope = canonical
        .normalize_scope()
        .expect("selected-aspect scope should normalize");

    assert_eq!(
        unordered_scope.family(),
        BranchMergeRequestScopeFamily::SelectedAspects
    );
    assert_eq!(
        unordered_scope.scope_digest(),
        canonical_scope.scope_digest()
    );
    assert_eq!(
        unordered_scope.selected_aspects(),
        &[
            SignalSelectedAspectRequestEntry::new(node_b, ASPECT_B),
            SignalSelectedAspectRequestEntry::new(node_a, ASPECT_A),
            SignalSelectedAspectRequestEntry::new(node_a, ASPECT_B),
        ]
    );
}

#[test]
fn guided_and_proof_visible_selected_aspect_requests_normalize_identically() {
    let graph = SignalGraph::new();
    let mut runtime = SignalRuntime::builder(graph).with_kernel_defaults().build();
    let feature = runtime
        .create_branch("feature")
        .expect("feature branch creation should succeed");
    let main = runtime.current_branch();

    let guided_scope = runtime
        .merge()
        .from(feature.clone())
        .into(main.clone())
        .selected_aspects([
            SignalSelectedAspectRequestEntry::new(NodeId::new(7, 1), ASPECT_B),
            SignalSelectedAspectRequestEntry::new(NodeId::new(7, 1), ASPECT_A),
            SignalSelectedAspectRequestEntry::new(NodeId::new(7, 1), ASPECT_B),
        ])
        .build_normalized_request()
        .expect("guided selected-aspect request should normalize");
    let proof_visible_scope = BranchMergeRequest::selected_aspects(
        feature,
        main,
        [
            SignalSelectedAspectRequestEntry::new(NodeId::new(7, 1), ASPECT_A),
            SignalSelectedAspectRequestEntry::new(NodeId::new(7, 1), ASPECT_B),
        ],
    )
    .normalize_scope()
    .expect("proof-visible selected-aspect request should normalize");

    assert_eq!(
        guided_scope.normalized_scope().family(),
        BranchMergeRequestScopeFamily::SelectedAspects
    );
    assert_eq!(
        guided_scope.normalized_scope().scope_digest(),
        proof_visible_scope.scope_digest()
    );
    assert_eq!(
        guided_scope.normalized_scope().selected_aspects(),
        proof_visible_scope.selected_aspects()
    );
}

#[test]
fn empty_scoped_requests_fail_before_merge_planning_begins() {
    let graph = SignalGraph::new();
    let mut runtime = SignalRuntime::builder(graph).with_kernel_defaults().build();

    let feature = runtime
        .create_branch("feature")
        .expect("feature branch creation should succeed");
    let main = runtime.current_branch();
    let plans_before = runtime.observe().metrics().planner.plans_built;

    let empty_nodes_request = runtime
        .merge()
        .from(feature.clone())
        .into(main.clone())
        .selected_nodes(Vec::<NodeId>::new())
        .build_request()
        .expect("guided merge should build an empty selected-node request for boundary denial");
    let empty_aspects_request = runtime
        .merge()
        .from(feature.clone())
        .into(main.clone())
        .selected_aspects(Vec::<SignalSelectedAspectRequestEntry>::new())
        .build_request()
        .expect("guided merge should build an empty selected-aspect request for boundary denial");

    assert_eq!(
        empty_nodes_request.normalize_scope(),
        Err(BranchMergeRequestDenial::EmptySelectedNodes)
    );
    assert_eq!(
        empty_aspects_request.normalize_scope(),
        Err(BranchMergeRequestDenial::EmptySelectedAspects)
    );

    let err = match runtime
        .merge()
        .from(feature.clone())
        .into(main.clone())
        .selected_nodes(Vec::<NodeId>::new())
        .plan()
    {
        Ok(_) => panic!("empty selected-node request should deny before planning"),
        Err(err) => err,
    };
    assert!(
        err.to_string()
            .contains("selected-node merge requests must name at least one source node"),
        "expected typed empty selected-node denial, got {err:?}"
    );
    let err = match runtime
        .merge()
        .from(feature)
        .into(main)
        .selected_aspects(Vec::<SignalSelectedAspectRequestEntry>::new())
        .plan()
    {
        Ok(_) => panic!("empty selected-aspect request should deny before planning"),
        Err(err) => err,
    };
    assert!(
        err.to_string()
            .contains("selected-aspect merge requests must name at least one aspect"),
        "expected typed empty selected-aspect denial, got {err:?}"
    );
    assert_eq!(
        runtime.observe().metrics().planner.plans_built,
        plans_before,
        "malformed scoped requests must fail before planner work begins"
    );
}

#[test]
fn scoped_requests_do_not_silently_widen_into_full_branch_planning() {
    let graph = SignalGraph::new();
    let mut runtime = SignalRuntime::builder(graph).with_kernel_defaults().build();
    let scoped_node = runtime.graph_mut().node().build();
    let feature = runtime
        .create_branch("feature")
        .expect("feature branch creation should succeed");
    let main = runtime.current_branch();

    let err = match runtime
        .merge()
        .from(feature)
        .into(main)
        .selected_nodes([scoped_node])
        .plan()
    {
        Err(err) => err,
        Ok(_) => panic!("phase-6 scoped requests must deny instead of silently widening a skipped selected node into full-branch planning"),
    };

    match err {
        SignalError::BranchMergeFailed {
            kind: BranchMergeFailureKind::ScopedMergeDenied,
            evidence: Some(BranchMergeFailureEvidence::ScopedDenial(evidence)),
            ..
        } => {
            assert_eq!(
                evidence.denial_kind,
                BranchMergeScopedDenialKind::SelectedNodeMissingFromSourceScope
            );
            assert_eq!(
                evidence.denied_locus,
                BranchMergeScopedDeniedLocus::Node(scoped_node)
            );
        }
        other => panic!("expected typed scoped selected-node denial, got {other:?}"),
    }
}

#[test]
fn selected_node_and_selected_aspect_requests_keep_family_distinct_even_on_same_locus() {
    let graph = SignalGraph::new();
    let mut runtime = SignalRuntime::builder(graph).with_kernel_defaults().build();
    let node = runtime.graph_mut().node().build();
    let feature = runtime
        .create_branch("feature")
        .expect("feature branch creation should succeed");
    let main = runtime.current_branch();

    let selected_nodes = runtime
        .merge()
        .from(feature.clone())
        .into(main.clone())
        .selected_nodes([node])
        .build_normalized_request()
        .expect("selected-node request should normalize");
    let selected_aspects = runtime
        .merge()
        .from(feature)
        .into(main)
        .selected_aspects([SignalSelectedAspectRequestEntry::new(node, ASPECT_A)])
        .build_normalized_request()
        .expect("selected-aspect request should normalize");

    assert_eq!(
        selected_nodes.normalized_scope().family(),
        BranchMergeRequestScopeFamily::SelectedNodes
    );
    assert_eq!(
        selected_aspects.normalized_scope().family(),
        BranchMergeRequestScopeFamily::SelectedAspects
    );
    assert_ne!(
        selected_nodes.normalized_scope().scope_digest(),
        selected_aspects.normalized_scope().scope_digest(),
        "family-distinct scoped requests must not collapse to the same normalized identity just because they mention the same node"
    );
}
