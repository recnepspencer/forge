use worth_spatial::facade::planar_boolean_edge_splitting::{
    PlanarBooleanEndpointContactDecision, PlanarBooleanPointSplitPosture,
};

fn main() {
    let _ = PlanarBooleanEndpointContactDecision {
        decision_identity: "forged".to_string(),
        normalized_cut_identity: "cut".to_string(),
        duplicate_report_identity: "duplicate-report".to_string(),
        source_edge_identity: "source".to_string(),
        carrier_identity: "carrier".to_string(),
        boundary_position: Default::default(),
        posture: PlanarBooleanPointSplitPosture::EndpointNoOp,
        source_endpoint_identity: "endpoint".to_string(),
        projected_endpoint_fact_identity: "projection".to_string(),
        provenance_entry_identities: Vec::new(),
        event_group_identities: Vec::new(),
        shared_endpoint_source_identities: Vec::new(),
        shared_endpoint_projection_fact_digests: Vec::new(),
    };
}
