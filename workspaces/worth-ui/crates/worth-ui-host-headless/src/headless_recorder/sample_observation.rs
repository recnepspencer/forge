#[derive(Clone, Debug)]
pub struct UiHeadlessPresentationSampleObservation {
    frame: worth_ui_host_contract::UiMountedFrameIdentity,
    epoch: worth_ui_host_contract::UiHostPresentationEpoch,
    changes: Box<[worth_ui_host_contract::UiMountedPresentationSampleChange]>,
    damage: Box<[worth_ui_host_contract::UiMountedLogicalDamage]>,
}

impl UiHeadlessPresentationSampleObservation {
    pub(super) fn from_retained(retained: &super::UiHeadlessRetainedPresentation) -> Option<Self> {
        let changes = retained.sample_overrides().collect::<Vec<_>>();
        Some(Self {
            frame: retained.frame,
            epoch: retained.epoch?,
            changes: changes.into_boxed_slice(),
            damage: retained.sample_damage().to_vec().into_boxed_slice(),
        })
    }

    pub const fn frame(&self) -> worth_ui_host_contract::UiMountedFrameIdentity {
        self.frame
    }

    pub const fn epoch(&self) -> worth_ui_host_contract::UiHostPresentationEpoch {
        self.epoch
    }

    pub fn changes(&self) -> &[worth_ui_host_contract::UiMountedPresentationSampleChange] {
        &self.changes
    }

    pub fn damage(&self) -> &[worth_ui_host_contract::UiMountedLogicalDamage] {
        &self.damage
    }
}

impl super::WorthUiHeadlessRecorder {
    pub fn retained_sample_observation(
        &self,
        binding: worth_ui_host_contract::UiSurfaceBindingGeneration,
    ) -> Option<UiHeadlessPresentationSampleObservation> {
        let state = self.state.borrow();
        UiHeadlessPresentationSampleObservation::from_retained(
            state.retained_presentations.get(&binding)?,
        )
    }
}
