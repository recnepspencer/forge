use super::{BlobCloseoutProofSummary, BlobCloseoutSources};
use forge_store_physical_certification::{
    FoundationalPhysicalCertificationEvidenceBundle, PhysicalCertificationEvidenceBundle,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobCloseoutEvidenceBundle {
    executed_sources: BlobCloseoutSources,
    foundational_evidence: FoundationalPhysicalCertificationEvidenceBundle,
}

pub fn materialize_blob_closeout_evidence(
    executed_sources: BlobCloseoutSources,
) -> BlobCloseoutEvidenceBundle {
    let foundational_evidence =
        materialize_foundational_evidence(executed_sources.evidence_bundle());
    BlobCloseoutEvidenceBundle {
        executed_sources,
        foundational_evidence,
    }
}

impl BlobCloseoutEvidenceBundle {
    pub const fn executed_sources(&self) -> &BlobCloseoutSources {
        &self.executed_sources
    }

    pub const fn foundational_evidence(&self) -> &FoundationalPhysicalCertificationEvidenceBundle {
        &self.foundational_evidence
    }

    pub const fn proof_summary(&self) -> BlobCloseoutProofSummary {
        self.executed_sources.proof_summary()
    }
}

fn materialize_foundational_evidence(
    evidence_bundle: &PhysicalCertificationEvidenceBundle,
) -> FoundationalPhysicalCertificationEvidenceBundle {
    evidence_bundle.materialize_foundational_evidence()
}
