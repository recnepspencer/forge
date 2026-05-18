use forge_query::facade::runtime::ForgeQueryIntentAdmissionOracleReport;

fn main() {
    let _ = ForgeQueryIntentAdmissionOracleReport {
        manifest_rows: Vec::new(),
        comparison_rows: Vec::new(),
        manifest_digest: String::new(),
        oracle_digest: String::new(),
    };
}
