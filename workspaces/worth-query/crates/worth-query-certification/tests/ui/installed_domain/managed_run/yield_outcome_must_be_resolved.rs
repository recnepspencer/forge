#![deny(unused_must_use)]

use worth_query_execution::facade::domain_computation::{
    WorthQueryPausedDirectGraphExecution, WorthQueryPausedWorkflowGraphExecution,
};

fn ignore_direct(paused: WorthQueryPausedDirectGraphExecution) {
    paused.yield_run();
}

fn ignore_workflow(paused: WorthQueryPausedWorkflowGraphExecution) {
    paused.yield_run();
}

fn main() {}
