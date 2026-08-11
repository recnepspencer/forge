use super::super::*;

pub(super) struct ResourceMilestoneCRetryEvidence {
    pub(super) denied_retry_report: ResourceRetryScheduleReport,
}

pub(super) fn resource_milestone_c_retry_evidence() -> ResourceMilestoneCRetryEvidence {
    let mut retry_graph = SignalGraph::new();
    let retry_first = retry_graph.node().build();
    let retry_second = retry_graph.node().build();
    let mut retry_runtime = TestRuntime::build(retry_graph);
    retry_runtime
        .declare_resource_node(retry_budgeted_timeout_resource_declaration(
            retry_first,
            3,
            7,
            ResourceRetryBudgetScope::Runtime,
            1,
        ))
        .expect("first retry declaration should lower");
    retry_runtime
        .declare_resource_node(retry_budgeted_timeout_resource_declaration(
            retry_second,
            3,
            7,
            ResourceRetryBudgetScope::Runtime,
            1,
        ))
        .expect("second retry declaration should lower");
    let _scheduled_retry = schedule_timed_out_retry(&mut retry_runtime, retry_first);
    let denied_retry_report = schedule_timed_out_retry(&mut retry_runtime, retry_second);

    ResourceMilestoneCRetryEvidence {
        denied_retry_report,
    }
}
