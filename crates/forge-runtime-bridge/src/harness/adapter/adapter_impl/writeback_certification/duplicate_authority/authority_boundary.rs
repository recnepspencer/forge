use crate::adapter::{TruthWritebackReceipt, TruthWritebackRequest};
use crate::writeback::{
    AdmittedBridgeWritebackContract, BridgeValidatedWritebackCandidate,
    BridgeWritebackStrategyBasis, BridgeWritebackStrategyCoherenceDisposition,
    BridgeWritebackStrategyCoherenceReport,
};

pub(in crate::harness::adapter::adapter_impl) struct DuplicateAuthorityBoundaryMatrix {
    contract: AdmittedBridgeWritebackContract,
    strategy_basis: BridgeWritebackStrategyBasis,
    first_strategy_coherence: BridgeWritebackStrategyCoherenceReport,
    first_candidate: BridgeValidatedWritebackCandidate,
    repeated_strategy_coherence: BridgeWritebackStrategyCoherenceReport,
    repeated_candidate: BridgeValidatedWritebackCandidate,
    first_authority_request: TruthWritebackRequest,
    repeated_authority_request: TruthWritebackRequest,
    first_authority_receipt: TruthWritebackReceipt,
    repeated_authority_receipt: TruthWritebackReceipt,
}

pub(in crate::harness::adapter::adapter_impl) struct DuplicateAuthorityBoundaryEvidence<'a> {
    pub(in crate::harness::adapter::adapter_impl) contract: &'a AdmittedBridgeWritebackContract,
    pub(in crate::harness::adapter::adapter_impl) strategy_basis: &'a BridgeWritebackStrategyBasis,
    pub(in crate::harness::adapter::adapter_impl) first_strategy_coherence:
        &'a BridgeWritebackStrategyCoherenceReport,
    pub(in crate::harness::adapter::adapter_impl) repeated_strategy_coherence:
        &'a BridgeWritebackStrategyCoherenceReport,
    pub(in crate::harness::adapter::adapter_impl) first_candidate:
        &'a BridgeValidatedWritebackCandidate,
    pub(in crate::harness::adapter::adapter_impl) repeated_candidate:
        &'a BridgeValidatedWritebackCandidate,
    pub(in crate::harness::adapter::adapter_impl) first_authority_request:
        &'a TruthWritebackRequest,
    pub(in crate::harness::adapter::adapter_impl) repeated_authority_request:
        &'a TruthWritebackRequest,
    pub(in crate::harness::adapter::adapter_impl) first_receipt: &'a TruthWritebackReceipt,
    pub(in crate::harness::adapter::adapter_impl) repeated_receipt: &'a TruthWritebackReceipt,
}

impl DuplicateAuthorityBoundaryMatrix {
    pub(super) fn from_authority_boundary_evidence(
        evidence: DuplicateAuthorityBoundaryEvidence<'_>,
    ) -> Self {
        Self {
            contract: evidence.contract.clone(),
            strategy_basis: evidence.strategy_basis.clone(),
            first_strategy_coherence: evidence.first_strategy_coherence.clone(),
            first_candidate: evidence.first_candidate.clone(),
            repeated_strategy_coherence: evidence.repeated_strategy_coherence.clone(),
            repeated_candidate: evidence.repeated_candidate.clone(),
            first_authority_request: evidence.first_authority_request.clone(),
            repeated_authority_request: evidence.repeated_authority_request.clone(),
            first_authority_receipt: evidence.first_receipt.clone(),
            repeated_authority_receipt: evidence.repeated_receipt.clone(),
        }
    }

    pub(in crate::harness::adapter::adapter_impl) fn contract(
        &self,
    ) -> &AdmittedBridgeWritebackContract {
        &self.contract
    }

    pub(in crate::harness::adapter::adapter_impl) fn contract_digest(&self) -> &str {
        self.contract.digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn strategy_basis(
        &self,
    ) -> &BridgeWritebackStrategyBasis {
        &self.strategy_basis
    }

    pub(in crate::harness::adapter::adapter_impl) fn strategy_basis_digest(&self) -> &str {
        self.strategy_basis.digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn first_strategy_coherence(
        &self,
    ) -> &BridgeWritebackStrategyCoherenceReport {
        &self.first_strategy_coherence
    }

    pub(in crate::harness::adapter::adapter_impl) fn first_strategy_coherence_digest(
        &self,
    ) -> &str {
        self.first_strategy_coherence.digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn first_strategy_coherence_disposition(
        &self,
    ) -> BridgeWritebackStrategyCoherenceDisposition {
        self.first_strategy_coherence.disposition()
    }

    pub(in crate::harness::adapter::adapter_impl) fn first_candidate(
        &self,
    ) -> &BridgeValidatedWritebackCandidate {
        &self.first_candidate
    }

    pub(in crate::harness::adapter::adapter_impl) fn first_candidate_digest(&self) -> &str {
        self.first_candidate.digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn repeated_strategy_coherence(
        &self,
    ) -> &BridgeWritebackStrategyCoherenceReport {
        &self.repeated_strategy_coherence
    }

    pub(in crate::harness::adapter::adapter_impl) fn repeated_strategy_coherence_digest(
        &self,
    ) -> &str {
        self.repeated_strategy_coherence.digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn repeated_strategy_coherence_disposition(
        &self,
    ) -> BridgeWritebackStrategyCoherenceDisposition {
        self.repeated_strategy_coherence.disposition()
    }

    pub(in crate::harness::adapter::adapter_impl) fn repeated_candidate(
        &self,
    ) -> &BridgeValidatedWritebackCandidate {
        &self.repeated_candidate
    }

    pub(in crate::harness::adapter::adapter_impl) fn repeated_candidate_digest(&self) -> &str {
        self.repeated_candidate.digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn first_authority_request_digest(&self) -> &str {
        self.first_authority_request.digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn repeated_authority_request_digest(
        &self,
    ) -> &str {
        self.repeated_authority_request.digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn first_authority_receipt_digest(&self) -> &str {
        self.first_authority_receipt.digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn repeated_authority_receipt_digest(
        &self,
    ) -> &str {
        self.repeated_authority_receipt.digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn first_authority_request(
        &self,
    ) -> &TruthWritebackRequest {
        &self.first_authority_request
    }

    pub(in crate::harness::adapter::adapter_impl) fn repeated_authority_request(
        &self,
    ) -> &TruthWritebackRequest {
        &self.repeated_authority_request
    }

    pub(in crate::harness::adapter::adapter_impl) fn first_authority_receipt(
        &self,
    ) -> &TruthWritebackReceipt {
        &self.first_authority_receipt
    }

    pub(in crate::harness::adapter::adapter_impl) fn repeated_authority_receipt(
        &self,
    ) -> &TruthWritebackReceipt {
        &self.repeated_authority_receipt
    }
}
