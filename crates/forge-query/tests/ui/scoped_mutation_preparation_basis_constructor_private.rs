use forge_query::facade::{AdmittedBasisCapability, ScopedBasisConstructionCounters, ScopedMutationPreparationBasis};

fn main() {
    let _ = ScopedMutationPreparationBasis {
        capability: panic!() as AdmittedBasisCapability,
        counters: panic!() as ScopedBasisConstructionCounters,
        scoped_digest: String::new(),
    };
}
