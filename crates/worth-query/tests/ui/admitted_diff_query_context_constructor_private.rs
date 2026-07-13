use worth_query::facade::policy::{AdmittedDiffQueryContext, ComparisonBasisFamily, QueryContextBudgetClass, QueryContextCostClass, QueryContextCounters, QueryContextDriftOutcome, QueryContextPredictionDriftOutcome, QueryContextPredictionReport};
use worth_query::facade::AdmittedQueryBasisContext;

fn basis() -> AdmittedQueryBasisContext {
    panic!()
}

fn report() -> QueryContextPredictionReport {
    panic!()
}

fn main() {
    let _ = AdmittedDiffQueryContext {
        left: basis(),
        right: basis(),
        family: ComparisonBasisFamily::BranchToBranch,
        drift_outcome: QueryContextDriftOutcome::BasisExact,
        cost_class: QueryContextCostClass::DiffComparisonBounded,
        budget_class: QueryContextBudgetClass::ComparisonBounded,
        prediction_report: report(),
        prediction_drift_outcome: QueryContextPredictionDriftOutcome::PendingComparison,
        counters: QueryContextCounters::default(),
    };
}
