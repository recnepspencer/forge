use worth_spatial::facade::planar_boolean_edge_splitting::PlanarBooleanSplitFragmentCoverageRow;

fn main() {
    let _ = PlanarBooleanSplitFragmentCoverageRow {
        row_identity: "forged".to_string(),
        schedule_identity: "schedule".to_string(),
        source_edge_identity: "source-edge".to_string(),
        carrier_identity: "carrier".to_string(),
        fragments_checked: 1,
        covered_domain_start_bits: 0.0f64.to_bits(),
        covered_domain_end_bits: 1.0f64.to_bits(),
    };
}
