use hadwiger_research::facade::{
    AspectDependencyGraph, ColorabilityAspectRecord, EmbeddingCandidate, GraphVersion,
    HadwigerAspectKind, HadwigerAspectPosture, HadwigerCanonicalArtifact,
    UnitDistanceAspectRecord,
};

fn closure_dx(embedding: EmbeddingCandidate, graph_version: GraphVersion) {
    let unit_distance = UnitDistanceAspectRecord::deferred(
        embedding.reference(),
        "unit-distance checker not executed in phase 3",
    )
    .expect("shape is valid");

    let colorability = ColorabilityAspectRecord::missing(
        graph_version.reference(),
        5,
        "solver verification not available yet",
    )
    .expect("shape is valid");

    let closure = AspectDependencyGraph::builder("lower-bound-closure-a")
        .with_aspect(unit_distance)
        .expect("aspect is unique")
        .with_aspect(colorability)
        .expect("aspect is unique")
        .requires(
            HadwigerAspectKind::LowerBoundWitness,
            HadwigerAspectKind::UnitDistanceEmbedding,
        )
        .expect("dependency is valid")
        .requires(
            HadwigerAspectKind::LowerBoundWitness,
            HadwigerAspectKind::NotKColorable,
        )
        .expect("dependency is valid")
        .finish()
        .expect("graph is valid")
        .evaluate_closure(HadwigerAspectKind::LowerBoundWitness);

    assert_eq!(closure.weakest_posture(), HadwigerAspectPosture::Missing);
    assert!(closure.requires_conservative_invalidation());
    assert!(!closure.admits_theorem_authority());
}

fn main() {
    let _ = closure_dx as fn(EmbeddingCandidate, GraphVersion);
}
