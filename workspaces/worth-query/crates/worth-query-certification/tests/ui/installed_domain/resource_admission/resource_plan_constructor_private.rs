use worth_query_host::facade::{
    domain::WorthQueryExecutionStrategyContract,
    installed::operation::{
        WorthQueryAdmittedExecutionResourcePlan, WorthQueryExecutionResourceAdmissionCounters,
        WorthQueryExecutionResourceRequest, WorthQueryExecutionResourceSupportSnapshot,
    },
};

fn forge(
    request: &WorthQueryExecutionResourceRequest,
    support: WorthQueryExecutionResourceSupportSnapshot,
    strategy: WorthQueryExecutionStrategyContract,
    counters: WorthQueryExecutionResourceAdmissionCounters,
) -> WorthQueryAdmittedExecutionResourcePlan {
    WorthQueryAdmittedExecutionResourcePlan::new(
        "forged-plan".to_owned(),
        request,
        support,
        strategy,
        counters,
    )
}

fn main() {}
