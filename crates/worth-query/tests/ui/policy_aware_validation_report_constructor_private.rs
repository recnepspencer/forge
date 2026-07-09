use worth_query::facade::PolicyAwareValidationReport;

fn main() {
    let _report = PolicyAwareValidationReport {
        digest: String::new(),
        failure_digests: Vec::new(),
        counter_snapshot_digest: String::new(),
    };
}
