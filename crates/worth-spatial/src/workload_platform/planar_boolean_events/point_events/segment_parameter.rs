use super::identity::segment_parameter_identity;

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarBooleanPointEventSegmentParameterFact {
    segment_identity: String,
    carrier_identity: String,
    parameter: f64,
    parameter_fact_identity: String,
}

impl PlanarBooleanPointEventSegmentParameterFact {
    pub(crate) fn new(segment_identity: &str, carrier_identity: &str, parameter: f64) -> Self {
        Self {
            segment_identity: segment_identity.to_string(),
            carrier_identity: carrier_identity.to_string(),
            parameter,
            parameter_fact_identity: segment_parameter_identity(
                segment_identity,
                carrier_identity,
                parameter,
            ),
        }
    }

    pub fn segment_identity(&self) -> &str {
        &self.segment_identity
    }

    pub fn carrier_identity(&self) -> &str {
        &self.carrier_identity
    }

    pub fn parameter(&self) -> f64 {
        self.parameter
    }

    pub fn parameter_fact_identity(&self) -> &str {
        &self.parameter_fact_identity
    }
}
