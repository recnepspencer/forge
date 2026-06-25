use super::{
    certified_segment_segment_2d_digest, certified_segment_segment_2d_identity_entries,
    CertifiedSegmentSegment2DBasis, CertifiedSegmentSegment2DClassification,
    CertifiedSegmentSegment2DMutationEvidence, CertifiedSegmentSegment2DPerformanceCounters,
};
use crate::planar_contracts::predicate_authority::PlanarPredicateFactReceipt;

#[derive(Clone, Debug, PartialEq)]
pub struct CertifiedSegmentSegment2DReceipt {
    basis: CertifiedSegmentSegment2DBasis,
    declaration_digest: String,
    envelope_digest: String,
    fact_digest: String,
    mutation_evidence: CertifiedSegmentSegment2DMutationEvidence,
    orientation_predicates: Vec<PlanarPredicateFactReceipt>,
    counters: CertifiedSegmentSegment2DPerformanceCounters,
}

impl CertifiedSegmentSegment2DReceipt {
    pub(crate) fn new(
        basis: CertifiedSegmentSegment2DBasis,
        declaration_digest: String,
        envelope_digest: String,
        fact_digest: String,
        mutation_evidence: CertifiedSegmentSegment2DMutationEvidence,
        orientation_predicates: Vec<PlanarPredicateFactReceipt>,
        counters: CertifiedSegmentSegment2DPerformanceCounters,
    ) -> Self {
        Self {
            basis,
            declaration_digest,
            envelope_digest,
            fact_digest,
            mutation_evidence,
            orientation_predicates,
            counters,
        }
    }

    pub(crate) fn digest_parts(
        basis: &CertifiedSegmentSegment2DBasis,
        declaration_digest: &str,
        envelope_digest: &str,
    ) -> Vec<String> {
        let mut parts = certified_segment_segment_2d_identity_entries(basis)
            .into_iter()
            .map(|entry| format!("{}:{}", entry.locus(), entry.value()))
            .collect::<Vec<_>>();
        parts.push(format!("declaration:{declaration_digest}"));
        parts.push(format!("envelope:{envelope_digest}"));
        parts
    }

    pub(crate) fn fact_digest_for(
        basis: &CertifiedSegmentSegment2DBasis,
        declaration_digest: &str,
        envelope_digest: &str,
    ) -> String {
        certified_segment_segment_2d_digest(&Self::digest_parts(
            basis,
            declaration_digest,
            envelope_digest,
        ))
    }

    pub fn basis(&self) -> &CertifiedSegmentSegment2DBasis {
        &self.basis
    }

    pub fn classification(&self) -> CertifiedSegmentSegment2DClassification {
        self.basis.classification()
    }

    pub fn declaration_digest(&self) -> &str {
        &self.declaration_digest
    }

    pub fn envelope_digest(&self) -> &str {
        &self.envelope_digest
    }

    pub fn fact_digest(&self) -> &str {
        &self.fact_digest
    }

    pub fn mutation_evidence(&self) -> &CertifiedSegmentSegment2DMutationEvidence {
        &self.mutation_evidence
    }

    pub fn orientation_predicate_receipts(&self) -> &[PlanarPredicateFactReceipt] {
        &self.orientation_predicates
    }

    pub fn counters(&self) -> CertifiedSegmentSegment2DPerformanceCounters {
        self.counters
    }
}
