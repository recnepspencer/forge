use super::{
    certified_polygon_winding_2d_identity_entries, CertifiedLoopContainment, CertifiedLoopWinding,
    CertifiedPolygonWinding2DBasis, CertifiedPolygonWinding2DPerformanceCounters,
};
use worth_primitives::{truth_digest_parts, TruthDigestScope};

#[derive(Clone, Debug, PartialEq)]
pub struct CertifiedPolygonWinding2DReceipt {
    basis: CertifiedPolygonWinding2DBasis,
    declaration_digest: String,
    envelope_digest: String,
    fact_digest: String,
    counters: CertifiedPolygonWinding2DPerformanceCounters,
}

impl CertifiedPolygonWinding2DReceipt {
    pub(crate) fn new(
        basis: CertifiedPolygonWinding2DBasis,
        declaration_digest: String,
        envelope_digest: String,
        fact_digest: String,
        counters: CertifiedPolygonWinding2DPerformanceCounters,
    ) -> Self {
        Self {
            basis,
            declaration_digest,
            envelope_digest,
            fact_digest,
            counters,
        }
    }

    pub(crate) fn digest_parts(
        basis: &CertifiedPolygonWinding2DBasis,
        declaration_digest: &str,
        envelope_digest: &str,
    ) -> Vec<String> {
        let mut parts = certified_polygon_winding_2d_identity_entries(basis)
            .into_iter()
            .map(|entry| format!("{}:{}", entry.locus(), entry.value()))
            .collect::<Vec<_>>();
        parts.push(format!("declaration:{declaration_digest}"));
        parts.push(format!("envelope:{envelope_digest}"));
        parts
    }

    pub(crate) fn fact_digest_for(
        basis: &CertifiedPolygonWinding2DBasis,
        declaration_digest: &str,
        envelope_digest: &str,
    ) -> String {
        certified_polygon_winding_2d_digest(&Self::digest_parts(
            basis,
            declaration_digest,
            envelope_digest,
        ))
    }

    pub fn basis(&self) -> &CertifiedPolygonWinding2DBasis {
        &self.basis
    }

    pub fn primary_winding(&self) -> CertifiedLoopWinding {
        self.basis.primary_winding()
    }

    pub fn containment_for(&self, loop_identity: &str) -> Option<CertifiedLoopContainment> {
        self.basis.containment_for(loop_identity)
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

    pub fn counters(&self) -> CertifiedPolygonWinding2DPerformanceCounters {
        self.counters
    }
}

fn certified_polygon_winding_2d_digest(parts: &[String]) -> String {
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, parts)
}
