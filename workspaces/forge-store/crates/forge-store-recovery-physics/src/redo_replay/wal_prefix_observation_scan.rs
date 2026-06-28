use super::{WalPrefixFrameObservation, WalPrefixIntegrityObservation};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalPrefixObservationScan {
    observations: Vec<WalPrefixIntegrityObservation>,
}

impl WalPrefixObservationScan {
    pub fn from_observations(observations: Vec<WalPrefixIntegrityObservation>) -> Self {
        Self { observations }
    }

    pub(crate) fn into_frame_observations(self) -> Vec<WalPrefixFrameObservation> {
        self.observations
            .into_iter()
            .map(WalPrefixIntegrityObservation::into_frame_observation)
            .collect()
    }
}
