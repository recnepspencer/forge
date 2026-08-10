use super::super::artifacts::S0ArtifactEnvelopeMetadata;
use super::super::counters::S0CounterSnapshot;
use super::aggregate::S0EvidenceBundle;
use super::certification::S0CertificationMatrixRow;
use super::provenance::S0AcceptedEvidenceBundleWitness;

impl S0EvidenceBundle {
    pub fn envelope(&self) -> &S0ArtifactEnvelopeMetadata {
        &self.envelope
    }

    pub fn certification_rows(&self) -> &[S0CertificationMatrixRow] {
        &self.certification_rows
    }

    pub fn counter_snapshot(&self) -> &S0CounterSnapshot {
        &self.counter_snapshot
    }

    pub fn witness(&self) -> S0AcceptedEvidenceBundleWitness {
        S0AcceptedEvidenceBundleWitness {
            source_revision: self.evidence_provenance.source_revision.clone(),
            audit_input_manifest_digest: self
                .evidence_provenance
                .audit_input_manifest_digest
                .clone(),
            evidence_bundle_digest: self.envelope.deterministic_digest().clone(),
        }
    }
}
