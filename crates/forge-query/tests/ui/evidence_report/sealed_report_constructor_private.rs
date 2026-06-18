use forge_query::facade::consumer_kit::{EvidenceReport, EvidenceReportScope};

fn main() {
    let _ = EvidenceReport {
        scope: EvidenceReportScope::new("forge-query.forged").unwrap(),
        report_name: "Forged".to_string(),
        fields: Vec::new(),
        field_index: std::collections::BTreeMap::new(),
        report_identity: todo!(),
        field_inventory_identity: todo!(),
        digest_participation_identity: todo!(),
    };
}
