use worth_kernel::graph_read_access_plan_adoption::WorthGraphReadAccessSelectedVerticalSlice;

fn main() {
    let _ = WorthGraphReadAccessSelectedVerticalSlice {
        requirement_identity: String::new(),
        source_posture_row_digest: String::new(),
        source_attempt_digest: None,
        source_carried_gap_digest: None,
        source_requirement_record_digest: String::new(),
        read_family_identity_digest: None,
        requirement_row_digest: None,
        query_family_name: None,
        query_family_digest_seed: String::new(),
        query_posture: String::new(),
        denial_kind: None,
        selection_reason: unimplemented!(),
        slice_digest: String::new(),
    };
}
