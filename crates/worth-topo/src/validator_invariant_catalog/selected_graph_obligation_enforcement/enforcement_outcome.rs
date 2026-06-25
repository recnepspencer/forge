use crate::validator_invariant_catalog::selected_graph_obligation_enforcement::{
    WorthTopologySelectedGraphObligationDiagnosticWitness,
    WorthTopologySelectedGraphObligationEnforcementDenial,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthTopologySelectedGraphObligationEnforcementOutcome {
    Passed,
    Advisory(WorthTopologySelectedGraphObligationDiagnosticWitness),
    Violation(WorthTopologySelectedGraphObligationDiagnosticWitness),
    DeniedBeforeExecution(WorthTopologySelectedGraphObligationEnforcementDenial),
}

impl WorthTopologySelectedGraphObligationEnforcementOutcome {
    pub const fn is_passed(&self) -> bool {
        matches!(self, Self::Passed)
    }

    pub const fn is_advisory(&self) -> bool {
        matches!(self, Self::Advisory(_))
    }

    pub const fn is_violation(&self) -> bool {
        matches!(self, Self::Violation(_))
    }

    pub const fn is_denied_before_execution(&self) -> bool {
        matches!(self, Self::DeniedBeforeExecution(_))
    }

    pub fn outcome_digest(&self) -> String {
        match self {
            Self::Passed => "worth-topo-selected-graph-obligation-outcome-v1|passed".to_string(),
            Self::Advisory(witness) => format!(
                "worth-topo-selected-graph-obligation-outcome-v1|advisory:{}",
                witness.witness_digest()
            ),
            Self::Violation(witness) => format!(
                "worth-topo-selected-graph-obligation-outcome-v1|violation:{}",
                witness.witness_digest()
            ),
            Self::DeniedBeforeExecution(denial) => format!(
                "worth-topo-selected-graph-obligation-outcome-v1|denied:{}",
                denial.denial_digest()
            ),
        }
    }
}
