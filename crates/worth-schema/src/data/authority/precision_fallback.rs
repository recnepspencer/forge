use math::arithmetic::precision::{EscalationEvent, PrecisionEscalation, PrecisionMode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum PrecisionRegime {
    Float64,
    ExpansionB,
    ExpansionC,
    ExactRational,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum PrecisionEscalationCause {
    NearBoundary,
    FloatDisagreement,
    ResidualExceeded,
    BudgetExceeded,
    TargetMismatch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum FallbackDisposition {
    NoneRequired,
    EscalatePrecision,
    FailClosed,
    BranchLocalReview,
    RetainedExactOverride,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum FallbackProofClass {
    LocalDiagnosticOnly,
    ReplayRequired,
    CertificationRequired,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrecisionFallbackRecord {
    pub resolved_regime: PrecisionRegime,
    pub escalation_cause: PrecisionEscalationCause,
    pub disposition: FallbackDisposition,
    pub proof_class: FallbackProofClass,
    pub float_agreed: bool,
    pub expansion_length: Option<usize>,
    pub target_triple: String,
    pub disagreement_magnitude: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrecisionBudgetFallbackRecord {
    pub resolved_regime: PrecisionRegime,
    pub escalation_cause: PrecisionEscalationCause,
    pub disposition: FallbackDisposition,
    pub proof_class: FallbackProofClass,
    pub bit_length_before: u32,
    pub bit_length_after: u32,
    pub threshold: u32,
    pub sign_preserved: bool,
}

impl PrecisionFallbackRecord {
    pub fn from_precision_escalation(escalation: &PrecisionEscalation) -> Self {
        let escalation_cause = if !escalation.float_agreed {
            PrecisionEscalationCause::FloatDisagreement
        } else {
            PrecisionEscalationCause::NearBoundary
        };

        let disposition = match escalation.resolved_at {
            PrecisionMode::Float64 => FallbackDisposition::NoneRequired,
            PrecisionMode::ExpansionB
            | PrecisionMode::ExpansionC
            | PrecisionMode::ExactRational => FallbackDisposition::EscalatePrecision,
        };

        let proof_class = match escalation.resolved_at {
            PrecisionMode::Float64 => FallbackProofClass::LocalDiagnosticOnly,
            PrecisionMode::ExpansionB | PrecisionMode::ExpansionC => {
                FallbackProofClass::ReplayRequired
            }
            PrecisionMode::ExactRational => FallbackProofClass::CertificationRequired,
        };

        Self {
            resolved_regime: escalation.resolved_at.into(),
            escalation_cause,
            disposition,
            proof_class,
            float_agreed: escalation.float_agreed,
            expansion_length: escalation.expansion_length,
            target_triple: escalation.target_triple.clone(),
            disagreement_magnitude: escalation.disagreement_magnitude,
        }
    }
}

impl PrecisionBudgetFallbackRecord {
    pub fn from_budget_escalation(event: &EscalationEvent) -> Self {
        Self {
            resolved_regime: PrecisionRegime::ExactRational,
            escalation_cause: PrecisionEscalationCause::BudgetExceeded,
            disposition: FallbackDisposition::RetainedExactOverride,
            proof_class: FallbackProofClass::CertificationRequired,
            bit_length_before: event.bit_length_before,
            bit_length_after: event.bit_length_after,
            threshold: event.threshold,
            sign_preserved: event.sign_preserved,
        }
    }
}

impl From<PrecisionMode> for PrecisionRegime {
    fn from(value: PrecisionMode) -> Self {
        match value {
            PrecisionMode::Float64 => Self::Float64,
            PrecisionMode::ExpansionB => Self::ExpansionB,
            PrecisionMode::ExpansionC => Self::ExpansionC,
            PrecisionMode::ExactRational => Self::ExactRational,
        }
    }
}

impl From<&PrecisionEscalation> for PrecisionFallbackRecord {
    fn from(value: &PrecisionEscalation) -> Self {
        Self::from_precision_escalation(value)
    }
}

impl From<PrecisionEscalation> for PrecisionFallbackRecord {
    fn from(value: PrecisionEscalation) -> Self {
        Self::from_precision_escalation(&value)
    }
}

impl From<&EscalationEvent> for PrecisionBudgetFallbackRecord {
    fn from(value: &EscalationEvent) -> Self {
        Self::from_budget_escalation(value)
    }
}

impl From<EscalationEvent> for PrecisionBudgetFallbackRecord {
    fn from(value: EscalationEvent) -> Self {
        Self::from_budget_escalation(&value)
    }
}
