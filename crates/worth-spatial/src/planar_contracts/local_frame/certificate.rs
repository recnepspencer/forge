use super::{
    planar_local_frame_basis_identity_entries, planar_local_frame_digest, PlanarLocalFrameBasis,
    PlanarLocalFramePerformanceCounters,
};

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarLocalFrameCertificateReceipt {
    basis: PlanarLocalFrameBasis,
    declaration_digest: String,
    envelope_digest: String,
    fact_digest: String,
    counters: PlanarLocalFramePerformanceCounters,
}

impl PlanarLocalFrameCertificateReceipt {
    pub(crate) fn new(
        basis: PlanarLocalFrameBasis,
        declaration_digest: String,
        envelope_digest: String,
        fact_digest: String,
        counters: PlanarLocalFramePerformanceCounters,
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
        basis: &PlanarLocalFrameBasis,
        declaration_digest: &str,
        envelope_digest: &str,
    ) -> Vec<String> {
        let mut parts = planar_local_frame_basis_identity_entries(basis)
            .into_iter()
            .map(|entry| entry.digest_part())
            .collect::<Vec<_>>();
        parts.push(format!("declaration:{declaration_digest}"));
        parts.push(format!("envelope:{envelope_digest}"));
        parts
    }

    pub(crate) fn fact_digest_for(
        basis: &PlanarLocalFrameBasis,
        declaration_digest: &str,
        envelope_digest: &str,
    ) -> String {
        planar_local_frame_digest(&Self::digest_parts(
            basis,
            declaration_digest,
            envelope_digest,
        ))
    }

    pub fn basis(&self) -> &PlanarLocalFrameBasis {
        &self.basis
    }

    pub fn frame_identity(&self) -> &str {
        self.basis.frame_identity()
    }

    pub fn precision_fact_digest(&self) -> &str {
        self.basis.precision_fact_digest()
    }

    pub fn scale_separation_orders(&self) -> i32 {
        self.basis.scale_separation_orders()
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

    pub fn counters(&self) -> PlanarLocalFramePerformanceCounters {
        self.counters
    }
}
