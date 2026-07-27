use crate::topology::WorkspaceSourceInventory;

const RUNNER_ROOT: &str = "apps/platform-pulse/tests";

#[derive(Clone)]
pub(super) struct Phase4RunnerSources {
    pub(super) progression: String,
    pub(super) source_action: String,
    pub(super) replacement: String,
    pub(super) preservation: String,
    pub(super) watched_observation: String,
    pub(super) watched_native: String,
    pub(super) native_color: String,
    pub(super) preservation_adjudication: String,
    pub(super) atomic_replacement: String,
    pub(super) normal_close: String,
    pub(super) failure_report: String,
    pub(super) courtroom: String,
}

pub(super) fn audit(inventory: &WorkspaceSourceInventory) -> Result<(), String> {
    audit_required_files(inventory)?;
    audit_runner_purity(inventory)?;
    audit_sources(&Phase4RunnerSources::capture(inventory))
}

pub(super) fn audit_sources(sources: &Phase4RunnerSources) -> Result<(), String> {
    audit_typestate(&sources.progression, &sources.source_action)?;
    audit_replacement(
        &sources.replacement,
        &sources.watched_observation,
        &sources.watched_native,
        &sources.native_color,
    )?;
    audit_preservation(&sources.preservation, &sources.preservation_adjudication)?;
    audit_atomic_action(&sources.atomic_replacement)?;
    audit_close_and_failure(&sources.normal_close, &sources.failure_report)?;
    audit_courtroom(&sources.courtroom)
}

fn audit_required_files(inventory: &WorkspaceSourceInventory) -> Result<(), String> {
    for path in [
        "apps/platform-pulse/tests/executable_world/source_delta/atomic_replacement.rs",
        "apps/platform-pulse/tests/executable_world/source_delta/canonical_deltas.rs",
        "apps/platform-pulse/tests/executable_world/adjudication/native_color.rs",
        "apps/platform-pulse/tests/executable_world/adjudication/publication_identity.rs",
        "apps/platform-pulse/tests/executable_world/adjudication/replacement_to_pixel.rs",
        "apps/platform-pulse/tests/executable_world/adjudication/predecessor_preservation.rs",
        "apps/platform-pulse/tests/executable_world/product_process/source_action_progression.rs",
        "apps/platform-pulse/tests/executable_world/product_process/replacement_progression.rs",
        "apps/platform-pulse/tests/executable_world/product_process/preservation_progression.rs",
        "apps/platform-pulse/tests/executable_world/product_process/watched_observation.rs",
        "apps/platform-pulse/tests/executable_world/product_process/watched_native_observation.rs",
        "apps/platform-pulse/tests/executable_world/courtroom/platform_pulse_journey.rs",
    ] {
        if inventory.source(path).is_none() {
            return Err(format!("Phase 4 executable world lost `{path}`"));
        }
    }
    Ok(())
}

fn audit_runner_purity(inventory: &WorkspaceSourceInventory) -> Result<(), String> {
    for source in inventory.rust_files_under(RUNNER_ROOT) {
        for forbidden in [
            "worth_ui_runtime",
            "worth_ui_dsl",
            "worth_ui_certification",
            "worth_ui_test_support",
            "eframe::",
            "egui::",
            "inject_snapshot(",
            "inject_watcher_event(",
            "inject_submission(",
            "inject_receipt(",
            "inject_generation(",
            "inject_frame(",
            "inject_shape(",
            "inject_source(",
        ] {
            if source.text().contains(forbidden) {
                return Err(format!(
                    "{} reopened product or adapter authority through `{forbidden}`",
                    source.relative_path().display()
                ));
            }
        }
    }
    Ok(())
}

fn audit_typestate(progression: &str, source_action: &str) -> Result<(), String> {
    for required in [
        "pub(crate) struct Published<Stage>",
        "pub(crate) struct InitialBlue",
        "pub(crate) struct AwaitingReplacement",
        "pub(crate) struct GreenSuccessor",
        "pub(crate) struct AwaitingPreservation",
        "pub(crate) struct PreservedPredecessor",
        "pub(crate) struct AwaitingRecovery",
        "pub(crate) struct RecoveredBlue",
    ] {
        require(progression, required, "Phase 4 typestate")?;
    }
    for required in [
        "impl PulseExecutableWorld<Published<InitialBlue>>",
        "fn apply_green(",
        "impl PulseExecutableWorld<Published<GreenSuccessor>>",
        "fn apply_malformed(",
        "impl PulseExecutableWorld<PreservedPredecessor>",
        "fn restore_canonical(",
        "PulseExecutableWorldFailure::SourceAction(failure)",
        "world.into_failure_resources()",
    ] {
        require(source_action, required, "typed source action")?;
    }
    Ok(())
}

fn audit_replacement(
    replacement: &str,
    watched: &str,
    native: &str,
    color: &str,
) -> Result<(), String> {
    for required in [
        "fn await_green_successor(",
        "fn await_recovered_blue(",
        "await_watched_observation(",
        "observe_watched_native(",
        "CausalReplacementObservationSet::new(",
        "ExpectedNativeColor::Green",
        "ExpectedNativeColor::Blue",
        "PulseExecutableWorldFailure::Replacement(failure)",
        "teardown_native_bound_world(",
    ] {
        require(replacement, required, "watched replacement progression")?;
    }
    for required in [
        "observed_exit()",
        "WatchedPulseObservationFailure::ChildExited",
        "WatchedPulseObservationFailure::Deadline",
        "lifecycle.next(slice_deadline)",
    ] {
        require(
            watched,
            required,
            "bounded watcher-versus-process observation",
        )?;
    }
    for required in [
        "observe_stable_process_liveness",
        ".observe_bound_client_area(",
        ".capture_client_area(",
    ] {
        require(native, required, "external replacement consequence")?;
    }
    for required in ["EXPECTED_BLUE", "EXPECTED_GREEN", "matching_samples * 4"] {
        require(color, required, "independent native color oracle")?;
    }
    Ok(())
}

fn audit_preservation(progression: &str, adjudication: &str) -> Result<(), String> {
    for required in [
        "fn await_preserved_predecessor(",
        "WatchedPulseTransition::MalformedPreservation",
        "CausalPredecessorPreservationObservationSet::new(",
        "observe_watched_native(",
        "PulseExecutableWorldFailure::Preservation(failure)",
    ] {
        require(progression, required, "watched preservation progression")?;
    }
    for required in [
        "ReplacementDeniedPreserving",
        "PlatformPulseReplacementDenialFamily::DslCompilation",
        "ActiveGenerationChanged",
        "ActiveFrameChanged",
        "ExpectedNativeColor::Green",
        "NativeWindowIdentityMismatch",
    ] {
        require(
            adjudication,
            required,
            "predecessor preservation adjudication",
        )?;
    }
    Ok(())
}

fn audit_atomic_action(source: &str) -> Result<(), String> {
    for required in [
        "file.sync_all()",
        "winsafe::ReplaceFile(",
        "winsafe::co::REPLACEFILE::WRITE_THROUGH",
        "temporary_cleanup: fs::remove_file(replacement)",
        "let observed = fs::read(&entry_source)",
        "ReadBackMismatch",
    ] {
        require(source, required, "atomic source replacement")?;
    }
    Ok(())
}

fn audit_close_and_failure(normal_close: &str, failure: &str) -> Result<(), String> {
    require(
        normal_close,
        "impl PulseExecutableWorld<Published<RecoveredBlue>>",
        "recovered-only normal close",
    )?;
    for required in [
        "SourceAction(PulseSourceActionFailure)",
        "WatchedObservation(WatchedPulseObservationFailure)",
        "Replacement(ExecutableReplacementFailure)",
        "Preservation(ExecutablePredecessorPreservationFailure)",
    ] {
        require(failure, required, "Phase 4 failure topology")?;
    }
    Ok(())
}

fn audit_courtroom(source: &str) -> Result<(), String> {
    for required in [
        "canonical_platform_pulse_survives_blue_green_denial_recovery_and_normal_shutdown",
        ".apply_green(",
        ".await_green_successor(",
        ".apply_malformed(",
        ".await_preserved_predecessor(",
        ".restore_canonical(",
        ".await_recovered_blue(",
        ".close_native_window(",
        "matching_color_samples() * 4",
        "matching_green_samples() * 4",
        "journey_started.elapsed() <= JOURNEY_BUDGET",
        "expired_green_observation_preserves_action_failure_and_teardown_disposition",
        "WatchedPulseTransition::GreenReplacement",
        "all_owned_resources_released()",
    ] {
        require(source, required, "cumulative Phase 4 courtroom")?;
    }
    for forbidden in [
        "#[ignore",
        ".kill()",
        "Stop-Process",
        "taskkill",
        "egui::",
        "eframe::",
        "lifecycle.next(",
        "inject_source(",
    ] {
        if source.contains(forbidden) {
            return Err(format!(
                "cumulative Phase 4 courtroom retained shortcut `{forbidden}`"
            ));
        }
    }
    Ok(())
}

fn require(source: &str, edge: &str, owner: &str) -> Result<(), String> {
    if source.contains(edge) {
        Ok(())
    } else {
        Err(format!("{owner} lost required edge `{edge}`"))
    }
}

impl Phase4RunnerSources {
    pub(super) fn capture(inventory: &WorkspaceSourceInventory) -> Self {
        let text = |path| inventory.text(path).to_owned();
        Self {
            progression: text(
                "apps/platform-pulse/tests/executable_world/product_process/progression.rs",
            ),
            source_action: text(
                "apps/platform-pulse/tests/executable_world/product_process/source_action_progression.rs",
            ),
            replacement: text(
                "apps/platform-pulse/tests/executable_world/product_process/replacement_progression.rs",
            ),
            preservation: text(
                "apps/platform-pulse/tests/executable_world/product_process/preservation_progression.rs",
            ),
            watched_observation: text(
                "apps/platform-pulse/tests/executable_world/product_process/watched_observation.rs",
            ),
            watched_native: text(
                "apps/platform-pulse/tests/executable_world/product_process/watched_native_observation.rs",
            ),
            native_color: text(
                "apps/platform-pulse/tests/executable_world/adjudication/native_color.rs",
            ),
            preservation_adjudication: text(
                "apps/platform-pulse/tests/executable_world/adjudication/predecessor_preservation.rs",
            ),
            atomic_replacement: text(
                "apps/platform-pulse/tests/executable_world/source_delta/atomic_replacement.rs",
            ),
            normal_close: text(
                "apps/platform-pulse/tests/executable_world/product_process/normal_close_progression.rs",
            ),
            failure_report: text(
                "apps/platform-pulse/tests/executable_world/failure_teardown/report.rs",
            ),
            courtroom: format!(
                "{}\n{}",
                text(
                    "apps/platform-pulse/tests/executable_world/courtroom/platform_pulse_lifecycle.rs",
                ),
                text(
                    "apps/platform-pulse/tests/executable_world/courtroom/platform_pulse_journey.rs",
                ),
            ),
        }
    }
}
