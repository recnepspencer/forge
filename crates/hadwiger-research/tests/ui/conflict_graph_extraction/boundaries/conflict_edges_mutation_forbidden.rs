use hadwiger_research::facade::TilingConflictGraphExtractionReport;

fn mutate_edges(graph: &mut TilingConflictGraphExtractionReport) {
    graph.conflict_edges_mut().clear();
}

fn main() {}
