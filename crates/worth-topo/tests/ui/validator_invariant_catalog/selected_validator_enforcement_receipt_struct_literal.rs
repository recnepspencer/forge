use topology::facade::WorthTopologySelectedValidatorEnforcementReceipt;

fn main() {
    let _ = WorthTopologySelectedValidatorEnforcementReceipt {
        validation_rule_identity: panic!("private validation rule identity unavailable"),
        selected_plan_digest: String::new(),
        selected_obligation_row_digest: String::new(),
        migrated_family_identity_digest: String::new(),
        witness_input_digest: String::new(),
        outcome: panic!("private enforcement outcome unavailable"),
        counters: panic!("private enforcement counters unavailable"),
        diagnostic_projection_digest: String::new(),
        enforcement_receipt_digest: String::new(),
    };
}
