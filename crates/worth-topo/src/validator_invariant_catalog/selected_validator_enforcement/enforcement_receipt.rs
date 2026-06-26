use crate::validation::loop_wiring_rule;
use crate::validation::TopologyValidationRuleIdentity;
use crate::validator_invariant_catalog::selected_validator_enforcement::{
    WorthTopologyLoopWiringWitnessInput, WorthTopologySelectedValidatorEnforcementCounters,
    WorthTopologySelectedValidatorEnforcementOutcome,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorthTopologySelectedValidatorEnforcementReceipt {
    selected_plan_digest: String,
    selected_obligation_row_digest: String,
    migrated_family_identity_digest: String,
    validation_rule_identity: TopologyValidationRuleIdentity,
    witness_input_digest: String,
    outcome: WorthTopologySelectedValidatorEnforcementOutcome,
    counters: WorthTopologySelectedValidatorEnforcementCounters,
    diagnostic_projection_digest: String,
    enforcement_receipt_digest: String,
}

impl WorthTopologySelectedValidatorEnforcementReceipt {
    pub(in crate::validator_invariant_catalog) fn loop_wiring(
        selected_plan_digest: &str,
        selected_obligation_row_digest: &str,
        migrated_family_identity_digest: &str,
        witness_input: &WorthTopologyLoopWiringWitnessInput,
        outcome: WorthTopologySelectedValidatorEnforcementOutcome,
        counters: WorthTopologySelectedValidatorEnforcementCounters,
        diagnostic_projection_digest: String,
    ) -> Self {
        let validation_rule_identity = loop_wiring_rule();
        let enforcement_receipt_digest = [
            "worth-topo-selected-validator-enforcement-receipt-v1",
            selected_plan_digest,
            selected_obligation_row_digest,
            migrated_family_identity_digest,
            validation_rule_identity.stable_key().as_str(),
            witness_input.input_digest(),
            &outcome.outcome_digest(),
            counters.counters_digest(),
            &diagnostic_projection_digest,
        ]
        .join("|");
        Self {
            selected_plan_digest: selected_plan_digest.to_string(),
            selected_obligation_row_digest: selected_obligation_row_digest.to_string(),
            migrated_family_identity_digest: migrated_family_identity_digest.to_string(),
            validation_rule_identity,
            witness_input_digest: witness_input.input_digest().to_string(),
            outcome,
            counters,
            diagnostic_projection_digest,
            enforcement_receipt_digest,
        }
    }

    pub fn selected_plan_digest(&self) -> &str {
        &self.selected_plan_digest
    }

    pub fn selected_obligation_row_digest(&self) -> &str {
        &self.selected_obligation_row_digest
    }

    pub fn migrated_family_identity_digest(&self) -> &str {
        &self.migrated_family_identity_digest
    }

    pub const fn validation_rule_identity(&self) -> &TopologyValidationRuleIdentity {
        &self.validation_rule_identity
    }

    pub fn witness_input_digest(&self) -> &str {
        &self.witness_input_digest
    }

    pub const fn outcome(&self) -> &WorthTopologySelectedValidatorEnforcementOutcome {
        &self.outcome
    }

    pub const fn counters(&self) -> &WorthTopologySelectedValidatorEnforcementCounters {
        &self.counters
    }

    pub fn diagnostic_projection_digest(&self) -> &str {
        &self.diagnostic_projection_digest
    }

    pub fn enforcement_receipt_digest(&self) -> &str {
        &self.enforcement_receipt_digest
    }

    pub const fn is_execution_backed(&self) -> bool {
        !matches!(
            self.outcome,
            WorthTopologySelectedValidatorEnforcementOutcome::DeniedBeforeExecution(_)
                | WorthTopologySelectedValidatorEnforcementOutcome::CertificationComparisonOnly(_)
        )
    }
}
