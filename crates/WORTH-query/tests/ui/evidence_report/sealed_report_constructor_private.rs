use worth_query::facade::consumer_kit::{EvidenceReport, EvidenceReportScope};

fn main() {
    let _ = EvidenceReport {
        scope: EvidenceReportScope::new("worth-query.Worthd").unwrap(),
        report_name: "Worthd".to_string(),
        fields: Vec::new(),
        field_index: std::collections::BTreeMap::new(),
        report_identity: todo!(),
        field_inventory_identity: todo!(),
        digest_participation_identity: todo!(),
    };
}
