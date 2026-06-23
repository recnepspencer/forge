use worth_spatial::facade::planar_boolean_edge_splitting::PlanarBooleanOverlapChainCoverageRow;

fn main() {
    let _ = PlanarBooleanOverlapChainCoverageRow {
        row_identity: "forged".to_string(),
        chain_identity: "chain".to_string(),
        interval_event_identity: "event".to_string(),
        source_interval_identity: "interval".to_string(),
        source_edge_identity: "source-edge".to_string(),
        carrier_identity: "carrier".to_string(),
        members_checked: 1,
    };
}
