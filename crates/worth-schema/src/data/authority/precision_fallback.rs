use serde::{Deserialize, Serialize};
use worth_math::arithmetic::precision::{EscalationEvent, PrecisionEscalation, PrecisionMode};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum WorthPrecisionRegime {
    Float64,
    ExpansionB,
    ExpansionC,
    ExactRational,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum WorthPrecisionEscalationCause {
    NearBoundary,
    FloatDisagreement,
    ResidualExceeded,
    BudgetExceeded,
    TargetMismatch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum WorthFallbackDisposition {
    NoneRequired,
    EscalatePrecision,
    FailClosed,
    BranchLocalReview,
    RetainedExactOverride,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum WorthFallbackProofClass {
    LocalDiagnosticOnly,
    ReplayRequired,
    CertificationRequired,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorthPrecisionFallbackRecord {
    pub resolved_regime: WorthPrecisionRegime,
    pub escalation_cause: WorthPrecisionEscalationCause,
    pub disposition: WorthFallbackDisposition,
    pub proof_class: WorthFallbackProofClass,
    pub float_agreed: bool,
    pub expansion_length: Option<usize>,
    pub target_triple: String,
    pub disagreement_magnitude: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthPrecisionBudgetFallbackRecord {
    pub resolved_regime: WorthPrecisionRegime,
    pub escalation_cause: WorthPrecisionEscalationCause,
    pub disposition: WorthFallbackDisposition,
    pub proof_class: WorthFallbackProofClass,
    pub bit_length_before: u32,
    pub bit_length_after: u32,
    pub threshold: u32,
    pub sign_preserved: bool,
}

impl WorthPrecisionFallbackRecord {
    pub fn from_precision_escalation(escalation: &PrecisionEscalation) -> Self {
        let escalation_cause = if !escalation.float_agreed {
            WorthPrecisionEscalationCause::FloatDisagreement
        } else {
            WorthPrecisionEscalationCause::NearBoundary
        };

        let disposition = match escalation.resolved_at {
            PrecisionMode::Float64 => WorthFallbackDisposition::NoneRequired,
            PrecisionMode::ExpansionB
            | PrecisionMode::ExpansionC
            | PrecisionMode::ExactRational => WorthFallbackDisposition::EscalatePrecision,
        };

        let proof_class = match escalation.resolved_at {
            PrecisionMode::Float64 => WorthFallbackProofClass::LocalDiagnosticOnly,
            PrecisionMode::ExpansionB | PrecisionMode::ExpansionC => {
                WorthFallbackProofClass::ReplayRequired
            }
            PrecisionMode::ExactRational => WorthFallbackProofClass::CertificationRequired,
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

impl WorthPrecisionBudgetFallbackRecord {
    pub fn from_budget_escalation(event: &EscalationEvent) -> Self {
        Self {
            resolved_regime: WorthPrecisionRegime::ExactRational,
            escalation_cause: WorthPrecisionEscalationCause::BudgetExceeded,
            disposition: WorthFallbackDisposition::RetainedExactOverride,
            proof_class: WorthFallbackProofClass::CertificationRequired,
            bit_length_before: event.bit_length_before,
            bit_length_after: event.bit_length_after,
            threshold: event.threshold,
            sign_preserved: event.sign_preserved,
        }
    }
}

impl From<PrecisionMode> for WorthPrecisionRegime {
    fn from(value: PrecisionMode) -> Self {
        match value {
            PrecisionMode::Float64 => Self::Float64,
            PrecisionMode::ExpansionB => Self::ExpansionB,
            PrecisionMode::ExpansionC => Self::ExpansionC,
            PrecisionMode::ExactRational => Self::ExactRational,
        }
    }
}

impl From<&PrecisionEscalation> for WorthPrecisionFallbackRecord {
    fn from(value: &PrecisionEscalation) -> Self {
        Self::from_precision_escalation(value)
    }
}

impl From<PrecisionEscalation> for WorthPrecisionFallbackRecord {
    fn from(value: PrecisionEscalation) -> Self {
        Self::from_precision_escalation(&value)
    }
}

impl From<&EscalationEvent> for WorthPrecisionBudgetFallbackRecord {
    fn from(value: &EscalationEvent) -> Self {
        Self::from_budget_escalation(value)
    }
}

impl From<EscalationEvent> for WorthPrecisionBudgetFallbackRecord {
    fn from(value: EscalationEvent) -> Self {
        Self::from_budget_escalation(&value)
    }
}
