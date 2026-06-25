use topology::derived_invalidation_family_catalog::{
    DerivedInvalidationFamilyCatalog, DerivedInvalidationFamilyCatalogCloseout,
    DerivedInvalidationPhaseThreeSeed,
};

fn main() {
    let _ = DerivedInvalidationFamilyCatalogCloseout {
        catalog: catalog(),
        phase_three_seed: phase_three_seed(),
    };
}

fn catalog() -> DerivedInvalidationFamilyCatalog {
    panic!("compile-fail fixture never executes")
}

fn phase_three_seed() -> DerivedInvalidationPhaseThreeSeed {
    panic!("compile-fail fixture never executes")
}
