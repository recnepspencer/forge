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

    pub fn capability_for_reporting(&self) -> String {
        self.capability_identity().as_str().to_string()
    }

    pub fn capability_identity(&self) -> ForgeQueryEvidenceIdentity {
        match self {
            Self::MutationPreparation(basis) => mutation_preparation_capability_identity(basis),
            Self::PreviewCloseout(basis) => preview_closeout_capability_identity(basis),
            Self::InspectionAdvisory(advisory) => {
                advisory_basis_capability_identity(advisory.normalized())
            }
            Self::DeferredFutureNeighbor(deferred) => {
                deferred_basis_capability_identity(deferred.normalized())
            }
        }
    }

    pub fn scoped_basis_for_reporting(&self) -> String {
        self.scoped_basis_identity().as_str().to_string()
    }

    pub fn scoped_basis_identity(&self) -> ForgeQueryEvidenceIdentity {
        match self {
            Self::MutationPreparation(basis) => mutation_preparation_scoped_basis_identity(basis),
            Self::PreviewCloseout(basis) => preview_closeout_scoped_basis_identity(basis),
            Self::InspectionAdvisory(advisory) => {
                advisory_basis_scoped_basis_identity(advisory.normalized())
            }
            Self::DeferredFutureNeighbor(deferred) => {
                deferred_basis_scoped_basis_identity(deferred.normalized())
            }
        }
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

fn mutation_preparation_capability_identity(
    basis: &ScopedMutationPreparationBasis,
) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::WorkflowMutationLowering)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "effect_authoring_capability_v1",
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("variant"),
            "mutation_preparation",
        )
        .field_shape(ForgeQueryEvidenceTag::new("family"), basis.family().as_str())
        .field_shape(
            ForgeQueryEvidenceTag::new("authority"),
            basis.authority().as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("lifecycle"),
            basis.lifecycle().as_str(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("capability"),
            &basis_lifecycle_admitted_capability_label_identity(basis.capability_digest()),
        )
        .seal()
}

fn preview_closeout_capability_identity(
    basis: &ScopedPreviewCloseoutBasis,
) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::WorkflowMutationLowering)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "effect_authoring_capability_v1",
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("variant"),
            "preview_closeout",
        )
        .field_shape(ForgeQueryEvidenceTag::new("family"), basis.family().as_str())
        .field_shape(
            ForgeQueryEvidenceTag::new("authority"),
            basis.authority().as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("lifecycle"),
            basis.lifecycle().as_str(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("capability"),
            &basis_lifecycle_admitted_capability_label_identity(basis.capability_digest()),
        )
        .seal()
}

fn advisory_basis_capability_identity(
    normalized: &crate::basis_lifecycle::NormalizedBasisIntent,
) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::WorkflowMutationLowering)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "effect_authoring_capability_v1",
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("variant"),
            "inspection_advisory",
        )
        .field_shape(ForgeQueryEvidenceTag::new("family"), normalized.family().as_str())
        .field_shape(
            ForgeQueryEvidenceTag::new("authority"),
            normalized.authority().as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("lifecycle"),
            normalized.lifecycle().as_str(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("normalized"),
            &basis_lifecycle_normalized_label_identity(normalized),
        )
        .seal()
}

fn deferred_basis_capability_identity(
    normalized: &crate::basis_lifecycle::NormalizedBasisIntent,
) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::WorkflowMutationLowering)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "effect_authoring_capability_v1",
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("variant"),
            "deferred_future_neighbor",
        )
        .field_shape(ForgeQueryEvidenceTag::new("family"), normalized.family().as_str())
        .field_shape(
            ForgeQueryEvidenceTag::new("authority"),
            normalized.authority().as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("lifecycle"),
            normalized.lifecycle().as_str(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("normalized"),
            &basis_lifecycle_normalized_label_identity(normalized),
        )
        .seal()
}

fn mutation_preparation_scoped_basis_identity(
    basis: &ScopedMutationPreparationBasis,
) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::WorkflowMutationLowering)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "effect_authoring_scoped_basis_v1",
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("variant"),
            "mutation_preparation",
        )
        .field_shape(ForgeQueryEvidenceTag::new("family"), basis.family().as_str())
        .field_shape(
            ForgeQueryEvidenceTag::new("authority"),
            basis.authority().as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("lifecycle"),
            basis.lifecycle().as_str(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("scoped_basis"),
            &basis_lifecycle_scoped_basis_label_identity(basis.scoped_basis_digest()),
        )
        .seal()
}

fn preview_closeout_scoped_basis_identity(
    basis: &ScopedPreviewCloseoutBasis,
) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::WorkflowMutationLowering)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "effect_authoring_scoped_basis_v1",
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("variant"),
            "preview_closeout",
        )
        .field_shape(ForgeQueryEvidenceTag::new("family"), basis.family().as_str())
        .field_shape(
            ForgeQueryEvidenceTag::new("authority"),
            basis.authority().as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("lifecycle"),
            basis.lifecycle().as_str(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("scoped_basis"),
            &basis_lifecycle_scoped_basis_label_identity(basis.scoped_basis_digest()),
        )
        .seal()
}

fn advisory_basis_scoped_basis_identity(
    normalized: &crate::basis_lifecycle::NormalizedBasisIntent,
) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::WorkflowMutationLowering)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "effect_authoring_scoped_basis_v1",
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("variant"),
            "inspection_advisory",
        )
        .field_shape(ForgeQueryEvidenceTag::new("family"), normalized.family().as_str())
        .field_shape(
            ForgeQueryEvidenceTag::new("authority"),
            normalized.authority().as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("lifecycle"),
            normalized.lifecycle().as_str(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("normalized"),
            &basis_lifecycle_normalized_label_identity(normalized),
        )
        .seal()
}

fn deferred_basis_scoped_basis_identity(
    normalized: &crate::basis_lifecycle::NormalizedBasisIntent,
) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::WorkflowMutationLowering)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "effect_authoring_scoped_basis_v1",
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("variant"),
            "deferred_future_neighbor",
        )
        .field_shape(ForgeQueryEvidenceTag::new("family"), normalized.family().as_str())
        .field_shape(
            ForgeQueryEvidenceTag::new("authority"),
            normalized.authority().as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("lifecycle"),
            normalized.lifecycle().as_str(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("normalized"),
            &basis_lifecycle_normalized_label_identity(normalized),
        )
        .seal()
}

fn basis_lifecycle_admitted_capability_label_identity(
    capability_digest: &str,
) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::WorkflowMutationLowering)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "basis_lifecycle_admitted_capability_label_v1",
        )
        .field_shape(ForgeQueryEvidenceTag::new("capability"), capability_digest)
        .seal()
}

fn basis_lifecycle_scoped_basis_label_identity(
    scoped_basis_digest: &str,
) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::WorkflowMutationLowering)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "basis_lifecycle_scoped_basis_label_v1",
        )
        .field_shape(ForgeQueryEvidenceTag::new("scoped_basis"), scoped_basis_digest)
        .seal()
}

fn basis_lifecycle_normalized_label_identity(
    normalized: &crate::basis_lifecycle::NormalizedBasisIntent,
) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::WorkflowMutationLowering)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "basis_lifecycle_normalized_label_v1",
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("normalized"),
            normalized.normalized_digest(),
        )
        .seal()
}

fn expected_lower_runtime_binding_identity(
    binding_digest: &str,
) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::WorkflowMutationLowering)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "expected_lower_runtime_binding_v1",
        )
        .field_shape(ForgeQueryEvidenceTag::new("binding"), binding_digest)
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
