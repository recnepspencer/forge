use super::{
    certified_signed_area_2d_identity_entries, AreaDegeneracyClass, CertifiedSignedArea2DBasis,
    CertifiedSignedArea2DPerformanceCounters, SignedAreaOrientation, SignedAreaRepairAction,
};
use worth_primitives::{truth_digest_parts, TruthDigestScope};

#[derive(Clone, Debug, PartialEq)]
pub struct CertifiedSignedArea2DReceipt {
    basis: CertifiedSignedArea2DBasis,
    declaration_digest: String,
    envelope_digest: String,
    fact_digest: String,
    counters: CertifiedSignedArea2DPerformanceCounters,
}

impl CertifiedSignedArea2DReceipt {
    pub(crate) fn new(
        basis: CertifiedSignedArea2DBasis,
        declaration_digest: String,
        envelope_digest: String,
        fact_digest: String,
        counters: CertifiedSignedArea2DPerformanceCounters,
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
        basis: &CertifiedSignedArea2DBasis,
        declaration_digest: &str,
        envelope_digest: &str,
    ) -> Vec<String> {
        let mut parts = certified_signed_area_2d_identity_entries(basis)
            .into_iter()
            .map(|entry| format!("{}:{}", entry.locus(), entry.value()))
            .collect::<Vec<_>>();
        parts.push(format!("declaration:{declaration_digest}"));
        parts.push(format!("envelope:{envelope_digest}"));
        parts
    }

    pub(crate) fn fact_digest_for(
        basis: &CertifiedSignedArea2DBasis,
        declaration_digest: &str,
        envelope_digest: &str,
    ) -> String {
        truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &Self::digest_parts(basis, declaration_digest, envelope_digest),
        )
    }

    pub fn basis(&self) -> &CertifiedSignedArea2DBasis {
        &self.basis
    }

    pub fn orientation(&self) -> SignedAreaOrientation {
        self.basis.orientation()
    }

    pub fn degeneracy(&self) -> AreaDegeneracyClass {
        self.basis.degeneracy()
    }

    pub fn repair_action(&self) -> Option<SignedAreaRepairAction> {
        None
    }

    pub fn used_local_frame_scale(&self) -> bool {
        self.basis.precision_receipt().basis().normalization_scale() > 0.0
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

    pub fn counters(&self) -> CertifiedSignedArea2DPerformanceCounters {
        self.counters
    }
}
