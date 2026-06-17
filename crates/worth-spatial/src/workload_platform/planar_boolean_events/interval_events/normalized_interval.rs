use crate::workload_platform::planar_boolean_events::canonical_parameter_range;

use super::identity::normalized_interval_identity;

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarBooleanNormalizedInterval {
    parameter_range: [f64; 2],
    local_frame_identity: String,
    precision_basis_identity: String,
    normalized_interval_identity: String,
}

impl PlanarBooleanNormalizedInterval {
    pub(crate) fn new(
        parameter_range: [f64; 2],
        local_frame_identity: &str,
        precision_basis_identity: &str,
    ) -> Self {
        let parameter_range = canonical_parameter_range(parameter_range);
        let normalized_interval_identity = normalized_interval_identity(
            parameter_range,
            local_frame_identity,
            precision_basis_identity,
        );
        Self {
            parameter_range,
            local_frame_identity: local_frame_identity.to_string(),
            precision_basis_identity: precision_basis_identity.to_string(),
            normalized_interval_identity,
        }
    }

    pub fn parameter_range(&self) -> [f64; 2] {
        self.parameter_range
    }

    pub fn local_frame_identity(&self) -> &str {
        &self.local_frame_identity
    }

    pub fn precision_basis_identity(&self) -> &str {
        &self.precision_basis_identity
    }

    pub fn normalized_interval_identity(&self) -> &str {
        &self.normalized_interval_identity
    }
}
