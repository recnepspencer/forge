use std::sync::Arc;

use worth_foundational::facade::{
    prepare_aspect_patch_for_canonical_basis, validate_aspect_value, AspectContract,
    AspectContractRevision, AspectIdentity, AspectKey, AspectValue,
    AuthoritativePatchConstructionDenial, AuthoritativeRecordAspectPatch,
    CanonicalBasisConstructionDenial, CanonicalBasisReadyArtifact, CanonicalizationRuleVersion,
    ContractValidationDenial,
};
use worth_proof::TransitionOutcome;
use sha2::{Digest, Sha256};

use crate::canonical_basis::canonical_basis_ready_text;

use super::super::BridgeWritebackEffectClass;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BridgeWritebackEffectIntentError {
    ContractValidation(ContractValidationDenial),
    PatchConstruction(AuthoritativePatchConstructionDenial),
    CanonicalBasis(CanonicalBasisConstructionDenial),
    CanonicalBasisTextUnsupported,
}

impl std::fmt::Display for BridgeWritebackEffectIntentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ContractValidation(error) => write!(f, "{error:?}"),
            Self::PatchConstruction(error) => write!(f, "{error:?}"),
            Self::CanonicalBasis(error) => write!(f, "{error:?}"),
            Self::CanonicalBasisTextUnsupported => {
                write!(
                    f,
                    "unsupported foundational canonical basis text for writeback effect intent"
                )
            }
        }
    }
}

impl std::error::Error for BridgeWritebackEffectIntentError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeWritebackEffectIntent {
    effect_class: BridgeWritebackEffectClass,
    authoritative_patch: AuthoritativeRecordAspectPatch,
    patch_canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeWritebackEffectIntent {
    pub fn from_authoritative_patch(
        effect_class: BridgeWritebackEffectClass,
        authoritative_patch: AuthoritativeRecordAspectPatch,
    ) -> Result<Self, BridgeWritebackEffectIntentError> {
        let ready = match prepare_aspect_patch_for_canonical_basis(
            effect_intent_rule_version(),
            &authoritative_patch,
        ) {
            TransitionOutcome::Success(ready) => ready,
            TransitionOutcome::Denied(denial) => {
                return Err(BridgeWritebackEffectIntentError::CanonicalBasis(denial));
            }
            TransitionOutcome::Deferred(never)
            | TransitionOutcome::Stale(never)
            | TransitionOutcome::RebindRequired(never)
            | TransitionOutcome::Failed(never) => match never {},
        };
        let patch_canonical_basis = Arc::<str>::from(canonical_basis_text(&ready)?);
        let digest = digest_effect_intent(effect_class, patch_canonical_basis.as_ref());

        Ok(Self {
            effect_class,
            authoritative_patch,
            patch_canonical_basis,
            digest,
        })
    }

    pub fn validated_scalar_patch(
        effect_class: BridgeWritebackEffectClass,
        key: AspectKey,
        value: AspectValue,
    ) -> Result<Self, BridgeWritebackEffectIntentError> {
        let contract = AspectContract::scalar(
            key,
            AspectIdentity(1),
            AspectContractRevision(1),
            value.value_family(),
        );
        let validated = match validate_aspect_value(&contract, value.into()) {
            TransitionOutcome::Success(validated) => validated,
            TransitionOutcome::Denied(denial) => {
                return Err(BridgeWritebackEffectIntentError::ContractValidation(denial));
            }
            TransitionOutcome::Deferred(never)
            | TransitionOutcome::Stale(never)
            | TransitionOutcome::RebindRequired(never)
            | TransitionOutcome::Failed(never) => match never {},
        };
        let patch = match AuthoritativeRecordAspectPatch::whole_aspect([validated], []) {
            TransitionOutcome::Success(patch) => patch,
            TransitionOutcome::Denied(denial) => {
                return Err(BridgeWritebackEffectIntentError::PatchConstruction(denial));
            }
            TransitionOutcome::Deferred(never)
            | TransitionOutcome::Stale(never)
            | TransitionOutcome::RebindRequired(never)
            | TransitionOutcome::Failed(never) => match never {},
        };

        Self::from_authoritative_patch(effect_class, patch)
    }

    pub fn effect_class(&self) -> BridgeWritebackEffectClass {
        self.effect_class
    }

    pub fn authoritative_patch(&self) -> &AuthoritativeRecordAspectPatch {
        &self.authoritative_patch
    }

    pub fn patch_canonical_basis(&self) -> &str {
        self.patch_canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

fn effect_intent_rule_version() -> CanonicalizationRuleVersion {
    CanonicalizationRuleVersion::new("bridge.writeback.effect-intent.v1")
        .expect("static writeback effect intent canonicalization rule version is valid")
}

fn canonical_basis_text(
    ready: &CanonicalBasisReadyArtifact,
) -> Result<String, BridgeWritebackEffectIntentError> {
    canonical_basis_ready_text(ready)
        .map_err(|_| BridgeWritebackEffectIntentError::CanonicalBasisTextUnsupported)
}

fn digest_effect_intent(
    effect_class: BridgeWritebackEffectClass,
    patch_canonical_basis: &str,
) -> Arc<str> {
    let canonical_basis = format!(
        "bridge-writeback-effect-intent|effect-class:{effect_class:?}|patch-basis={patch_canonical_basis}"
    );
    let digest = Sha256::digest(canonical_basis.as_bytes());
    Arc::from(format!("bridge-writeback-effect-intent:sha256:{digest:x}"))
}
