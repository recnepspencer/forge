use super::normalized_endpoint::PlanarBooleanNormalizedEndpoint;

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarBooleanNormalizedEndpointPair {
    low: PlanarBooleanNormalizedEndpoint,
    high: PlanarBooleanNormalizedEndpoint,
    orientation_was_reversed: bool,
}

impl PlanarBooleanNormalizedEndpointPair {
    pub(crate) fn new(
        low: PlanarBooleanNormalizedEndpoint,
        high: PlanarBooleanNormalizedEndpoint,
        orientation_was_reversed: bool,
    ) -> Self {
        Self {
            low,
            high,
            orientation_was_reversed,
        }
    }

    pub fn low(&self) -> &PlanarBooleanNormalizedEndpoint {
        &self.low
    }

    pub fn high(&self) -> &PlanarBooleanNormalizedEndpoint {
        &self.high
    }

    pub fn orientation_was_reversed(&self) -> bool {
        self.orientation_was_reversed
    }
}
