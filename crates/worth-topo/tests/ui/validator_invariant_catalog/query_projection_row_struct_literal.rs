use topology::facade::WorthTopologyQueryGraphObligationRegistrationProjectionRow;

fn main() {
    let _ = WorthTopologyQueryGraphObligationRegistrationProjectionRow {
        worth_family_identity_digest: String::new(),
        query_rule_identity_digest: String::new(),
        query_obligation_kind: panic!("private Query kind unavailable"),
        support_lane: panic!("private support lane unavailable"),
        support_status: panic!("private support status unavailable"),
        support_posture_digest: String::new(),
        operating_world_selector: String::new(),
        operating_world_selector_digest: String::new(),
        touch_selector_digest: String::new(),
        registration_digest: String::new(),
        row_digest: String::new(),
    };
}
