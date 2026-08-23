use std::path::PathBuf;

use crate::topology::WorkspaceSourceInventory;

use super::runner_contract::{self, phase4_courtroom_paths, Phase4RunnerSources};
use super::visual_identity_contract::{self, VisualIdentityRunnerSources};

#[test]
fn live_phase4_watched_replacement_satisfies_the_runner_contract() {
    let inventory = WorkspaceSourceInventory::capture(workspace_root());
    runner_contract::audit(&inventory).expect("live Phase 4 runner");
}

#[test]
fn live_phase4_visual_identity_sources_satisfy_the_runner_contract() {
    let inventory = WorkspaceSourceInventory::capture(workspace_root());
    visual_identity_contract::audit(&inventory).expect("live Phase 4 visual identity runner");
}

#[test]
fn phase4_courtroom_scope_excludes_ignored_successors_but_keeps_predecessor_edges() {
    let inventory = WorkspaceSourceInventory::capture(workspace_root());
    let successor = "apps/platform-pulse/tests/executable_world/courtroom/native_phase2.rs";
    assert!(inventory.text(successor).contains("#[ignore"));
    assert!(!phase4_courtroom_paths().contains(&successor));

    let mut sources = Phase4RunnerSources::capture(&inventory);
    assert!(!sources.courtroom.contains("#[ignore"));
    sources.courtroom = mutate_required_edge(
        &sources.courtroom,
        "close_recovered(self.recovered)",
        "drop(self.recovered)",
    );
    let error = runner_contract::audit_sources(&sources)
        .expect_err("a required predecessor edge must remain governed");
    assert!(error.contains("close_recovered(self.recovered)"), "{error}");
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

    let mut detached_close = sources.clone();
    detached_close.courtroom = mutate_required_edge(
        &detached_close.courtroom,
        "close_recovered(self.recovered)",
        "drop(self.recovered)",
    );
    let error =
        runner_contract::audit_sources(&detached_close).expect_err("detached close must fail");
    assert!(error.contains("close_recovered(self.recovered)"));

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

#[test]
fn visual_event_only_and_pixel_only_success_are_rejected() {
    let inventory = WorkspaceSourceInventory::capture(workspace_root());
    let sources = VisualIdentityRunnerSources::capture(&inventory);

    let mut event_only = sources.clone();
    event_only.visual_progression = mutate_required_edge(
        &event_only.visual_progression,
        "observe_watched_native(&mut world)",
        "assume_native_pixels(&mut world)",
    );
    let error = visual_identity_contract::audit_sources(&event_only)
        .expect_err("visual event-only success must fail");
    assert!(error.contains("observe_watched_native"));

    let mut pixel_only = sources;
    pixel_only.visual_progression = mutate_required_edge(
        &pixel_only.visual_progression,
        "await_visual_event(",
        "assume_product_event(",
    );
    let error = visual_identity_contract::audit_sources(&pixel_only)
        .expect_err("visual pixel-only success must fail");
    assert!(error.contains("await_visual_event"));
}

#[test]
fn sole_node_and_wrong_target_visual_proofs_are_rejected() {
    let inventory = WorkspaceSourceInventory::capture(workspace_root());
    let sources = VisualIdentityRunnerSources::capture(&inventory);

    let mut sole_node = sources.clone();
    sole_node.identity_trace = mutate_required_edge(
        &sole_node.identity_trace,
        "snapshot.visible_region_count() != PLATFORM_PULSE_VISIBLE_REGION_COUNT",
        "snapshot.visible_region_count() == 0",
    );
    let error = visual_identity_contract::audit_sources(&sole_node)
        .expect_err("sole-node fallback must fail");
    assert!(error.contains("visible_region_count"));

    let mut wrong_target = sources;
    wrong_target.identity_trace = mutate_required_edge(
        &wrong_target.identity_trace,
        "PLATFORM_PULSE_IDENTITY_TARGET_AUTHORED_NAME",
        "CALLER_SELECTED_AUTHORED_NAME",
    );
    let error = visual_identity_contract::audit_sources(&wrong_target)
        .expect_err("wrong target identity must fail");
    assert!(error.contains("PLATFORM_PULSE_IDENTITY_TARGET_AUTHORED_NAME"));
}

#[test]
fn restored_pixel_resampling_and_unexposed_capture_shortcuts_are_rejected() {
    let inventory = WorkspaceSourceInventory::capture(workspace_root());
    let sources = VisualIdentityRunnerSources::capture(&inventory);

    let mut no_clear_pixels = sources.clone();
    no_clear_pixels.overlay_pixels = mutate_required_edge(
        &no_clear_pixels.overlay_pixels,
        "matching != 0",
        "matching == usize::MAX",
    );
    let error = visual_identity_contract::audit_sources(&no_clear_pixels)
        .expect_err("missing restored-pixel predicate must fail");
    assert!(error.contains("matching != 0"));

    let mut resampled = sources.clone();
    resampled
        .wgc_capture
        .push_str("\nfn counterfeit() { imageops::resize(); }\n");
    let error = visual_identity_contract::audit_sources(&resampled)
        .expect_err("resampled client capture must fail");
    assert!(error.contains("imageops::resize"));

    let mut unexposed = sources;
    unexposed.windows_capture =
        mutate_required_edge(&unexposed.windows_capture, "win::DwmFlush()", "Ok(())");
    let error = visual_identity_contract::audit_sources(&unexposed)
        .expect_err("capture without a compositor exposure barrier must fail");
    assert!(error.contains("win::DwmFlush()"));
}

#[test]
fn exact_capture_identity_and_structural_residue_are_required() {
    let inventory = WorkspaceSourceInventory::capture(workspace_root());
    let sources = VisualIdentityRunnerSources::capture(&inventory);

    for edge in [
        "window.pid().ok() == Some(process_id)",
        "window.id().ok() == Some(window_id)",
    ] {
        let mut identity_blind = sources.clone();
        identity_blind.wgc_capture =
            mutate_required_edge(&identity_blind.wgc_capture, edge, "true");
        let error = visual_identity_contract::audit_sources(&identity_blind)
            .expect_err("capture without exact process and HWND identity must fail");
        assert!(error.contains(edge), "{error}");
    }

    let mut structural_blind = sources;
    structural_blind.lifecycle_cleanup = mutate_required_edge(
        &structural_blind.lifecycle_cleanup,
        "shutdown.disposed_visual_structural_bytes()",
        "0",
    );
    let error = visual_identity_contract::audit_sources(&structural_blind)
        .expect_err("cleanup without structural-byte residue must fail");
    assert!(
        error.contains("disposed_visual_structural_bytes"),
        "{error}"
    );
}

#[test]
fn spawning_without_a_process_owned_native_desktop_lease_is_rejected() {
    let inventory = WorkspaceSourceInventory::capture(workspace_root());
    let mut sources = VisualIdentityRunnerSources::capture(&inventory);
    sources.process_launch = mutate_required_edge(
        &sources.process_launch,
        "_native_desktop_lease: NativeDesktopLease",
        "_native_desktop_lease: ()",
    );
    let error = visual_identity_contract::audit_sources(&sources)
        .expect_err("unleased native child spawn must fail");
    assert!(error.contains("_native_desktop_lease"));
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
