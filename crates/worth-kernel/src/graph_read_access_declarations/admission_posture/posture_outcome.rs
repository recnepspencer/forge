use super::query_admission_projection::{
    WorthGraphReadAdmissionAttempt, WorthGraphReadAdmissionCapabilityGap,
};
use crate::graph_read_access_declarations::WorthGraphReadRequirementDerivationCapabilityGap;
use forge_query::facade::ForgeQueryGraphReadAccessAdmissionPosture;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthGraphReadAccessAdmissionPostureOutcome {
    QueryAdmissionEvidence(WorthGraphReadQueryAdmissionEvidence),
    RequiredSupportCapabilityGap {
        admission_attempt: WorthGraphReadAdmissionAttempt,
        admission_gap: WorthGraphReadAdmissionCapabilityGap,
    },
    RequirementDerivationGapCarriedForward {
        admission_attempt: WorthGraphReadAdmissionAttempt,
        admission_gap: WorthGraphReadAdmissionCapabilityGap,
        requirement_gap: WorthGraphReadRequirementDerivationCapabilityGap,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadQueryAdmissionEvidence {
    admission_digest: String,
    requirement_set_digest: String,
    admission_posture: ForgeQueryGraphReadAccessAdmissionPosture,
}

impl WorthGraphReadAccessAdmissionPostureOutcome {
    pub const fn claims_graph_read_execution(&self) -> bool {
        false
    }

    pub const fn claims_access_plan_consumption(&self) -> bool {
        false
    }

    pub fn admission_gap(&self) -> Option<&WorthGraphReadAdmissionCapabilityGap> {
        match self {
            Self::QueryAdmissionEvidence(_) => None,
            Self::RequiredSupportCapabilityGap { admission_gap, .. }
            | Self::RequirementDerivationGapCarriedForward { admission_gap, .. } => {
                Some(admission_gap)
            }
        }
    }

    pub fn requirement_derivation_gap(
        &self,
    ) -> Option<&WorthGraphReadRequirementDerivationCapabilityGap> {
        match self {
            Self::RequirementDerivationGapCarriedForward {
                requirement_gap, ..
            } => Some(requirement_gap),
            Self::QueryAdmissionEvidence(_) | Self::RequiredSupportCapabilityGap { .. } => None,
        }
    }

    pub(crate) fn digest_part(&self) -> String {
        match self {
            Self::QueryAdmissionEvidence(evidence) => {
                format!("query_admission:{}", evidence.admission_digest)
            }
            Self::RequiredSupportCapabilityGap { admission_gap, .. }
            | Self::RequirementDerivationGapCarriedForward { admission_gap, .. } => {
                format!("admission_gap:{}", admission_gap.gap_digest())
            }
        }
    }
}

impl WorthGraphReadQueryAdmissionEvidence {
    pub(crate) fn from_query_admission(
        admission: &forge_query::facade::ForgeQueryGraphReadAccessAdmission,
    ) -> Self {
        Self {
            admission_digest: admission.digest().to_string(),
            requirement_set_digest: admission.requirement_set().digest().as_str().to_string(),
            admission_posture: admission.posture().clone(),
        }
    }

    pub fn admission_digest(&self) -> &str {
        &self.admission_digest
    }

    pub fn requirement_set_digest(&self) -> &str {
        &self.requirement_set_digest
    }

    pub fn admission_posture(&self) -> &ForgeQueryGraphReadAccessAdmissionPosture {
        &self.admission_posture
    }
}
