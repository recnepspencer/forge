use crate::evidence_identity::{
    ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};
use crate::identity::hash_parts;

use super::semantics::ForgeQueryBindingTargetSemantics;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryBindingTargetKind {
    IntentDeclaration,
    AdmittedIntentPlan,
    LowerRuntimeBoundaryEnvelope,
    AdmittedDeclarationProgression,
    DeclarationRoutePlan,
    DeclarationReceipt,
    DeclarationEnvelope,
}

impl ForgeQueryBindingTargetKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::IntentDeclaration => "intent-declaration",
            Self::AdmittedIntentPlan => "admitted-intent-plan",
            Self::LowerRuntimeBoundaryEnvelope => "lower-runtime-boundary-envelope",
            Self::AdmittedDeclarationProgression => "admitted-declaration-progression",
            Self::DeclarationRoutePlan => "declaration-route-plan",
            Self::DeclarationReceipt => "declaration-receipt",
            Self::DeclarationEnvelope => "declaration-envelope",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryBindingTarget {
    kind: ForgeQueryBindingTargetKind,
    target_digest: String,
    binding_digest: String,
    semantics: ForgeQueryBindingTargetSemantics,
}

impl ForgeQueryBindingTarget {
    pub fn kind(&self) -> ForgeQueryBindingTargetKind {
        self.kind
    }

    pub fn target_digest(&self) -> &str {
        &self.target_digest
    }

    pub fn binding_digest(&self) -> &str {
        &self.binding_digest
    }

    pub fn target_identity(&self) -> ForgeQueryEvidenceIdentity {
        ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
            .field_shape(
                ForgeQueryEvidenceTag::new("identity_family"),
                "forge_query_binding_target_target_v1",
            )
            .field_shape(ForgeQueryEvidenceTag::new("kind"), self.kind.as_str())
            .field_value(ForgeQueryEvidenceTag::new("target"), &self.target_digest)
            .seal()
    }

    pub fn binding_identity(&self) -> ForgeQueryEvidenceIdentity {
        ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
            .field_shape(
                ForgeQueryEvidenceTag::new("identity_family"),
                "forge_query_binding_target_binding_v1",
            )
            .field_shape(ForgeQueryEvidenceTag::new("kind"), self.kind.as_str())
            .field_evidence_identity(
                ForgeQueryEvidenceTag::new("target"),
                &self.target_identity(),
            )
            .field_value(ForgeQueryEvidenceTag::new("binding"), &self.binding_digest)
            .seal()
    }

    pub fn semantics(&self) -> &ForgeQueryBindingTargetSemantics {
        &self.semantics
    }

    pub(crate) fn new(
        kind: ForgeQueryBindingTargetKind,
        target_digest: String,
        semantics: ForgeQueryBindingTargetSemantics,
    ) -> Self {
        let binding_digest = hash_parts(&[
            "forge_query_binding_target_v1".to_string(),
            format!("kind:{}", kind.as_str()),
            format!("target:{target_digest}"),
            format!("semantics:{}", semantics.binding_digest_material()),
        ]);
        Self {
            kind,
            target_digest,
            binding_digest,
            semantics,
        }
    }
}

pub(crate) mod sealed {
    pub trait Sealed {}
}

pub trait ForgeQueryBindingTargetWitness: Clone + sealed::Sealed {
    fn erased_target(&self) -> &ForgeQueryBindingTarget;
    fn into_erased_target(self) -> ForgeQueryBindingTarget;

    fn kind(&self) -> ForgeQueryBindingTargetKind {
        self.erased_target().kind()
    }

    fn target_digest(&self) -> &str {
        self.erased_target().target_digest()
    }

    fn binding_digest(&self) -> &str {
        self.erased_target().binding_digest()
    }

    fn target_identity(&self) -> ForgeQueryEvidenceIdentity {
        self.erased_target().target_identity()
    }

    fn binding_identity(&self) -> ForgeQueryEvidenceIdentity {
        self.erased_target().binding_identity()
    }

    fn semantics(&self) -> &ForgeQueryBindingTargetSemantics {
        self.erased_target().semantics()
    }
}
