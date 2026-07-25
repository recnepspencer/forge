use std::{io::Write, path::Path, process::ExitStatus};

use worth_store::physical_runtime::PhysicalStoreClosePhase;

const PHASE_ENV: &str = "WORTH_STORE_C5_CLOSE_DEATH_PHASE";

pub(super) fn writer(root: &Path) {
    let serving = super::serving_from_initialization(root);
    let plan = serving.close_plan();
    let observation = plan.observation();
    let selected = selected_phase();
    let watcher = std::thread::spawn(move || exit_when_reached(observation, selected));
    let _ = plan.execute();
    watcher.join().unwrap();
    panic!("selected close phase did not terminate the process");
}

pub(super) fn reopener(root: &Path) {
    let serving = super::serving_from_open(root);
    println!("C5_CLOSE_REOPENED {:?}", serving.store_identity());
    let _ = serving.close_plan().execute();
}

pub(super) fn kill_writer_at(root: &Path, phase: PhysicalStoreClosePhase) -> String {
    let output = super::child_process::child_command("close_phase_writer", root)
        .env(PHASE_ENV, format!("{phase:?}"))
        .output()
        .unwrap();
    assert_expected_death(output.status);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout
            .lines()
            .any(|line| line == format!("C5_CLOSE_PHASE {phase:?}")),
        "writer did not reach {phase:?}; stdout:\n{stdout}"
    );
    stdout
}

fn selected_phase() -> PhysicalStoreClosePhase {
    let selected = std::env::var(PHASE_ENV).expect("close death phase");
    [
        PhysicalStoreClosePhase::AdmissionStopped,
        PhysicalStoreClosePhase::SafeCancellationComplete,
        PhysicalStoreClosePhase::DispatchSettlementComplete,
        PhysicalStoreClosePhase::SignalDisposed,
        PhysicalStoreClosePhase::ResidencyClosed,
        PhysicalStoreClosePhase::MediaReleased,
    ]
    .into_iter()
    .find(|phase| selected == phase_name(*phase))
    .expect("known close death phase")
}

fn exit_when_reached(
    observation: worth_store::physical_runtime::PhysicalStoreCloseObservation,
    selected: PhysicalStoreClosePhase,
) {
    while !observation.reached(selected) {
        std::thread::yield_now();
    }
    println!("C5_CLOSE_PHASE {selected:?}");
    std::io::stdout().flush().unwrap();
    std::process::exit(86);
}

const fn phase_name(phase: PhysicalStoreClosePhase) -> &'static str {
    match phase {
        PhysicalStoreClosePhase::AdmissionStopped => "AdmissionStopped",
        PhysicalStoreClosePhase::SafeCancellationComplete => "SafeCancellationComplete",
        PhysicalStoreClosePhase::DispatchSettlementComplete => "DispatchSettlementComplete",
        PhysicalStoreClosePhase::SignalDisposed => "SignalDisposed",
        PhysicalStoreClosePhase::ResidencyClosed => "ResidencyClosed",
        PhysicalStoreClosePhase::MediaReleased => "MediaReleased",
    }
}

fn assert_expected_death(status: ExitStatus) {
    assert_eq!(
        status.code(),
        Some(86),
        "close writer must die at its selected production phase"
    );
}
