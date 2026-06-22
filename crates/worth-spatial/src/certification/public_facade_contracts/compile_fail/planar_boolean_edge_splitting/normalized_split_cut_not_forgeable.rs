use worth_spatial::facade::planar_boolean_edge_splitting::{
    PlanarBooleanNormalizedSplitCut, PlanarBooleanRawEdgeSplitScheduleEntryKind,
};

fn main() {
    let _ = PlanarBooleanNormalizedSplitCut {
        cut_identity: "forged".to_string(),
        duplicate_report_identity: "duplicate-report".to_string(),
        source_edge_identity: "source".to_string(),
        carrier_identity: "carrier".to_string(),
        parameter: 0.5,
        parameter_bits: 0.5f64.to_bits(),
        kind: PlanarBooleanRawEdgeSplitScheduleEntryKind::Interval,
        local_frame_identity: "frame".to_string(),
        precision_basis_identity: "precision".to_string(),
        provenance_entry_identities: Vec::new(),
        event_identities: Vec::new(),
        parameter_fact_identities: Vec::new(),
        event_group_identities: Vec::new(),
        segment_pair_identities: Vec::new(),
        predicate_receipt_identities: Vec::new(),
        exact_endpoint_source_identity: None,
        exact_projected_endpoint_fact_identity: None,
        shared_endpoint_source_identities: Vec::new(),
        shared_endpoint_projection_fact_digests: Vec::new(),
    };
}
