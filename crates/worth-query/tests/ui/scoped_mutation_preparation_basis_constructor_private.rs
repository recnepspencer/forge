use worth_query::facade::foundation::{AdmittedBasisCapability, ScopedMutationPreparationBasis};
use worth_query::facade::policy::ScopedBasisConstructionCounters;

fn main() {
    let _ = ScopedMutationPreparationBasis {
        capability: panic!() as AdmittedBasisCapability,
        counters: panic!() as ScopedBasisConstructionCounters,
        scoped_digest: String::new(),
    };
}
