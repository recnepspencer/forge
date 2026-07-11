use crate::{
    RoadmapScope, StableDigest, StoreContractError, StoreContractResult,
    StorePhysicalAuthorityWitness, ROADMAP_2_S1_SCOPE,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum S0HandoffArtifactKind {
    BackendCapabilityMatrix,
    DeferredPhysicalGuaranteeMap,
    HarnessMaturityReport,
    TerminologyRiskReport,
    AuditInputManifest,
    ComplexityContractSummary,
    EvidenceProvenanceReport,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HandoffEvidenceDigestSet {
    backend_capability_matrix: StableDigest,
    deferred_physical_guarantee_map: StableDigest,
    harness_maturity_report: StableDigest,
    terminology_risk_report: StableDigest,
    audit_input_manifest: StableDigest,
    complexity_contract_summary: StableDigest,
    evidence_provenance_report: StableDigest,
}

impl HandoffEvidenceDigestSet {
    pub fn new(
        backend_capability_matrix: StableDigest,
        deferred_physical_guarantee_map: StableDigest,
        harness_maturity_report: StableDigest,
        terminology_risk_report: StableDigest,
        audit_input_manifest: StableDigest,
        complexity_contract_summary: StableDigest,
        evidence_provenance_report: StableDigest,
    ) -> Self {
        Self {
            backend_capability_matrix,
            deferred_physical_guarantee_map,
            harness_maturity_report,
            terminology_risk_report,
            audit_input_manifest,
            complexity_contract_summary,
            evidence_provenance_report,
        }
    }

    pub fn digest_for(&self, kind: S0HandoffArtifactKind) -> &StableDigest {
        match kind {
            S0HandoffArtifactKind::BackendCapabilityMatrix => &self.backend_capability_matrix,
            S0HandoffArtifactKind::DeferredPhysicalGuaranteeMap => {
                &self.deferred_physical_guarantee_map
            }
            S0HandoffArtifactKind::HarnessMaturityReport => &self.harness_maturity_report,
            S0HandoffArtifactKind::TerminologyRiskReport => &self.terminology_risk_report,
            S0HandoffArtifactKind::AuditInputManifest => &self.audit_input_manifest,
            S0HandoffArtifactKind::ComplexityContractSummary => &self.complexity_contract_summary,
            S0HandoffArtifactKind::EvidenceProvenanceReport => &self.evidence_provenance_report,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AcceptedHandoffReadiness {
    scope: RoadmapScope,
    evidence_digests: HandoffEvidenceDigestSet,
}

impl AcceptedHandoffReadiness {
    pub fn from_foundational_handoff_artifacts(
        scope: RoadmapScope,
        evidence_digests: HandoffEvidenceDigestSet,
    ) -> StoreContractResult<Self> {
        if scope != ROADMAP_2_S1_SCOPE {
            return Err(StoreContractError::HandoffScopeMismatch);
        }
        Ok(Self {
            scope,
            evidence_digests,
        })
    }

    pub const fn scope(&self) -> RoadmapScope {
        self.scope
    }

    pub fn evidence_digests(&self) -> &HandoffEvidenceDigestSet {
        &self.evidence_digests
    }

    pub fn physical_authority_scope(&self) -> StoreContractResult<StorePhysicalAuthorityWitness> {
        StorePhysicalAuthorityWitness::for_physical_format_vocabulary(self.scope)
    }
}
