use crate::basis_lifecycle::{
    BasisAuthorityPosture, BasisFamily, BasisLifecyclePosture, ScopedMutationPreparationBasis,
    ScopedPreviewCloseoutBasis,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EffectAuthoringBasis {
    MutationPreparation(ScopedMutationPreparationBasis),
    PreviewCloseout(ScopedPreviewCloseoutBasis),
}

impl EffectAuthoringBasis {
    pub fn family(&self) -> BasisFamily {
        match self {
            Self::MutationPreparation(basis) => basis.family(),
            Self::PreviewCloseout(basis) => basis.family(),
        }
    }

    pub fn authority(&self) -> BasisAuthorityPosture {
        match self {
            Self::MutationPreparation(basis) => basis.authority(),
            Self::PreviewCloseout(basis) => basis.authority(),
        }
    }

    pub fn lifecycle(&self) -> BasisLifecyclePosture {
        match self {
            Self::MutationPreparation(basis) => basis.lifecycle(),
            Self::PreviewCloseout(basis) => basis.lifecycle(),
        }
    }

    pub fn capability_digest(&self) -> &str {
        match self {
            Self::MutationPreparation(basis) => basis.capability_digest(),
            Self::PreviewCloseout(basis) => basis.capability_digest(),
        }
    }

    pub fn scoped_basis_digest(&self) -> &str {
        match self {
            Self::MutationPreparation(basis) => basis.scoped_basis_digest(),
            Self::PreviewCloseout(basis) => basis.scoped_basis_digest(),
        }
    }

    pub fn expected_lower_runtime_binding_digest(&self) -> Option<&str> {
        match self {
            Self::MutationPreparation(basis) => basis.expected_lower_runtime_binding_digest(),
            Self::PreviewCloseout(basis) => basis.expected_lower_runtime_binding_digest(),
        }
    }

    pub(crate) fn requires_preview_workflow_binding(&self) -> bool {
        matches!(self, Self::PreviewCloseout(_))
    }
}

impl From<ScopedMutationPreparationBasis> for EffectAuthoringBasis {
    fn from(value: ScopedMutationPreparationBasis) -> Self {
        Self::MutationPreparation(value)
    }
}

impl From<ScopedPreviewCloseoutBasis> for EffectAuthoringBasis {
    fn from(value: ScopedPreviewCloseoutBasis) -> Self {
        Self::PreviewCloseout(value)
    }
}
