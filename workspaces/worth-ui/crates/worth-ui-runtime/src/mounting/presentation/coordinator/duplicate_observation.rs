use super::UiMountedPresentationCoordinator;

impl UiMountedPresentationCoordinator {
    pub(crate) fn admit_duplicate_native_presentation_observation(
        &mut self,
        presentation: worth_ui_host_native::UiNativePhysicalPresentationCorrelation,
    ) -> Result<(), ()> {
        self.presentation_async
            .as_mut()
            .ok_or(())?
            .admit_duplicate_owner_observation(presentation)
            .map_err(|_| ())
    }
}
