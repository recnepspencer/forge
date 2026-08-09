use super::{
    UiNativeApplicationDefinition, UiNativeApplicationPreparation,
    UiNativeApplicationPreparationOutcome, UiNativePlatformOutcome,
    UiNativePlatformPreparationDenial, UiNativePlatformProfile, UiNativePlatformStop,
};
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_PREPARATION_IDENTITY: AtomicU64 = AtomicU64::new(1);

pub struct WorthUiNativePlatform {
    _sealed: (),
}

#[must_use]
pub struct UiPreparedNativePlatform {
    profile: UiNativePlatformProfile,
}

impl WorthUiNativePlatform {
    pub fn prepare(
        profile: UiNativePlatformProfile,
    ) -> Result<UiPreparedNativePlatform, UiNativePlatformPreparationDenial> {
        profile.validate()?;
        Ok(UiPreparedNativePlatform { profile })
    }
}

impl UiPreparedNativePlatform {
    pub fn profile(&self) -> &UiNativePlatformProfile {
        &self.profile
    }

    pub fn run<Application>(self, application: Application) -> UiNativePlatformOutcome
    where
        Application: UiNativeApplicationDefinition,
    {
        let preparation_identity = NEXT_PREPARATION_IDENTITY.fetch_add(1, Ordering::Relaxed);
        let binding = crate::native_platform_binding::UiNativePlatformBindingGrant::issue(
            preparation_identity,
        );
        let prepared = match application.prepare(UiNativeApplicationPreparation::new(
            preparation_identity,
            binding,
        )) {
            UiNativeApplicationPreparationOutcome::Prepared(prepared) => prepared,
            UiNativeApplicationPreparationOutcome::Denied(denial) => {
                return UiNativePlatformOutcome::ApplicationPreparationDenied(denial);
            }
        };
        prepared.settle_phase_one();
        let _profile = self.profile;
        UiNativePlatformOutcome::Stopped(UiNativePlatformStop::phase_one_activation_boundary())
    }
}
