use std::path::PathBuf;

use crate::topology::WorkspaceSourceInventory;

use super::runner_contract::{self, Phase4RunnerSources};

#[test]
fn live_phase4_watched_replacement_satisfies_the_runner_contract() {
    let inventory = WorkspaceSourceInventory::capture(workspace_root());
    runner_contract::audit(&inventory).expect("live Phase 4 runner");
}

#[test]
fn event_only_and_pixel_only_replacements_are_rejected() {
    let inventory = WorkspaceSourceInventory::capture(workspace_root());
    let sources = Phase4RunnerSources::capture(&inventory);

    let mut event_only = sources.clone();
    event_only.replacement = mutate_required_edge(
        &event_only.replacement,
        "observe_watched_native(world)",
        "event_only(world)",
    );
    let error =
        runner_contract::audit_sources(&event_only).expect_err("event-only proof must fail");
    assert!(error.contains("observe_watched_native"));

    let mut pixel_only = sources;
    pixel_only.replacement = mutate_required_edge(
        &pixel_only.replacement,
        "await_watched_observation(",
        "capture_without_event(",
    );
    let error =
        runner_contract::audit_sources(&pixel_only).expect_err("pixel-only proof must fail");
    assert!(error.contains("await_watched_observation"));
}

#[test]
fn wrong_denial_liveness_and_direct_paint_shortcuts_are_rejected() {
    let inventory = WorkspaceSourceInventory::capture(workspace_root());
    let sources = Phase4RunnerSources::capture(&inventory);

    let mut wrong_reason = sources.clone();
    wrong_reason.preservation_adjudication = mutate_required_edge(
        &wrong_reason.preservation_adjudication,
        "PlatformPulseReplacementDenialFamily::DslCompilation",
        "PlatformPulseReplacementDenialFamily::SourceIngress",
    );
    let error = runner_contract::audit_sources(&wrong_reason).expect_err("wrong denial must fail");
    assert!(error.contains("DslCompilation"));

    let mut no_liveness = sources.clone();
    no_liveness.watched_native = mutate_required_edge(
        &no_liveness.watched_native,
        "observe_stable_process_liveness",
        "assume_process_liveness",
    );
    let error =
        runner_contract::audit_sources(&no_liveness).expect_err("missing liveness must fail");
    assert!(error.contains("observe_stable_process_liveness"));

    let mut direct_paint = sources;
    direct_paint.courtroom = append_counterfeit(&direct_paint.courtroom, "egui::CentralPanel;");
    let error = runner_contract::audit_sources(&direct_paint).expect_err("direct paint must fail");
    assert!(error.contains("egui::"));
}

#[test]
fn premature_exit_and_injected_source_shortcuts_are_rejected() {
    let inventory = WorkspaceSourceInventory::capture(workspace_root());
    let sources = Phase4RunnerSources::capture(&inventory);

    let mut premature_exit = sources.clone();
    premature_exit.watched_observation = mutate_required_edge(
        &premature_exit.watched_observation,
        "WatchedPulseObservationFailure::ChildExited",
        "WatchedPulseObservationFailure::Deadline",
    );
    let error = runner_contract::audit_sources(&premature_exit)
        .expect_err("premature child exit must not disappear into timeout");
    assert!(error.contains("ChildExited"));

    let mut injected_source = sources;
    injected_source.courtroom =
        append_counterfeit(&injected_source.courtroom, "inject_source(snapshot);");
    let error =
        runner_contract::audit_sources(&injected_source).expect_err("injected source must fail");
    assert!(error.contains("inject_source"));
}

#[test]
fn forced_close_hidden_teardown_and_non_atomic_actions_are_rejected() {
    let inventory = WorkspaceSourceInventory::capture(workspace_root());
    let sources = Phase4RunnerSources::capture(&inventory);

    let mut forced_close = sources.clone();
    forced_close.courtroom = mutate_required_edge(
        &forced_close.courtroom,
        ".close_native_window(",
        ".terminate_after_failure(",
    );
    let error = runner_contract::audit_sources(&forced_close).expect_err("forced close must fail");
    assert!(error.contains("close_native_window"));

    let mut hidden_teardown = sources.clone();
    hidden_teardown.source_action = mutate_required_edge(
        &hidden_teardown.source_action,
        "world.into_failure_resources()",
        "drop(world)",
    );
    let error =
        runner_contract::audit_sources(&hidden_teardown).expect_err("hidden teardown must fail");
    assert!(error.contains("into_failure_resources"));

    let mut non_atomic = sources;
    non_atomic.atomic_replacement = mutate_required_edge(
        &non_atomic.atomic_replacement,
        "winsafe::ReplaceFile(",
        "std::fs::write(",
    );
    let error = runner_contract::audit_sources(&non_atomic).expect_err("non-atomic edit must fail");
    assert!(error.contains("winsafe::ReplaceFile"));
}

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("crates")
        .parent()
        .expect("workspace")
        .to_path_buf()
}

fn mutate_required_edge(source: &str, edge: &str, counterfeit: &str) -> String {
    assert!(
        source.contains(edge),
        "negative fixture cannot exercise absent edge `{edge}`"
    );
    let mutated = source.replace(edge, counterfeit);
    assert_ne!(mutated, source, "negative fixture must change its source");
    mutated
}

fn append_counterfeit(source: &str, counterfeit: &str) -> String {
    assert!(
        !source.contains(counterfeit),
        "negative fixture counterfeit must not exist in live source"
    );
    let mutated = format!("{source}\n{counterfeit}\n");
    assert_ne!(mutated, source, "negative fixture must change its source");
    mutated
}
