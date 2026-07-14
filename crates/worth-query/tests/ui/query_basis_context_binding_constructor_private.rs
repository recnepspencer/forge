use worth_query::facade::policy::{QueryContextCounters, QueryContextDriftOutcome};
use worth_query::facade::{QueryBasisContextBinding, QueryBasisContextRequest};

fn main() {
    let _binding = QueryBasisContextBinding {
        request: QueryBasisContextRequest::current_branch_head(),
        query_digest: String::from("query"),
        basis_digest: String::from("basis"),
        drift_outcome: QueryContextDriftOutcome::BasisExact,
        counters: QueryContextCounters::default(),
    };
}
