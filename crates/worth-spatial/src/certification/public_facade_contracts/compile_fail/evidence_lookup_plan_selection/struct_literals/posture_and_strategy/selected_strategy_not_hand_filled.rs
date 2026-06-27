use worth_spatial::facade::evidence_lookup_plan_selection::{
    EvidenceLookupSelectedStrategy, EvidenceLookupSelectedStrategyKind,
};

fn main() {
    let _ = EvidenceLookupSelectedStrategy {
        kind: EvidenceLookupSelectedStrategyKind::SparseIndexedLookupPlan,
    };
}
