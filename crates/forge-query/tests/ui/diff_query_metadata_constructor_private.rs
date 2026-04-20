use forge_query::facade::DiffQueryMetadata;

fn main() {
    let _ = DiffQueryMetadata {
        query_digest: String::new(),
        comparison_basis_family: unsafe { std::mem::zeroed() },
        left_basis_digest: String::new(),
        right_basis_digest: String::new(),
        left_result_digest: String::new(),
        right_result_digest: String::new(),
        comparison_result_digest: String::new(),
        cost_class: unsafe { std::mem::zeroed() },
        budget_class: unsafe { std::mem::zeroed() },
        prediction_report: unsafe { std::mem::zeroed() },
        prediction_drift_outcome: unsafe { std::mem::zeroed() },
        drift_outcome: unsafe { std::mem::zeroed() },
    };
}
