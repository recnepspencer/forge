use serde::Serialize;

use super::super::loop_cycles::LoopCycleMigrationCloseout;
use super::super::{
    MigratedDerivedProductFamilyCloseout, MigratedDerivedProductFamilyProofAuthority,
};
use crate::derived_topology::invalidation_plan::catalog::DerivedTopologyProductFamilyIdentity;

pub fn status_rows_from_loop_cycle_migration_closeout(
    loop_cycle_closeout: &LoopCycleMigrationCloseout,
) -> Vec<CoveredDerivedProductStatusRow> {
    status_rows_from_migrated_family_closeouts(
        &[loop_cycle_closeout.migrated_family_closeout()],
        loop_cycle_closeout.old_authority_residue_digest(),
    )
}

pub fn status_rows_from_migrated_family_closeouts(
    migrated_closeouts: &[&MigratedDerivedProductFamilyCloseout],
    certification_residue_digest: &str,
) -> Vec<CoveredDerivedProductStatusRow> {
    DerivedTopologyProductFamilyIdentity::REQUIRED
        .iter()
        .map(|family| {
            if let Some(closeout) = migrated_closeouts
                .iter()
                .copied()
                .find(|closeout| closeout.family_identity() == *family)
            {
                CoveredDerivedProductStatusRow::from_migrated_family_closeout(closeout)
            } else {
                CoveredDerivedProductStatusRow::certification_residue_only(
                    *family,
                    certification_residue_digest,
                )
            }
        })
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum CoveredDerivedProductMigrationStatus {
    Migrated,
    DeletedOldAuthority,
    CertificationResidueOnly,
}

impl CoveredDerivedProductMigrationStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Migrated => "migrated",
            Self::DeletedOldAuthority => "deleted_old_authority",
            Self::CertificationResidueOnly => "certification_residue_only",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CoveredDerivedProductStatusRow {
    family_identity: DerivedTopologyProductFamilyIdentity,
    status: CoveredDerivedProductMigrationStatus,
    ordinary_invalidation_consumable: bool,
    selected_plan_digest: Option<String>,
    execution_receipt_digest: Option<String>,
    proof_authority: Option<MigratedDerivedProductFamilyProofAuthority>,
    proof_digest: String,
    row_digest: String,
}

impl CoveredDerivedProductStatusRow {
    pub(crate) fn from_migrated_family_closeout(
        closeout: &MigratedDerivedProductFamilyCloseout,
    ) -> Self {
        Self::new(
            closeout.family_identity(),
            CoveredDerivedProductMigrationStatus::Migrated,
            true,
            Some(closeout.selected_plan_digest()),
            Some(closeout.execution_receipt_digest()),
            Some(closeout.proof_authority()),
            closeout.proof_digest(),
        )
    }

    fn certification_residue_only(
        family_identity: DerivedTopologyProductFamilyIdentity,
        proof_digest: &str,
    ) -> Self {
        Self::new(
            family_identity,
            CoveredDerivedProductMigrationStatus::CertificationResidueOnly,
            false,
            None,
            None,
            None,
            proof_digest,
        )
    }

    fn new(
        family_identity: DerivedTopologyProductFamilyIdentity,
        status: CoveredDerivedProductMigrationStatus,
        ordinary_invalidation_consumable: bool,
        selected_plan_digest: Option<&str>,
        execution_receipt_digest: Option<&str>,
        proof_authority: Option<MigratedDerivedProductFamilyProofAuthority>,
        proof_digest: &str,
    ) -> Self {
        let row_digest = super::super::super::catalog::catalog_digest([
            "worth-topo:covered-derived-product-status-row:v1".to_string(),
            format!("family:{}", family_identity.as_str()),
            format!("status:{}", status.as_str()),
            format!("ordinary-consumable:{ordinary_invalidation_consumable}"),
            format!("selected-plan:{}", selected_plan_digest.unwrap_or("none")),
            format!(
                "execution-receipt:{}",
                execution_receipt_digest.unwrap_or("none")
            ),
            format!(
                "proof-authority:{}",
                proof_authority
                    .map(MigratedDerivedProductFamilyProofAuthority::as_str)
                    .unwrap_or("none")
            ),
            format!("proof:{proof_digest}"),
        ]);
        Self {
            family_identity,
            status,
            ordinary_invalidation_consumable,
            selected_plan_digest: selected_plan_digest.map(str::to_string),
            execution_receipt_digest: execution_receipt_digest.map(str::to_string),
            proof_authority,
            proof_digest: proof_digest.to_string(),
            row_digest,
        }
    }

    pub const fn family_identity(&self) -> DerivedTopologyProductFamilyIdentity {
        self.family_identity
    }

    pub const fn status(&self) -> CoveredDerivedProductMigrationStatus {
        self.status
    }

    pub const fn ordinary_invalidation_consumable(&self) -> bool {
        self.ordinary_invalidation_consumable
    }

    pub fn selected_plan_digest(&self) -> Option<&str> {
        self.selected_plan_digest.as_deref()
    }

    pub fn execution_receipt_digest(&self) -> Option<&str> {
        self.execution_receipt_digest.as_deref()
    }

    pub const fn proof_authority(&self) -> Option<MigratedDerivedProductFamilyProofAuthority> {
        self.proof_authority
    }

    pub fn proof_digest(&self) -> &str {
        &self.proof_digest
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}
