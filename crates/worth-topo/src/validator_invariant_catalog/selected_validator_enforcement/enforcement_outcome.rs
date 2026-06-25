use crate::validator_invariant_catalog::selected_validator_enforcement::{
    WorthTopologyLoopWiringWitnessRow, WorthTopologySelectedValidatorEnforcementDenial,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WorthTopologySelectedValidatorEnforcementOutcome {
    Passed,
    Violation(WorthTopologyLoopWiringWitnessRow),
    DeniedBeforeExecution(WorthTopologySelectedValidatorEnforcementDenial),
    CertificationComparisonOnly(String),
}

impl WorthTopologySelectedValidatorEnforcementOutcome {
    pub const fn is_violation(&self) -> bool {
        matches!(self, Self::Violation(_))
    }

    pub const fn is_denied_before_execution(&self) -> bool {
        matches!(self, Self::DeniedBeforeExecution(_))
    }

    pub fn outcome_digest(&self) -> String {
        match self {
            Self::Passed => "worth-topo-selected-validator-outcome-v1|passed".to_string(),
            Self::Violation(row) => format!(
                "worth-topo-selected-validator-outcome-v1|violation:{}",
                row.witness_digest()
            ),
            Self::DeniedBeforeExecution(denial) => {
                format!(
                    "worth-topo-selected-validator-outcome-v1|denied:{}",
                    denial.denial_digest()
                )
            }
            Self::CertificationComparisonOnly(reason) => format!(
                "worth-topo-selected-validator-outcome-v1|certification-comparison-only:{reason}"
            ),
        }
    }
}
