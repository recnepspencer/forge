use crate::facade::entry::UiIntentWiringSatisfied;
use crate::facade::{
    UiChangeProfileInstalled, UiChangeProfileMissing, WorthUi, WorthUiApplicationBuilder,
    WorthUiHostNeutralApp,
};

use super::native_platform_binding::UiNativePlatformBindingGrant;
type UiNativeBuilderMissingProfile =
    WorthUiApplicationBuilder<UiChangeProfileMissing, UiIntentWiringSatisfied>;
type UiNativeBuilderReady =
    WorthUiApplicationBuilder<UiChangeProfileInstalled, UiIntentWiringSatisfied>;

enum UiNativeBuilderState {
    MissingProfile(UiNativeBuilderMissingProfile),
    Ready(UiNativeBuilderReady),
}

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
    builder: Option<UiNativeBuilderState>,
    program: Option<crate::facade::entry::UiNativeApplicationProgram>,
}

#[must_use]
pub struct UiNativeApplicationBuilder<'preparation> {
    builder: &'preparation mut Option<UiNativeBuilderState>,
}

#[must_use]
pub struct UiPreparedNativeApplication {
    application: WorthUiHostNeutralApp,
    binding: UiNativePlatformBindingGrant,
    program: crate::facade::entry::UiNativeApplicationProgram,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiNativeApplicationPreparationDenialCause {
    ApplicationRejected,
    ApplicationBuilderUnavailable,
    ChangeProfileMissing,
    ChangeProfileAlreadyInstalled,
    ApplicationFreezeRejected,
    FrameProgramAlreadyInstalled,
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
        let builder = UiNativeBuilderState::MissingProfile(WorthUi::app());
        Self {
            preparation_identity,
            binding,
            builder: Some(builder),
            program: None,
        }
    }

    pub fn builder(&mut self) -> UiNativeApplicationBuilder<'_> {
        UiNativeApplicationBuilder {
            builder: &mut self.builder,
        }
    }

    pub fn install_frame_program(
        &mut self,
        program: crate::facade::entry::UiNativeApplicationProgram,
    ) -> Result<(), UiNativeApplicationPreparationDenialCause> {
        if self.program.is_some() {
            return Err(UiNativeApplicationPreparationDenialCause::FrameProgramAlreadyInstalled);
        }
        self.program = Some(program);
        Ok(())
    }

    pub fn complete(mut self) -> UiNativeApplicationPreparationOutcome {
        let Some(builder) = self.builder.take() else {
            return self
                .deny(UiNativeApplicationPreparationDenialCause::ApplicationBuilderUnavailable);
        };
        let UiNativeBuilderState::Ready(builder) = builder else {
            return self.deny(UiNativeApplicationPreparationDenialCause::ChangeProfileMissing);
        };
        match builder.freeze() {
            Ok(application) => {
                UiNativeApplicationPreparationOutcome::Prepared(UiPreparedNativeApplication {
                    application,
                    binding: self.binding,
                    program: self.program.take().unwrap_or_else(
                        crate::facade::entry::UiNativeApplicationProgram::single_frame,
                    ),
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
    pub fn with_change_profile(
        &mut self,
        profile: crate::facade::rebind::UiChangeProfile,
    ) -> Result<(), UiNativeApplicationPreparationDenialCause> {
        let state = self.take()?;
        match state {
            UiNativeBuilderState::MissingProfile(builder) => {
                self.put(UiNativeBuilderState::Ready(
                    builder.with_change_profile(profile),
                ));
                Ok(())
            }
            ready @ UiNativeBuilderState::Ready(_) => {
                self.put(ready);
                Err(UiNativeApplicationPreparationDenialCause::ChangeProfileAlreadyInstalled)
            }
        }
    }

    pub fn with_rust_authored_input(
        &mut self,
        input: worth_ui_dsl::WorthUiRustAuthoredArtifactInput,
    ) -> Result<(), UiNativeApplicationPreparationDenialCause> {
        self.map(|builder| builder.with_rust_authored_input(input))
    }

    pub fn register_component(
        &mut self,
        descriptor: crate::facade::registry::descriptor::ComponentDescriptor,
    ) -> Result<(), UiNativeApplicationPreparationDenialCause> {
        self.map(|builder| builder.register_component(descriptor))
    }

    pub fn register_theme_token(
        &mut self,
        descriptor: crate::facade::registry::descriptor::ThemeTokenDescriptor,
    ) -> Result<(), UiNativeApplicationPreparationDenialCause> {
        self.map(|builder| builder.register_theme_token(descriptor))
    }

    pub fn register_surface(
        &mut self,
        descriptor: crate::facade::registry::descriptor::SurfaceDescriptor,
    ) -> Result<(), UiNativeApplicationPreparationDenialCause> {
        self.map(|builder| builder.register_surface(descriptor))
    }

    pub fn with_visual_inspection_policy(
        &mut self,
        policy: worth_ui_inspection::UiVisualInspectionPolicy,
    ) -> Result<(), UiNativeApplicationPreparationDenialCause> {
        self.map(|builder| builder.with_visual_inspection_policy(policy))
    }

    pub fn with_host_observation_capacity(
        &mut self,
        capacity: crate::facade::observation_report::UiHostObservationCapacity,
    ) -> Result<(), UiNativeApplicationPreparationDenialCause> {
        self.map(|builder| builder.with_host_observation_capacity(capacity))
    }

    pub fn with_mounted_frame_retention_budget(
        &mut self,
        budget: crate::facade::mounted::UiMountedFrameRetentionBudget,
    ) -> Result<(), UiNativeApplicationPreparationDenialCause> {
        self.map(|builder| builder.with_mounted_frame_retention_budget(budget))
    }

    pub fn with_minimal_registration_diagnostics(
        &mut self,
    ) -> Result<(), UiNativeApplicationPreparationDenialCause> {
        self.map(WorthUiApplicationBuilder::with_minimal_registration_diagnostics)
    }

    pub fn with_rich_registration_diagnostics(
        &mut self,
    ) -> Result<(), UiNativeApplicationPreparationDenialCause> {
        self.map(WorthUiApplicationBuilder::with_rich_registration_diagnostics)
    }

    fn map(
        &mut self,
        transition: impl FnOnce(UiNativeBuilderReady) -> UiNativeBuilderReady,
    ) -> Result<(), UiNativeApplicationPreparationDenialCause> {
        match self.take()? {
            UiNativeBuilderState::Ready(builder) => {
                self.put(UiNativeBuilderState::Ready(transition(builder)));
                Ok(())
            }
            missing @ UiNativeBuilderState::MissingProfile(_) => {
                self.put(missing);
                Err(UiNativeApplicationPreparationDenialCause::ChangeProfileMissing)
            }
        }
    }

    fn take(&mut self) -> Result<UiNativeBuilderState, UiNativeApplicationPreparationDenialCause> {
        self.builder
            .take()
            .ok_or(UiNativeApplicationPreparationDenialCause::ApplicationBuilderUnavailable)
    }

    fn put(&mut self, state: UiNativeBuilderState) {
        *self.builder = Some(state);
    }
}

impl UiPreparedNativeApplication {
    pub(crate) fn bind_qualified_native(
        self,
        host: worth_ui_host_native::WorthUiPreparedNativeMechanics,
    ) -> (crate::facade::WorthUiApp, super::UiNativeApplicationProgram) {
        debug_assert_eq!(
            self.binding.profile(),
            worth_ui_host_native::UiNativePlatformProfileIdentity::WORTH_UI_WINDOWS_DX12_V1
        );
        (self.application.bind_qualified_native(host), self.program)
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

#[cfg(test)]
mod tests {
    use super::{UiNativeApplicationPreparation, UiNativeApplicationPreparationOutcome};

    #[test]
    fn complete_preparation_freezes_only_a_host_neutral_application() {
        let identity = 41;
        let binding =
            crate::native_platform::native_platform_binding::UiNativePlatformBindingGrant::issue(
                identity,
            );
        let mut preparation = UiNativeApplicationPreparation::new(identity, binding);
        preparation
            .builder()
            .with_change_profile(crate::facade::rebind::UiChangeProfile::platform_pulse())
            .unwrap();
        assert!(matches!(
            preparation.complete(),
            UiNativeApplicationPreparationOutcome::Prepared(_)
        ));
    }

    #[test]
    fn preparation_builder_rejects_replacement_and_preserves_affine_slot() {
        let identity = 42;
        let binding =
            crate::native_platform::native_platform_binding::UiNativePlatformBindingGrant::issue(
                identity,
            );
        let mut preparation = UiNativeApplicationPreparation::new(identity, binding);
        let mut builder = preparation.builder();
        builder
            .with_change_profile(crate::facade::rebind::UiChangeProfile::platform_pulse())
            .unwrap();
        assert_eq!(
            builder.with_change_profile(crate::facade::rebind::UiChangeProfile::platform_pulse()),
            Err(super::UiNativeApplicationPreparationDenialCause::ChangeProfileAlreadyInstalled)
        );
        drop(builder);
        assert!(matches!(
            preparation.complete(),
            UiNativeApplicationPreparationOutcome::Prepared(_)
        ));
    }
}
