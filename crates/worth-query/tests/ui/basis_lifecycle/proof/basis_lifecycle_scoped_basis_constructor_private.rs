use worth_query::facade::{BasisEligibilityCounters, ScopedObservationBasis};

fn counters() -> BasisEligibilityCounters {
    unimplemented!()
}

fn main() {
    let _ = ScopedObservationBasis {
        capability_digest: String::new(),
        scoped_basis_digest: String::new(),
        counters: counters(),
    };
}
