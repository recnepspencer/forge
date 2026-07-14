use hadwiger_research::facade::*;

fn graph_version() -> GraphVersion {
    let handle = crate::installed_support::installed_hadwiger_research_handle()
        .expect("default Hadwiger research handle should admit");
    let declaration = declare_research_request_checked(
        &handle,
        CandidateGraphDeclaration::new("candidate-a").with_graph_version("v1"),
    )
    .admitted()
    .expect("candidate graph declaration should admit");
    let graph = GraphIdentity::from_query_declaration("candidate-a", declaration.into())
        .expect("query declaration should build graph identity");
    GraphVersion::builder(graph.reference(), "v1")
        .with_vertex("a")
        .unwrap()
        .with_vertex("b")
        .unwrap()
        .with_undirected_edge("a", "b")
        .unwrap()
        .finish()
        .unwrap()
}

#[test]
fn aspect_records_preserve_artifact_authority_and_stable_identity() {
    let version = graph_version();
    let graph_shape = GraphShapeAspectRecord::admitted_shape(version.reference()).unwrap();
    let embedding = EmbeddingCandidate::new(version.reference(), "embedding-a").unwrap();
    let unit_distance =
        UnitDistanceAspectRecord::deferred(embedding.reference(), "not checked").unwrap();

    let graph_shape: HadwigerAspectRecord = graph_shape.into();
    let unit_distance: HadwigerAspectRecord = unit_distance.into();

    assert_eq!(
        graph_shape.aspect_kind(),
        HadwigerAspectKind::GraphVersionShape
    );
    assert_eq!(
        graph_shape.aspect_posture(),
        HadwigerAspectPosture::Admitted
    );
    assert_eq!(
        graph_shape.authority_owner(),
        HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder
    );
    assert_eq!(
        unit_distance.aspect_kind(),
        HadwigerAspectKind::UnitDistanceEmbedding
    );
    assert_eq!(
        unit_distance.aspect_posture(),
        HadwigerAspectPosture::Deferred
    );
    assert_eq!(
        unit_distance.authority_owner(),
        HadwigerArtifactAuthorityOwner::Checker
    );
    assert!(!unit_distance.stable_token().is_empty());
}

#[test]
fn dependency_closure_reports_missing_dependencies_and_conservative_escalation() {
    let version = graph_version();
    let embedding = EmbeddingCandidate::new(version.reference(), "embedding-a").unwrap();
    let unit_distance =
        UnitDistanceAspectRecord::deferred(embedding.reference(), "not checked").unwrap();
    let colorability = ColorabilityAspectRecord::missing(
        version.reference(),
        5,
        "solver verification not available yet",
    )
    .unwrap();

    let closure = AspectDependencyGraph::builder("lower-bound-closure-a")
        .with_aspect(unit_distance)
        .unwrap()
        .with_aspect(colorability)
        .unwrap()
        .requires(
            HadwigerAspectKind::LowerBoundWitness,
            HadwigerAspectKind::UnitDistanceEmbedding,
        )
        .unwrap()
        .requires(
            HadwigerAspectKind::LowerBoundWitness,
            HadwigerAspectKind::NotKColorable,
        )
        .unwrap()
        .finish()
        .unwrap()
        .evaluate_closure(HadwigerAspectKind::LowerBoundWitness);

    assert_eq!(closure.weakest_posture(), HadwigerAspectPosture::Missing);
    assert!(closure.requires_conservative_invalidation());
    assert!(!closure.admits_theorem_authority());
    assert_eq!(closure.closure_graph_id(), "lower-bound-closure-a");
    assert_eq!(closure.required_aspects().len(), 2);
    assert_eq!(closure.present_aspects().len(), 2);
    assert_eq!(closure.present_aspect_tokens().len(), 2);
    assert_eq!(closure.blockers().len(), 2);
    assert!(!closure.closure_digest().is_empty());
}

#[test]
fn equivalent_dependency_graphs_converge_despite_insertion_order() {
    let version = graph_version();
    let embedding = EmbeddingCandidate::new(version.reference(), "embedding-a").unwrap();
    let unit_distance =
        UnitDistanceAspectRecord::deferred(embedding.reference(), "not checked").unwrap();
    let colorability =
        ColorabilityAspectRecord::deferred(version.reference(), 5, "not checked").unwrap();

    let left = AspectDependencyGraph::builder("closure-a")
        .with_aspect(unit_distance.clone())
        .unwrap()
        .with_aspect(colorability.clone())
        .unwrap()
        .requires(
            HadwigerAspectKind::LowerBoundWitness,
            HadwigerAspectKind::UnitDistanceEmbedding,
        )
        .unwrap()
        .requires(
            HadwigerAspectKind::LowerBoundWitness,
            HadwigerAspectKind::NotKColorable,
        )
        .unwrap()
        .finish()
        .unwrap()
        .evaluate_closure(HadwigerAspectKind::LowerBoundWitness);
    let right = AspectDependencyGraph::builder("closure-a")
        .requires(
            HadwigerAspectKind::LowerBoundWitness,
            HadwigerAspectKind::NotKColorable,
        )
        .unwrap()
        .with_aspect(colorability)
        .unwrap()
        .requires(
            HadwigerAspectKind::LowerBoundWitness,
            HadwigerAspectKind::UnitDistanceEmbedding,
        )
        .unwrap()
        .with_aspect(unit_distance)
        .unwrap()
        .finish()
        .unwrap()
        .evaluate_closure(HadwigerAspectKind::LowerBoundWitness);

    assert_eq!(left.closure_digest(), right.closure_digest());
    assert_eq!(left.required_aspects(), right.required_aspects());
    assert_eq!(left.blockers(), right.blockers());
}

#[test]
fn closure_digest_changes_when_present_aspect_identity_changes() {
    let version = graph_version();
    let embedding = EmbeddingCandidate::new(version.reference(), "embedding-a").unwrap();

    let left = AspectDependencyGraph::builder("closure-a")
        .with_aspect(
            UnitDistanceAspectRecord::deferred(embedding.reference(), "not checked").unwrap(),
        )
        .unwrap()
        .requires(
            HadwigerAspectKind::LowerBoundWitness,
            HadwigerAspectKind::UnitDistanceEmbedding,
        )
        .unwrap()
        .finish()
        .unwrap()
        .evaluate_closure(HadwigerAspectKind::LowerBoundWitness);
    let right = AspectDependencyGraph::builder("closure-a")
        .with_aspect(
            UnitDistanceAspectRecord::deferred(embedding.reference(), "different deferred source")
                .unwrap(),
        )
        .unwrap()
        .requires(
            HadwigerAspectKind::LowerBoundWitness,
            HadwigerAspectKind::UnitDistanceEmbedding,
        )
        .unwrap()
        .finish()
        .unwrap()
        .evaluate_closure(HadwigerAspectKind::LowerBoundWitness);

    assert_ne!(left.closure_digest(), right.closure_digest());
    assert_ne!(left.present_aspect_tokens(), right.present_aspect_tokens());
    assert!(left.blockers()[0].observed_aspect_token().is_some());
}

#[test]
fn graph_builder_rejects_duplicate_self_and_cyclic_dependencies() {
    assert_eq!(
        AspectDependencyGraph::builder("closure-a").requires(
            HadwigerAspectKind::LowerBoundWitness,
            HadwigerAspectKind::LowerBoundWitness
        ),
        Err(HadwigerAspectAuthorityError::SelfDependency {
            aspect_kind: HadwigerAspectKind::LowerBoundWitness
        })
    );
    assert_eq!(
        AspectDependencyGraph::builder("closure-a")
            .requires(
                HadwigerAspectKind::LowerBoundWitness,
                HadwigerAspectKind::NotKColorable
            )
            .unwrap()
            .requires(
                HadwigerAspectKind::LowerBoundWitness,
                HadwigerAspectKind::NotKColorable
            ),
        Err(HadwigerAspectAuthorityError::DuplicateDependency {
            required_by: HadwigerAspectKind::LowerBoundWitness,
            required: HadwigerAspectKind::NotKColorable
        })
    );
    assert_eq!(
        AspectDependencyGraph::builder("closure-a")
            .requires(
                HadwigerAspectKind::LowerBoundWitness,
                HadwigerAspectKind::NotKColorable
            )
            .unwrap()
            .requires(
                HadwigerAspectKind::NotKColorable,
                HadwigerAspectKind::LowerBoundWitness
            )
            .unwrap()
            .finish(),
        Err(HadwigerAspectAuthorityError::CyclicDependency {
            aspect_kind: HadwigerAspectKind::NotKColorable
        })
    );
}

#[test]
fn closure_blockers_preserve_distinct_non_admitted_postures() {
    let version = graph_version();
    let embedding = EmbeddingCandidate::new(version.reference(), "embedding-a").unwrap();
    let graph = AspectDependencyGraph::builder("closure-blocker-matrix")
        .with_aspect(UnitDistanceAspectRecord::rejected(embedding.reference(), "bad edge").unwrap())
        .unwrap()
        .with_aspect(
            ColorabilityAspectRecord::unsupported(version.reference(), 5, "solver gap").unwrap(),
        )
        .unwrap()
        .with_dependency(
            HadwigerAspectKind::LowerBoundWitness,
            HadwigerAspectKind::UnitDistanceEmbedding,
            HadwigerAspectDependencyRole::MathematicalRequirement,
            HadwigerAspectInvalidationScope::DependencyClosure,
        )
        .unwrap()
        .with_dependency(
            HadwigerAspectKind::LowerBoundWitness,
            HadwigerAspectKind::NotKColorable,
            HadwigerAspectDependencyRole::MathematicalRequirement,
            HadwigerAspectInvalidationScope::UnsafeLocalScope,
        )
        .unwrap()
        .finish()
        .unwrap();
    let closure = graph.evaluate_closure(HadwigerAspectKind::LowerBoundWitness);

    let postures = closure
        .blockers()
        .iter()
        .map(HadwigerDependencyClosureBlocker::observed_posture)
        .collect::<Vec<_>>();
    assert!(postures.contains(&HadwigerAspectPosture::Rejected));
    assert!(postures.contains(&HadwigerAspectPosture::Unsupported));
    assert_eq!(closure.weakest_posture(), HadwigerAspectPosture::Rejected);
    assert!(closure.requires_conservative_invalidation());
}

#[test]
fn advisory_aspects_do_not_satisfy_mathematical_dependencies() {
    let version = graph_version();
    let advisory = AIAdvisoryArtifact::new(
        version.reference(),
        "motif looks reusable",
        "advisory-source-digest",
    )
    .unwrap();
    let graph = AspectDependencyGraph::builder("advisory-math-boundary")
        .with_aspect(
            AdvisoryAspectRecord::advisory(advisory.reference(), "advisory source").unwrap(),
        )
        .unwrap()
        .with_dependency(
            HadwigerAspectKind::LowerBoundWitness,
            HadwigerAspectKind::AIAdvisory,
            HadwigerAspectDependencyRole::MathematicalRequirement,
            HadwigerAspectInvalidationScope::DependencyClosure,
        )
        .unwrap()
        .finish()
        .unwrap();
    let closure = graph.evaluate_closure(HadwigerAspectKind::LowerBoundWitness);

    assert_eq!(closure.weakest_posture(), HadwigerAspectPosture::Advisory);
    assert_eq!(closure.blockers().len(), 1);
    assert!(!closure.admits_theorem_authority());
}
