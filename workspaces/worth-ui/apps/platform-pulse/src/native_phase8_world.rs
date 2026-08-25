use std::process::ExitCode;

use worth_ui_native_platform::{
    UiNativePlatformOutcome, UiNativePlatformProfile, UiNativeWindowSpec, WorthUiNativePlatform,
};

pub(crate) fn run() -> ExitCode {
    let profile = UiNativePlatformProfile::single_window(UiNativeWindowSpec::new(
        "WORTH UI Platform Pulse Phase 8",
        [160, 96],
    ));
    let Ok(platform) = WorthUiNativePlatform::prepare(profile) else {
        return ExitCode::from(2);
    };
    let application = worth_ui_platform_pulse::PlatformPulseNativeSeedApplication::new()
        .with_surface_successor_capture();
    match platform.run(application) {
        UiNativePlatformOutcome::Closed(receipt) if receipt.terminal_census().is_zero() => {
            let Some(evidence) = crate::native_phase8_evidence::evidence(&receipt) else {
                return ExitCode::from(3);
            };
            println!("{evidence}");
            ExitCode::SUCCESS
        }
        outcome => {
            eprintln!("worth-ui-native-phase8 stopped: {outcome:?}");
            ExitCode::from(3)
        }
    }
}
