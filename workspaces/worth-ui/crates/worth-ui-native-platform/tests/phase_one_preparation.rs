use worth_ui_native_platform::{
    UiNativeApplicationDefinition, UiNativeApplicationPreparation,
    UiNativeApplicationPreparationDenialCause, UiNativeApplicationPreparationOutcome,
    UiNativePlatformOutcome, UiNativePlatformProfile, UiNativePlatformStopReason,
    UiNativeWindowSpec, WorthUiNativePlatform,
};

struct CompleteApplication;

impl UiNativeApplicationDefinition for CompleteApplication {
    fn prepare(
        self,
        mut preparation: UiNativeApplicationPreparation,
    ) -> UiNativeApplicationPreparationOutcome {
        let policy = worth_ui::facade::inspection::UiVisualInspectionPolicy::production_default(
            worth_ui::facade::inspection::UiVisualInspectionDisclosure::local_development_unredacted(),
        )
        .unwrap();
        preparation
            .builder()
            .with_visual_inspection_policy(policy)
            .unwrap();
        preparation.complete()
    }
}

struct DeniedApplication;

impl UiNativeApplicationDefinition for DeniedApplication {
    fn prepare(
        self,
        preparation: UiNativeApplicationPreparation,
    ) -> UiNativeApplicationPreparationOutcome {
        preparation.deny(UiNativeApplicationPreparationDenialCause::ApplicationRejected)
    }
}

#[test]
fn phase_one_preparation_is_effect_free_and_stops_before_native_activation() {
    let platform = WorthUiNativePlatform::prepare(profile()).unwrap();
    let UiNativePlatformOutcome::Stopped(stop) = platform.run(CompleteApplication) else {
        panic!("Phase 1 complete preparation must stop before native effects");
    };
    assert_eq!(
        stop.reason(),
        UiNativePlatformStopReason::NativeEffectsNotActivatedInPhaseOne
    );
}

#[test]
fn application_denial_proves_no_subsystem_or_event_loop_client_existed() {
    let platform = WorthUiNativePlatform::prepare(profile()).unwrap();
    let UiNativePlatformOutcome::ApplicationPreparationDenied(denial) =
        platform.run(DeniedApplication)
    else {
        panic!("explicit application denial must remain before native effects");
    };
    assert_eq!(
        denial.cause(),
        UiNativeApplicationPreparationDenialCause::ApplicationRejected
    );
    assert!(denial.preparation_identity() > 0);
}

#[test]
fn profile_validation_denies_before_a_prepared_platform_exists() {
    let empty_title =
        UiNativePlatformProfile::single_window(UiNativeWindowSpec::new("", [160, 96]));
    assert!(WorthUiNativePlatform::prepare(empty_title).is_err());
    let empty_extent =
        UiNativePlatformProfile::single_window(UiNativeWindowSpec::new("Pulse", [0, 96]));
    assert!(WorthUiNativePlatform::prepare(empty_extent).is_err());
}

fn profile() -> UiNativePlatformProfile {
    UiNativePlatformProfile::single_window(UiNativeWindowSpec::new(
        "WORTH UI Platform Pulse",
        [160, 96],
    ))
}
