use worth_query::facade::consumer_kit::WorthQueryGraphReadBypassReport;

fn main() {
    let _ = WorthQueryGraphReadBypassReport {
        consumer_name: String::new(),
        audited_source_labels: Vec::new(),
        source_inventory_identities: Vec::new(),
        findings: Vec::new(),
        finding_identities: Vec::new(),
        report_identity: todo!(),
        counters: todo!(),
    };
}
