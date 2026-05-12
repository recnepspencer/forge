use forge_query::facade::CausalInspectionRepresentativeMatrix;

fn main() {
    let _ = CausalInspectionRepresentativeMatrix {
        representative_count: 0,
        missing_evidence_row_count: 0,
        query_only_consumer_row_count: 0,
        matrix_digest: String::new(),
    };
}
