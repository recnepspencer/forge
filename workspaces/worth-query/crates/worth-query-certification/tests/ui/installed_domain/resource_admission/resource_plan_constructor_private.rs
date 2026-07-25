use worth_query_host::facade::{
    admission::resource_admission::{
        WorthQueryAdmittedExecutionResourcePlan, WorthQueryExecutionResourceAdmissionCounters,
        WorthQueryExecutionResourceSupportSnapshot,
    },
    declaration::domain_computation::WorthQueryExecutionResourceRequest,
    domain::WorthQueryExecutionStrategyContract,
};

fn forge(
    request: &WorthQueryExecutionResourceRequest,
    support: WorthQueryExecutionResourceSupportSnapshot,
    strategy: WorthQueryExecutionStrategyContract,
    counters: WorthQueryExecutionResourceAdmissionCounters,
) -> WorthQueryAdmittedExecutionResourcePlan {
    WorthQueryAdmittedExecutionResourcePlan::new(
        "forged-plan".to_owned(),
        "forged-binding",
        "forged-contract".to_owned(),
        request,
        support,
        strategy,
        counters,
    )
}

fn main() {}
