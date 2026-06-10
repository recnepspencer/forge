use super::{
    planar_contract_bundle_digest, planar_contract_bundle_identity_entries,
    PlanarContractBundleValidationBasis, PlanarContractBundleValidationCounters,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanReadinessStatus {
    ReadyForM7,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarContractBundleBooleanResult {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarContractBundleImprintAction {}

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarContractBundleValidationReceipt {
    basis: PlanarContractBundleValidationBasis,
    declaration_digest: String,
    envelope_digest: String,
    fact_digest: String,
    status: PlanarBooleanReadinessStatus,
    counters: PlanarContractBundleValidationCounters,
}

impl PlanarContractBundleValidationReceipt {
    pub(crate) fn new(
        basis: PlanarContractBundleValidationBasis,
        declaration_digest: String,
        envelope_digest: String,
        fact_digest: String,
        counters: PlanarContractBundleValidationCounters,
    ) -> Self {
        Self {
            basis,
            declaration_digest,
            envelope_digest,
            fact_digest,
            status: PlanarBooleanReadinessStatus::ReadyForM7,
            counters,
        }
    }

    pub(crate) fn digest_parts(
        basis: &PlanarContractBundleValidationBasis,
        declaration_digest: &str,
        envelope_digest: &str,
    ) -> Vec<String> {
        let mut parts = planar_contract_bundle_identity_entries(basis)
            .into_iter()
            .map(|entry| entry.digest_part())
            .collect::<Vec<_>>();
        parts.push(format!("declaration:{declaration_digest}"));
        parts.push(format!("envelope:{envelope_digest}"));
        parts
    }

    pub(crate) fn fact_digest_for(
        basis: &PlanarContractBundleValidationBasis,
        declaration_digest: &str,
        envelope_digest: &str,
    ) -> String {
        planar_contract_bundle_digest(&Self::digest_parts(
            basis,
            declaration_digest,
            envelope_digest,
        ))
    }

    pub fn basis(&self) -> &PlanarContractBundleValidationBasis {
        &self.basis
    }

    pub fn status(&self) -> PlanarBooleanReadinessStatus {
        self.status
    }

    pub fn is_ready_for_m7(&self) -> bool {
        self.status == PlanarBooleanReadinessStatus::ReadyForM7
    }

    pub fn boolean_result(&self) -> Option<PlanarContractBundleBooleanResult> {
        None
    }

    pub fn imprint_action(&self) -> Option<PlanarContractBundleImprintAction> {
        None
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

    pub fn counters(&self) -> PlanarContractBundleValidationCounters {
        self.counters
    }
}
