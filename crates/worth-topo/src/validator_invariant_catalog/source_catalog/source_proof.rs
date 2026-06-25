use crate::validator_invariant_catalog::{
    WorthTopologyEnforcementPhase, WorthTopologyWitnessPosture,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthTopologyLegalityFamilySourceAuthorityKind {
    ValidatorRuleSpec,
    RuntimeInvariantRegistration,
}

impl WorthTopologyLegalityFamilySourceAuthorityKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ValidatorRuleSpec => "validator-rule-spec",
            Self::RuntimeInvariantRegistration => "runtime-invariant-registration",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTopologyLegalityFamilySourceProof {
    authority_kind: WorthTopologyLegalityFamilySourceAuthorityKind,
    source_identity_digest: String,
    rule_name: String,
    semantic_version: String,
    execution_point: Option<String>,
    applicability_digest: String,
    enforcement_phase: WorthTopologyEnforcementPhase,
    witness_posture: WorthTopologyWitnessPosture,
    proof_digest: String,
}

impl WorthTopologyLegalityFamilySourceProof {
    pub(in crate::validator_invariant_catalog) fn new(
        input: WorthTopologyLegalityFamilySourceProofInput,
    ) -> Self {
        let proof_digest = [
            "worth-topo-legality-family-source-proof-v1",
            input.authority_kind.as_str(),
            input.source_identity_digest.as_str(),
            input.rule_name.as_str(),
            input.semantic_version.as_str(),
            input.execution_point.as_deref().unwrap_or("none"),
            input.applicability_digest.as_str(),
            input.enforcement_phase.as_str(),
            input.witness_posture.as_str(),
        ]
        .join("|");
        Self {
            authority_kind: input.authority_kind,
            source_identity_digest: input.source_identity_digest,
            rule_name: input.rule_name,
            semantic_version: input.semantic_version,
            execution_point: input.execution_point,
            applicability_digest: input.applicability_digest,
            enforcement_phase: input.enforcement_phase,
            witness_posture: input.witness_posture,
            proof_digest,
        }
    }

    pub const fn authority_kind(&self) -> WorthTopologyLegalityFamilySourceAuthorityKind {
        self.authority_kind
    }

    pub fn source_identity_digest(&self) -> &str {
        &self.source_identity_digest
    }

    pub fn rule_name(&self) -> &str {
        &self.rule_name
    }

    pub fn semantic_version(&self) -> &str {
        &self.semantic_version
    }

    pub fn execution_point(&self) -> Option<&str> {
        self.execution_point.as_deref()
    }

    pub fn applicability_digest(&self) -> &str {
        &self.applicability_digest
    }

    pub const fn enforcement_phase(&self) -> WorthTopologyEnforcementPhase {
        self.enforcement_phase
    }

    pub const fn witness_posture(&self) -> WorthTopologyWitnessPosture {
        self.witness_posture
    }

    pub fn proof_digest(&self) -> &str {
        &self.proof_digest
    }
}

pub(in crate::validator_invariant_catalog) struct WorthTopologyLegalityFamilySourceProofInput {
    pub authority_kind: WorthTopologyLegalityFamilySourceAuthorityKind,
    pub source_identity_digest: String,
    pub rule_name: String,
    pub semantic_version: String,
    pub execution_point: Option<String>,
    pub applicability_digest: String,
    pub enforcement_phase: WorthTopologyEnforcementPhase,
    pub witness_posture: WorthTopologyWitnessPosture,
}
