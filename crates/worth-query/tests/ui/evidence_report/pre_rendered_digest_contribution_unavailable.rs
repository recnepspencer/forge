use worth_query::facade::consumer_kit::{EvidenceReportDeclaration, EvidenceReportScope};

fn main() {
    let _ = EvidenceReportDeclaration::new(
        EvidenceReportScope::new("worth-query.rendered-digest").unwrap(),
        "RenderedDigest",
    )
    .unwrap()
    .pre_rendered_digest_part("status:supported")
    .unwrap();
}
