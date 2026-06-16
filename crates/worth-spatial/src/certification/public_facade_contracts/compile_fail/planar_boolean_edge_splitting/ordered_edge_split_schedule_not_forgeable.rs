use worth_spatial::facade::planar_boolean_edge_splitting::{
    PlanarBooleanOrderedEdgeSplitSchedule, PlanarBooleanSplitScheduleOrderKey,
};

fn main() {
    let _ = PlanarBooleanSplitScheduleOrderKey {
        source_edge_identity: "source".to_string(),
        parameter_bits: 0,
        entry_kind_rank: 0,
        event_identity: "event".to_string(),
        event_group_identities: vec!["event-group".to_string()],
        carrier_identity: "carrier".to_string(),
        candidate_identity: "candidate".to_string(),
    };
    let _ = PlanarBooleanOrderedEdgeSplitSchedule {
        schedule_identity: "forged".to_string(),
        raw_schedule_identity: "raw".to_string(),
        source_edge_identity: "source".to_string(),
        carrier_identity: "carrier".to_string(),
        order_digest: "digest".to_string(),
        ordered_entries: Vec::new(),
    };
}
