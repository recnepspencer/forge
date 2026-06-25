#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorthTopologySelectedValidatorEnforcementPhaseFiveSeed {
    selected_plan_digest: String,
    enforcement_receipt_digest: String,
    migrated_family_identity_digest: String,
    executed_validator_family_count: usize,
    violation_count: usize,
    denied_before_execution_count: usize,
    seed_digest: String,
}

impl WorthTopologySelectedValidatorEnforcementPhaseFiveSeed {
    pub(in crate::validator_invariant_catalog) fn from_receipt(
        selected_plan_digest: &str,
        enforcement_receipt_digest: &str,
        migrated_family_identity_digest: &str,
        executed_validator_family_count: usize,
        violation_count: usize,
        denied_before_execution_count: usize,
    ) -> Self {
        let seed_digest = [
            "worth-topo-selected-validator-enforcement-phase-five-seed-v1",
            selected_plan_digest,
            enforcement_receipt_digest,
            migrated_family_identity_digest,
            &executed_validator_family_count.to_string(),
            &violation_count.to_string(),
            &denied_before_execution_count.to_string(),
        ]
        .join("|");
        Self {
            selected_plan_digest: selected_plan_digest.to_string(),
            enforcement_receipt_digest: enforcement_receipt_digest.to_string(),
            migrated_family_identity_digest: migrated_family_identity_digest.to_string(),
            executed_validator_family_count,
            violation_count,
            denied_before_execution_count,
            seed_digest,
        }
    }

    pub fn selected_plan_digest(&self) -> &str {
        &self.selected_plan_digest
    }

    pub fn enforcement_receipt_digest(&self) -> &str {
        &self.enforcement_receipt_digest
    }

    pub fn migrated_family_identity_digest(&self) -> &str {
        &self.migrated_family_identity_digest
    }

    pub const fn executed_validator_family_count(&self) -> usize {
        self.executed_validator_family_count
    }

    pub const fn violation_count(&self) -> usize {
        self.violation_count
    }

    pub const fn denied_before_execution_count(&self) -> usize {
        self.denied_before_execution_count
    }

    pub fn seed_digest(&self) -> &str {
        &self.seed_digest
    }
}
