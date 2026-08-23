use super::{port, UiNativePendingExternalObligation};

pub(crate) struct UiNativeQualifiedExternalObligation {
    inner: Box<dyn UiNativePendingExternalObligation>,
    effects_indeterminate_armed: bool,
    effects_indeterminate_observed: bool,
    duplicate_completed: bool,
    duplicate_completed_observed: bool,
}

impl UiNativeQualifiedExternalObligation {
    pub(crate) fn new(
        inner: Box<dyn UiNativePendingExternalObligation>,
        effects_indeterminate: bool,
        duplicate_completed: bool,
    ) -> Self {
        Self {
            inner,
            effects_indeterminate_armed: effects_indeterminate,
            effects_indeterminate_observed: false,
            duplicate_completed,
            duplicate_completed_observed: false,
        }
    }
}

impl UiNativePendingExternalObligation for UiNativeQualifiedExternalObligation {
    fn poll_observation(
        &mut self,
        basis: crate::native::physical_work_signal::UiNativePhysicalSignalExternalBasis,
        device: Option<&wgpu::Device>,
    ) -> crate::native::physical_work_signal::UiNativePhysicalSignalExternalObservation {
        if self.effects_indeterminate_armed && !self.effects_indeterminate_observed {
            self.effects_indeterminate_observed = true;
            return basis.observe_qualified_external(
                crate::native::physical_work_signal::UiNativePhysicalSignalStatus::EffectsIndeterminate,
            );
        }
        self.inner.poll_observation(basis, device)
    }

    fn take_presented_observation(&mut self) -> Option<port::UiNativePresentationPortObservation> {
        self.inner.take_presented_observation()
    }

    fn take_duplicate_completed_observation(
        &mut self,
        basis: crate::native::physical_work_signal::UiNativePhysicalSignalExternalBasis,
    ) -> Option<crate::native::physical_work_signal::UiNativePhysicalSignalExternalObservation>
    {
        if !self.duplicate_completed || self.duplicate_completed_observed {
            return None;
        }
        self.duplicate_completed_observed = true;
        Some(basis.observe_qualified_external(
            crate::native::physical_work_signal::UiNativePhysicalSignalStatus::Completed,
        ))
    }
}
