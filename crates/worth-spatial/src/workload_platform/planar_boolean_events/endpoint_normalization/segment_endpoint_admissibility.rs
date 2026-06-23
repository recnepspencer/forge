use crate::workload_platform::planar_boolean_events::segment_carriers::PlanarBooleanSegmentCarrier;
use crate::workload_platform::planar_boolean_events::segment_identity::{
    PlanarBooleanCanonicalSegmentSetDenial, PlanarBooleanCanonicalSegmentSetDenialKind,
};

pub(crate) fn validate_segment_endpoint_admissibility(
    carrier: &PlanarBooleanSegmentCarrier,
) -> Result<(), PlanarBooleanCanonicalSegmentSetDenial> {
    validate_endpoint_coordinates_are_finite(carrier)?;
    if carrier.start().point() == carrier.end().point() {
        return Err(PlanarBooleanCanonicalSegmentSetDenial::from_carrier(
            PlanarBooleanCanonicalSegmentSetDenialKind::CollapsedProjectedSegment,
            carrier,
            "canonical segment construction rejects collapsed projected segments before pair enumeration",
        ));
    }
    Ok(())
}

fn validate_endpoint_coordinates_are_finite(
    carrier: &PlanarBooleanSegmentCarrier,
) -> Result<(), PlanarBooleanCanonicalSegmentSetDenial> {
    if !all_coordinates_are_finite(carrier.start().point())
        || !all_coordinates_are_finite(carrier.end().point())
    {
        return Err(PlanarBooleanCanonicalSegmentSetDenial::from_carrier(
            PlanarBooleanCanonicalSegmentSetDenialKind::NonFiniteEndpointCoordinate,
            carrier,
            "canonical segment construction requires finite projected endpoint coordinates",
        ));
    }
    Ok(())
}

fn all_coordinates_are_finite(point: [f64; 2]) -> bool {
    point[0].is_finite() && point[1].is_finite()
}
