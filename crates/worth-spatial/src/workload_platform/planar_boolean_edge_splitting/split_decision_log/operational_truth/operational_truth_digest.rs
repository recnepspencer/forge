use super::super::{
    PlanarBooleanSplitChainValidationReceipt, PlanarBooleanSplitEdgeFragmentSet,
    PlanarBooleanSplitPersistentNamingReceipt,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanSplitOperationalTruthDigest {
    digest_identity: String,
    split_edge_fragment_set_identity: String,
    split_chain_validation_receipt_identity: String,
    split_persistent_naming_receipt_identity: String,
}

impl PlanarBooleanSplitOperationalTruthDigest {
    pub fn from_split_products(
        fragments: &PlanarBooleanSplitEdgeFragmentSet,
        validation: &PlanarBooleanSplitChainValidationReceipt,
        naming: &PlanarBooleanSplitPersistentNamingReceipt,
    ) -> Self {
        let split_edge_fragment_set_identity = fragments.fragment_set_identity().to_string();
        let split_chain_validation_receipt_identity = validation.receipt_identity().to_string();
        let split_persistent_naming_receipt_identity = naming.receipt_identity().to_string();
        let digest_identity = format!(
            "split-operational-truth:{split_edge_fragment_set_identity}:{split_chain_validation_receipt_identity}:{split_persistent_naming_receipt_identity}"
        );
        Self {
            digest_identity,
            split_edge_fragment_set_identity,
            split_chain_validation_receipt_identity,
            split_persistent_naming_receipt_identity,
        }
    }

    pub fn digest_identity(&self) -> &str {
        &self.digest_identity
    }

    pub fn split_edge_fragment_set_identity(&self) -> &str {
        &self.split_edge_fragment_set_identity
    }

    pub fn split_chain_validation_receipt_identity(&self) -> &str {
        &self.split_chain_validation_receipt_identity
    }

    pub fn split_persistent_naming_receipt_identity(&self) -> &str {
        &self.split_persistent_naming_receipt_identity
    }
}
