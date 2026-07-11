use forge_store_contracts::{
    S8RuntimeCase, S8RuntimeExactCounterEvidence, S8RuntimeExecutionIdentity, S8RuntimeOwnerFact,
    S8RuntimeScanPosture,
};
use forge_store_physical_certification::layout_harness::runtime::S8RuntimeCoverageMatrix;

fn main() {
    let copied_fact = S8RuntimeOwnerFact::new(
        S8RuntimeExecutionIdentity::from_owner_seed(99),
        S8RuntimeCase::Success,
        S8RuntimeScanPosture::OwnerBounded,
        S8RuntimeExactCounterEvidence::new(1, 1),
    );
    let _ = S8RuntimeCoverageMatrix::default().record(copied_fact);
}
