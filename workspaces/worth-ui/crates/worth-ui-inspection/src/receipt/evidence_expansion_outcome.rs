use super::{
    UiEvidenceAuthorityGeneration, UiEvidenceMaterializationPosture, UiEvidenceRetentionPosture,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum UiEvidenceExpansionOutcome {
    Available,
    Discarded {
        retention: UiEvidenceRetentionPosture,
    },
    WrongGeneration {
        requested_generation: UiEvidenceAuthorityGeneration,
        current_generation: UiEvidenceAuthorityGeneration,
    },
    NotMaterialized {
        posture: UiEvidenceMaterializationPosture,
    },
    Unsupported,
}

impl UiEvidenceExpansionOutcome {
    pub const fn is_available(self) -> bool {
        matches!(self, Self::Available)
    }
}
