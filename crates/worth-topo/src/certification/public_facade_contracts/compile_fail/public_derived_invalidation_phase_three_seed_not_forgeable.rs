use topology::derived_invalidation_family_catalog::DerivedInvalidationPhaseThreeSeed;

fn main() {
    let _ = DerivedInvalidationPhaseThreeSeed {
        inventory_seed_digest: String::new(),
        catalog_digest: String::new(),
        required_family_count: 7,
        declared_family_count: 7,
        query_required_family_count: 7,
        legality_required_family_count: 7,
        spatial_receipt_required_family_count: 0,
        no_spatial_evidence_family_count: 7,
        bounded_rebuild_family_count: 3,
        incremental_eligible_family_count: 4,
        seed_digest: String::new(),
    };
}
