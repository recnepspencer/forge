use forge_query::facade::consumer_kit::{EvidenceReportDeclaration, EvidenceReportScope};

fn main() {
    let _ = EvidenceReportDeclaration::new(
        EvidenceReportScope::new("forge-query.rendered-digest").unwrap(),
        "RenderedDigest",
    )
    .unwrap()
    .pre_rendered_digest_part("status:supported")
    .unwrap();
}
