use worth_spatial::facade::planar_boolean_edge_splitting::PlanarBooleanSplitEdgeChain;

fn main() {
    let _chain = PlanarBooleanSplitEdgeChain {
        chain_identity: "chain".to_string(),
        source_edge_identity: "source".to_string(),
        carrier_identity: "carrier".to_string(),
        endpoint_boundary_schedule_identity: "endpoint".to_string(),
        interval_subdivision_schedule_identity: "interval".to_string(),
        split_vertex_schedule_identity: "vertices".to_string(),
        split_fragment_schedule_identity: "fragments".to_string(),
        fragment_identities: Vec::new(),
        split_vertex_identities: Vec::new(),
        overlap_chain_identities: Vec::new(),
        persistent_name_row_identities: Vec::new(),
        decision_identities: Vec::new(),
        validation_fragment_coverage_identities: Vec::new(),
        validation_overlap_coverage_identities: Vec::new(),
    };
}
