use worth_query::facade::ProjectionConsumptionReceipt;

use super::WorthServerDirectMaterializationDigest;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerDirectProjectionFactReceipt {
    declaration_digest: String,
    contract_digest: String,
    fact_set_digest: String,
    counter_snapshot_digest: String,
    integrity_digest: String,
    receipt_digest: String,
    materialization_digest: WorthServerDirectMaterializationDigest,
}

impl WorthServerDirectProjectionFactReceipt {
    pub(crate) fn from_projection_receipt(
        receipt: &ProjectionConsumptionReceipt,
        materialization_digest: WorthServerDirectMaterializationDigest,
    ) -> Self {
        Self {
            declaration_digest: receipt.declaration_digest().to_string(),
            contract_digest: receipt.contract_digest().to_string(),
            fact_set_digest: receipt.fact_set_digest().to_string(),
            counter_snapshot_digest: receipt.counter_snapshot_digest().to_string(),
            integrity_digest: receipt.integrity_digest().to_string(),
            receipt_digest: receipt.receipt_digest().to_string(),
            materialization_digest,
        }
    }

    pub fn declaration_digest(&self) -> &str {
        &self.declaration_digest
    }

    pub fn contract_digest(&self) -> &str {
        &self.contract_digest
    }

    pub fn fact_set_digest(&self) -> &str {
        &self.fact_set_digest
    }

    pub fn counter_snapshot_digest(&self) -> &str {
        &self.counter_snapshot_digest
    }

    pub fn integrity_digest(&self) -> &str {
        &self.integrity_digest
    }

    pub fn receipt_digest(&self) -> &str {
        &self.receipt_digest
    }

    pub fn materialization_digest(&self) -> &WorthServerDirectMaterializationDigest {
        &self.materialization_digest
    }
}
