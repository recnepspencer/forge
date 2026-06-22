use forge_query::facade::consumer_kit::{EvidenceReportDeclaration, EvidenceReportScope};

fn main() {
    let mut report = EvidenceReportDeclaration::new(
        EvidenceReportScope::new("forge-query.private-fields").unwrap(),
        "PrivateFields",
    )
    .unwrap()
    .shape_participating("status", "supported")
    .unwrap()
    .seal()
    .unwrap();

    report.fields = Vec::new();
}
