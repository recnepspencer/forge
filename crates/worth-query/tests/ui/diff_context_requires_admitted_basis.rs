use worth_query::facade::policy::{bind_diff_query_context, QueryContextCounters, QueryContextDriftOutcome};
use worth_query::facade::{QueryBasisContextBinding, QueryBasisContextRequest};

fn main() {
    let binding = QueryBasisContextBinding {
        request: QueryBasisContextRequest::current_branch_head(),
        query_digest: String::from("query"),
        basis_digest: String::from("basis"),
        drift_outcome: QueryContextDriftOutcome::BasisExact,
        counters: QueryContextCounters::default(),
    };

    let _ = bind_diff_query_context(&binding, &binding);
}
