use forge_signal::facade::*;

const RAW_FEATURES: Aspect = Aspect::new(0);
const FEATURE_STATS: Aspect = Aspect::new(1);
const NORMALIZED: Aspect = Aspect::new(2);
const SCORE: Aspect = Aspect::new(3);

#[derive(Default)]
struct MlState {
    raw_features_version: u64,
    feature_stats_version: u64,
    normalized_version: u64,
    score_version: u64,
}

fn main() -> Result<(), SignalError> {
    let mut graph = SignalGraph::new();
    let raw_features = graph.node().build();
    let feature_stats = graph.node().build();
    let normalized_features = graph.node().build();
    let model_score = graph.node().on_demand().build();

    graph.set_dependencies(
        normalized_features,
        [
            DependencyEdge::new(raw_features, RAW_FEATURES),
            DependencyEdge::new(feature_stats, FEATURE_STATS),
        ],
    )?;
    graph.set_dependencies(
        model_score,
        [DependencyEdge::new(normalized_features, NORMALIZED)],
    )?;

    let mut runtime = SignalRuntime::build_for::<MlState>(graph);

    let mut state = MlState {
        raw_features_version: 8,
        feature_stats_version: 3,
        normalized_version: 11,
        score_version: 21,
    };

    let evaluate = |view: &mut EvaluationContext<'_, MlState>| {
        let result = if view.node() == raw_features {
            view.finish(NodeEvaluationResult::from_version(
                AspectVersion::from_updates([(RAW_FEATURES, view.domain().raw_features_version)]),
            ))
        } else if view.node() == feature_stats {
            view.finish(NodeEvaluationResult::from_version(
                AspectVersion::from_updates([(
                    FEATURE_STATS,
                    view.domain().feature_stats_version,
                )]),
            ))
        } else if view.node() == normalized_features {
            let _raw = view.read_aspect_version(raw_features, RAW_FEATURES)?;
            let _stats = view.read_aspect_version(feature_stats, FEATURE_STATS)?;
            view.finish(NodeEvaluationResult::from_version(
                AspectVersion::from_updates([(NORMALIZED, view.domain().normalized_version)]),
            ))
        } else {
            let _normalized = view.read_aspect_version(normalized_features, NORMALIZED)?;
            view.finish(NodeEvaluationResult::from_version(
                AspectVersion::from_updates([(SCORE, view.domain().score_version)]),
            ))
        };
        Ok::<_, SignalError>(result)
    };

    state.raw_features_version += 1;
    state.feature_stats_version += 1;
    state.normalized_version += 1;
    state.score_version += 1;

    runtime.transaction(&mut state, |tx| {
        tx.batch_changes()
            .mark(raw_features, RAW_FEATURES)
            .mark(feature_stats, FEATURE_STATS)
            .apply()?;
        tx.evaluate_dirty(&evaluate)?;
        Ok(())
    })?;

    let score = runtime.target(model_score).read(&state, &evaluate)?;
    assert_eq!(score.get(SCORE), 22);

    let snapshot = runtime.history().snapshot();
    let _ = snapshot;
    Ok(())
}
