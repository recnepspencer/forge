use topology::facade::{
    WorthTopologySelectedGraphObligationEnforcementOutcome,
    WorthTopologySelectedGraphObligationEnforcementReceipt,
};

fn main() {
    let _receipt = WorthTopologySelectedGraphObligationEnforcementReceipt {
        selected_plan_digest: String::new(),
        selected_obligation_row_digest: String::new(),
        worth_family_identity_digest: String::new(),
        query_registration_digest: String::new(),
        query_rule_identity_digest: String::new(),
        query_execution_row_digest: String::new(),
        query_execution_envelope_digest: String::new(),
        query_execution_status: String::new(),
        query_support_lane: String::new(),
        query_support_status: String::new(),
        query_support_posture_digest: String::new(),
        query_execution_budget_digest: String::new(),
        query_execution_cost_class: String::new(),
        query_execution_scope: String::new(),
        query_budget_exceeded_policy: String::new(),
        query_diagnostic_materialization: String::new(),
        query_state_load_counters_digest: String::new(),
        outcome: WorthTopologySelectedGraphObligationEnforcementOutcome::Passed,
        diagnostic_witness_digest: None,
        enforcement_receipt_digest: String::new(),
    };
}
