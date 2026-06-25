use topology::derived_invalidation_operator_cutover::{
    DerivedInvalidationOperatorCutoverCloseout, DerivedInvalidationOperatorCutoverCounters,
    DerivedInvalidationOperatorCutoverReceipt, DerivedInvalidationPhaseEightSeed,
    DerivedInvalidationProjectionReadStageReceipt,
};

fn main() {
    let _ = DerivedInvalidationOperatorCutoverCloseout {
        operator_cutover: fake_operator_cutover(),
        projection_read_stage: fake_projection_read_stage(),
        counters: fake_counters(),
        phase_eight_seed: fake_phase_eight_seed(),
        closeout_digest: String::new(),
    };
}

fn fake_operator_cutover() -> DerivedInvalidationOperatorCutoverReceipt {
    panic!("compile-fail fixture does not execute")
}

fn fake_projection_read_stage() -> DerivedInvalidationProjectionReadStageReceipt {
    panic!("compile-fail fixture does not execute")
}

fn fake_counters() -> DerivedInvalidationOperatorCutoverCounters {
    panic!("compile-fail fixture does not execute")
}

fn fake_phase_eight_seed() -> DerivedInvalidationPhaseEightSeed {
    panic!("compile-fail fixture does not execute")
}
