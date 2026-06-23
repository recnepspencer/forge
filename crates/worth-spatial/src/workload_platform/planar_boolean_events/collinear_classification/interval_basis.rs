use crate::workload_platform::planar_boolean_events::canonical_parameter_range;

use super::identity::interval_basis_identity;

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarBooleanCollinearIntervalBasis {
    left_parameter_range: [f64; 2],
    right_parameter_range: [f64; 2],
    left_source_parameter_range: [f64; 2],
    right_source_parameter_range: [f64; 2],
    interval_basis_identity: String,
}

impl PlanarBooleanCollinearIntervalBasis {
    pub(crate) fn from_source_ranges(
        left_source_parameter_range: [f64; 2],
        right_source_parameter_range: [f64; 2],
    ) -> Self {
        let left_source_parameter_range = canonical_parameter_range(left_source_parameter_range);
        let right_source_parameter_range = canonical_parameter_range(right_source_parameter_range);
        let left_parameter_range = ordered_range(left_source_parameter_range);
        let right_parameter_range = ordered_range(right_source_parameter_range);
        let interval_basis_identity =
            interval_basis_identity(left_parameter_range, right_parameter_range);
        Self {
            left_parameter_range,
            right_parameter_range,
            left_source_parameter_range,
            right_source_parameter_range,
            interval_basis_identity,
        }
    }

    pub fn left_parameter_range(&self) -> [f64; 2] {
        self.left_parameter_range
    }

    pub fn right_parameter_range(&self) -> [f64; 2] {
        self.right_parameter_range
    }

    pub fn left_source_parameter_range(&self) -> [f64; 2] {
        self.left_source_parameter_range
    }

    pub fn right_source_parameter_range(&self) -> [f64; 2] {
        self.right_source_parameter_range
    }

    pub fn interval_basis_identity(&self) -> &str {
        &self.interval_basis_identity
    }
}

fn ordered_range(range: [f64; 2]) -> [f64; 2] {
    if range[0] <= range[1] {
        range
    } else {
        [range[1], range[0]]
    }
}
