use forge_signal::facade::*;
use forge_signal::facade::diagnostics::DiagnosticsTier;

const PRICES: Aspect = Aspect::new(0);
const POSITIONS: Aspect = Aspect::new(1);
const RISK: Aspect = Aspect::new(2);
const PNL: Aspect = Aspect::new(3);

#[derive(Default)]
struct RiskState {
    prices_version: u64,
    positions_version: u64,
    risk_version: u64,
    pnl_version: u64,
}

fn main() -> Result<(), SignalError> {
    let mut graph = SignalGraph::new();
    let market_prices = graph.node().build();
    let positions = graph.node().build();
    let risk_summary = graph.node().on_demand().build();
    let pnl_summary = graph.node().on_demand().build();

    graph.set_dependencies(
        risk_summary,
        [
            DependencyEdge::new(market_prices, PRICES),
            DependencyEdge::new(positions, POSITIONS),
        ],
    )?;
    graph.set_dependencies(
        pnl_summary,
        [
            DependencyEdge::new(market_prices, PRICES),
            DependencyEdge::new(positions, POSITIONS),
        ],
    )?;

    let mut runtime = SignalRuntime::build_for::<RiskState>(graph);

    let mut state = RiskState {
        prices_version: 100,
        positions_version: 40,
        risk_version: 12,
        pnl_version: 77,
    };

    let evaluate = |view: &mut EvaluationContext<'_, RiskState>| {
        let result = if view.node() == market_prices {
            view.finish(NodeEvaluationResult::from_version(
                AspectVersion::from_updates([(PRICES, view.domain().prices_version)]),
            ))
        } else if view.node() == positions {
            view.finish(NodeEvaluationResult::from_version(
                AspectVersion::from_updates([(POSITIONS, view.domain().positions_version)]),
            ))
        } else if view.node() == risk_summary {
            let _prices = view.read_aspect_version(market_prices, PRICES)?;
            let _positions = view.read_aspect_version(positions, POSITIONS)?;
            view.finish(NodeEvaluationResult::from_version(
                AspectVersion::from_updates([(RISK, view.domain().risk_version)]),
            ))
        } else {
            let _prices = view.read_aspect_version(market_prices, PRICES)?;
            let _positions = view.read_aspect_version(positions, POSITIONS)?;
            view.finish(NodeEvaluationResult::from_version(
                AspectVersion::from_updates([(PNL, view.domain().pnl_version)]),
            ))
        };
        Ok::<_, SignalError>(result)
    };

    state.prices_version += 1;
    state.positions_version += 1;
    state.risk_version += 1;
    state.pnl_version += 1;

    runtime.transaction(&mut state, |tx| {
        tx.batch_changes()
            .mark(market_prices, PRICES)
            .mark(positions, POSITIONS)
            .apply()?;
        tx.read_many(&[risk_summary, pnl_summary], &evaluate)?;
        Ok(())
    })?;

    let versions = runtime.read_many(&[risk_summary, pnl_summary], &state, &evaluate)?;
    assert_eq!(versions[0].get(RISK), 13);
    assert_eq!(versions[1].get(PNL), 78);

    let _health = runtime.diagnostics().health_now();
    let _ = DiagnosticsTier::Development;
    Ok(())
}
