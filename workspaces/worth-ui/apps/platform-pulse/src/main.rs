mod application;
mod launch_configuration;
mod lifecycle_observation_publication;
mod native_frame;
mod query_source;
mod source_watch;
mod visual_identity_adjudication;
mod visual_identity_execution;
mod visual_observation_publication;

use std::process::ExitCode;

use launch_configuration::AdmittedPlatformPulseLaunchConfiguration;
use lifecycle_observation_publication::PlatformPulseObservationPublisher;

fn main() -> ExitCode {
    if std::env::args_os().any(|argument| argument == "--worth-ui-native-phase2-world") {
        return run_native_phase2_world();
    }
    let publisher = match PlatformPulseObservationPublisher::start() {
        Ok(publisher) => publisher,
        Err(denial) => {
            eprintln!("WORTH UI platform pulse observation stream could not start: {denial:?}");
            return ExitCode::FAILURE;
        }
    };
    let launch = match AdmittedPlatformPulseLaunchConfiguration::from_process() {
        Ok(launch) => launch,
        Err(denial) => {
            if let Err(publication) = publisher.launch_configuration_failure(&denial) {
                eprintln!(
                    "WORTH UI platform pulse launch denial could not be observed: {publication:?}"
                );
            }
            eprintln!("WORTH UI platform pulse launch was denied: {denial:?}");
            return ExitCode::from(2);
        }
    };
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([160.0, 96.0])
            .with_min_inner_size([160.0, 96.0]),
        renderer: eframe::Renderer::Wgpu,
        ..Default::default()
    };
    let frame_publisher = publisher.clone();
    let event_loop = eframe::run_native(
        "WORTH UI Platform Pulse",
        options,
        Box::new(move |creation| {
            Ok(Box::new(native_frame::PlatformPulseNativeFrame::new(
                creation,
                launch,
                frame_publisher,
            )))
        }),
    );
    match event_loop {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            if let Err(publication) = publisher.native_event_loop_failure() {
                eprintln!(
                    "WORTH UI platform pulse event-loop failure could not be observed: {publication:?}"
                );
            }
            eprintln!("WORTH UI platform pulse native event loop failed: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run_native_phase2_world() -> ExitCode {
    use worth_ui_native_platform::{
        UiNativePlatformOutcome, UiNativePlatformProfile, UiNativeWindowSpec, WorthUiNativePlatform,
    };
    let window = UiNativeWindowSpec::new("WORTH UI Platform Pulse", [160, 96]);
    let profile = UiNativePlatformProfile::single_window(window);
    let Ok(platform) = WorthUiNativePlatform::prepare(profile) else {
        return ExitCode::from(2);
    };
    match platform.run(worth_ui_platform_pulse::PlatformPulseNativeSeedApplication::new()) {
        UiNativePlatformOutcome::Closed(receipt)
            if receipt.terminal_census().is_zero()
                && receipt.presentation().retained_center_rgba8() == [47, 129, 247, 255]
                && receipt.presentation().retained_baseline_rgba8() == [0, 0, 0, 0] =>
        {
            println!("{}", native_phase2_evidence(&receipt));
            ExitCode::SUCCESS
        }
        outcome => {
            eprintln!("worth-ui-native-phase2 stopped: {outcome:?}");
            ExitCode::from(3)
        }
    }
}

fn native_phase2_evidence(
    receipt: &worth_ui_native_platform::UiNativePlatformCloseReceipt,
) -> serde_json::Value {
    let presentation = receipt.presentation();
    let cost = presentation.cost();
    let peak = receipt.peak_census();
    let presentation = serde_json::json!({
        "presented_source": presentation.source_rgba8(),
        "retained_center": presentation.retained_center_rgba8(),
        "retained_baseline": presentation.retained_baseline_rgba8(),
        "client_physical_size": presentation.client_physical_size(),
        "scale_factor_milli": presentation.scale_factor_milli(),
        "frame": presentation.presented_frame(),
        "surface": presentation.semantic_surface(),
        "binding": presentation.binding_generation(),
        "mounted_instance": presentation.mounted_instance(),
        "node_receipt": presentation.node_receipt(),
        "presentation_attempt": presentation.presentation_attempt(),
        "logical_bounds_milli": presentation.logical_bounds_milli(),
        "order_ordinal": presentation.order_ordinal(),
    });
    let attribution = receipt.client_attribution();
    let runtime_attribution = serde_json::json!({
        "frame": attribution.frame(),
        "surface": attribution.surface(),
        "binding": attribution.binding(),
        "mounted_instance": attribution.mounted_instance(),
        "node_receipt": attribution.node_receipt(),
        "presentation_attempt": attribution.presentation_attempt(),
    });
    let counters = serde_json::json!({
        "surface_acquisitions": cost.surface_acquisitions(),
        "queue_submissions": cost.queue_submissions(),
        "presents": cost.presents(),
        "render_passes": cost.render_passes(),
        "readiness_signals": receipt.readiness_signals(),
        "redraw_turns": receipt.redraw_turns(),
        "idle_wait_turns": receipt.idle_wait_turns(),
        "coalesced_wakes": receipt.coalesced_wakes(),
        "port_crossings": receipt.port_crossings(),
    });
    let graphics = serde_json::json!({
        "event_loop_thread": receipt.event_loop_thread(),
        "event_loop_thread_matches_launch": receipt.event_loop_thread_matches_launch(),
        "adapter": receipt.graphics().adapter_name(),
        "vendor": receipt.graphics().vendor(),
        "device": receipt.graphics().device(),
        "driver": receipt.graphics().driver(),
        "driver_info": receipt.graphics().driver_info(),
        "device_type": receipt.graphics().device_type(),
        "backend": receipt.graphics().backend(),
        "surface_format": receipt.graphics().surface_format(),
        "present_mode": receipt.graphics().present_mode(),
        "alpha_mode": receipt.graphics().alpha_mode(),
        "retained_format": receipt.graphics().retained_format(),
        "max_texture_dimension_2d": receipt.graphics().max_texture_dimension_2d(),
    });
    let peak = serde_json::Value::Object(
        peak.entries()
            .map(|(class, count)| (class.to_owned(), serde_json::Value::from(count)))
            .collect::<serde_json::Map<_, _>>(),
    );
    let terminal_census = serde_json::Value::Object(
        receipt
            .terminal_census()
            .entries()
            .map(|(class, count)| (class.to_owned(), serde_json::Value::from(count)))
            .collect::<serde_json::Map<_, _>>(),
    );
    serde_json::json!({
        "schema": "worth-ui-native-phase2-evidence-v1",
        "presentation": presentation,
        "runtime_attribution": runtime_attribution,
        "counters": counters,
        "graphics": graphics,
        "peak": peak,
        "terminal_census": terminal_census,
        "terminal_zero": receipt.terminal_census().is_zero(),
    })
}
