use std::panic::{catch_unwind, resume_unwind, AssertUnwindSafe};
use std::thread;
use std::time::{Duration, Instant};

use crate::native_platform::{
    NativePlatformContract, NativePlatformFailure, WindowsNativePlatform,
};
use crate::product_process::{CargoBuiltPlatformPulse, SuccessfulPlatformPulseExit};

#[test]
#[ignore = "requires the serialized interactive Windows 11 DX12 desktop"]
fn windows_native_boundary_world_retains_click_time_pointer_after_cursor_moves() {
    let platform = WindowsNativePlatform::certified().expect("Windows observation is qualified");
    let os_version = platform
        .observed_os_version()
        .expect("the executable environment reports a qualified Windows build");
    let mut launch = CargoBuiltPlatformPulse::exact()
        .and_then(CargoBuiltPlatformPulse::launch_native_phase6)
        .expect("the native phase 6 product process launches");
    let process_id = launch.process.id();
    let outcome = catch_unwind(AssertUnwindSafe(|| {
        execute_boundary_world(&platform, &os_version, &mut launch, process_id)
    }));
    if let Err(failure) = outcome {
        finalize_failed_world(&platform, &mut launch.process, process_id);
        resume_unwind(failure);
    }
}

fn execute_boundary_world(
    platform: &WindowsNativePlatform,
    os_version: &str,
    launch: &mut crate::product_process::NativePhase2ProcessLaunch,
    process_id: u32,
) {
    let deadline = Instant::now() + Duration::from_secs(20);
    let client = platform
        .bind_process_client_area(process_id, deadline)
        .expect("one process-owned native client area appears");
    let capture = platform
        .capture_client_area(&client)
        .expect("the native client area is externally capturable");
    let point = crate::external_observation::NativeClientPixelPoint::interior(
        &capture,
        capture.width() / 3,
        capture.height() / 2,
        2,
    )
    .expect("the click point is inside the captured client area");
    let delivered = match platform.deliver_pointer_activation(&client, point) {
        Ok(delivered) => delivered,
        Err(NativePlatformFailure::InputEnvironment(denial)) => {
            eprintln!(
                "WORTH_UI_LEDGER_ENVIRONMENT_DENIAL={}",
                serde_json::json!({
                    "schema": "worth-ui-native-phase6-environment-denial-v1",
                    "requirement": "P6-WINDOWS-WORLD-01",
                    "delivery_route": "system-input-to-native-message-queue",
                    "result": "environment-denied",
                    "denial": denial.to_string(),
                })
            );
            panic!("the Windows input world is not qualified: {denial}");
        }
        Err(failure) => panic!("the real OS input boundary failed: {failure}"),
    };
    let click_screen = delivered.screen_point();
    thread::sleep(Duration::from_millis(250));

    let bounds = platform
        .observe_bound_client_area(&client)
        .expect("the bound client area remains stable after delivery")
        .bounds();
    let moved_screen = (bounds.right() - 3, bounds.bottom() - 3);
    assert_ne!(moved_screen, click_screen);
    platform
        .move_cursor(moved_screen)
        .expect("the cursor moves after the click has been delivered");
    thread::sleep(Duration::from_millis(250));

    let close = platform
        .request_normal_close(&client)
        .expect("the real OS window accepts one normal close");
    assert_eq!(close.process_id(), process_id);
    assert_eq!(close.request_count(), 1);
    let exit = SuccessfulPlatformPulseExit::wait(&mut launch.process, deadline)
        .expect("native product exits successfully after OS close");
    assert!(exit.status().success());
    platform
        .verify_process_window_released(process_id)
        .expect("the process leaves no native window");

    let mut stdout = String::new();
    launch
        .stdout
        .read_to_string(&mut stdout)
        .expect("native phase 6 report is readable");
    let evidence: serde_json::Value =
        serde_json::from_str(stdout.trim()).expect("versioned native phase 6 evidence is JSON");
    assert_phase6_evidence(&evidence, &capture, bounds, click_screen, moved_screen);
    println!(
        "WORTH_UI_LEDGER_OBSERVATION={}",
        ledger_observation(&evidence, &capture, &os_version, click_screen, moved_screen)
    );
    let pointer_witnesses = u64::from(evidence["input"]["last_pointer_button"].is_object());
    assert_eq!(
        pointer_witnesses, 1,
        "phase 6 boundary world must retain one event-time pointer witness"
    );
    println!(
        "WORTH_UI_LEDGER_COUNTERS={{\"P6-WINDOWS-WORLD-01\":{}}}",
        pointer_witnesses
    );
}

fn assert_phase6_evidence(
    evidence: &serde_json::Value,
    capture: &crate::external_observation::NativeClientPixelCapture,
    bounds: crate::external_observation::NativeClientAreaBounds,
    click_screen: (i32, i32),
    moved_screen: (i32, i32),
) {
    assert_eq!(evidence["schema"], "worth-ui-native-phase6-evidence-v1");
    assert_eq!(evidence["terminal_zero"], true);
    assert_eq!(
        evidence["presentation"]["client_physical_size"],
        serde_json::json!([capture.width(), capture.height()])
    );
    let input = &evidence["input"];
    assert!(input["retained_events"]
        .as_u64()
        .is_some_and(|count| count > 0));
    let retained_batches = input["retained_batches"]
        .as_u64()
        .expect("the retained report includes a batch count");
    assert!(retained_batches > 0);
    let ingress = evidence["runtime_ingress"]
        .as_object()
        .expect("phase 6 evidence includes runtime ingress settlement");
    assert!(ingress["applied_batches"]
        .as_u64()
        .is_some_and(|count| count > 0));
    assert_eq!(ingress["drain_denied"], 0);
    assert!(ingress["typed_disposition_count"]
        .as_u64()
        .is_some_and(|count| count >= retained_batches));
    let terminal_census = evidence["terminal_census"]
        .as_object()
        .expect("phase 6 evidence includes terminal census");
    assert!(!terminal_census.is_empty());
    assert!(terminal_census
        .values()
        .all(|value| value.as_u64() == Some(0)));
    let button = input["last_pointer_button"]
        .as_object()
        .expect("the retained report includes the pointer button witness");
    let click_client = (
        i64::from(click_screen.0 - bounds.left()),
        i64::from(click_screen.1 - bounds.top()),
    );
    let moved_client = (
        i64::from(moved_screen.0 - bounds.left()),
        i64::from(moved_screen.1 - bounds.top()),
    );
    let scale_factor_milli = evidence["presentation"]["scale_factor_milli"]
        .as_i64()
        .expect("the presentation reports its observed scale factor");
    assert!(scale_factor_milli > 0);
    let click_surface = (
        logical_subpixels(click_client.0, scale_factor_milli),
        logical_subpixels(click_client.1, scale_factor_milli),
    );
    let moved_surface = (
        logical_subpixels(moved_client.0, scale_factor_milli),
        logical_subpixels(moved_client.1, scale_factor_milli),
    );
    let observed = (
        button["x_subpixels"]
            .as_i64()
            .expect("pointer x is an integer subpixel coordinate"),
        button["y_subpixels"]
            .as_i64()
            .expect("pointer y is an integer subpixel coordinate"),
    );
    assert!(
        within_subpixels(observed.0, click_surface.0, 3_000),
        "event-time x mismatch: observed={:?}; click_surface={:?}; moved_surface={:?}; scale_factor_milli={scale_factor_milli}; bounds={:?}",
        observed,
        click_surface,
        moved_surface,
        bounds,
    );
    assert!(
        within_subpixels(observed.1, click_surface.1, 3_000),
        "event-time y mismatch: observed={:?}; click_surface={:?}; moved_surface={:?}; scale_factor_milli={scale_factor_milli}; bounds={:?}",
        observed,
        click_surface,
        moved_surface,
        bounds,
    );
    assert!(
        !within_subpixels(observed.0, moved_surface.0, 3_000)
            || !within_subpixels(observed.1, moved_surface.1, 3_000),
        "the retained pointer position must not be reconstructed from the post-delivery cursor"
    );
    assert_eq!(button["coordinate_space"], "Viewport");
    assert_eq!(button["coordinate_unit"], "LogicalPoint");
    assert!(input["last_sequence"].as_u64().is_some_and(|sequence| {
        button["sequence"]
            .as_u64()
            .is_some_and(|button_sequence| sequence >= button_sequence && button_sequence > 0)
    }));
}

fn within_subpixels(actual: i64, expected: i64, tolerance: i64) -> bool {
    actual.abs_diff(expected) <= tolerance as u64
}

fn logical_subpixels(physical_coordinate: i64, scale_factor_milli: i64) -> i64 {
    (physical_coordinate * 1_000_000 + scale_factor_milli / 2) / scale_factor_milli
}

fn ledger_observation(
    evidence: &serde_json::Value,
    capture: &crate::external_observation::NativeClientPixelCapture,
    os_version: &str,
    click_screen: (i32, i32),
    moved_screen: (i32, i32),
) -> serde_json::Value {
    serde_json::json!({
        "schema": "worth-ui-native-phase6-boundary-observation-v1",
        "os_version": os_version,
        "architecture": std::env::consts::ARCH,
        "product_processes": 1,
        "client_physical_size": [capture.width(), capture.height()],
        "terminal_zero": evidence["terminal_zero"],
        "peak": evidence["peak"],
        "terminal_census": evidence["terminal_census"],
        "counters": evidence["counters"],
        "graphics": evidence["graphics"],
        "presentation": evidence["presentation"],
        "runtime_attribution": evidence["runtime_attribution"],
        "input": evidence["input"],
        "runtime_ingress": evidence["runtime_ingress"],
        "click_screen": click_screen,
        "post_delivery_cursor": moved_screen,
        "event_time_pointer_distinct": true,
    })
}

fn finalize_failed_world(
    platform: &WindowsNativePlatform,
    process: &mut crate::product_process::LivePlatformPulseProcess,
    process_id: u32,
) {
    let teardown = process.terminate_after_failure(Instant::now() + Duration::from_secs(5));
    let window_release = platform.verify_process_window_released(process_id);
    eprintln!(
        "WORTH_UI_LEDGER_FAILURE={}",
        serde_json::json!({
            "schema": "worth-ui-native-phase6-world-failure-v1",
            "process_id": process_id,
            "teardown": format!("{teardown:?}"),
            "window_release": format!("{window_release:?}"),
        })
    );
    assert!(
        teardown.is_ok(),
        "failed world teardown was incomplete: {teardown:?}"
    );
    assert!(
        window_release.is_ok(),
        "failed world retained a process window: {window_release:?}"
    );
}
