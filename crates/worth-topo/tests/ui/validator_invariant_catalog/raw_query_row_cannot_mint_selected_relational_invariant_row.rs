use topology::facade::WorthTopologySelectedRelationalInvariantFamilyRow;

fn main() {
    let _ = WorthTopologySelectedRelationalInvariantFamilyRow {
        worth_family_identity_digest: String::from("copied-query-row-family-digest"),
        query_rule_identity_digest: String::from("copied-query-row-rule-digest"),
        support_lane: panic!("private support lane unavailable"),
        support_status: panic!("private support status unavailable"),
        registration_digest: String::from("copied-query-registration-digest"),
        selected_obligation_row_digest: String::from("copied-selected-row-digest"),
        row_digest: String::from("forged-selected-invariant-row"),
    };
}
