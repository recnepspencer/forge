use super::{
    coplanar_overlap_contract_identity_entries, AmbiguousContactRow, ContainmentRelationRow,
    CoplanarOverlapBooleanResult, CoplanarOverlapContractBasis, CoplanarOverlapImprintAction,
    CoplanarOverlapPerformanceCounters, OverlapIslandRow, PolicyRequiredExitRow, SharedIntervalRow,
};
use worth_primitives::{truth_digest_parts, TruthDigestScope};

#[derive(Clone, Debug, PartialEq)]
pub struct CoplanarOverlapContractReceipt {
    basis: CoplanarOverlapContractBasis,
    declaration_digest: String,
    envelope_digest: String,
    fact_digest: String,
    counters: CoplanarOverlapPerformanceCounters,
}

impl CoplanarOverlapContractReceipt {
    pub(crate) fn new(
        basis: CoplanarOverlapContractBasis,
        declaration_digest: String,
        envelope_digest: String,
        fact_digest: String,
        counters: CoplanarOverlapPerformanceCounters,
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
        basis: &CoplanarOverlapContractBasis,
        declaration_digest: &str,
        envelope_digest: &str,
    ) -> Vec<String> {
        let mut parts = coplanar_overlap_contract_identity_entries(basis)
            .into_iter()
            .map(|entry| format!("{}:{}", entry.locus(), entry.value()))
            .collect::<Vec<_>>();
        parts.push(format!("declaration:{declaration_digest}"));
        parts.push(format!("envelope:{envelope_digest}"));
        parts
    }

    pub(crate) fn fact_digest_for(
        basis: &CoplanarOverlapContractBasis,
        declaration_digest: &str,
        envelope_digest: &str,
    ) -> String {
        truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &Self::digest_parts(basis, declaration_digest, envelope_digest),
        )
    }

    pub fn basis(&self) -> &CoplanarOverlapContractBasis {
        &self.basis
    }

    pub fn shared_intervals(&self) -> &[SharedIntervalRow] {
        self.basis.shared_intervals()
    }

    pub fn overlap_islands(&self) -> &[OverlapIslandRow] {
        self.basis.overlap_islands()
    }

    pub fn containment_relations(&self) -> &[ContainmentRelationRow] {
        self.basis.containment_relations()
    }

    pub fn ambiguous_contacts(&self) -> &[AmbiguousContactRow] {
        self.basis.ambiguous_contacts()
    }

    pub fn policy_required_exits(&self) -> &[PolicyRequiredExitRow] {
        self.basis.policy_required_exits()
    }

    pub fn boolean_result(&self) -> Option<CoplanarOverlapBooleanResult> {
        None
    }

    pub fn imprint_action(&self) -> Option<CoplanarOverlapImprintAction> {
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

    pub fn counters(&self) -> CoplanarOverlapPerformanceCounters {
        self.counters
    }
}
