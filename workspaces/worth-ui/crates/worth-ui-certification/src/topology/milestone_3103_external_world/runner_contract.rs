use std::collections::BTreeSet;
use std::path::Path;

use crate::topology::WorkspaceSourceInventory;

const TEST_ROOT: &str = "apps/platform-pulse/tests";

pub(super) fn audit(inventory: &WorkspaceSourceInventory) -> Result<(), String> {
    audit_required_topology(inventory)?;
    audit_runner_purity(inventory)?;
    audit_entry(inventory.text("apps/platform-pulse/tests/executable_world.rs"))?;
    audit_process(
        inventory.text("apps/platform-pulse/tests/executable_world/product_process/launch.rs"),
        inventory.text(
            "apps/platform-pulse/tests/executable_world/product_process/progression.rs",
        ),
        inventory.text(
            "apps/platform-pulse/tests/executable_world/product_process/first_frame_progression.rs",
        ),
        inventory.text(
            "apps/platform-pulse/tests/executable_world/product_process/normal_close_progression.rs",
        ),
    )?;
    audit_failure_teardown(
        inventory.text("apps/platform-pulse/tests/executable_world/failure_teardown/report.rs"),
        inventory.text(
            "apps/platform-pulse/tests/executable_world/failure_teardown/resource_cleanup.rs",
        ),
    )?;
    audit_native_boundary(
        inventory.text("apps/platform-pulse/tests/executable_world/native_platform/windows.rs"),
        inventory.text(
            "apps/platform-pulse/tests/executable_world/native_platform/windows/client_capture.rs",
        ),
    )?;
    let courtroom = format!(
        "{}\n{}\n{}",
        inventory.text(
            "apps/platform-pulse/tests/executable_world/courtroom/platform_pulse_lifecycle.rs",
        ),
        inventory.text(
            "apps/platform-pulse/tests/executable_world/courtroom/platform_pulse_journey.rs",
        ),
        inventory.text(
            "apps/platform-pulse/tests/executable_world/courtroom/platform_pulse_cleanup.rs",
        ),
    );
    audit_courtroom(&courtroom)
}

fn audit_required_topology(inventory: &WorkspaceSourceInventory) -> Result<(), String> {
    let required = [
        "executable_world.rs",
        "executable_world/adjudication/lifecycle_cleanup.rs",
        "executable_world/adjudication/mod.rs",
        "executable_world/adjudication/source_to_pixel.rs",
        "executable_world/courtroom/mod.rs",
        "executable_world/courtroom/platform_pulse_lifecycle.rs",
        "executable_world/external_observation/lifecycle_stream.rs",
        "executable_world/external_observation/lifecycle_teardown.rs",
        "executable_world/external_observation/mod.rs",
        "executable_world/external_observation/native_client_area.rs",
        "executable_world/external_observation/process_liveness.rs",
        "executable_world/failure_teardown/mod.rs",
        "executable_world/failure_teardown/report.rs",
        "executable_world/failure_teardown/resource_cleanup.rs",
        "executable_world/installation/canonical_platform_pulse.rs",
        "executable_world/installation/isolated_source_sandbox.rs",
        "executable_world/installation/mod.rs",
        "executable_world/native_platform/contract.rs",
        "executable_world/native_platform/mod.rs",
        "executable_world/native_platform/windows.rs",
        "executable_world/native_platform/windows/client_capture.rs",
        "executable_world/product_process/launch.rs",
        "executable_world/product_process/first_frame_progression.rs",
        "executable_world/product_process/mod.rs",
        "executable_world/product_process/normal_close_progression.rs",
        "executable_world/product_process/progression.rs",
        "executable_world/product_process/shutdown.rs",
    ]
    .into_iter()
    .map(|path| Path::new(TEST_ROOT).join(path))
    .collect::<BTreeSet<_>>();
    let observed = inventory
        .rust_files_under(TEST_ROOT)
        .map(|source| source.relative_path().to_path_buf())
        .collect::<BTreeSet<_>>();
    let missing = required.difference(&observed).collect::<Vec<_>>();
    if !missing.is_empty() {
        return Err(format!(
            "Phase 3 executable-world runner topology lost required files: {missing:?}"
        ));
    }
    Ok(())
}

fn audit_runner_purity(inventory: &WorkspaceSourceInventory) -> Result<(), String> {
    for source in inventory.rust_files_under(TEST_ROOT) {
        for forbidden in [
            "worth_ui_runtime",
            "worth_ui_dsl",
            "worth_ui_certification",
            "worth_ui_test_support",
            "eframe::",
            "egui::",
        ] {
            if source.text().contains(forbidden) {
                return Err(format!(
                    "{} cannot import product internals or adapter mechanics via `{forbidden}`",
                    source.relative_path().display()
                ));
            }
        }
    }
    Ok(())
}

fn audit_entry(source: &str) -> Result<(), String> {
    for required in [
        "#[path = \"executable_world/courtroom/mod.rs\"]",
        "#[path = \"executable_world/failure_teardown/mod.rs\"]",
        "#[path = \"executable_world/native_platform/mod.rs\"]",
        "#[cfg(not(target_os = \"windows\"))]",
        "NativePlatformPosture::CompileOnly",
    ] {
        require(source, required, "test entry")?;
    }
    if source.contains("#[ignore") {
        return Err("the executable-world entry cannot ignore its platform posture".to_owned());
    }
    Ok(())
}

fn audit_process(
    launch: &str,
    progression: &str,
    first_frame: &str,
    normal_close: &str,
) -> Result<(), String> {
    require(
        launch,
        "env!(\"CARGO_BIN_EXE_worth-ui-platform-pulse\")",
        "Cargo-built binary identity",
    )?;
    require(
        launch,
        ".stdout(Stdio::piped())",
        "typed stdout lifecycle stream",
    )?;
    let nested_cargo_command = ["Command", "::new(", "\"cargo\"", ")"].concat();
    if launch.contains(&nested_cargo_command) {
        return Err("the runner cannot recursively launch Cargo".to_owned());
    }
    for state in [
        "pub(crate) struct Installed",
        "pub(crate) struct AwaitingFirstFrame",
        "pub(crate) struct Published",
        "pub(crate) struct Closed",
    ] {
        require(progression, state, "typed executable-world state")?;
    }
    for transition in ["fn install(", "fn launch("] {
        require(progression, transition, "typed executable-world transition")?;
    }
    require(
        first_frame,
        "fn await_first_frame(",
        "typed first-frame transition",
    )?;
    require(
        normal_close,
        "fn close_native_window(",
        "typed normal-close transition",
    )?;
    Ok(())
}

fn audit_failure_teardown(report: &str, cleanup: &str) -> Result<(), String> {
    for required in [
        "pub(crate) struct PulseExecutableWorldFailureReport",
        "primary: PulseExecutableWorldFailure",
        "teardown: ExecutableWorldFailureTeardown",
        "InstallationOnly(InstallationOnlyFailureTeardown)",
        "Unbound(UnboundFailureTeardown)",
        "NativeBound(NativeBoundFailureTeardown)",
        "all_owned_resources_released",
    ] {
        require(report, required, "typed failure-teardown report")?;
    }
    for required in [
        "process.terminate_after_failure(deadline)",
        "lifecycle.teardown_after_failure(deadline)",
        "platform.verify_process_window_released(process_id)",
        "installation.close()",
    ] {
        require(cleanup, required, "bounded failure teardown")?;
    }
    if cleanup.contains("Closed {") || cleanup.contains("adjudicate_lifecycle_cleanup") {
        return Err("emergency teardown cannot construct normal-close success".to_owned());
    }
    Ok(())
}

pub(super) fn audit_native_boundary(owner: &str, capture: &str) -> Result<(), String> {
    for required in [
        "win::SetProcessDPIAware()",
        "win::EnumWindows(",
        "GetWindowThreadProcessId()",
        "owner_process_id == process_id",
        "window.IsWindowVisible()",
        "!window.IsIconic()",
        "GetClientRect()",
        "ClientToScreenRc(",
        "owner_process_id != bound.observation.process_id()",
        "client_bounds(&bound.window)? != bound.observation.bounds()",
        "Monitor::from_point(",
        "monitor_capture_region(",
        "client_capture::exact_window(process_id, candidate.window.ptr() as u32)?",
        "capture_window: Window",
        "struct WindowsCaptureExposure<'bound>",
        "HwndPlace::Place(co::HWND_PLACE::TOP)",
        "win::DwmFlush()",
        "let exposure = self.expose_bound_client_area(bound)?",
        "Self::capture_exposed_client_area(exposure)",
        "client_capture::capture_client(&bound.capture_window, client)?",
        "NativeClientPixelCapture::new(",
        "get_pattern::<UIWindowPattern>()",
        ".and_then(|pattern| pattern.close())",
    ] {
        require(owner, required, "Windows HWND owner")?;
    }
    for forbidden in [
        "Screenshot",
        "Window::all()",
        "window.pid()",
        ".capture_image()",
        ".capture_region(",
        "capture_as_image",
        "get_name()",
        "title() ==",
        "Stop-Process",
        "taskkill",
    ] {
        if owner.contains(forbidden) {
            return Err(format!(
                "Windows HWND owner retained forbidden shortcut `{forbidden}`"
            ));
        }
    }
    audit_wgc_capture(capture)?;
    for resampling in ["imageops::resize", "FilterType::Nearest", "scale_floor("] {
        if owner.contains(resampling) || capture.contains(resampling) {
            return Err(format!(
                "Windows native boundary retained forbidden shortcut `{resampling}`"
            ));
        }
    }
    Ok(())
}

fn audit_wgc_capture(source: &str) -> Result<(), String> {
    for required in [
        "Window::all()",
        "window.pid().ok() == Some(process_id)",
        "window.id().ok() == Some(window_id)",
        "window.capture_image()",
        "crop_client(screenshot, window_left, window_top, client)",
        "right > screenshot.width() || bottom > screenshot.height()",
        "let raw = screenshot.into_raw()",
        "cropped.extend_from_slice(",
    ] {
        require(source, required, "exact-HWND WGC capture")?;
    }
    if source.matches("Window::all()").count() != 1
        || source.matches("window.capture_image()").count() != 1
    {
        return Err("exact-HWND WGC capture must enumerate and capture exactly once".to_owned());
    }
    Ok(())
}

fn audit_courtroom(source: &str) -> Result<(), String> {
    for required in [
        "canonical_platform_pulse_survives_blue_green_denial_recovery_and_normal_shutdown",
        "CargoBuiltPlatformPulse::exact()",
        ".await_first_frame(",
        ".close_native_window(",
        "host_session_released()",
        "installation_removed()",
        "journey_started.elapsed() <= JOURNEY_BUDGET",
        "expired_first_frame_deadline_preserves_primary_failure_and_teardown_disposition",
        "PulseExecutableWorldFailure::Lifecycle(",
        "all_owned_resources_released()",
    ] {
        require(source, required, "cumulative native courtroom")?;
    }
    require_same_line(
        source,
        "matching_blue_samples() * 4 >=",
        ".sampled_pixels() * 3",
        "independent blue-pixel threshold",
    )?;
    for forbidden in ["#[ignore", ".kill()", "Stop-Process", "taskkill"] {
        if source.contains(forbidden) {
            return Err(format!(
                "cumulative native courtroom retained forbidden shortcut `{forbidden}`"
            ));
        }
    }
    Ok(())
}

fn require(source: &str, required: &str, owner: &str) -> Result<(), String> {
    if source.contains(required) {
        Ok(())
    } else {
        Err(format!("{owner} lost required edge `{required}`"))
    }
}

fn require_same_line(source: &str, left: &str, right: &str, owner: &str) -> Result<(), String> {
    if source
        .lines()
        .any(|line| line.contains(left) && line.contains(right))
    {
        Ok(())
    } else {
        Err(format!(
            "{owner} lost same-line invariant `{left} ... {right}`"
        ))
    }
}
