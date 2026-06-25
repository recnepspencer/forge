use std::collections::BTreeSet;
use std::fmt;

use serde::Serialize;

use super::{
    CoveredDerivedProductMigrationCounters, CoveredDerivedProductMigrationStatus,
    CoveredDerivedProductPhaseSevenSeed, CoveredDerivedProductStatusRow,
};
use crate::derived_topology::invalidation_plan::catalog::DerivedTopologyProductFamilyIdentity;
use crate::derived_topology::invalidation_plan::migrated_products::MigratedDerivedProductFamilyProofAuthority;
use crate::derived_topology::invalidation_plan::selection::DerivedInvalidationSelectedPlan;

pub fn close_covered_derived_product_migration_sweep(
    selected_plan: &DerivedInvalidationSelectedPlan,
    status_rows: Vec<CoveredDerivedProductStatusRow>,
) -> Result<CoveredDerivedProductMigrationSweepCloseout, CoveredDerivedProductMigrationError> {
    CoveredDerivedProductMigrationSweepCloseout::close(selected_plan, status_rows)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoveredDerivedProductMigrationError {
    MissingRequiredFamily,
    DuplicateFamilyStatus,
    SelectedFamilyNotMigrated,
    PlaceholderStatusCannotClose,
    RequiredFamilyNotOrdinaryConsumable,
    RequiredFamilyProofNotFamilySpecific,
    RequiredFamilySelectedPlanMismatch,
}

impl fmt::Display for CoveredDerivedProductMigrationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingRequiredFamily => {
                write!(f, "covered product sweep missed a required family")
            }
            Self::DuplicateFamilyStatus => {
                write!(f, "covered product sweep carried duplicate family status")
            }
            Self::SelectedFamilyNotMigrated => {
                write!(f, "selected covered product family was not migrated")
            }
            Self::PlaceholderStatusCannotClose => {
                write!(
                    f,
                    "placeholder or non-consumable family status cannot close sweep"
                )
            }
            Self::RequiredFamilyNotOrdinaryConsumable => {
                write!(
                    f,
                    "covered product sweep requires every family-specific proof to be ordinary consumable"
                )
            }
            Self::RequiredFamilyProofNotFamilySpecific => {
                write!(
                    f,
                    "covered product sweep requires every family to use a family-specific migration closeout proof"
                )
            }
            Self::RequiredFamilySelectedPlanMismatch => {
                write!(
                    f,
                    "covered product sweep requires every family proof to bind the selected plan"
                )
            }
        }
    }
}

impl std::error::Error for CoveredDerivedProductMigrationError {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CoveredDerivedProductMigrationSweepCloseout {
    selected_plan_digest: String,
    status_rows: Vec<CoveredDerivedProductStatusRow>,
    counters: CoveredDerivedProductMigrationCounters,
    phase_seven_seed: CoveredDerivedProductPhaseSevenSeed,
    closeout_digest: String,
}

impl CoveredDerivedProductMigrationSweepCloseout {
    fn close(
        selected_plan: &DerivedInvalidationSelectedPlan,
        status_rows: Vec<CoveredDerivedProductStatusRow>,
    ) -> Result<Self, CoveredDerivedProductMigrationError> {
        require_exact_required_family_coverage(&status_rows)?;
        require_all_required_families_ordinary_consumable(&status_rows)?;
        require_all_required_families_family_specific(&status_rows)?;
        require_all_required_families_bind_selected_plan(selected_plan, &status_rows)?;
        require_selected_families_migrated(selected_plan, &status_rows)?;
        reject_non_consumable_selected_status(selected_plan, &status_rows)?;

        let counters = CoveredDerivedProductMigrationCounters::from_rows(
            &status_rows,
            selected_plan.selected_rows().len(),
        );
        let closeout_digest = closeout_digest(selected_plan, &status_rows, &counters);
        let phase_seven_seed =
            CoveredDerivedProductPhaseSevenSeed::from_closeout(&closeout_digest, &counters);
        Ok(Self {
            selected_plan_digest: selected_plan.selected_plan_digest().to_string(),
            status_rows,
            counters,
            phase_seven_seed,
            closeout_digest,
        })
    }

    pub fn selected_plan_digest(&self) -> &str {
        &self.selected_plan_digest
    }

    pub fn status_rows(&self) -> &[CoveredDerivedProductStatusRow] {
        &self.status_rows
    }

    pub const fn counters(&self) -> &CoveredDerivedProductMigrationCounters {
        &self.counters
    }

    pub const fn phase_seven_seed(&self) -> &CoveredDerivedProductPhaseSevenSeed {
        &self.phase_seven_seed
    }

    pub fn closeout_digest(&self) -> &str {
        &self.closeout_digest
    }
}

fn require_all_required_families_ordinary_consumable(
    rows: &[CoveredDerivedProductStatusRow],
) -> Result<(), CoveredDerivedProductMigrationError> {
    if rows
        .iter()
        .any(|row| !row.ordinary_invalidation_consumable())
    {
        return Err(CoveredDerivedProductMigrationError::RequiredFamilyNotOrdinaryConsumable);
    }
    Ok(())
}

fn require_all_required_families_family_specific(
    rows: &[CoveredDerivedProductStatusRow],
) -> Result<(), CoveredDerivedProductMigrationError> {
    if rows.iter().any(|row| {
        row.proof_authority()
            != Some(MigratedDerivedProductFamilyProofAuthority::FamilySpecificMigrationCloseout)
    }) {
        return Err(CoveredDerivedProductMigrationError::RequiredFamilyProofNotFamilySpecific);
    }
    Ok(())
}

fn require_all_required_families_bind_selected_plan(
    selected_plan: &DerivedInvalidationSelectedPlan,
    rows: &[CoveredDerivedProductStatusRow],
) -> Result<(), CoveredDerivedProductMigrationError> {
    if rows
        .iter()
        .any(|row| row.selected_plan_digest() != Some(selected_plan.selected_plan_digest()))
    {
        return Err(CoveredDerivedProductMigrationError::RequiredFamilySelectedPlanMismatch);
    }
    Ok(())
}

fn require_exact_required_family_coverage(
    rows: &[CoveredDerivedProductStatusRow],
) -> Result<(), CoveredDerivedProductMigrationError> {
    let mut seen = BTreeSet::new();
    for row in rows {
        if !seen.insert(row.family_identity()) {
            return Err(CoveredDerivedProductMigrationError::DuplicateFamilyStatus);
        }
    }
    if DerivedTopologyProductFamilyIdentity::REQUIRED
        .iter()
        .any(|required| !seen.contains(required))
    {
        return Err(CoveredDerivedProductMigrationError::MissingRequiredFamily);
    }
    Ok(())
}

fn require_selected_families_migrated(
    selected_plan: &DerivedInvalidationSelectedPlan,
    rows: &[CoveredDerivedProductStatusRow],
) -> Result<(), CoveredDerivedProductMigrationError> {
    for selected in selected_plan.selected_rows() {
        let Some(row) = rows
            .iter()
            .find(|row| row.family_identity() == selected.family_identity())
        else {
            return Err(CoveredDerivedProductMigrationError::MissingRequiredFamily);
        };
        if row.status() != CoveredDerivedProductMigrationStatus::Migrated {
            return Err(CoveredDerivedProductMigrationError::SelectedFamilyNotMigrated);
        }
    }
    Ok(())
}

fn reject_non_consumable_selected_status(
    selected_plan: &DerivedInvalidationSelectedPlan,
    rows: &[CoveredDerivedProductStatusRow],
) -> Result<(), CoveredDerivedProductMigrationError> {
    for selected in selected_plan.selected_rows() {
        let row = rows
            .iter()
            .find(|row| row.family_identity() == selected.family_identity())
            .ok_or(CoveredDerivedProductMigrationError::MissingRequiredFamily)?;
        if !row.ordinary_invalidation_consumable() {
            return Err(CoveredDerivedProductMigrationError::PlaceholderStatusCannotClose);
        }
    }
    Ok(())
}

fn closeout_digest(
    selected_plan: &DerivedInvalidationSelectedPlan,
    status_rows: &[CoveredDerivedProductStatusRow],
    counters: &CoveredDerivedProductMigrationCounters,
) -> String {
    let mut parts = vec![
        "worth-topo:covered-derived-product-migration-sweep-closeout:v1".to_string(),
        format!("selected-plan:{}", selected_plan.selected_plan_digest()),
        format!("counters:{}", counters.counters_digest()),
    ];
    parts.extend(
        status_rows
            .iter()
            .map(|row| format!("status:{}", row.row_digest())),
    );
    super::super::super::catalog::catalog_digest(parts)
}
