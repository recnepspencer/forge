use worth_query::facade::consumer_kit::{EvidenceReportDeclaration, EvidenceReportScope};

fn main() {
    let _ = EvidenceReportDeclaration::new(
        EvidenceReportScope::new("worth-query.missing-participation").unwrap(),
        "MissingParticipation",
    )
    .unwrap()
    .field("status", "supported")
    .unwrap();
}
