mod comparison;
mod entry;
mod forensic;
mod health;
mod inspectors;

use crate::data::graph::SignalGraph;
use crate::logic::transaction::SignalRuntime;

/// Public diagnostics facade over one committed signal graph.
pub struct GraphDiagnostics<'a> {
    graph: &'a SignalGraph,
}

pub struct GraphComparisonDiagnostics;

pub struct GraphHealthDiagnostics<'a> {
    graph: &'a SignalGraph,
}

pub struct GraphInspectDiagnostics<'a> {
    graph: &'a SignalGraph,
}

pub struct GraphForensicDiagnostics<'a> {
    graph: &'a SignalGraph,
}

impl<'a> GraphDiagnostics<'a> {
    pub(crate) fn new(graph: &'a SignalGraph) -> Self {
        Self { graph }
    }
}

pub type RuntimeDiagnostics<'a> = GraphDiagnostics<'a>;

pub fn diagnostics_for_graph(graph: &SignalGraph) -> GraphDiagnostics<'_> {
    GraphDiagnostics::new(graph)
}

pub fn diagnostics_for_runtime<D, I, E, Ctx, T>(
    runtime: &SignalRuntime<D, I, E, Ctx, T>,
) -> RuntimeDiagnostics<'_>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    GraphDiagnostics::new(runtime.graph())
}
