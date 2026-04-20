use forge_query::facade::QueryBasisMetadata;

fn main() {
    let _ = QueryBasisMetadata {
        query_digest: String::new(),
        basis_digest: String::new(),
        basis_family: unsafe { std::mem::zeroed() },
        cost_class: unsafe { std::mem::zeroed() },
        budget_class: unsafe { std::mem::zeroed() },
        historical_admission_class: None,
        historical_materialization_cost_class: None,
        requested_path_identity: None,
        admitted_path_identity: None,
        resolved_path_identity: None,
        materialization_path_identity: None,
        preview_provenance_identity: None,
        result_digest: String::new(),
        drift_outcome: unsafe { std::mem::zeroed() },
        prediction_report: None,
        prediction_drift_outcome: None,
    };
}
