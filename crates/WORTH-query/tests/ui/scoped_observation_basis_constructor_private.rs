use worth_query::facade::policy::{
    BasisCapabilityAdmission, ScopedBasisConstructionCounters, ScopedObservationBasis,
};

fn main() {
    let _ = ScopedObservationBasis {
        admission: panic!() as BasisCapabilityAdmission,
        counters: panic!() as ScopedBasisConstructionCounters,
        scoped_digest: String::new(),
    };
}
