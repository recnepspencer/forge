use worth_query::facade::certification::{WorthQueryPublicDocCoverageAudit, WorthQueryPublicDocCoverageInventory};

fn main() {
    let coverage = WorthQueryPublicDocCoverageInventory::current();
    let audit = WorthQueryPublicDocCoverageAudit::current();

    let row = coverage
        .row_for_public_name("orchestrate_signal_compatibility")
        .expect("coverage row should exist");

    let _ = coverage.source_inventory_digest();
    let _ = coverage.coverage_digest();
    let _ = row.doc_reference().path();
    let _ = row.readme_discovery_label();
    let _ = row.golden_transcript().unwrap().path();
    let _ = row.journey().unwrap().as_str();
    let _ = audit.undocumented_public_surfaces();
}
