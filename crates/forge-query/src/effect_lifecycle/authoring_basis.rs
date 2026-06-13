use crate::basis_lifecycle::{
    evaluate_basis_effect_authoring_deferred_eligibility, normalize_raw_basis_intent,
    AdvisoryBasisEligibility, BasisAuthorityPosture, BasisFamily, BasisLifecyclePosture,
    DeferredBasisEligibility, InspectionLaneWitness, RawBasisIntent,
    ScopedMutationPreparationBasis, ScopedPreviewCloseoutBasis,
};
use crate::evidence_identity::{
    ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EffectAuthoringBasis {
    MutationPreparation(ScopedMutationPreparationBasis),
    PreviewCloseout(ScopedPreviewCloseoutBasis),
    InspectionAdvisory(AdvisoryBasisEligibility<InspectionLaneWitness>),
    DeferredFutureNeighbor(
        DeferredBasisEligibility<crate::basis_lifecycle::EffectAuthoringLaneWitness>,
    ),
}

impl EffectAuthoringBasis {
    pub fn store_backed(store_basis_identity: impl Into<String>) -> Self {
        Self::deferred_future_neighbor(RawBasisIntent::StoreBacked {
            store_basis_identity: store_basis_identity.into(),
        })
    }

    pub fn durable_reload(reload_identity: impl Into<String>) -> Self {
        Self::deferred_future_neighbor(RawBasisIntent::DurableReload {
            reload_identity: reload_identity.into(),
        })
    }

    fn deferred_future_neighbor(raw: RawBasisIntent) -> Self {
        let normalized = normalize_raw_basis_intent(raw, "effect_authoring")
            .expect("future-neighbor effect basis should normalize");
        let deferred = evaluate_basis_effect_authoring_deferred_eligibility(normalized)
            .expect("future-neighbor effect basis should return deferred proof");
        Self::DeferredFutureNeighbor(deferred)
    }

    pub fn family(&self) -> BasisFamily {
        match self {
            Self::MutationPreparation(basis) => basis.family(),
            Self::PreviewCloseout(basis) => basis.family(),
            Self::InspectionAdvisory(advisory) => advisory.normalized().family(),
            Self::DeferredFutureNeighbor(deferred) => deferred.normalized().family(),
        }
    }

    pub fn authority(&self) -> BasisAuthorityPosture {
        match self {
            Self::MutationPreparation(basis) => basis.authority(),
            Self::PreviewCloseout(basis) => basis.authority(),
            Self::InspectionAdvisory(advisory) => advisory.normalized().authority(),
            Self::DeferredFutureNeighbor(deferred) => deferred.normalized().authority(),
        }
    }

    pub fn lifecycle(&self) -> BasisLifecyclePosture {
        match self {
            Self::MutationPreparation(basis) => basis.lifecycle(),
            Self::PreviewCloseout(basis) => basis.lifecycle(),
            Self::InspectionAdvisory(advisory) => advisory.normalized().lifecycle(),
            Self::DeferredFutureNeighbor(deferred) => deferred.normalized().lifecycle(),
        }
    }

    pub fn capability_digest(&self) -> String {
        match self {
            Self::MutationPreparation(basis) => basis.capability_digest().to_string(),
            Self::PreviewCloseout(basis) => basis.capability_digest().to_string(),
            Self::InspectionAdvisory(advisory) => advisory.authoring_digest(),
            Self::DeferredFutureNeighbor(deferred) => deferred.authoring_digest(),
        }
    }

    pub fn capability_identity(&self) -> ForgeQueryEvidenceIdentity {
        capability_identity_from_digest(&self.capability_digest())
    }

    pub fn scoped_basis_digest(&self) -> &str {
        match self {
            Self::MutationPreparation(basis) => basis.scoped_basis_digest(),
            Self::PreviewCloseout(basis) => basis.scoped_basis_digest(),
            Self::InspectionAdvisory(advisory) => advisory.normalized().normalized_digest(),
            Self::DeferredFutureNeighbor(deferred) => deferred.normalized().normalized_digest(),
        }
    }

    pub fn scoped_basis_identity(&self) -> ForgeQueryEvidenceIdentity {
        scoped_basis_identity_from_digest(self.scoped_basis_digest())
    }

    pub fn expected_lower_runtime_binding_digest(&self) -> Option<&str> {
        match self {
            Self::MutationPreparation(basis) => basis.expected_lower_runtime_binding_digest(),
            Self::PreviewCloseout(basis) => basis.expected_lower_runtime_binding_digest(),
            Self::InspectionAdvisory(advisory) => {
                advisory.normalized().lower_runtime_binding_digest()
            }
            Self::DeferredFutureNeighbor(deferred) => {
                deferred.normalized().lower_runtime_binding_digest()
            }
        }
    }

    pub fn expected_lower_runtime_binding_identity(&self) -> Option<ForgeQueryEvidenceIdentity> {
        self.expected_lower_runtime_binding_digest()
            .map(expected_lower_runtime_binding_identity)
    }

    pub(crate) fn requires_preview_workflow_binding(&self) -> bool {
        matches!(self, Self::PreviewCloseout(_))
    }
}

fn expected_lower_runtime_binding_identity(
    binding_digest: &str,
) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::WorkflowMutationLowering)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "expected_lower_runtime_binding_v1",
        )
        .field_identity(ForgeQueryEvidenceTag::new("binding"), binding_digest)
        .seal()
}

fn capability_identity_from_digest(capability_digest: &str) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::WorkflowMutationLowering)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "effect_authoring_capability_v1",
        )
        .field_identity(ForgeQueryEvidenceTag::new("capability"), capability_digest)
        .seal()
}

fn scoped_basis_identity_from_digest(scoped_basis_digest: &str) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::WorkflowMutationLowering)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "effect_authoring_scoped_basis_v1",
        )
        .field_identity(ForgeQueryEvidenceTag::new("scoped_basis"), scoped_basis_digest)
        .seal()
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

impl From<AdvisoryBasisEligibility<InspectionLaneWitness>> for EffectAuthoringBasis {
    fn from(value: AdvisoryBasisEligibility<InspectionLaneWitness>) -> Self {
        Self::InspectionAdvisory(value)
    }
}
