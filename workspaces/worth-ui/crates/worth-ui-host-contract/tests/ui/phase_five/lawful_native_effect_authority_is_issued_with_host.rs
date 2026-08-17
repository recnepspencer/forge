use worth_ui_native_platform::{
    UiNativeApplicationDefinition, UiNativeApplicationPreparation,
    UiNativeApplicationPreparationOutcome, UiNativePlatformOutcome, UiNativePlatformProfile,
    UiNativeWindowSpec, WorthUiNativePlatform,
};

struct LawfulRuntimeApplication;

impl UiNativeApplicationDefinition for LawfulRuntimeApplication {
    fn prepare(
        self,
        mut preparation: UiNativeApplicationPreparation,
    ) -> UiNativeApplicationPreparationOutcome {
        preparation
            .builder()
            .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse())
            .unwrap();
        preparation.complete()
    }
}

fn main() {
    let profile = UiNativePlatformProfile::single_window(UiNativeWindowSpec::new(
        "lawful native runtime binding",
        [160, 96],
    ));
    let platform = WorthUiNativePlatform::prepare(profile).unwrap();
    let _: UiNativePlatformOutcome = platform.run(LawfulRuntimeApplication);
}
