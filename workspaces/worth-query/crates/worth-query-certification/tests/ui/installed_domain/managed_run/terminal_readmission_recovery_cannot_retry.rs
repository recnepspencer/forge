use worth_query_execution::facade::domain_computation::{
    WorthQueryDirectReadmissionTerminalRecovery,
    WorthQueryWorkflowReadmissionTerminalRecovery,
};

fn retry_direct(recovery: WorthQueryDirectReadmissionTerminalRecovery) {
    recovery.retry_to_yielded();
}

fn retry_workflow(recovery: WorthQueryWorkflowReadmissionTerminalRecovery) {
    recovery.retry_to_yielded();
}

fn main() {}
