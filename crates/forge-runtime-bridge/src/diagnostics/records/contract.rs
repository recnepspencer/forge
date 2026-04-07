use crate::routing::BridgeRouteContractProof;
use crate::routing::context::BridgeMappingContext;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeContractDiagnosticsRecord {
    contract_proof: BridgeRouteContractProof,
}

impl BridgeContractDiagnosticsRecord {
    pub(crate) fn new(contract_proof: BridgeRouteContractProof) -> Self {
        Self { contract_proof }
    }

    pub fn contract_proof(&self) -> &BridgeRouteContractProof {
        &self.contract_proof
    }

    pub fn mapping_context(&self) -> &BridgeMappingContext {
        self.contract_proof.mapping_context()
    }
}
