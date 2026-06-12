use hadwiger_research::facade::{
    extract_conflict_core_checked, extract_conflict_graph_checked, ConflictCoreExtractionReport,
    ConflictCoreExtractionRequest, ConflictGraphError, HadwigerResearchHandle,
    TilingConflictGraphExtractionReport, TilingConflictGraphExtractionRequest,
    TilingContactReplayReport,
};

fn extract_from_exact_contact(
    handle: &HadwigerResearchHandle,
    contact: TilingContactReplayReport,
) -> Result<TilingConflictGraphExtractionReport, ConflictGraphError> {
    extract_conflict_graph_checked(
        handle,
        TilingConflictGraphExtractionRequest::from_tiling_contact_report(
            "compile-conflict-graph",
            contact,
        ),
    )
}

fn extract_core(
    handle: &HadwigerResearchHandle,
    graph: &TilingConflictGraphExtractionReport,
) -> Result<ConflictCoreExtractionReport, ConflictGraphError> {
    extract_conflict_core_checked(
        handle,
        ConflictCoreExtractionRequest::new("compile-core", graph, 1),
    )
}

fn main() {}
