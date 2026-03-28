use forge_signal::facade::*;

const QUERY: Aspect = Aspect::new(0);
const CATALOG: Aspect = Aspect::new(1);
const RESULTS: Aspect = Aspect::new(2);

#[derive(Default)]
struct WebState {
    query_version: u64,
    catalog_version: u64,
    results_version: u64,
}

fn main() -> Result<(), SignalError> {
    let mut graph = SignalGraph::new();
    let query_input = graph.node().build();
    let product_index = graph.node().build();
    let search_results = graph.node().on_demand().build();

    graph.set_dependencies(
        search_results,
        [
            DependencyEdge::new(query_input, QUERY),
            DependencyEdge::new(product_index, CATALOG),
        ],
    )?;

    let mut runtime = SignalRuntime::build_for::<WebState>(graph);

    let mut state = WebState {
        query_version: 4,
        catalog_version: 12,
        results_version: 29,
    };

    let evaluate = |view: &mut EvaluationContext<'_, WebState>| {
        let result = if view.node() == query_input {
            view.finish(NodeEvaluationResult::from_version(
                AspectVersion::from_updates([(QUERY, view.domain().query_version)]),
            ))
        } else if view.node() == product_index {
            view.finish(NodeEvaluationResult::from_version(
                AspectVersion::from_updates([(CATALOG, view.domain().catalog_version)]),
            ))
        } else {
            let _query = view.read_aspect_version(query_input, QUERY)?;
            let _catalog = view.read_aspect_version(product_index, CATALOG)?;
            view.finish(NodeEvaluationResult::from_version(
                AspectVersion::from_updates([(RESULTS, view.domain().results_version)]),
            ))
        };
        Ok::<_, SignalError>(result)
    };

    state.query_version += 1;
    state.results_version += 1;

    runtime.transaction(&mut state, |tx| {
        tx.batch_changes().mark(query_input, QUERY).apply()?;
        tx.target(search_results).read(&evaluate)?;
        Ok(())
    })?;

    let version = runtime.target(search_results).read(&state, &evaluate)?;
    assert_eq!(version.get(RESULTS), 30);

    let _why = runtime.diagnostics().why(search_results)?;
    Ok(())
}
