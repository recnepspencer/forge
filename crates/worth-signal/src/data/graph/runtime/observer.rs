mod branches;
mod explanation;
mod lineage;
mod materialization;
mod replay;
mod summary;

use crate::data::graph::signal_graph::SignalGraph;

pub struct GraphObserver<'a> {
    graph: &'a SignalGraph,
}

pub struct GraphMaterializer<'a> {
    graph: &'a SignalGraph,
}

impl<'a> GraphObserver<'a> {
    pub(crate) fn new(graph: &'a SignalGraph) -> Self {
        Self { graph }
    }

    pub fn graph(&self) -> &'a SignalGraph {
        self.graph
    }

    pub fn materialize(&self) -> GraphMaterializer<'a> {
        GraphMaterializer { graph: self.graph }
    }
}
