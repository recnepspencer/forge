use forge_query::facade::{
    ForgeQueryGraphReadAccessAdmissionPosture, ForgeQueryGraphReadAccessDenialKind,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthGraphReadAdmissionExpectedDenial {
    RequirementDerivationGap,
    MissingQueryReadFamilyArtifact,
    QueryAdmissionDenied(ForgeQueryGraphReadAccessDenialKind),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthGraphReadAdmissionSuggestedPosture {
    RequirementDerivationMustSucceed,
    QueryReadFamilyArtifactRequired,
    QueryAdmissionPosture(ForgeQueryGraphReadAccessAdmissionPosture),
}

impl WorthGraphReadAdmissionExpectedDenial {
    pub fn digest_part(&self) -> String {
        match self {
            Self::RequirementDerivationGap => "requirement_derivation_gap".to_string(),
            Self::MissingQueryReadFamilyArtifact => {
                "missing_query_read_family_artifact".to_string()
            }
            Self::QueryAdmissionDenied(kind) => format!("query_denial:{}", kind.as_str()),
        }
    }
}

impl WorthGraphReadAdmissionSuggestedPosture {
    pub fn digest_part(&self) -> String {
        match self {
            Self::RequirementDerivationMustSucceed => {
                "requirement_derivation_must_succeed_before_admission".to_string()
            }
            Self::QueryReadFamilyArtifactRequired => {
                "query_read_family_artifact_required".to_string()
            }
            Self::QueryAdmissionPosture(posture) => {
                format!("query_posture:{}", posture.as_str())
            }
        }
    }
}
