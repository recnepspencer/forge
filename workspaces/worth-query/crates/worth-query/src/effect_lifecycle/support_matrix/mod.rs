mod discovery;
mod lookup;
mod posture;
mod row;

use crate::WorthQueryEvidenceScope;

const EFFECT_LIFECYCLE_IDENTITY_SCOPE: WorthQueryEvidenceScope =
    WorthQueryEvidenceScope::WorkflowMutationLowering;

pub use discovery::{
    discover_effect_lifecycle_support, effect_lifecycle_support_matrix,
    EffectLifecycleSupportDiscovery, EffectLifecycleSupportMatrix,
};
pub use posture::{EffectSupportCause, EffectSupportPosture};
pub use row::EffectLifecycleSupportRow;

pub(crate) use lookup::support_decision_for;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct EffectSupportDecision {
    posture: EffectSupportPosture,
    cause: EffectSupportCause,
    matched_row: Option<EffectLifecycleSupportRow>,
    rows_consulted: usize,
}

impl EffectSupportDecision {
    pub(crate) fn posture(&self) -> EffectSupportPosture {
        self.posture
    }

    pub(crate) fn rows_consulted(&self) -> usize {
        self.rows_consulted
    }

    pub(crate) fn cause(&self) -> EffectSupportCause {
        self.cause
    }
}
