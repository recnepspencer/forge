use crate::facade::BridgeWritebackReplayBundle;
use crate::writeback::{BridgeDerivedWritebackEffect, BridgeWritebackExecutionRecord};

pub(in crate::harness::adapter::adapter_impl) struct FamilyExtensionSameFamilyEquivalence {
    projected_effect: BridgeDerivedWritebackEffect,
    projected_bundle: BridgeWritebackReplayBundle,
    rebuilt_projected_effect: BridgeDerivedWritebackEffect,
    rebuilt_projected_bundle: BridgeWritebackReplayBundle,
    rebuilt_execution_record: BridgeWritebackExecutionRecord,
}

impl FamilyExtensionSameFamilyEquivalence {
    pub(in crate::harness::adapter::adapter_impl::writeback_certification::family_extension) fn from_rebuilt_family(
        projected_effect: &BridgeDerivedWritebackEffect,
        projected_bundle: &BridgeWritebackReplayBundle,
        rebuilt_projected_effect: &BridgeDerivedWritebackEffect,
        rebuilt_projected_bundle: &BridgeWritebackReplayBundle,
        rebuilt_execution_record: &BridgeWritebackExecutionRecord,
    ) -> Self {
        Self {
            projected_effect: projected_effect.clone(),
            projected_bundle: projected_bundle.clone(),
            rebuilt_projected_effect: rebuilt_projected_effect.clone(),
            rebuilt_projected_bundle: rebuilt_projected_bundle.clone(),
            rebuilt_execution_record: rebuilt_execution_record.clone(),
        }
    }

    pub(in crate::harness::adapter::adapter_impl) fn projected_effect(
        &self,
    ) -> &BridgeDerivedWritebackEffect {
        &self.projected_effect
    }

    pub(in crate::harness::adapter::adapter_impl) fn rebuilt_projected_effect(
        &self,
    ) -> &BridgeDerivedWritebackEffect {
        &self.rebuilt_projected_effect
    }

    pub(in crate::harness::adapter::adapter_impl) fn projected_bundle(
        &self,
    ) -> &BridgeWritebackReplayBundle {
        &self.projected_bundle
    }

    pub(in crate::harness::adapter::adapter_impl) fn rebuilt_projected_bundle(
        &self,
    ) -> &BridgeWritebackReplayBundle {
        &self.rebuilt_projected_bundle
    }

    pub(in crate::harness::adapter::adapter_impl) fn rebuilt_execution_record(
        &self,
    ) -> &BridgeWritebackExecutionRecord {
        &self.rebuilt_execution_record
    }

    pub(in crate::harness::adapter::adapter_impl) fn semantic_digest_equal(&self) -> bool {
        self.projected_bundle.semantic_digest() == self.rebuilt_projected_bundle.semantic_digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn bundle_digest_equal(&self) -> bool {
        self.projected_bundle.digest() == self.rebuilt_projected_bundle.digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn effect_intent_digest_equal(&self) -> bool {
        self.projected_effect.effect_intent_digest()
            == self.rebuilt_projected_effect.effect_intent_digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn mapped_input_digest_equal(&self) -> bool {
        self.projected_effect.mapped_input_digest()
            == self.rebuilt_projected_effect.mapped_input_digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn family_execution_record_digest(&self) -> &str {
        self.rebuilt_execution_record.digest()
    }
}
