use schema::facade::topology_authoring::DerivedTopologyReadBasis;

use crate::derived_topology::compiled_product_consumer_cutover::DerivedEquivalenceContractReport;

const TOPOLOGY_DIAGNOSTIC_CONTRACT_NAME: &str = "topology-derived-read-diagnostic-projection";

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub(crate) struct TopologyDerivedDiagnosticProjectionSource {
    truth_basis_identity_digest: String,
    diagnostic_contract_name: String,
}

impl TopologyDerivedDiagnosticProjectionSource {
    pub(crate) fn truth_basis_identity_digest(&self) -> &str {
        &self.truth_basis_identity_digest
    }

    pub(crate) fn diagnostic_contract_name(&self) -> &str {
        &self.diagnostic_contract_name
    }
}

pub(crate) fn topology_derived_diagnostic_projection_source(
    read_basis: &DerivedTopologyReadBasis,
    _equivalence_contract_report: &DerivedEquivalenceContractReport,
) -> TopologyDerivedDiagnosticProjectionSource {
    TopologyDerivedDiagnosticProjectionSource {
        truth_basis_identity_digest: read_basis
            .authority
            .truth_basis_identity
            .mutation_digest_hex
            .clone(),
        diagnostic_contract_name: TOPOLOGY_DIAGNOSTIC_CONTRACT_NAME.to_string(),
    }
}
