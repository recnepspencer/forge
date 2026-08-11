#![deny(unused_must_use)]
#![allow(path_statements)]

use worth_query_host::facade::installed::domain_computation::{
    WorthQueryYieldedDirectRun, WorthQueryYieldedWorkflowRun,
};

fn discard_direct(run: WorthQueryYieldedDirectRun) {
    run;
}

fn discard_workflow(run: WorthQueryYieldedWorkflowRun) {
    run;
}

fn discard_direct_cleanup(run: WorthQueryYieldedDirectRun) {
    run.cleanup();
}

fn discard_workflow_cleanup(run: WorthQueryYieldedWorkflowRun) {
    run.cleanup();
}

fn main() {}
