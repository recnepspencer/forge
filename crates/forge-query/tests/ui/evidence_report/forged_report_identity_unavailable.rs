use forge_query::facade::consumer_kit::{EvidenceReportDeclaration, EvidenceReportScope};

fn main() {
    let report = EvidenceReportDeclaration::new(
        EvidenceReportScope::new("forge-query.forged-identity").unwrap(),
        "ForgedIdentity",
    )
    .unwrap()
    .shape_participating("status", "supported")
    .unwrap()
    .seal()
    .unwrap();

    report.report_identity = todo!();
}
