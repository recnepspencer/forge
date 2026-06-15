use super::{
    identity::topology_contract_digest, planar_topology_contract_authority_entries,
    PlanarTopologyContractCompletenessBasis, PlanarTopologyContractCompletenessCounters,
};

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarTopologyContractCompletenessReceipt {
    basis: PlanarTopologyContractCompletenessBasis,
    declaration_digest: String,
    envelope_digest: String,
    fact_digest: String,
    counters: PlanarTopologyContractCompletenessCounters,
}

impl PlanarTopologyContractCompletenessReceipt {
    pub(crate) fn new(
        basis: PlanarTopologyContractCompletenessBasis,
        declaration_digest: String,
        envelope_digest: String,
        fact_digest: String,
        counters: PlanarTopologyContractCompletenessCounters,
    ) -> Self {
        Self {
            basis,
            declaration_digest,
            envelope_digest,
            fact_digest,
            counters,
        }
    }

    pub(crate) fn fact_digest_for(basis: &PlanarTopologyContractCompletenessBasis) -> String {
        topology_contract_digest(
            &planar_topology_contract_authority_entries(basis)
                .into_iter()
                .map(|entry| entry.digest_part())
                .collect::<Vec<_>>(),
        )
    }

    pub fn basis(&self) -> &PlanarTopologyContractCompletenessBasis {
        &self.basis
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

    pub fn counters(&self) -> PlanarTopologyContractCompletenessCounters {
        self.counters
    }
}
