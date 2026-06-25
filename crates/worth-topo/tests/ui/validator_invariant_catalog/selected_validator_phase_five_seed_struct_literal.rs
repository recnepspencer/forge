use topology::facade::WorthTopologySelectedValidatorEnforcementPhaseFiveSeed;

fn main() {
    let _ = WorthTopologySelectedValidatorEnforcementPhaseFiveSeed {
        selected_plan_digest: String::new(),
        enforcement_receipt_digest: String::new(),
        migrated_family_identity_digest: String::new(),
        executed_validator_family_count: 0,
        violation_count: 0,
        denied_before_execution_count: 0,
        seed_digest: String::new(),
    };
}
