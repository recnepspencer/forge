use worth_query::facade::foundation::WorthQuerySupportReport;

fn main() {
    let _ = WorthQuerySupportReport {
        support_matrix: todo!(),
        admitted_capability_count: 0,
        deferred_capability_count: 0,
        unsupported_capability_count: 0,
        admitted_capability_families: Vec::new(),
        deferred_capability_families: Vec::new(),
        unsupported_capability_families: Vec::new(),
        section_postures: Vec::new(),
        validated_config_digest: String::new(),
        counters: todo!(),
        report_digest: String::new(),
    };
}
