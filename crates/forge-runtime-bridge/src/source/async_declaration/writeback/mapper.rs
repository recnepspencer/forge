use std::sync::Arc;

use forge_foundational::facade::{AspectValue, ContractValidatedAspectValueView};
use sha2::{Digest, Sha256};

use crate::identity::{AsyncWritebackMapperOutputIdentityTag, BridgeIdentity};
use crate::writeback::BridgeWritebackEffectIntent;

use super::{
    AdmittedBridgeAsyncWriteback, BridgeAsyncWritebackCounters, BridgeAsyncWritebackFamily,
    BridgeAsyncWritebackRejection, BridgeAsyncWritebackRejectionKind,
};

pub type BridgeAsyncWritebackMapperOutputIdentity =
    BridgeIdentity<AsyncWritebackMapperOutputIdentityTag>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeAsyncWritebackMapperOutput {
    mapper_output_identity: BridgeAsyncWritebackMapperOutputIdentity,
    admission_digest: Arc<str>,
    writeback_family: BridgeAsyncWritebackFamily,
    completion_identity: Arc<str>,
    effect_intent: BridgeWritebackEffectIntent,
    counters: BridgeAsyncWritebackCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeAsyncWritebackMapperOutput {
    pub(crate) fn map(
        admitted: &AdmittedBridgeAsyncWriteback,
    ) -> Result<Self, BridgeAsyncWritebackRejection> {
        validate_phase10_authoritative_mapper_shape(admitted.effect_intent())?;
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-async-writeback-mapper-output|admission={}|family={:?}|completion={}|effect-intent={}|effect-intent-basis={}",
            admitted.digest(),
            admitted.family(),
            admitted.completion().completion_identity(),
            admitted.effect_intent().digest(),
            admitted.effect_intent().patch_canonical_basis(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Ok(Self {
            mapper_output_identity: BridgeAsyncWritebackMapperOutputIdentity::new(format!(
                "bridge-async-writeback-mapper-output-id:sha256:{digest:x}"
            )),
            admission_digest: Arc::from(admitted.digest().to_owned()),
            writeback_family: admitted.family(),
            completion_identity: Arc::from(admitted.completion().completion_identity().to_owned()),
            effect_intent: admitted.effect_intent().clone(),
            counters: BridgeAsyncWritebackCounters::mapped(),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-async-writeback-mapper-output:sha256:{digest:x}"
            )),
        })
    }

    pub fn mapper_output_identity(&self) -> &BridgeAsyncWritebackMapperOutputIdentity {
        &self.mapper_output_identity
    }

    pub fn admission_digest(&self) -> &str {
        self.admission_digest.as_ref()
    }

    pub fn writeback_family(&self) -> BridgeAsyncWritebackFamily {
        self.writeback_family
    }

    pub fn completion_identity(&self) -> &str {
        self.completion_identity.as_ref()
    }

    pub fn effect_intent(&self) -> &BridgeWritebackEffectIntent {
        &self.effect_intent
    }

    pub fn counters(&self) -> &BridgeAsyncWritebackCounters {
        &self.counters
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

fn validate_phase10_authoritative_mapper_shape(
    effect_intent: &BridgeWritebackEffectIntent,
) -> Result<(), BridgeAsyncWritebackRejection> {
    for (aspect_key, validated_value) in effect_intent.authoritative_patch().whole_aspect_sets() {
        match validated_value.view() {
            ContractValidatedAspectValueView::Scalar(AspectValue::String(_)) => {}
            ContractValidatedAspectValueView::Scalar(value) => {
                return Err(BridgeAsyncWritebackRejection::new(
                    BridgeAsyncWritebackRejectionKind::MapperFailed,
                    format!(
                        "bridge async writeback mapper only admits string scalar projected-state-diff values in phase 10, but aspect `{}` carried scalar family `{:?}`",
                        aspect_key.as_str(),
                        value.value_family(),
                    ),
                ));
            }
            ContractValidatedAspectValueView::Struct(_) => {
                return Err(BridgeAsyncWritebackRejection::new(
                    BridgeAsyncWritebackRejectionKind::MapperFailed,
                    format!(
                        "bridge async writeback mapper does not admit struct projected-state-diff values for aspect `{}` in phase 10",
                        aspect_key.as_str(),
                    ),
                ));
            }
        }
    }

    Ok(())
}
