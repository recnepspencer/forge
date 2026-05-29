use topology::facade::{
    TopologyDomainQueryExecutionEngine, TopologyDomainQueryRequestFamily,
    TopologyDomainQueryRequestReport,
};

fn main() {
    let _ = TopologyDomainQueryRequestReport {
        request_family: TopologyDomainQueryRequestFamily::LoopCycleNeighborhood,
        lowering_artifact: todo!(),
        execution_engine: TopologyDomainQueryExecutionEngine::QueryRuntimeCurrent,
        executed_scope_class: None,
        executed_query_digest: None,
        executed_built_in_operator_coverage: Vec::new(),
        fallback_posture: todo!(),
        query_execution_count: 0,
        lowered_traversal_count: 0,
        relationship_proof_admission_count: 0,
        row_scan_fallback_count: 0,
        whole_view_fallback_count: 0,
        repeated_rediscovery_denied_count: 0,
    };
}




