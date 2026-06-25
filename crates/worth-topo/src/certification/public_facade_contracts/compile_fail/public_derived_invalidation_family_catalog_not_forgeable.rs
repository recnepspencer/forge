use topology::derived_invalidation_family_catalog::{
    DerivedInvalidationFamilyCatalog, DerivedInvalidationFamilyCatalogCounters,
};
use topology::derived_invalidation_authority_inventory::DerivedInvalidationPhaseTwoSeed;

fn main() {
    let _ = DerivedInvalidationFamilyCatalog {
        phase_two_seed: phase_two_seed(),
        families: Vec::new(),
        counters: counters(),
        catalog_digest: String::new(),
    };
}

fn phase_two_seed() -> DerivedInvalidationPhaseTwoSeed {
    panic!("compile-fail fixture never executes")
}

fn counters() -> DerivedInvalidationFamilyCatalogCounters {
    panic!("compile-fail fixture never executes")
}
