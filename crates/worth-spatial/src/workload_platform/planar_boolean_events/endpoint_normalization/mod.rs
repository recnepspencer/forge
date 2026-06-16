mod endpoint_order;
mod normalized_endpoint;
mod normalized_endpoint_pair;
mod segment_endpoint_admissibility;

pub use normalized_endpoint::PlanarBooleanNormalizedEndpoint;
pub use normalized_endpoint_pair::PlanarBooleanNormalizedEndpointPair;

pub(crate) use endpoint_order::normalize_endpoint_order;
pub(crate) use segment_endpoint_admissibility::validate_segment_endpoint_admissibility;
