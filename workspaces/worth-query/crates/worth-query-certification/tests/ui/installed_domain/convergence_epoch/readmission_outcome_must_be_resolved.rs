#![deny(unused_must_use)]

use worth_query_execution::facade::convergence_epoch::{
    WorthQueryYieldedDirectConvergenceIteration, WorthQueryYieldedWorkflowConvergenceIteration,
};
use worth_query_execution::facade::runtime::WorthQueryExecutionRuntime;
use worth_runtime_bridge::facade::RuntimeBridge;

fn ignore_direct(
    yielded: WorthQueryYieldedDirectConvergenceIteration,
    query: &WorthQueryExecutionRuntime,
    bridge: &RuntimeBridge,
) {
    yielded.readmit_same_runtime(query, bridge);
}

fn ignore_workflow(
    yielded: WorthQueryYieldedWorkflowConvergenceIteration,
    query: &WorthQueryExecutionRuntime,
    bridge: &RuntimeBridge,
) {
    yielded.readmit_same_runtime(query, bridge);
}

fn main() {}
