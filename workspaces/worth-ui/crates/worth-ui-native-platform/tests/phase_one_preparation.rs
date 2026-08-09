use worth_ui_native_platform::{
    UiNativeApplicationDefinition, UiNativeApplicationPreparation,
    UiNativeApplicationPreparationDenialCause, UiNativeApplicationPreparationOutcome,
    UiNativePlatformOutcome, UiNativePlatformProfile, UiNativeWindowSpec, WorthUiNativePlatform,
};

struct MissingChangeProfile;

impl UiNativeApplicationDefinition for MissingChangeProfile {
    fn prepare(
        self,
        preparation: UiNativeApplicationPreparation,
    ) -> UiNativeApplicationPreparationOutcome {
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
fn product_definition_must_select_its_change_profile() {
    let platform = WorthUiNativePlatform::prepare(profile()).unwrap();
    let UiNativePlatformOutcome::ApplicationPreparationDenied(denial) =
        platform.run(MissingChangeProfile)
    else {
        panic!("a profile-free application definition must be denied");
    };
    assert_eq!(
        denial.cause(),
        UiNativeApplicationPreparationDenialCause::ChangeProfileMissing
    );
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
