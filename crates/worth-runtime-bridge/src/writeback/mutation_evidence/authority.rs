use super::{
    existing_truth::BridgeExistingTruthBindingBundle,
    provenance::{BridgeMutationCausalityBundle, BridgeMutationProvenanceBundle},
};
use crate::writeback::{
    BridgeContinuityMutationBundle, BridgeDerivedWritebackEffect, BridgeNamingMutationBundle,
    BridgeSymbolicTargetReferenceBundle, BridgeWritebackAuthorityOutcome,
    BridgeWritebackExecutionRecord, BridgeWritebackFeedbackProvenance,
    BridgeWritebackNativeCausalityInputs, BridgeWritebackOutcomeClass,
};

/// One bridge-authored carry-forward packet suitable for Query receipt lowering.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeMutationAuthorityBundle {
    causality: BridgeMutationCausalityBundle,
    provenance: BridgeMutationProvenanceBundle,
    existing_truth_binding: Option<BridgeExistingTruthBindingBundle>,
    symbolic_target_reference: Option<BridgeSymbolicTargetReferenceBundle>,
    naming_mutation: Option<BridgeNamingMutationBundle>,
    continuity_mutation: Option<BridgeContinuityMutationBundle>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BridgeMutationAuthorityBundleError {
    MissingMutationSubject,
    MutationSubjectEffectIntentMismatch,
    CausalityEffectMismatch,
    FeedbackEffectMismatch,
    ExecutionRecordEffectMismatch,
    ExecutionRecordOutcomeMismatch,
    NonAuthoritativeOutcome,
}

impl std::fmt::Display for BridgeMutationAuthorityBundleError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingMutationSubject => formatter
                .write_str("bridge mutation authority requires a retained mutation subject"),
            Self::MutationSubjectEffectIntentMismatch => formatter.write_str(
                "bridge mutation subject patch does not match the executed effect intent",
            ),
            Self::CausalityEffectMismatch => {
                formatter.write_str("bridge mutation causality does not match the executed effect")
            }
            Self::FeedbackEffectMismatch => formatter.write_str(
                "bridge writeback feedback provenance does not match the executed effect",
            ),
            Self::ExecutionRecordEffectMismatch => formatter
                .write_str("bridge writeback execution record does not match the executed effect"),
            Self::ExecutionRecordOutcomeMismatch => formatter.write_str(
                "bridge writeback execution record does not match the authority outcome",
            ),
            Self::NonAuthoritativeOutcome => formatter
                .write_str("bridge mutation authority requires an authoritative commit outcome"),
        }
    }
}

impl std::error::Error for BridgeMutationAuthorityBundleError {}

pub(crate) struct SuccessfulWritebackArtifactChain<'a> {
    pub(crate) causality: &'a BridgeWritebackNativeCausalityInputs,
    pub(crate) effect: &'a BridgeDerivedWritebackEffect,
    pub(crate) feedback: &'a BridgeWritebackFeedbackProvenance,
    pub(crate) execution_record: &'a BridgeWritebackExecutionRecord,
    pub(crate) outcome: &'a BridgeWritebackAuthorityOutcome,
}

impl BridgeMutationAuthorityBundle {
    pub(crate) fn from_successful_writeback_artifacts(
        artifacts: SuccessfulWritebackArtifactChain<'_>,
    ) -> Result<Self, BridgeMutationAuthorityBundleError> {
        validate_successful_artifact_chain(&artifacts)?;
        Ok(Self {
            causality: BridgeMutationCausalityBundle::from_writeback_causality(artifacts.causality),
            provenance: BridgeMutationProvenanceBundle::from_writeback_artifacts(
                artifacts.effect,
                artifacts.feedback,
                artifacts.execution_record,
                Some(artifacts.outcome),
            ),
            existing_truth_binding: None,
            symbolic_target_reference: None,
            naming_mutation: None,
            continuity_mutation: None,
        })
    }

    pub fn causality(&self) -> &BridgeMutationCausalityBundle {
        &self.causality
    }

    pub fn provenance(&self) -> &BridgeMutationProvenanceBundle {
        &self.provenance
    }

    pub fn existing_truth_binding(&self) -> Option<&BridgeExistingTruthBindingBundle> {
        self.existing_truth_binding.as_ref()
    }

    pub fn with_existing_truth_binding(
        mut self,
        binding: BridgeExistingTruthBindingBundle,
    ) -> Self {
        self.existing_truth_binding = Some(binding);
        self
    }

    pub fn symbolic_target_reference(&self) -> Option<&BridgeSymbolicTargetReferenceBundle> {
        self.symbolic_target_reference.as_ref()
    }

    pub fn with_symbolic_target_reference(
        mut self,
        reference: BridgeSymbolicTargetReferenceBundle,
    ) -> Self {
        self.symbolic_target_reference = Some(reference);
        self
    }

    pub fn naming_mutation(&self) -> Option<&BridgeNamingMutationBundle> {
        self.naming_mutation.as_ref()
    }

    pub fn with_naming_mutation(mut self, naming: BridgeNamingMutationBundle) -> Self {
        self.naming_mutation = Some(naming);
        self
    }

    pub fn continuity_mutation(&self) -> Option<&BridgeContinuityMutationBundle> {
        self.continuity_mutation.as_ref()
    }

    pub fn with_continuity_mutation(mut self, continuity: BridgeContinuityMutationBundle) -> Self {
        self.continuity_mutation = Some(continuity);
        self
    }
}

fn validate_successful_artifact_chain(
    artifacts: &SuccessfulWritebackArtifactChain<'_>,
) -> Result<(), BridgeMutationAuthorityBundleError> {
    let subject = artifacts
        .causality
        .mutation_subject()
        .ok_or(BridgeMutationAuthorityBundleError::MissingMutationSubject)?;
    if !subject.matches_effect_intent(artifacts.effect.effect_intent()) {
        return Err(BridgeMutationAuthorityBundleError::MutationSubjectEffectIntentMismatch);
    }
    if artifacts.causality.digest() != artifacts.effect.causality_digest() {
        return Err(BridgeMutationAuthorityBundleError::CausalityEffectMismatch);
    }
    if !feedback_matches_effect(artifacts.feedback, artifacts.effect) {
        return Err(BridgeMutationAuthorityBundleError::FeedbackEffectMismatch);
    }
    if !execution_record_matches_effect(artifacts.execution_record, artifacts.effect) {
        return Err(BridgeMutationAuthorityBundleError::ExecutionRecordEffectMismatch);
    }
    if artifacts.outcome.outcome_class() != BridgeWritebackOutcomeClass::AuthoritativeCommit {
        return Err(BridgeMutationAuthorityBundleError::NonAuthoritativeOutcome);
    }
    if !execution_record_matches_outcome(artifacts.execution_record, artifacts.outcome) {
        return Err(BridgeMutationAuthorityBundleError::ExecutionRecordOutcomeMismatch);
    }
    Ok(())
}

fn feedback_matches_effect(
    feedback: &BridgeWritebackFeedbackProvenance,
    effect: &BridgeDerivedWritebackEffect,
) -> bool {
    feedback.contract_digest() == effect.contract_digest()
        && feedback.writeback_effect_artifact_digest() == effect.digest()
        && feedback.family_kind() == effect.family_kind()
        && feedback.effect_class() == effect.effect_class()
        && feedback.effect_intent_digest() == effect.effect_intent_digest()
        && feedback.effect_intent_patch_canonical_basis()
            == effect.effect_intent().patch_canonical_basis()
        && feedback.causality_digest() == effect.causality_digest()
        && feedback.strategy_class() == effect.strategy_class()
        && feedback.strategy_descriptor_basis() == effect.strategy_descriptor_basis()
}

fn execution_record_matches_effect(
    execution_record: &BridgeWritebackExecutionRecord,
    effect: &BridgeDerivedWritebackEffect,
) -> bool {
    execution_record.contract_digest() == effect.contract_digest()
        && execution_record.writeback_effect_artifact_digest() == effect.digest()
        && execution_record.effect_intent_digest() == effect.effect_intent_digest()
        && execution_record.effect_intent_patch_canonical_basis()
            == effect.effect_intent().patch_canonical_basis()
        && execution_record.family_kind() == effect.family_kind()
        && execution_record.strategy_class() == effect.strategy_class()
        && execution_record.causality_digest() == effect.causality_digest()
}

fn execution_record_matches_outcome(
    execution_record: &BridgeWritebackExecutionRecord,
    outcome: &BridgeWritebackAuthorityOutcome,
) -> bool {
    execution_record.outcome_digest() == Some(outcome.digest())
        && execution_record.outcome_class() == Some(outcome.outcome_class())
        && execution_record.authority_request().is_some()
        && execution_record.authority_receipt().is_some()
        && execution_record.failure_class().is_none()
}
