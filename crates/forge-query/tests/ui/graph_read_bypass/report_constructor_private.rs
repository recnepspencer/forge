use forge_query::facade::consumer_kit::ForgeQueryGraphReadBypassReport;

fn main() {
    let _ = ForgeQueryGraphReadBypassReport {
        consumer_name: String::new(),
        audited_source_labels: Vec::new(),
        source_inventory_identities: Vec::new(),
        findings: Vec::new(),
        finding_identities: Vec::new(),
        report_identity: todo!(),
        counters: todo!(),
    };
}
