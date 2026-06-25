use serde::Serialize;

use crate::derived_topology::invalidation_plan::catalog::DerivedTopologyProductFamilyIdentity;
#[cfg(test)]
use crate::derived_topology::invalidation_plan::execution::{
    DerivedInvalidationExecutedProductRow, DerivedInvalidationExecutionReceipt,
    DerivedInvalidationUnaffectedProductExecutionRow,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MigratedDerivedProductFamilyCloseout {
    family_identity: DerivedTopologyProductFamilyIdentity,
    selected_plan_digest: String,
    execution_receipt_digest: String,
    executed_row_digest: String,
    product_output_digest: String,
    old_authority_residue_digest: String,
    counters_digest: String,
    proof_authority: MigratedDerivedProductFamilyProofAuthority,
    proof_digest: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum MigratedDerivedProductFamilyProofAuthority {
    FamilySpecificMigrationCloseout,
    ReceiptSyntheticRow,
}

impl MigratedDerivedProductFamilyProofAuthority {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FamilySpecificMigrationCloseout => "family_specific_migration_closeout",
            Self::ReceiptSyntheticRow => "receipt_synthetic_row",
        }
    }
}

impl MigratedDerivedProductFamilyCloseout {
    pub(crate) fn new(
        family_identity: DerivedTopologyProductFamilyIdentity,
        selected_plan_digest: &str,
        execution_receipt_digest: &str,
        executed_row_digest: &str,
        product_output_digest: &str,
        old_authority_residue_digest: &str,
        counters_digest: &str,
    ) -> Self {
        let proof_digest = super::super::catalog::catalog_digest([
            "worth-topo:migrated-derived-product-family-closeout:v1".to_string(),
            format!("family:{}", family_identity.as_str()),
            format!("selected-plan:{selected_plan_digest}"),
            format!("execution-receipt:{execution_receipt_digest}"),
            format!("executed-row:{executed_row_digest}"),
            format!("product-output:{product_output_digest}"),
            format!("old-authority-residue:{old_authority_residue_digest}"),
            format!("counters:{counters_digest}"),
            format!(
                "proof-authority:{}",
                MigratedDerivedProductFamilyProofAuthority::FamilySpecificMigrationCloseout
                    .as_str()
            ),
        ]);
        Self {
            family_identity,
            selected_plan_digest: selected_plan_digest.to_string(),
            execution_receipt_digest: execution_receipt_digest.to_string(),
            executed_row_digest: executed_row_digest.to_string(),
            product_output_digest: product_output_digest.to_string(),
            old_authority_residue_digest: old_authority_residue_digest.to_string(),
            counters_digest: counters_digest.to_string(),
            proof_authority:
                MigratedDerivedProductFamilyProofAuthority::FamilySpecificMigrationCloseout,
            proof_digest,
        }
    }

    #[cfg(test)]
    pub(crate) fn from_executed_product_row(
        receipt: &DerivedInvalidationExecutionReceipt,
        row: &DerivedInvalidationExecutedProductRow,
    ) -> Self {
        let product_output_digest = row
            .product_output_digest()
            .unwrap_or_else(|| row.execution_report_digest());
        let old_authority_residue_digest = receipt_bound_digest(
            "executed-old-authority-residue",
            receipt.execution_receipt_digest(),
            row.row_digest(),
        );
        let counters_digest = receipt_bound_digest(
            "executed-counters",
            receipt.execution_receipt_digest(),
            row.row_digest(),
        );
        Self::new_with_authority(
            row.family_identity(),
            receipt.selected_plan_digest(),
            receipt.execution_receipt_digest(),
            row.row_digest(),
            product_output_digest,
            &old_authority_residue_digest,
            &counters_digest,
            MigratedDerivedProductFamilyProofAuthority::ReceiptSyntheticRow,
        )
    }

    #[cfg(test)]
    pub(crate) fn from_unaffected_product_row(
        receipt: &DerivedInvalidationExecutionReceipt,
        row: &DerivedInvalidationUnaffectedProductExecutionRow,
    ) -> Self {
        let product_output_digest = receipt_bound_digest(
            "unaffected-product-output",
            receipt.execution_receipt_digest(),
            row.row_digest(),
        );
        let old_authority_residue_digest = receipt_bound_digest(
            "unaffected-old-authority-residue",
            receipt.execution_receipt_digest(),
            row.row_digest(),
        );
        let counters_digest = receipt_bound_digest(
            "unaffected-counters",
            receipt.execution_receipt_digest(),
            row.row_digest(),
        );
        Self::new_with_authority(
            row.family_identity(),
            receipt.selected_plan_digest(),
            receipt.execution_receipt_digest(),
            row.row_digest(),
            &product_output_digest,
            &old_authority_residue_digest,
            &counters_digest,
            MigratedDerivedProductFamilyProofAuthority::ReceiptSyntheticRow,
        )
    }

    #[cfg(test)]
    fn new_with_authority(
        family_identity: DerivedTopologyProductFamilyIdentity,
        selected_plan_digest: &str,
        execution_receipt_digest: &str,
        executed_row_digest: &str,
        product_output_digest: &str,
        old_authority_residue_digest: &str,
        counters_digest: &str,
        proof_authority: MigratedDerivedProductFamilyProofAuthority,
    ) -> Self {
        let proof_digest = super::super::catalog::catalog_digest([
            "worth-topo:migrated-derived-product-family-closeout:v1".to_string(),
            format!("family:{}", family_identity.as_str()),
            format!("selected-plan:{selected_plan_digest}"),
            format!("execution-receipt:{execution_receipt_digest}"),
            format!("executed-row:{executed_row_digest}"),
            format!("product-output:{product_output_digest}"),
            format!("old-authority-residue:{old_authority_residue_digest}"),
            format!("counters:{counters_digest}"),
            format!("proof-authority:{}", proof_authority.as_str()),
        ]);
        Self {
            family_identity,
            selected_plan_digest: selected_plan_digest.to_string(),
            execution_receipt_digest: execution_receipt_digest.to_string(),
            executed_row_digest: executed_row_digest.to_string(),
            product_output_digest: product_output_digest.to_string(),
            old_authority_residue_digest: old_authority_residue_digest.to_string(),
            counters_digest: counters_digest.to_string(),
            proof_authority,
            proof_digest,
        }
    }

    pub const fn family_identity(&self) -> DerivedTopologyProductFamilyIdentity {
        self.family_identity
    }

    pub fn selected_plan_digest(&self) -> &str {
        &self.selected_plan_digest
    }

    pub fn execution_receipt_digest(&self) -> &str {
        &self.execution_receipt_digest
    }

    pub fn executed_row_digest(&self) -> &str {
        &self.executed_row_digest
    }

    pub fn product_output_digest(&self) -> &str {
        &self.product_output_digest
    }

    pub fn old_authority_residue_digest(&self) -> &str {
        &self.old_authority_residue_digest
    }

    pub fn counters_digest(&self) -> &str {
        &self.counters_digest
    }

    pub const fn proof_authority(&self) -> MigratedDerivedProductFamilyProofAuthority {
        self.proof_authority
    }

    pub fn proof_digest(&self) -> &str {
        &self.proof_digest
    }
}

#[cfg(test)]
fn receipt_bound_digest(label: &str, receipt_digest: &str, row_digest: &str) -> String {
    super::super::catalog::catalog_digest([
        "worth-topo:receipt-bound-derived-product-family-proof:v1".to_string(),
        format!("label:{label}"),
        format!("receipt:{receipt_digest}"),
        format!("row:{row_digest}"),
    ])
}
