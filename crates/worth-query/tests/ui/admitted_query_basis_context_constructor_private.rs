use worth_query::facade::{AdmittedQueryBasisContext, QueryBasisContextBinding, QueryBasisContextRequest, QueryContextCounters, QueryContextDriftOutcome};

fn main() {
    let binding = QueryBasisContextBinding {
        request: QueryBasisContextRequest::current_branch_head(),
        query_digest: String::from("query"),
        basis_digest: String::from("basis"),
        drift_outcome: QueryContextDriftOutcome::BasisExact,
        counters: QueryContextCounters::default(),
    };
    let _context = AdmittedQueryBasisContext { binding };
}
