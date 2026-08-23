use super::mutation_receipt::{emit, MutationReceiptCase, MutationTrace};
use worth_ui_host_native::{UiNativeReadinessContract, UiNativeReadinessContractOutcome};

#[test]
fn readiness_mutation_is_rejected() {
    let mut baseline = UiNativeReadinessContract::new().expect("production readiness registry");
    assert_eq!(baseline.commit_latest(1_000, [800, 600]), Ok(1));
    assert_eq!(
        baseline.signal_committed(),
        Ok(UiNativeReadinessContractOutcome::RedrawRequested)
    );
    assert_eq!(baseline.commit_latest(1_500, [1_200, 900]), Ok(2));
    assert_eq!(
        baseline.signal_committed(),
        Ok(UiNativeReadinessContractOutcome::Coalesced)
    );
    let work = baseline.take_committed().expect("coalesced latest work");
    assert_eq!(work.generation, 2);
    assert_eq!(work.client_physical_size, [1_200, 900]);
    let baseline_state = format!(
        "{:?}:{}:{}",
        UiNativeReadinessContractOutcome::Coalesced,
        work.generation,
        baseline.redraw_requests()
    );

    let mut mutant = UiNativeReadinessContract::new().expect("production readiness registry");
    assert_eq!(
        mutant.signal_level_ready(false),
        Ok(UiNativeReadinessContractOutcome::NoWork)
    );
    assert_eq!(mutant.redraw_requests(), 0);
    let mutant_state = format!(
        "{:?}:{}",
        UiNativeReadinessContractOutcome::NoWork,
        mutant.redraw_requests()
    );
    assert_ne!(baseline_state, mutant_state);
    emit(MutationReceiptCase {
        requirement: "P6-READINESS-01",
        case: "silent-level-wake",
        baseline: MutationTrace {
            posture: "committed work requests one redraw and coalesces the latest generation",
            state: &baseline_state,
        },
        mutant: MutationTrace {
            posture: "level-only no-work is mistaken for a committed readiness wake",
            state: &mutant_state,
        },
        denial: "NoWork must not request a redraw",
        first_divergence: "committed readiness versus an empty level wake",
    });
}
