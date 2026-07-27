mod application;
mod launch_configuration;
mod lifecycle_observation_publication;
mod native_frame;
mod source_watch;

use std::process::ExitCode;

use launch_configuration::AdmittedPlatformPulseLaunchConfiguration;
use lifecycle_observation_publication::PlatformPulseObservationPublisher;

fn main() -> ExitCode {
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
