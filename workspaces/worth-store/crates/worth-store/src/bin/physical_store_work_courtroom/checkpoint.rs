use std::io::Write;

use worth_store::physical_runtime::certification::{
    CertificationMediaFaultActivation, CertificationPhysicalClosePauseGate,
    CertificationPhysicalExecutionPauseGate, CertificationPhysicalMutationPauseGate,
    MediaPauseGate,
};

pub(super) fn watch_execution(
    scenario: &'static str,
    gate: CertificationPhysicalExecutionPauseGate,
) {
    std::thread::spawn(move || {
        if !gate.await_arrival() {
            eprintln!("C5_1_COURTROOM_CHECKPOINT_TIMEOUT {scenario}");
            std::process::exit(70);
        }
        emit(scenario, &format!("{:?}", gate.checkpoint()), None);
        park_until_parent_kills();
    });
}

pub(super) fn watch_media(scenario: &'static str, gate: MediaPauseGate) {
    std::thread::spawn(move || {
        gate.wait_until_reached();
        let context = gate
            .reached_context()
            .expect("reached media gate retains its operation context");
        let detail = format!(
            "{}:{}:{}:{}",
            context.role().metric_name(),
            context.role_ordinal(),
            context.identified_operation_ordinal().unwrap_or(0),
            context.requested_bytes(),
        );
        emit(scenario, "MediaEffect", Some(&detail));
        park_until_parent_kills();
    });
}

pub(super) fn watch_close(scenario: &'static str, gate: CertificationPhysicalClosePauseGate) {
    std::thread::spawn(move || {
        if !gate.await_arrival() {
            eprintln!("C5_1_COURTROOM_CHECKPOINT_TIMEOUT {scenario}");
            std::process::exit(70);
        }
        emit(scenario, &format!("{:?}", gate.phase()), None);
        park_until_parent_kills();
    });
}

pub(super) fn watch_mutation(
    marker: &'static str,
    scenario: &'static str,
    gate: CertificationPhysicalMutationPauseGate,
) {
    std::thread::spawn(move || {
        if !gate.await_arrival() {
            eprintln!("C7_COURTROOM_CHECKPOINT_TIMEOUT {scenario}");
            std::process::exit(70);
        }
        emit_with_marker(marker, scenario, &format!("{:?}", gate.checkpoint()), None);
        park_until_parent_kills();
    });
}

pub(super) fn watch_c7_media(
    marker: &'static str,
    scenario: &'static str,
    gate: MediaPauseGate,
    activation: CertificationMediaFaultActivation,
) {
    std::thread::spawn(move || {
        gate.wait_until_reached();
        let context = gate
            .reached_context()
            .expect("reached media gate retains its operation context");
        let detail = format!(
            "{}:{}:{}:{}:{}",
            context.role().metric_name(),
            context.role_ordinal(),
            context.identified_operation_ordinal().unwrap_or(0),
            context.requested_bytes(),
            activation.matching_operation_count(),
        );
        emit_with_marker(marker, scenario, "MediaEffect", Some(&detail));
        park_until_parent_kills();
    });
}

pub(super) fn emit(scenario: &str, checkpoint: &str, detail: Option<&str>) {
    emit_with_marker("C5_1_COURTROOM_CHECKPOINT", scenario, checkpoint, detail);
}

fn emit_with_marker(marker: &str, scenario: &str, checkpoint: &str, detail: Option<&str>) {
    println!(
        "{marker} {scenario} {checkpoint} {} {}",
        std::process::id(),
        detail.unwrap_or("-"),
    );
    std::io::stdout()
        .flush()
        .expect("checkpoint marker must flush");
}

pub(super) fn park_until_parent_kills() -> ! {
    loop {
        std::thread::park();
    }
}
