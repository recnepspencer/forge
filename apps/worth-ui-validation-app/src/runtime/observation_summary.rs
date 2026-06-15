use worth_ui::facade::WorthUiActiveRuntimeObservation;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RuntimeObservationSummary {
    observation: WorthUiActiveRuntimeObservation,
}

impl RuntimeObservationSummary {
    pub fn from_observation(observation: WorthUiActiveRuntimeObservation) -> Self {
        Self { observation }
    }

    pub fn observation(self) -> WorthUiActiveRuntimeObservation {
        self.observation
    }
}
