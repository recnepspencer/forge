use std::collections::BTreeSet;

use super::*;

#[test]
fn final_bundle_composes_every_required_evidence_digest() {
    let bundle = certify_declarative_product_boundary().expect("product boundary should certify");
    assert_eq!(bundle.grammar_row_count(), 10);
    assert_eq!(bundle.hostile_row_count(), 9);
    assert_eq!(bundle.sabotage_row_count(), 6);
    let names = bundle
        .component_digests()
        .iter()
        .map(|(name, _)| *name)
        .collect::<BTreeSet<_>>();
    for required in [
        "facade",
        "grammar",
        "prohibition",
        "residue",
        "dx",
        "reference-consumer",
        "semantic-parity",
        "lifecycle",
        "bounded-work",
        "hostile",
        "compile-boundary",
        "sabotage",
    ] {
        assert!(names.contains(required), "missing {required} digest");
        assert!(!bundle.component_digest(required).unwrap().is_empty());
    }
    assert!(!bundle.closure_digest().is_empty());
}

#[test]
fn every_sabotage_case_names_its_enforcement_layer() {
    let rows = worth_query_product_boundary_evidence_rows();
    for row in rows.iter().filter(|row| row.sabotage_case().is_some()) {
        assert!(!row.enforcement_layer().trim().is_empty());
    }
}
