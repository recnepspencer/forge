use crate::facade::{BridgeWritebackIdempotenceBasis, BridgeWritebackReplayBundle};
use crate::writeback::BridgeDerivedWritebackEffect;

pub(in crate::harness::adapter::adapter_impl) struct ReplayLoopFamilyEvidence {
    effect: BridgeDerivedWritebackEffect,
    idempotence: BridgeWritebackIdempotenceBasis,
    replay_bundle: BridgeWritebackReplayBundle,
}

impl ReplayLoopFamilyEvidence {
    pub(super) fn from_family_evidence(
        effect: &BridgeDerivedWritebackEffect,
        idempotence: &BridgeWritebackIdempotenceBasis,
        bundle: &BridgeWritebackReplayBundle,
    ) -> Self {
        Self {
            effect: effect.clone(),
            idempotence: idempotence.clone(),
            replay_bundle: bundle.clone(),
        }
    }

    pub(in crate::harness::adapter::adapter_impl) fn effect(
        &self,
    ) -> &BridgeDerivedWritebackEffect {
        &self.effect
    }

    pub(in crate::harness::adapter::adapter_impl) fn idempotence(
        &self,
    ) -> &BridgeWritebackIdempotenceBasis {
        &self.idempotence
    }

    pub(in crate::harness::adapter::adapter_impl) fn replay_bundle(
        &self,
    ) -> &BridgeWritebackReplayBundle {
        &self.replay_bundle
    }

    pub(in crate::harness::adapter::adapter_impl) fn writeback_effect_artifact_digest(
        &self,
    ) -> &str {
        self.effect.digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn effect_intent_digest(&self) -> &str {
        self.effect.effect_intent_digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn effect_intent_patch_canonical_basis(
        &self,
    ) -> &str {
        self.effect.effect_intent().patch_canonical_basis()
    }

    pub(in crate::harness::adapter::adapter_impl) fn mapped_input_digest(&self) -> &str {
        self.effect.mapped_input_digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn causality_digest(&self) -> &str {
        self.effect.causality_digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn idempotence_digest(&self) -> &str {
        self.idempotence.digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn replay_bundle_digest(&self) -> &str {
        self.replay_bundle.digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn replay_semantic_digest(&self) -> &str {
        self.replay_bundle.semantic_digest()
    }
}
