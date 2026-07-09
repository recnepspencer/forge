use worth_query::facade::runtime::WorthQueryIntentAdmissionOracleReport;

fn main() {
    let _ = WorthQueryIntentAdmissionOracleReport {
        manifest_rows: Vec::new(),
        comparison_rows: Vec::new(),
        manifest_digest: String::new(),
        oracle_digest: String::new(),
    };
}
