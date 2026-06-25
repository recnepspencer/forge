use super::production_phase_two_closeout;
use crate::validator_invariant_catalog::{
    WorthTopologyLegalityCatalogSourceFirewallReport,
    WorthTopologyLegalityFamilySourceAuthorityKind,
};

#[test]
fn catalog_growth_surface_is_source_catalog_not_query_lowering() {
    let closeout = production_phase_two_closeout();
    let firewall = WorthTopologyLegalityCatalogSourceFirewallReport::for_query_lowering()
        .expect("query lowering source firewall should scan");

    assert_eq!(closeout.catalog().records().len(), 19);
    assert!(firewall.scanned_file_count() >= 3);
    assert!(firewall.forbidden_token_count() >= 7);
    assert!(
        firewall.violations().is_empty(),
        "query lowering must not contain family-specific routing branches: {:?}",
        firewall.violations()
    );
}

#[test]
fn every_family_has_source_authority_proof() {
    let closeout = production_phase_two_closeout();
    let catalog = closeout.catalog();

    assert_eq!(catalog.source_proofs().len(), catalog.records().len());
    assert!(catalog.source_proofs().iter().any(|proof| {
        proof.authority_kind() == WorthTopologyLegalityFamilySourceAuthorityKind::ValidatorRuleSpec
    }));
    assert!(catalog.source_proofs().iter().any(|proof| {
        proof.authority_kind()
            == WorthTopologyLegalityFamilySourceAuthorityKind::RuntimeInvariantRegistration
    }));
    for proof in catalog.source_proofs() {
        assert!(!proof.source_identity_digest().is_empty());
        assert!(!proof.rule_name().is_empty());
        assert!(!proof.semantic_version().is_empty());
        assert!(!proof.applicability_digest().is_empty());
        assert!(!proof.proof_digest().is_empty());
    }
}
