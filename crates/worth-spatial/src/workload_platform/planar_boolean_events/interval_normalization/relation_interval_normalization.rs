use crate::workload_platform::planar_boolean_events::PlanarBooleanCollinearIntervalBasis;

pub(crate) fn normalized_parameter_range(
    interval_basis: &PlanarBooleanCollinearIntervalBasis,
) -> [f64; 2] {
    interval_basis.left_parameter_range()
}
