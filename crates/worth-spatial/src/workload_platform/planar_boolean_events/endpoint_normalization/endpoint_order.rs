use crate::workload_platform::planar_boolean_events::segment_carriers::PlanarBooleanSegmentCarrier;

use super::normalized_endpoint::PlanarBooleanNormalizedEndpoint;
use super::normalized_endpoint_pair::PlanarBooleanNormalizedEndpointPair;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum PlanarBooleanEndpointOrder {
    SourceDirection,
    ReversedFromSource,
}

impl PlanarBooleanEndpointOrder {
    pub(crate) fn orientation_was_reversed(self) -> bool {
        self == Self::ReversedFromSource
    }
}

pub(crate) fn normalize_endpoint_order(
    carrier: &PlanarBooleanSegmentCarrier,
) -> PlanarBooleanNormalizedEndpointPair {
    let order = canonical_endpoint_order(carrier.start().point(), carrier.end().point());
    match order {
        PlanarBooleanEndpointOrder::SourceDirection => PlanarBooleanNormalizedEndpointPair::new(
            PlanarBooleanNormalizedEndpoint::new(carrier.start(), 0.0),
            PlanarBooleanNormalizedEndpoint::new(carrier.end(), 1.0),
            order.orientation_was_reversed(),
        ),
        PlanarBooleanEndpointOrder::ReversedFromSource => PlanarBooleanNormalizedEndpointPair::new(
            PlanarBooleanNormalizedEndpoint::new(carrier.end(), 0.0),
            PlanarBooleanNormalizedEndpoint::new(carrier.start(), 1.0),
            order.orientation_was_reversed(),
        ),
    }
}

fn canonical_endpoint_order(start: [f64; 2], end: [f64; 2]) -> PlanarBooleanEndpointOrder {
    if start[0] < end[0] || (start[0] == end[0] && start[1] <= end[1]) {
        PlanarBooleanEndpointOrder::SourceDirection
    } else {
        PlanarBooleanEndpointOrder::ReversedFromSource
    }
}
