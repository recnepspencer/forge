use worth_query::facade::policy::{ComparisonBasisFamily, QueryContextPredictionDriftOutcome, QueryDiffChangeSetArtifact};

fn main() {
    let _ = QueryDiffChangeSetArtifact {
        query_digest: String::from("query"),
        comparison_basis_family: ComparisonBasisFamily::BranchToBranch,
        left_basis_digest: String::from("left"),
        right_basis_digest: String::from("right"),
        result_digest: String::from("result"),
        prediction_drift_outcome: QueryContextPredictionDriftOutcome::WithinBudget,
        rows: Vec::new(),
    };
}
