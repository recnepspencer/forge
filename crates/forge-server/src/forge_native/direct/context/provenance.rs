use forge_foundational::facade::{
    DiagnosticRichnessProfile, FoundationalBoundaryEvidenceFreshnessPosture,
    FoundationalBoundaryEvidenceLocality, FoundationalBoundaryEvidenceProvenanceArtifact,
    FoundationalBoundaryEvidenceSourceBasisKind,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerDirectProvenance {
    artifact: FoundationalBoundaryEvidenceProvenanceArtifact,
    diagnostics_profile: DiagnosticRichnessProfile,
    provenance_digest: String,
}

impl ForgeServerDirectProvenance {
    pub(crate) fn new(
        artifact: &FoundationalBoundaryEvidenceProvenanceArtifact,
        diagnostics_profile: DiagnosticRichnessProfile,
    ) -> Self {
        let provenance_digest = format!(
            "forge-server-direct-provenance-v1|locality:{}|freshness:{}|source:{}|authority:{}|strategy:{}|profile:{}|comparison:{}|canonical:{}|support_contexts:{}|diagnostics:{}",
            locality_label(artifact.locality()),
            freshness_label(artifact.freshness_posture()),
            source_kind_label(artifact.source_basis().kind()),
            artifact.authority_path().is_some(),
            artifact.strategy_basis().is_some(),
            artifact.profile_basis().is_some(),
            artifact.comparison_basis().is_some(),
            artifact.canonical_digest_basis().is_some(),
            artifact.support_context_attachments().len(),
            diagnostics_profile_label(diagnostics_profile),
        );
        Self {
            artifact: artifact.clone(),
            diagnostics_profile,
            provenance_digest,
        }
    }

    pub fn locality(&self) -> FoundationalBoundaryEvidenceLocality {
        self.artifact.locality()
    }

    pub fn freshness_posture(&self) -> FoundationalBoundaryEvidenceFreshnessPosture {
        self.artifact.freshness_posture()
    }

    pub fn source_basis_kind(&self) -> FoundationalBoundaryEvidenceSourceBasisKind {
        self.artifact.source_basis().kind()
    }

    pub fn has_authority_path(&self) -> bool {
        self.artifact.authority_path().is_some()
    }

    pub fn has_strategy_basis(&self) -> bool {
        self.artifact.strategy_basis().is_some()
    }

    pub fn has_profile_basis(&self) -> bool {
        self.artifact.profile_basis().is_some()
    }

    pub fn has_comparison_basis(&self) -> bool {
        self.artifact.comparison_basis().is_some()
    }

    pub fn has_canonical_digest_basis(&self) -> bool {
        self.artifact.canonical_digest_basis().is_some()
    }

    pub fn support_context_attachment_count(&self) -> usize {
        self.artifact.support_context_attachments().len()
    }

    pub fn diagnostics_profile(&self) -> DiagnosticRichnessProfile {
        self.diagnostics_profile
    }

    pub fn provenance_digest(&self) -> &str {
        &self.provenance_digest
    }
}

fn locality_label(locality: FoundationalBoundaryEvidenceLocality) -> &'static str {
    match locality {
        FoundationalBoundaryEvidenceLocality::Current => "current",
        FoundationalBoundaryEvidenceLocality::BranchLocal => "branch_local",
        FoundationalBoundaryEvidenceLocality::Historical => "historical",
        FoundationalBoundaryEvidenceLocality::ComparisonPaired => "comparison_paired",
        FoundationalBoundaryEvidenceLocality::SnapshotBound => "snapshot_bound",
        FoundationalBoundaryEvidenceLocality::ReplayDerived => "replay_derived",
        FoundationalBoundaryEvidenceLocality::RestoredReadmitted => "restored_readmitted",
    }
}

fn freshness_label(freshness: FoundationalBoundaryEvidenceFreshnessPosture) -> &'static str {
    match freshness {
        FoundationalBoundaryEvidenceFreshnessPosture::FreshRetained => "fresh_retained",
        FoundationalBoundaryEvidenceFreshnessPosture::StaleRetained => "stale_retained",
        FoundationalBoundaryEvidenceFreshnessPosture::ReducedRetained => "reduced_retained",
        FoundationalBoundaryEvidenceFreshnessPosture::ReconstructedFromReplay => {
            "reconstructed_from_replay"
        }
        FoundationalBoundaryEvidenceFreshnessPosture::RestoredFromCheckpoint => {
            "restored_from_checkpoint"
        }
    }
}

fn source_kind_label(kind: FoundationalBoundaryEvidenceSourceBasisKind) -> &'static str {
    match kind {
        FoundationalBoundaryEvidenceSourceBasisKind::BoundaryArtifact => "boundary_artifact",
        FoundationalBoundaryEvidenceSourceBasisKind::Transition => "transition",
    }
}

fn diagnostics_profile_label(profile: DiagnosticRichnessProfile) -> &'static str {
    match profile {
        DiagnosticRichnessProfile::OperationalMinimal => "operational_minimal",
        DiagnosticRichnessProfile::Standard => "standard",
        DiagnosticRichnessProfile::Forensic => "forensic",
    }
}
