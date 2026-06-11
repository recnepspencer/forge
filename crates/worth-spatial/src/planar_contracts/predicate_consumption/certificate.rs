use super::{
    predicate_certificate_consumption_digest, predicate_certificate_consumption_identity_entries,
    PredicateCertificateConsumptionBasis, PredicateCertificateConsumptionCounters,
};

#[derive(Clone, Debug, PartialEq)]
pub struct PredicateCertificateConsumptionReceipt {
    basis: PredicateCertificateConsumptionBasis,
    declaration_digest: String,
    envelope_digest: String,
    fact_digest: String,
    counters: PredicateCertificateConsumptionCounters,
}

impl PredicateCertificateConsumptionReceipt {
    pub(crate) fn new(
        basis: PredicateCertificateConsumptionBasis,
        declaration_digest: String,
        envelope_digest: String,
        fact_digest: String,
        counters: PredicateCertificateConsumptionCounters,
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
        basis: &PredicateCertificateConsumptionBasis,
        declaration_digest: &str,
        envelope_digest: &str,
    ) -> Vec<String> {
        let mut parts = predicate_certificate_consumption_identity_entries(basis)
            .into_iter()
            .map(|entry| entry.digest_part())
            .collect::<Vec<_>>();
        parts.push(format!("declaration:{declaration_digest}"));
        parts.push(format!("envelope:{envelope_digest}"));
        parts
    }

    pub(crate) fn fact_digest_for(
        basis: &PredicateCertificateConsumptionBasis,
        declaration_digest: &str,
        envelope_digest: &str,
    ) -> String {
        predicate_certificate_consumption_digest(&Self::digest_parts(
            basis,
            declaration_digest,
            envelope_digest,
        ))
    }

    pub fn basis(&self) -> &PredicateCertificateConsumptionBasis {
        &self.basis
    }

    pub fn proves_no_second_predicate_engine(&self) -> bool {
        self.counters.rejected_substitute_rows() == 0
            && self
                .basis
                .consumption_rows()
                .iter()
                .all(|row| !row.precision_escalation_identity().is_empty())
    }

    pub fn certified_predicate_rows(&self) -> usize {
        self.counters.certified_predicate_rows()
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

    pub fn counters(&self) -> PredicateCertificateConsumptionCounters {
        self.counters
    }
}
