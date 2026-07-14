use worth_query::facade::foundation::{CorrespondenceCostPosture, CorrespondenceEvidenceResolved, StructuralCandidateBudget, StructuralCandidateDiscoveryPlan};

fn bogus<T>() -> T {
    panic!()
}

fn main() {
    let _ = CorrespondenceEvidenceResolved {
        outcome: bogus(),
        discovery_plan: StructuralCandidateDiscoveryPlan::IndexBackedBounded,
        budget: StructuralCandidateBudget::bounded(1),
        cost_posture: CorrespondenceCostPosture::LineageDirect,
        complexity_contract: bogus(),
        counters: bogus(),
    };
}
