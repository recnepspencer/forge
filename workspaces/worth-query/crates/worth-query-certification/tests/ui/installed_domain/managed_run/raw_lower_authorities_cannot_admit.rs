use worth_query_execution::facade::domain_computation::{
    WorthQueryDirectExecutionResourceAttempt, WorthQueryExecutionBoundOperationAuthority,
    WorthQueryExecutionRuntime,
};
use worth_relational::facade::branch::AdmittedRelationalBranchBasis;
use worth_runtime_bridge::facade::BridgeBoundExecutionBasis;

fn bypass(
    runtime: &WorthQueryExecutionRuntime,
    operation: &WorthQueryExecutionBoundOperationAuthority,
    attempt: WorthQueryDirectExecutionResourceAttempt,
    bridge: BridgeBoundExecutionBasis,
    relational: AdmittedRelationalBranchBasis,
) {
    let _ = runtime.admit_direct_run(operation, attempt, bridge, relational);
}

fn main() {}
