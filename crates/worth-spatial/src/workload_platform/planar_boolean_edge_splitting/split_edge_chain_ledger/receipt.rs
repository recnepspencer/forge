use super::counters::PlanarBooleanSplitEdgeChainLedgerCounters;
use super::identity;
use super::ledger::PlanarBooleanSplitEdgeChainLedger;
use crate::workload_platform::evidence_ledger::{
    BooleanEvidenceReceipt, BooleanEvidenceRowAuthority, BooleanEvidenceStageKind,
    WorkloadEvidenceStageCounters, WorkloadEvidenceSupport,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanSplitEdgeChainLedgerReceipt {
    receipt_identity: String,
    ledger_identity: String,
    downstream_consumption_identity: String,
    split_request_identity: String,
    split_chain_validation_receipt_identity: String,
    split_persistent_naming_receipt_identity: String,
    split_decision_log_receipt_identity: String,
    chain_identities: Vec<String>,
    counters: PlanarBooleanSplitEdgeChainLedgerCounters,
}

impl PlanarBooleanSplitEdgeChainLedgerReceipt {
    pub(crate) fn from_ledger(ledger: &PlanarBooleanSplitEdgeChainLedger) -> Self {
        let chain_identities = ledger
            .chains()
            .iter()
            .map(|chain| chain.chain_identity().to_string())
            .collect::<Vec<_>>();
        let consumed_identities = vec![
            ledger.split_request_identity().to_string(),
            ledger.split_chain_validation_receipt_identity().to_string(),
            ledger
                .split_persistent_naming_receipt_identity()
                .to_string(),
            ledger.split_decision_log_receipt_identity().to_string(),
        ];
        let receipt_identity =
            identity::receipt_identity(ledger.ledger_identity(), &consumed_identities);
        let mut counters = ledger.counters();
        counters.emitted_downstream_identity();
        Self {
            downstream_consumption_identity: identity::downstream_consumption_identity(
                &receipt_identity,
            ),
            receipt_identity,
            ledger_identity: ledger.ledger_identity().to_string(),
            split_request_identity: ledger.split_request_identity().to_string(),
            split_chain_validation_receipt_identity: ledger
                .split_chain_validation_receipt_identity()
                .to_string(),
            split_persistent_naming_receipt_identity: ledger
                .split_persistent_naming_receipt_identity()
                .to_string(),
            split_decision_log_receipt_identity: ledger
                .split_decision_log_receipt_identity()
                .to_string(),
            chain_identities,
            counters,
        }
    }

    pub fn receipt_identity(&self) -> &str {
        &self.receipt_identity
    }
    pub fn ledger_identity(&self) -> &str {
        &self.ledger_identity
    }
    pub fn downstream_consumption_identity(&self) -> &str {
        &self.downstream_consumption_identity
    }
    pub fn split_request_identity(&self) -> &str {
        &self.split_request_identity
    }
    pub fn split_chain_validation_receipt_identity(&self) -> &str {
        &self.split_chain_validation_receipt_identity
    }
    pub fn split_persistent_naming_receipt_identity(&self) -> &str {
        &self.split_persistent_naming_receipt_identity
    }
    pub fn split_decision_log_receipt_identity(&self) -> &str {
        &self.split_decision_log_receipt_identity
    }
    pub fn chain_identities(&self) -> &[String] {
        &self.chain_identities
    }
    pub fn counters(&self) -> PlanarBooleanSplitEdgeChainLedgerCounters {
        self.counters
    }
    pub fn certifies_split_edge_chain_ledger(&self) -> bool {
        self.counters.ledger_chains_emitted() == self.chain_identities.len()
            && self.counters.validation_receipts_consumed() == 1
            && self.counters.downstream_identities_emitted() == 1
            && self.counters.foreign_product_denials() == 0
            && self.counters.missing_validation_denials() == 0
            && self.counters.missing_persistent_name_denials() == 0
            && self.counters.missing_decision_log_denials() == 0
            && self.counters.duplicate_chain_identity_denials() == 0
            && !self.downstream_consumption_identity.is_empty()
    }
}

impl BooleanEvidenceReceipt for PlanarBooleanSplitEdgeChainLedgerReceipt {
    fn boolean_stage(&self) -> BooleanEvidenceStageKind {
        BooleanEvidenceStageKind::Split
    }

    fn evidence_identity(&self) -> &str {
        self.receipt_identity()
    }

    fn evidence_support(&self) -> WorkloadEvidenceSupport {
        WorkloadEvidenceSupport::Admitted
    }

    fn evidence_counters(&self) -> WorkloadEvidenceStageCounters {
        WorkloadEvidenceStageCounters::boolean_split()
    }
}

impl BooleanEvidenceRowAuthority for PlanarBooleanSplitEdgeChainLedgerReceipt {}
