use worth_ui::facade::app::{
    UiApplicationHostUnbound, UiChangeProfileInstalled, UiIntentWiringSatisfied, WorthUi,
    WorthUiApplicationBuilder, WorthUiHostNeutralApp,
};

use crate::native_platform_binding::UiNativePlatformBindingGrant;
type UiNativeHostNeutralBuilder = WorthUiApplicationBuilder<
    UiChangeProfileInstalled,
    UiIntentWiringSatisfied,
    UiApplicationHostUnbound,
>;

pub trait UiNativeApplicationDefinition: Sized {
    fn prepare(
        self,
        preparation: UiNativeApplicationPreparation,
    ) -> UiNativeApplicationPreparationOutcome;
}

#[must_use]
pub enum UiNativeApplicationPreparationOutcome {
    Prepared(UiPreparedNativeApplication),
    Denied(UiNativeApplicationPreparationDenial),
}

#[must_use]
pub struct UiNativeApplicationPreparation {
    preparation_identity: u64,
    binding: UiNativePlatformBindingGrant,
    builder: Option<UiNativeHostNeutralBuilder>,
}

#[must_use]
pub struct UiNativeApplicationBuilder<'preparation> {
    builder: &'preparation mut Option<UiNativeHostNeutralBuilder>,
}

#[must_use]
pub struct UiPreparedNativeApplication {
    application: WorthUiHostNeutralApp,
    binding: UiNativePlatformBindingGrant,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiNativeApplicationPreparationDenialCause {
    ApplicationRejected,
    ApplicationBuilderUnavailable,
    ApplicationFreezeRejected,
}

#[must_use]
#[derive(Debug)]
pub struct UiNativeApplicationPreparationDenial {
    preparation_identity: u64,
    cause: UiNativeApplicationPreparationDenialCause,
}

impl UiNativeApplicationPreparation {
    pub(crate) fn new(preparation_identity: u64, binding: UiNativePlatformBindingGrant) -> Self {
        debug_assert_eq!(binding.preparation_identity(), preparation_identity);
        debug_assert_eq!(
            binding.profile(),
            worth_ui_host_native::UiNativePlatformProfileIdentity::WORTH_UI_WINDOWS_DX12_V1
        );
        let builder = WorthUi::app()
            .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse());
        Self {
            preparation_identity,
            binding,
            builder: Some(builder),
        }
    }

    pub fn builder(&mut self) -> UiNativeApplicationBuilder<'_> {
        UiNativeApplicationBuilder {
            builder: &mut self.builder,
        }
    }

    pub fn complete(mut self) -> UiNativeApplicationPreparationOutcome {
        let Some(builder) = self.builder.take() else {
            return self
                .deny(UiNativeApplicationPreparationDenialCause::ApplicationBuilderUnavailable);
        };
        match builder.freeze() {
            Ok(application) => {
                UiNativeApplicationPreparationOutcome::Prepared(UiPreparedNativeApplication {
                    application,
                    binding: self.binding,
                })
            }
            Err(_) => {
                self.deny(UiNativeApplicationPreparationDenialCause::ApplicationFreezeRejected)
            }
        }
    }

    pub fn deny(
        self,
        cause: UiNativeApplicationPreparationDenialCause,
    ) -> UiNativeApplicationPreparationOutcome {
        UiNativeApplicationPreparationOutcome::Denied(UiNativeApplicationPreparationDenial {
            preparation_identity: self.preparation_identity,
            cause,
        })
    }
}

impl UiNativeApplicationBuilder<'_> {
    pub fn with_visual_inspection_policy(
        &mut self,
        policy: worth_ui::facade::inspection::UiVisualInspectionPolicy,
    ) -> Result<(), UiNativeApplicationPreparationDenialCause> {
        let builder = self
            .builder
            .take()
            .ok_or(UiNativeApplicationPreparationDenialCause::ApplicationBuilderUnavailable)?;
        *self.builder = Some(builder.with_visual_inspection_policy(policy));
        Ok(())
    }
}

impl UiPreparedNativeApplication {
    pub(crate) fn settle_phase_one(self) {
        let _application = self.application;
        let _binding = self.binding;
    }
}

impl UiNativeApplicationPreparationDenial {
    pub const fn preparation_identity(&self) -> u64 {
        self.preparation_identity
    }

    pub const fn cause(&self) -> UiNativeApplicationPreparationDenialCause {
        self.cause
    }
}
