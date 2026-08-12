use worth_query_host::facade::installed::domain_computation::{
    WorthQueryYieldedDirectRun, WorthQueryYieldedDirectRunInspection,
    WorthQueryYieldedWorkflowRun, WorthQueryYieldedWorkflowRunInspection,
};

fn rebuild_direct(run: WorthQueryYieldedDirectRun) -> WorthQueryYieldedDirectRun {
    WorthQueryYieldedDirectRun { ..run }
}

fn rebuild_workflow(run: WorthQueryYieldedWorkflowRun) -> WorthQueryYieldedWorkflowRun {
    WorthQueryYieldedWorkflowRun { ..run }
}

fn rebuild_direct_inspection(
    inspection: WorthQueryYieldedDirectRunInspection,
) -> WorthQueryYieldedDirectRunInspection {
    WorthQueryYieldedDirectRunInspection { ..inspection }
}

fn rebuild_workflow_inspection(
    inspection: WorthQueryYieldedWorkflowRunInspection,
) -> WorthQueryYieldedWorkflowRunInspection {
    WorthQueryYieldedWorkflowRunInspection { ..inspection }
}

fn main() {}
