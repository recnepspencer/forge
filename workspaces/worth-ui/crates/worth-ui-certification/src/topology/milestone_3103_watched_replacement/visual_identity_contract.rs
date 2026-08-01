use crate::topology::WorkspaceSourceInventory;

#[derive(Clone)]
pub(super) struct VisualIdentityRunnerSources {
    pub(super) progression: String,
    pub(super) visual_progression: String,
    pub(super) identity_trace: String,
    pub(super) overlay_pixels: String,
    pub(super) lifecycle_cleanup: String,
    pub(super) courtroom: String,
    pub(super) windows_capture: String,
    pub(super) wgc_capture: String,
    pub(super) process_launch: String,
    pub(super) native_lease: String,
}

pub(super) fn audit(inventory: &WorkspaceSourceInventory) -> Result<(), String> {
    audit_required_files(inventory)?;
    audit_sources(&VisualIdentityRunnerSources::capture(inventory))
}

pub(super) fn audit_sources(sources: &VisualIdentityRunnerSources) -> Result<(), String> {
    audit_typestate(&sources.progression, &sources.visual_progression)?;
    audit_event_pixel_join(&sources.visual_progression)?;
    audit_identity_oracle(&sources.identity_trace)?;
    audit_overlay_oracle(&sources.overlay_pixels)?;
    audit_cleanup(&sources.lifecycle_cleanup)?;
    audit_courtroom(&sources.courtroom)?;
    audit_native_capture(&sources.windows_capture, &sources.wgc_capture)?;
    audit_native_desktop_lease(&sources.process_launch, &sources.native_lease)
}

fn audit_required_files(inventory: &WorkspaceSourceInventory) -> Result<(), String> {
    for path in [
        "apps/platform-pulse/tests/executable_world/adjudication/identity_trace.rs",
        "apps/platform-pulse/tests/executable_world/adjudication/visual_overlay_pixels.rs",
        "apps/platform-pulse/tests/executable_world/product_process/visual_snapshot_progression.rs",
        "apps/platform-pulse/tests/executable_world/native_platform/windows.rs",
        "apps/platform-pulse/tests/executable_world/native_platform/windows/client_capture.rs",
        "apps/platform-pulse/tests/executable_world/product_process/native_desktop_lease.rs",
        "apps/platform-pulse/tests/executable_world/courtroom/platform_pulse_cleanup.rs",
    ] {
        if inventory.source(path).is_none() {
            return Err(format!("visual identity courtroom lost `{path}`"));
        }
    }
    Ok(())
}

fn audit_typestate(progression: &str, visual: &str) -> Result<(), String> {
    for edge in [
        "pub(crate) struct SnapshotCaptured<Stage>",
        "pub(crate) struct IdentityTraced<Stage>",
        "pub(crate) struct OverlayPublished<Stage>",
        "pub(crate) struct OverlayCleared<Stage>",
    ] {
        require(progression, edge, "visual executable typestate")?;
    }
    for edge in [
        "impl PulseExecutableWorld<Published<FirstCurrent>>",
        "PulseExecutableWorld<Published<SnapshotCaptured<FirstCurrent>>>",
        "impl PulseExecutableWorld<Published<SnapshotCaptured<FirstCurrent>>>",
        "PulseExecutableWorld<Published<IdentityTraced<FirstCurrent>>>",
        "impl PulseExecutableWorld<Published<IdentityTraced<FirstCurrent>>>",
        "PulseExecutableWorld<Published<OverlayPublished<FirstCurrent>>>",
        "impl PulseExecutableWorld<Published<OverlayPublished<FirstCurrent>>>",
        "PulseExecutableWorld<Published<OverlayCleared<FirstCurrent>>>",
    ] {
        require(visual, edge, "visual executable progression")?;
    }
    Ok(())
}

fn audit_event_pixel_join(source: &str) -> Result<(), String> {
    for edge in [
        "await_visual_event(",
        "WatchedPulseTransition::VisualOverlayPublished",
        "WatchedPulseTransition::VisualOverlayCleared",
        "observe_watched_native(&mut world)",
        "adjudicate_overlay_pixels(",
        "adjudicate_restored_pixels(",
    ] {
        require(source, edge, "event-plus-pixel progression")?;
    }
    Ok(())
}

fn audit_identity_oracle(source: &str) -> Result<(), String> {
    for edge in [
        "snapshot.visible_region_count() != PLATFORM_PULSE_VISIBLE_REGION_COUNT",
        "snapshot.hit_test_region_count() != PLATFORM_PULSE_HIT_TEST_REGION_COUNT",
        "PLATFORM_PULSE_TARGET_LOGICAL_POINT",
        "PLATFORM_PULSE_BACKGROUND_LOGICAL_POINT",
        "trace.target().visible_region() != snapshot.expected_target_region()?",
        "require_same_resolution_identity(trace.target().visible(), trace.target().hit())",
        "trace.target().hit().mounted().node_receipt()",
        "== trace.background().hit().mounted().node_receipt()",
        "PLATFORM_PULSE_IDENTITY_TARGET_AUTHORED_NAME",
    ] {
        require(source, edge, "nondegenerate independent identity oracle")?;
    }
    Ok(())
}

fn audit_overlay_oracle(source: &str) -> Result<(), String> {
    for edge in [
        "PlatformPulseLifecycleObservation::VisualOverlayPublished",
        "PlatformPulseLifecycleObservation::VisualOverlayCleared",
        "EXPECTED_MAGENTA",
        "matching * 4 < samples.len() * 3",
        "matching != 0",
        "require_control_pixels(",
        "PLATFORM_PULSE_TARGET_RGB",
        "EXPECTED_BLUE",
    ] {
        require(source, edge, "independent overlay pixel oracle")?;
    }
    Ok(())
}

fn audit_cleanup(source: &str) -> Result<(), String> {
    for edge in [
        "shutdown.cancelled_visual_capture_count()",
        "shutdown.disposed_visual_snapshot_count()",
        "shutdown.disposed_visual_pixel_bytes()",
        "shutdown.disposed_visual_structural_bytes()",
        "shutdown.cancelled_pending_overlay_count()",
        "shutdown.disposed_published_overlay_count()",
        "shutdown.disposed_clearing_overlay_count()",
        "if capture != (0, 0, 0, 0)",
        "if overlay != (0, 0, 0)",
    ] {
        require(source, edge, "visual residue adjudication")?;
    }
    Ok(())
}

fn audit_courtroom(source: &str) -> Result<(), String> {
    let ordered = [
        ".await_visual_snapshot(",
        ".await_identity_trace(",
        ".await_overlay_published(",
        ".await_overlay_cleared(",
        ".apply_green(",
        ".await_green_successor(",
    ];
    let mut prior = 0;
    for edge in ordered {
        let position = source[prior..]
            .find(edge)
            .map(|offset| prior + offset)
            .ok_or_else(|| format!("cumulative visual courtroom lost `{edge}`"))?;
        prior = position + edge.len();
    }
    for edge in [
        "assert!(matching * 4 >= sampled * 3)",
        "close_recovered(self.recovered)",
        "cancelled_visual_capture_count(), 0",
        "disposed_published_overlay_count(), 0",
    ] {
        require(source, edge, "cumulative visual courtroom")?;
    }
    Ok(())
}

fn audit_native_capture(owner: &str, capture: &str) -> Result<(), String> {
    for edge in [
        "Monitor::from_point(",
        "monitor_capture_region(",
        "client_capture::exact_window(process_id, candidate.window.ptr() as u32)?",
        "self.observe_bound_client_area(bound)?",
        "struct WindowsCaptureExposure<'bound>",
        "HwndPlace::Place(co::HWND_PLACE::TOP)",
        "win::DwmFlush()",
        "let exposure = self.expose_bound_client_area(bound)?",
        "Self::capture_exposed_client_area(exposure)",
        "client_capture::capture_client(&bound.capture_window, client)?",
        "ClientOutsideCaptureMonitor",
    ] {
        require(owner, edge, "process-bound HWND capture owner")?;
    }
    for edge in [
        "Window::all()",
        "window.pid().ok() == Some(process_id)",
        "window.id().ok() == Some(window_id)",
        "window.capture_image()",
        "crop_client(screenshot, window_left, window_top, client)",
        "right > screenshot.width() || bottom > screenshot.height()",
        "cropped.extend_from_slice(",
    ] {
        require(capture, edge, "exact-HWND WGC capture")?;
    }
    if capture.matches("Window::all()").count() != 1
        || capture.matches("window.capture_image()").count() != 1
    {
        return Err("exact-HWND WGC capture must enumerate and capture exactly once".to_owned());
    }
    let public_capture = owner
        .rfind("fn capture_client_area(")
        .ok_or_else(|| "native client capture implementation is missing".to_owned())?;
    let expose = owner[public_capture..]
        .find("let exposure = self.expose_bound_client_area(bound)?")
        .ok_or_else(|| "native capture lost its explicit exposure witness".to_owned())?;
    let consume = owner[public_capture..]
        .find("Self::capture_exposed_client_area(exposure)")
        .ok_or_else(|| "native capture no longer consumes its exposure witness".to_owned())?;
    if expose >= consume {
        return Err("native capture must mint exposure before consuming it".to_owned());
    }
    for resampling in ["imageops::resize", "FilterType::Nearest", "scale_floor("] {
        forbid(owner, resampling, "native client capture owner")?;
        forbid(capture, resampling, "exact-HWND WGC capture")?;
    }
    Ok(())
}

fn audit_native_desktop_lease(launch: &str, lease: &str) -> Result<(), String> {
    for edge in [
        "_native_desktop_lease: NativeDesktopLease",
        "NativeDesktopLease::acquire(desktop_deadline)",
        "Command::new(&self.executable)",
    ] {
        require(launch, edge, "process-owned native desktop lease")?;
    }
    let acquisition = launch
        .find("NativeDesktopLease::acquire(desktop_deadline)")
        .ok_or_else(|| "native desktop lease acquisition is missing".to_owned())?;
    let spawn = launch
        .find("Command::new(&self.executable)")
        .ok_or_else(|| "product child spawn is missing".to_owned())?;
    if acquisition >= spawn {
        return Err("native desktop lease must be acquired before child spawn".to_owned());
    }
    for edge in [
        "OnceLock<Mutex<()>>",
        "desktop.try_lock()",
        "Instant::now() < deadline",
        "Err(TryLockError::WouldBlock) => return Err(NativeDesktopLeaseDeadline)",
    ] {
        require(lease, edge, "bounded native desktop lease")?;
    }
    Ok(())
}

fn require(source: &str, edge: &str, owner: &str) -> Result<(), String> {
    source
        .contains(edge)
        .then_some(())
        .ok_or_else(|| format!("{owner} lost required edge `{edge}`"))
}

fn forbid(source: &str, shortcut: &str, owner: &str) -> Result<(), String> {
    (!source.contains(shortcut))
        .then_some(())
        .ok_or_else(|| format!("{owner} reopened shortcut `{shortcut}`"))
}

impl VisualIdentityRunnerSources {
    pub(super) fn capture(inventory: &WorkspaceSourceInventory) -> Self {
        let text = |path| inventory.text(path).to_owned();
        Self {
            progression: text(
                "apps/platform-pulse/tests/executable_world/product_process/progression.rs",
            ),
            visual_progression: text(
                "apps/platform-pulse/tests/executable_world/product_process/visual_snapshot_progression.rs",
            ),
            identity_trace: text(
                "apps/platform-pulse/tests/executable_world/adjudication/identity_trace.rs",
            ),
            overlay_pixels: text(
                "apps/platform-pulse/tests/executable_world/adjudication/visual_overlay_pixels.rs",
            ),
            lifecycle_cleanup: text(
                "apps/platform-pulse/tests/executable_world/adjudication/lifecycle_cleanup.rs",
            ),
            courtroom: inventory
                .rust_files_under("apps/platform-pulse/tests/executable_world/courtroom")
                .map(|source| source.text())
                .collect::<Vec<_>>()
                .join("\n"),
            windows_capture: text(
                "apps/platform-pulse/tests/executable_world/native_platform/windows.rs",
            ),
            wgc_capture: text(
                "apps/platform-pulse/tests/executable_world/native_platform/windows/client_capture.rs",
            ),
            process_launch: text(
                "apps/platform-pulse/tests/executable_world/product_process/launch.rs",
            ),
            native_lease: text(
                "apps/platform-pulse/tests/executable_world/product_process/native_desktop_lease.rs",
            ),
        }
    }
}
