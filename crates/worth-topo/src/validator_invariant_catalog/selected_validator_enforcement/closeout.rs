use crate::validation::loop_wiring_rule;
use crate::validator_invariant_catalog::selected_validator_enforcement::loop_wiring::{
    admit_loop_wiring_witness_input, execute_loop_wiring_obligation,
    loop_wiring_diagnostic_projection, WorthTopologyLoopWiringAdmittedLocalFacts,
    WorthTopologyLoopWiringDiagnosticProjection, WorthTopologyLoopWiringWitnessIntakeReceipt,
};
use crate::validator_invariant_catalog::selected_validator_enforcement::selected_family_lookup::selected_loop_wiring_obligation;
use crate::validator_invariant_catalog::{
    WorthTopologyLegalityCatalogError, WorthTopologyLegalitySelectionCloseout,
    WorthTopologyLoopWiringWitnessInput, WorthTopologySelectedValidatorEnforcementCounters,
    WorthTopologySelectedValidatorEnforcementDenial,
    WorthTopologySelectedValidatorEnforcementPhaseFiveSeed,
    WorthTopologySelectedValidatorEnforcementReceipt, WorthTopologyValidatorFamilyIdentity,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorthTopologySelectedValidatorEnforcementCloseout {
    selected_plan_digest: String,
    enforcement_receipt: WorthTopologySelectedValidatorEnforcementReceipt,
    witness_intake_receipt: WorthTopologyLoopWiringWitnessIntakeReceipt,
    diagnostic_projection: WorthTopologyLoopWiringDiagnosticProjection,
    phase_five_seed: WorthTopologySelectedValidatorEnforcementPhaseFiveSeed,
    closeout_digest: String,
}

impl WorthTopologySelectedValidatorEnforcementCloseout {
    pub fn execute_loop_wiring_family_from_admitted_facts(
        selection_closeout: &WorthTopologyLegalitySelectionCloseout,
        admitted_facts: &WorthTopologyLoopWiringAdmittedLocalFacts,
    ) -> Result<Self, WorthTopologyLegalityCatalogError> {
        let selected_plan = selection_closeout.selected_plan();
        let selected_obligation = selected_loop_wiring_obligation(selected_plan)?;
        let (witness_input, witness_intake_receipt) =
            admit_loop_wiring_witness_input(selected_obligation, admitted_facts)?;
        Self::execute_loop_wiring_family_from_witness_and_intake(
            selected_plan.selected_plan_digest(),
            selected_obligation.row_digest(),
            &witness_input,
            witness_intake_receipt,
        )
    }

    pub fn execute_loop_wiring_family(
        selection_closeout: &WorthTopologyLegalitySelectionCloseout,
        witness_input: &WorthTopologyLoopWiringWitnessInput,
    ) -> Result<Self, WorthTopologyLegalityCatalogError> {
        let selected_plan = selection_closeout.selected_plan();
        let selected_obligation = selected_loop_wiring_obligation(selected_plan)?;
        if witness_input.selected_obligation_digest() != selected_obligation.row_digest() {
            return Err(WorthTopologyLegalityCatalogError::PhaseFourEnforcement(
                WorthTopologySelectedValidatorEnforcementDenial::witness_input_not_bound(
                    "loop_wiring",
                    selected_obligation.row_digest(),
                    witness_input.selected_obligation_digest(),
                ),
            ));
        }
        let witness_intake_receipt =
            WorthTopologyLoopWiringWitnessIntakeReceipt::from_legacy_witness_input(
                witness_input,
                "legacy-direct-witness-input",
            );
        Self::execute_loop_wiring_family_from_witness_and_intake(
            selected_plan.selected_plan_digest(),
            selected_obligation.row_digest(),
            witness_input,
            witness_intake_receipt,
        )
    }

    fn execute_loop_wiring_family_from_witness_and_intake(
        selected_plan_digest: &str,
        selected_obligation_row_digest: &str,
        witness_input: &WorthTopologyLoopWiringWitnessInput,
        witness_intake_receipt: WorthTopologyLoopWiringWitnessIntakeReceipt,
    ) -> Result<Self, WorthTopologyLegalityCatalogError> {
        let family_identity =
            WorthTopologyValidatorFamilyIdentity::from_registered_rule(loop_wiring_rule());
        let outcome = execute_loop_wiring_obligation(witness_input);
        let counters =
            WorthTopologySelectedValidatorEnforcementCounters::from_loop_wiring_execution(
                witness_input,
                &outcome,
            );
        let diagnostic_projection = loop_wiring_diagnostic_projection(
            family_identity.identity_digest(),
            witness_input,
            &outcome,
        );
        let enforcement_receipt = WorthTopologySelectedValidatorEnforcementReceipt::loop_wiring(
            selected_plan_digest,
            selected_obligation_row_digest,
            family_identity.identity_digest(),
            witness_input,
            outcome,
            counters,
            diagnostic_projection
                .diagnostic_projection_digest()
                .to_string(),
        );
        let phase_five_seed = WorthTopologySelectedValidatorEnforcementPhaseFiveSeed::from_receipt(
            selected_plan_digest,
            enforcement_receipt.enforcement_receipt_digest(),
            family_identity.identity_digest(),
            enforcement_receipt
                .counters()
                .executed_validator_family_count(),
            enforcement_receipt.counters().violation_count(),
            enforcement_receipt
                .counters()
                .denied_before_execution_count(),
        );
        let closeout_digest = [
            "worth-topo-selected-validator-enforcement-closeout-v1",
            selected_plan_digest,
            witness_intake_receipt.intake_receipt_digest(),
            enforcement_receipt.enforcement_receipt_digest(),
            diagnostic_projection.diagnostic_projection_digest(),
            phase_five_seed.seed_digest(),
        ]
        .join("|");
        Ok(Self {
            selected_plan_digest: selected_plan_digest.to_string(),
            enforcement_receipt,
            witness_intake_receipt,
            diagnostic_projection,
            phase_five_seed,
            closeout_digest,
        })
    }

    pub fn selected_plan_digest(&self) -> &str {
        &self.selected_plan_digest
    }

    pub const fn enforcement_receipt(&self) -> &WorthTopologySelectedValidatorEnforcementReceipt {
        &self.enforcement_receipt
    }

    pub const fn witness_intake_receipt(&self) -> &WorthTopologyLoopWiringWitnessIntakeReceipt {
        &self.witness_intake_receipt
    }

    pub const fn diagnostic_projection(&self) -> &WorthTopologyLoopWiringDiagnosticProjection {
        &self.diagnostic_projection
    }

    pub const fn phase_five_seed(&self) -> &WorthTopologySelectedValidatorEnforcementPhaseFiveSeed {
        &self.phase_five_seed
    }

    pub fn closeout_digest(&self) -> &str {
        &self.closeout_digest
    }
}
