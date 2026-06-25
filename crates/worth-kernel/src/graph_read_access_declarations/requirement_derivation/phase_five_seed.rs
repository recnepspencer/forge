use super::derivation_record::WorthGraphReadRequirementDerivationRecord;
use crate::graph_read_access_inventory::WorthGraphReadDeletionLedgerItem;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessDeclarationPhaseFiveSeed {
    requirement_records: Vec<WorthGraphReadRequirementDerivationRecord>,
    deletion_items: Vec<WorthGraphReadDeletionLedgerItem>,
    requirement_derivation_digest: String,
}

impl WorthGraphReadAccessDeclarationPhaseFiveSeed {
    pub(crate) fn new(
        requirement_records: Vec<WorthGraphReadRequirementDerivationRecord>,
        deletion_items: Vec<WorthGraphReadDeletionLedgerItem>,
        requirement_derivation_digest: impl Into<String>,
    ) -> Self {
        Self {
            requirement_records,
            deletion_items,
            requirement_derivation_digest: requirement_derivation_digest.into(),
        }
    }

    pub fn requirement_records(&self) -> &[WorthGraphReadRequirementDerivationRecord] {
        &self.requirement_records
    }

    pub fn requirement_derivation_digest(&self) -> &str {
        &self.requirement_derivation_digest
    }

    pub fn deletion_items(&self) -> &[WorthGraphReadDeletionLedgerItem] {
        &self.deletion_items
    }

    pub const fn claims_graph_read_execution(&self) -> bool {
        false
    }

    pub const fn claims_access_plan_consumption(&self) -> bool {
        false
    }
}
