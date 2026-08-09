use worth_ui_native_platform::{
    UiNativeApplicationDefinition, UiNativeApplicationPreparation,
    UiNativeApplicationPreparationOutcome, UiNativePlatformOutcome, UiNativePlatformProfile,
    UiNativeWindowSpec, WorthUiNativePlatform,
};

struct Application;

impl UiNativeApplicationDefinition for Application {
    fn prepare(
        self,
        mut preparation: UiNativeApplicationPreparation,
    ) -> UiNativeApplicationPreparationOutcome {
        let borrowed = preparation.builder();
        drop(borrowed);
        match preparation.complete() {
            UiNativeApplicationPreparationOutcome::Prepared(prepared) => {
                UiNativeApplicationPreparationOutcome::Prepared(prepared)
            }
            UiNativeApplicationPreparationOutcome::Denied(denial) => {
                UiNativeApplicationPreparationOutcome::Denied(denial)
            }
        }
    }
}

fn main() {
    let profile = UiNativePlatformProfile::single_window(UiNativeWindowSpec::new(
        "WORTH UI",
        [160, 96],
    ));
    let platform = WorthUiNativePlatform::prepare(profile).unwrap();
    let _: UiNativePlatformOutcome = platform.run(Application);
}
