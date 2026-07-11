use crate::{
    FoundationalPhysicalCertificationEvidenceBundle, PhysicalCertificationEvidenceBundle,
    S7CloseoutProofSummary, S7ExecutedCloseoutSources,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S7MaterializedCloseoutEvidenceBundle {
    executed_sources: S7ExecutedCloseoutSources,
    foundational_evidence: FoundationalPhysicalCertificationEvidenceBundle,
}

pub fn materialize_s7_closeout_evidence(
    executed_sources: S7ExecutedCloseoutSources,
) -> S7MaterializedCloseoutEvidenceBundle {
    let foundational_evidence =
        materialize_foundational_evidence(executed_sources.evidence_bundle());
    S7MaterializedCloseoutEvidenceBundle {
        executed_sources,
        foundational_evidence,
    }
}

impl S7MaterializedCloseoutEvidenceBundle {
    pub const fn executed_sources(&self) -> &S7ExecutedCloseoutSources {
        &self.executed_sources
    }

    pub const fn foundational_evidence(&self) -> &FoundationalPhysicalCertificationEvidenceBundle {
        &self.foundational_evidence
    }

    pub const fn proof_summary(&self) -> S7CloseoutProofSummary {
        self.executed_sources.proof_summary()
    }
}

fn materialize_foundational_evidence(
    evidence_bundle: &PhysicalCertificationEvidenceBundle,
) -> FoundationalPhysicalCertificationEvidenceBundle {
    evidence_bundle.materialize_foundational_evidence()
}
