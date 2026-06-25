use crate::validator_invariant_catalog::selected_validator_enforcement::{
    WorthTopologyLoopWiringViolationKind, WorthTopologyLoopWiringWitnessInput,
    WorthTopologySelectedValidatorEnforcementOutcome,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorthTopologyLoopWiringDiagnosticProjection {
    selected_obligation_digest: String,
    family_identity_digest: String,
    witness_input_digest: String,
    violation_kind: Option<WorthTopologyLoopWiringViolationKind>,
    touched_loop_id: Option<forge_relational::facade::identity::EntityId>,
    touched_half_edge_id: Option<forge_relational::facade::identity::EntityId>,
    related_half_edge_id: Option<forge_relational::facade::identity::EntityId>,
    diagnostic_projection_digest: String,
}

impl WorthTopologyLoopWiringDiagnosticProjection {
    pub(in crate::validator_invariant_catalog) fn from_witness_outcome(
        family_identity_digest: &str,
        witness_input: &WorthTopologyLoopWiringWitnessInput,
        outcome: &WorthTopologySelectedValidatorEnforcementOutcome,
    ) -> Self {
        let (
            violation_kind,
            touched_loop_id,
            touched_half_edge_id,
            related_half_edge_id,
            outcome_digest,
        ) = match outcome {
            WorthTopologySelectedValidatorEnforcementOutcome::Violation(witness) => (
                Some(witness.violation_kind()),
                witness.touched_loop_id(),
                witness.touched_half_edge_id(),
                witness.related_half_edge_id(),
                witness.witness_digest().to_string(),
            ),
            _ => (None, None, None, None, outcome.outcome_digest()),
        };
        let diagnostic_projection_digest = [
            "worth-topo-loop-wiring-diagnostic-projection-v1".to_string(),
            format!(
                "selected-obligation:{}",
                witness_input.selected_obligation_digest()
            ),
            format!("family:{family_identity_digest}"),
            format!("witness-input:{}", witness_input.input_digest()),
            format!("outcome:{outcome_digest}"),
            format!("violation-kind:{violation_kind:?}"),
            format!("touched-loop:{touched_loop_id:?}"),
            format!("touched-half-edge:{touched_half_edge_id:?}"),
            format!("related-half-edge:{related_half_edge_id:?}"),
        ]
        .join("|");
        Self {
            selected_obligation_digest: witness_input.selected_obligation_digest().to_string(),
            family_identity_digest: family_identity_digest.to_string(),
            witness_input_digest: witness_input.input_digest().to_string(),
            violation_kind,
            touched_loop_id,
            touched_half_edge_id,
            related_half_edge_id,
            diagnostic_projection_digest,
        }
    }

    pub fn selected_obligation_digest(&self) -> &str {
        &self.selected_obligation_digest
    }

    pub fn family_identity_digest(&self) -> &str {
        &self.family_identity_digest
    }

    pub fn witness_input_digest(&self) -> &str {
        &self.witness_input_digest
    }

    pub const fn violation_kind(&self) -> Option<WorthTopologyLoopWiringViolationKind> {
        self.violation_kind
    }

    pub const fn touched_loop_id(&self) -> Option<forge_relational::facade::identity::EntityId> {
        self.touched_loop_id
    }

    pub const fn touched_half_edge_id(
        &self,
    ) -> Option<forge_relational::facade::identity::EntityId> {
        self.touched_half_edge_id
    }

    pub const fn related_half_edge_id(
        &self,
    ) -> Option<forge_relational::facade::identity::EntityId> {
        self.related_half_edge_id
    }

    pub fn diagnostic_projection_digest(&self) -> &str {
        &self.diagnostic_projection_digest
    }
}

pub(in crate::validator_invariant_catalog) fn loop_wiring_diagnostic_projection(
    family_identity_digest: &str,
    witness_input: &WorthTopologyLoopWiringWitnessInput,
    outcome: &WorthTopologySelectedValidatorEnforcementOutcome,
) -> WorthTopologyLoopWiringDiagnosticProjection {
    WorthTopologyLoopWiringDiagnosticProjection::from_witness_outcome(
        family_identity_digest,
        witness_input,
        outcome,
    )
}
