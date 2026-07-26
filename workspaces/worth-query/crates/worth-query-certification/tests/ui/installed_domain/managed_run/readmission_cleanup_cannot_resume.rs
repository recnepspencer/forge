use worth_query_execution::facade::domain_computation::{
    WorthQueryDirectReadmissionCleanupRequired, WorthQueryWorkflowReadmissionCleanupRequired,
};

fn retry_direct(cleanup: WorthQueryDirectReadmissionCleanupRequired) {
    cleanup.retry_to_yielded();
}

fn retry_workflow(cleanup: WorthQueryWorkflowReadmissionCleanupRequired) {
    cleanup.readmit_same_runtime();
}

fn main() {}
