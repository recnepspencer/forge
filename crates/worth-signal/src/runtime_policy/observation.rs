use crate::logic::transaction::{SignalObservationRequest, SignalObservationSurface};
use serde::{Deserialize, Serialize};
use worth_foundational::ObservationActivationProfile;

use super::definition::SignalRuntimePolicy;

impl SignalRuntimePolicy {
    pub const fn observation_activation(self) -> ObservationActivationProfile {
        self.observation_activation
    }

    pub fn with_observation_activation(mut self, activation: ObservationActivationProfile) -> Self {
        self.observation_activation = activation;
        self
    }
}

/// Immutable compiler output describing which optional surfaces run without
/// an explicit observation request.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SignalObservationCapturePlan {
    default_surface_mask: u8,
}

impl Default for SignalObservationCapturePlan {
    fn default() -> Self {
        Self::from_activation(ObservationActivationProfile::Continuous)
    }
}

impl SignalObservationCapturePlan {
    pub(crate) const fn from_activation(activation: ObservationActivationProfile) -> Self {
        let default_surface_mask = match activation {
            ObservationActivationProfile::OnDemand => 0,
            ObservationActivationProfile::Continuous => {
                SignalObservationRequest::default_continuous_mask()
            }
        };
        Self {
            default_surface_mask,
        }
    }

    pub const fn default_request(self) -> SignalObservationRequest {
        SignalObservationRequest::from_mask(self.default_surface_mask)
    }

    pub const fn captures(self, surface: SignalObservationSurface) -> bool {
        self.default_surface_mask & surface.bit() != 0
    }

    pub(crate) const fn default_surface_mask(self) -> u8 {
        self.default_surface_mask
    }
}
