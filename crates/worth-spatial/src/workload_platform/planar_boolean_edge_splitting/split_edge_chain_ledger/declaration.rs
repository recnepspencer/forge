use super::denial::{
    PlanarBooleanSplitEdgeChainLedgerDenial, PlanarBooleanSplitEdgeChainLedgerDenialKind,
};
use super::identity;
use crate::workload_platform::planar_boolean_edge_splitting::{
    PlanarBooleanEdgeSplitRequest, PlanarBooleanSplitChainValidationReceipt,
    PlanarBooleanSplitDecisionLogQueryResult, PlanarBooleanSplitPersistentNamingReceipt,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanSplitEdgeChainLedgerDeclaration {
    declaration_identity: String,
    split_request_identity: String,
    split_chain_validation_receipt_identity: String,
    split_persistent_naming_receipt_identity: String,
    split_decision_log_receipt_identity: String,
    lowered_plan_identity: String,
}

impl PlanarBooleanSplitEdgeChainLedgerDeclaration {
    pub fn from_query_products(
        split_request: &PlanarBooleanEdgeSplitRequest,
        split_chain_validation: &PlanarBooleanSplitChainValidationReceipt,
        split_persistent_naming: &PlanarBooleanSplitPersistentNamingReceipt,
        split_decision_log: &PlanarBooleanSplitDecisionLogQueryResult,
    ) -> Result<Self, PlanarBooleanSplitEdgeChainLedgerDenial> {
        Self::from_product_identities_impl(
            split_request.split_request_identity(),
            split_chain_validation.receipt_identity(),
            split_persistent_naming.receipt_identity(),
            split_decision_log.receipt().receipt_identity(),
        )
    }

    #[cfg(test)]
    pub(crate) fn from_product_identities(
        split_request_identity: impl Into<String>,
        split_chain_validation_receipt_identity: impl Into<String>,
        split_persistent_naming_receipt_identity: impl Into<String>,
        split_decision_log_receipt_identity: impl Into<String>,
    ) -> Result<Self, PlanarBooleanSplitEdgeChainLedgerDenial> {
        Self::from_product_identities_impl(
            split_request_identity,
            split_chain_validation_receipt_identity,
            split_persistent_naming_receipt_identity,
            split_decision_log_receipt_identity,
        )
    }

    fn from_product_identities_impl(
        split_request_identity: impl Into<String>,
        split_chain_validation_receipt_identity: impl Into<String>,
        split_persistent_naming_receipt_identity: impl Into<String>,
        split_decision_log_receipt_identity: impl Into<String>,
    ) -> Result<Self, PlanarBooleanSplitEdgeChainLedgerDenial> {
        let split_request_identity = split_request_identity.into();
        let split_chain_validation_receipt_identity =
            split_chain_validation_receipt_identity.into();
        let split_persistent_naming_receipt_identity =
            split_persistent_naming_receipt_identity.into();
        let split_decision_log_receipt_identity = split_decision_log_receipt_identity.into();
        if split_request_identity.is_empty()
            || split_chain_validation_receipt_identity.is_empty()
            || split_persistent_naming_receipt_identity.is_empty()
            || split_decision_log_receipt_identity.is_empty()
        {
            return Err(PlanarBooleanSplitEdgeChainLedgerDenial::new(
                PlanarBooleanSplitEdgeChainLedgerDenialKind::EmptyQueryDeclarationIdentity,
                "empty-split-edge-chain-ledger-declaration",
                Default::default(),
                "split edge-chain ledger Query declaration requires product identities",
            ));
        }
        let declaration_identity = identity::declaration_identity(
            &split_request_identity,
            &split_chain_validation_receipt_identity,
            &split_persistent_naming_receipt_identity,
            &split_decision_log_receipt_identity,
        );
        let lowered_plan_identity = format!("lowered:{declaration_identity}");
        Ok(Self {
            declaration_identity,
            split_request_identity,
            split_chain_validation_receipt_identity,
            split_persistent_naming_receipt_identity,
            split_decision_log_receipt_identity,
            lowered_plan_identity,
        })
    }

    pub fn declaration_identity(&self) -> &str {
        &self.declaration_identity
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
    pub fn lowered_plan_identity(&self) -> &str {
        &self.lowered_plan_identity
    }
}
