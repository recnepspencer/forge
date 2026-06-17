use crate::runtime::{ForgeQueryBridgeMutationArtifactIdentity, ForgeQueryMutationEvidenceDigest};
use forge_runtime_bridge::facade::{
    BridgeMutationAuthorityBundle, BridgeWritebackFailureClass, BridgeWritebackOutcomeClass,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryMutationProvenanceEvidence {
    contract_digest: ForgeQueryMutationEvidenceDigest,
    writeback_effect_artifact_digest: ForgeQueryMutationEvidenceDigest,
    effect_intent_digest: ForgeQueryMutationEvidenceDigest,
    effect_intent_patch_canonical_basis: ForgeQueryMutationEvidenceDigest,
    feedback_provenance_digest: ForgeQueryMutationEvidenceDigest,
    causality_digest: ForgeQueryMutationEvidenceDigest,
    strategy_descriptor_digest: ForgeQueryMutationEvidenceDigest,
    execution_record_digest: ForgeQueryMutationEvidenceDigest,
    outcome_class: Option<BridgeWritebackOutcomeClass>,
    authoritative_artifact_digest: Option<ForgeQueryMutationEvidenceDigest>,
    request_digest: Option<ForgeQueryMutationEvidenceDigest>,
    receipt_digest: Option<ForgeQueryMutationEvidenceDigest>,
    failure_class: Option<BridgeWritebackFailureClass>,
}

impl ForgeQueryMutationProvenanceEvidence {
    pub(in crate::runtime) fn from_bridge(bundle: &BridgeMutationAuthorityBundle) -> Self {
        let provenance = bundle.provenance();
        Self {
            contract_digest: imported_artifact("provenance-contract", provenance.contract_digest()),
            writeback_effect_artifact_digest: imported_artifact(
                "provenance-writeback-effect-artifact",
                provenance.writeback_effect_artifact_digest(),
            ),
            effect_intent_digest: imported_artifact(
                "provenance-effect-intent",
                provenance.effect_intent_digest(),
            ),
            effect_intent_patch_canonical_basis: imported_artifact(
                "provenance-effect-intent-patch-canonical-basis",
                provenance.effect_intent_patch_canonical_basis(),
            ),
            feedback_provenance_digest: imported_artifact(
                "provenance-feedback",
                provenance.feedback_provenance_digest(),
            ),
            causality_digest: imported_artifact(
                "provenance-causality",
                provenance.causality_digest(),
            ),
            strategy_descriptor_digest: imported_artifact(
                "provenance-strategy-descriptor",
                provenance.strategy_descriptor_digest(),
            ),
            execution_record_digest: imported_artifact(
                "provenance-execution-record",
                provenance.execution_record_digest(),
            ),
            outcome_class: provenance.outcome_class(),
            authoritative_artifact_digest: provenance
                .authoritative_artifact_digest()
                .map(|digest| imported_artifact("provenance-authoritative-artifact", digest)),
            request_digest: provenance
                .request_digest()
                .map(|digest| imported_artifact("provenance-request", digest)),
            receipt_digest: provenance
                .receipt_digest()
                .map(|digest| imported_artifact("provenance-receipt", digest)),
            failure_class: provenance.failure_class(),
        }
    }

    pub fn contract_digest(&self) -> &ForgeQueryMutationEvidenceDigest {
        &self.contract_digest
    }

    pub fn writeback_effect_artifact_digest(&self) -> &ForgeQueryMutationEvidenceDigest {
        &self.writeback_effect_artifact_digest
    }

    pub fn effect_intent_digest(&self) -> &ForgeQueryMutationEvidenceDigest {
        &self.effect_intent_digest
    }

    pub fn effect_intent_patch_canonical_basis(&self) -> &ForgeQueryMutationEvidenceDigest {
        &self.effect_intent_patch_canonical_basis
    }

    pub fn feedback_provenance_digest(&self) -> &ForgeQueryMutationEvidenceDigest {
        &self.feedback_provenance_digest
    }

    pub fn causality_digest(&self) -> &ForgeQueryMutationEvidenceDigest {
        &self.causality_digest
    }

    pub fn strategy_descriptor_digest(&self) -> &ForgeQueryMutationEvidenceDigest {
        &self.strategy_descriptor_digest
    }

    pub fn execution_record_digest(&self) -> &ForgeQueryMutationEvidenceDigest {
        &self.execution_record_digest
    }

    pub fn outcome_class(&self) -> Option<BridgeWritebackOutcomeClass> {
        self.outcome_class
    }

    pub fn authoritative_artifact_digest(&self) -> Option<&ForgeQueryMutationEvidenceDigest> {
        self.authoritative_artifact_digest.as_ref()
    }

    pub fn request_digest(&self) -> Option<&ForgeQueryMutationEvidenceDigest> {
        self.request_digest.as_ref()
    }

    pub fn receipt_digest(&self) -> Option<&ForgeQueryMutationEvidenceDigest> {
        self.receipt_digest.as_ref()
    }

    pub fn failure_class(&self) -> Option<BridgeWritebackFailureClass> {
        self.failure_class
    }

    #[cfg(test)]
    pub(crate) fn test_only(execution_record_digest: impl Into<String>) -> Self {
        Self {
            contract_digest: imported_artifact("provenance-contract", "contract:test"),
            writeback_effect_artifact_digest: imported_artifact(
                "provenance-writeback-effect-artifact",
                "writeback-effect:test",
            ),
            effect_intent_digest: imported_artifact(
                "provenance-effect-intent",
                "effect-intent:test",
            ),
            effect_intent_patch_canonical_basis: imported_artifact(
                "provenance-effect-intent-patch-canonical-basis",
                "effect-intent-patch-basis:test",
            ),
            feedback_provenance_digest: imported_artifact("provenance-feedback", "feedback:test"),
            causality_digest: imported_artifact("provenance-causality", "causality:test"),
            strategy_descriptor_digest: imported_artifact(
                "provenance-strategy-descriptor",
                "strategy:test",
            ),
            execution_record_digest: imported_artifact(
                "provenance-execution-record",
                execution_record_digest,
            ),
            outcome_class: None,
            authoritative_artifact_digest: None,
            request_digest: None,
            receipt_digest: None,
            failure_class: None,
        }
    }
}

fn imported_artifact(
    role: &'static str,
    artifact: impl Into<String>,
) -> ForgeQueryMutationEvidenceDigest {
    let artifact = ForgeQueryBridgeMutationArtifactIdentity::imported(role, artifact);
    ForgeQueryMutationEvidenceDigest::source_identity(role, artifact.evidence_identity())
}
