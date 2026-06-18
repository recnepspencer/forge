use forge_query::facade::consumer_kit::{EvidenceReportDeclaration, EvidenceReportScope};

fn main() {
    let _ = EvidenceReportDeclaration::new(
        EvidenceReportScope::new("forge-query.missing-participation").unwrap(),
        "MissingParticipation",
    )
    .unwrap()
    .field("status", "supported")
    .unwrap();
}
