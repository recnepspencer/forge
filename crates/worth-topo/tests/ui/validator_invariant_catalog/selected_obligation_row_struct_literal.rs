use topology::facade::WorthTopologySelectedLegalityObligationRow;

fn main() {
    let _ = WorthTopologySelectedLegalityObligationRow {
        worth_family_identity_digest: String::new(),
        query_rule_identity_digest: String::new(),
        query_obligation_kind: panic!("private Query kind unavailable"),
        support_lane: panic!("private support lane unavailable"),
        support_status: panic!("private support status unavailable"),
        support_posture_digest: String::new(),
        execution_budget_digest: String::new(),
        registration_digest: String::new(),
        row_digest: String::new(),
    };
}
