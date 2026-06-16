use crate::workload_platform::planar_boolean_events::PlanarBooleanPointEventCoordinateFact;

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarBooleanCollinearTouchPoint {
    coordinate_fact: PlanarBooleanPointEventCoordinateFact,
    left_parameter: f64,
    right_parameter: f64,
}

impl PlanarBooleanCollinearTouchPoint {
    pub(crate) fn new(
        coordinate_fact: PlanarBooleanPointEventCoordinateFact,
        left_parameter: f64,
        right_parameter: f64,
    ) -> Self {
        Self {
            coordinate_fact,
            left_parameter,
            right_parameter,
        }
    }

    pub fn coordinate_fact(&self) -> &PlanarBooleanPointEventCoordinateFact {
        &self.coordinate_fact
    }

    pub fn left_parameter(&self) -> f64 {
        self.left_parameter
    }

    pub fn right_parameter(&self) -> f64 {
        self.right_parameter
    }
}
