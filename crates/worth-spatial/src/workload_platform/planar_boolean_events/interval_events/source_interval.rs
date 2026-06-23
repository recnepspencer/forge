use crate::workload_platform::planar_boolean_events::canonical_parameter_range;

use super::identity::source_interval_identity;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum PlanarBooleanSourceIntervalSense {
    Forward,
    Reversed,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarBooleanSourceInterval {
    segment_identity: String,
    carrier_identity: String,
    source_parameter_range: [f64; 2],
    sense: PlanarBooleanSourceIntervalSense,
    source_interval_identity: String,
}

impl PlanarBooleanSourceInterval {
    pub(crate) fn new(
        segment_identity: &str,
        carrier_identity: &str,
        source_parameter_range: [f64; 2],
    ) -> Self {
        let source_parameter_range = canonical_parameter_range(source_parameter_range);
        let sense = if source_parameter_range[0] <= source_parameter_range[1] {
            PlanarBooleanSourceIntervalSense::Forward
        } else {
            PlanarBooleanSourceIntervalSense::Reversed
        };
        let source_interval_identity =
            source_interval_identity(segment_identity, carrier_identity, source_parameter_range);
        Self {
            segment_identity: segment_identity.to_string(),
            carrier_identity: carrier_identity.to_string(),
            source_parameter_range,
            sense,
            source_interval_identity,
        }
    }

    pub fn segment_identity(&self) -> &str {
        &self.segment_identity
    }

    pub fn carrier_identity(&self) -> &str {
        &self.carrier_identity
    }

    pub fn source_parameter_range(&self) -> [f64; 2] {
        self.source_parameter_range
    }

    pub fn sense(&self) -> PlanarBooleanSourceIntervalSense {
        self.sense
    }

    pub fn source_interval_identity(&self) -> &str {
        &self.source_interval_identity
    }
}
