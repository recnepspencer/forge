use topology::facade::WorthTopologyLegalityCatalogPhaseThreeSeed;

fn main() {
    let _ = WorthTopologyLegalityCatalogPhaseThreeSeed {
        catalog_digest: String::new(),
        query_registration_catalog_digest: String::new(),
        validator_family_count: 0,
        invariant_family_count: 0,
        supported_family_count: 0,
        unsupported_family_count: 0,
        no_execution_proof_digest: String::new(),
        selected_obligation_count: 0,
        enforcement_receipt_count: 0,
        seed_digest: String::new(),
    };
}
