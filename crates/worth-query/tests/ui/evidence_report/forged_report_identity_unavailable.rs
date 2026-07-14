use worth_query::facade::consumer_kit::{EvidenceReportDeclaration, EvidenceReportScope};

fn main() {
    let report = EvidenceReportDeclaration::new(
        EvidenceReportScope::new("worth-query.worthd-identity").unwrap(),
        "worthd_identity",
    )
    .unwrap()
    .shape_participating("status", "supported")
    .unwrap()
    .seal()
    .unwrap();

    report.report_identity = todo!();
}
