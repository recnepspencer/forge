use crate::workload_platform::planar_boolean_events::segment_carriers::PlanarBooleanSegmentCarrierEndpointFacts;

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarBooleanNormalizedEndpoint {
    endpoint: PlanarBooleanSegmentCarrierEndpointFacts,
    parameter: f64,
}

impl PlanarBooleanNormalizedEndpoint {
    pub(crate) fn new(endpoint: &PlanarBooleanSegmentCarrierEndpointFacts, parameter: f64) -> Self {
        Self {
            endpoint: endpoint.clone(),
            parameter,
        }
    }

    pub fn point(&self) -> [f64; 2] {
        self.endpoint.point()
    }

    pub fn parameter(&self) -> f64 {
        self.parameter
    }

    pub fn source_endpoint_identity(&self) -> &str {
        self.endpoint.source_endpoint_identity()
    }

    pub fn projected_endpoint_fact_identity(&self) -> &str {
        self.endpoint.projected_endpoint_fact_identity()
    }

    pub fn projected_loop_identity(&self) -> &str {
        self.endpoint.projected_loop_identity()
    }
}
