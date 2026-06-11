use std::collections::HashSet;

use super::classification::{
    ReceiptPosture, SurfaceAuthority, SurfaceScope, TopologyPosture, WorkloadSurfaceId,
};
use super::decision::InventoryDecision;
use super::report::SeedInventoryRow;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum InventoryValidationErrorKind {
    DuplicateSurface,
    MissingHumanReason,
    MissingSourcePath,
    UnownedEndToEndClaim,
    UnitFixtureMarkedForElevation,
    WorkloadCandidateWithoutElevationDecision,
    WorkloadCandidateWithoutProductionReceipt,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InventoryValidationError {
    kind: InventoryValidationErrorKind,
    surface_id: WorkloadSurfaceId,
    message: String,
}

impl InventoryValidationError {
    fn new(
        kind: InventoryValidationErrorKind,
        surface_id: WorkloadSurfaceId,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            surface_id,
            message: message.into(),
        }
    }

    pub const fn kind(&self) -> &InventoryValidationErrorKind {
        &self.kind
    }

    pub const fn surface_id(&self) -> WorkloadSurfaceId {
        self.surface_id
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

pub fn validate_inventory_rows(rows: &[SeedInventoryRow]) -> Result<(), InventoryValidationError> {
    let mut seen = HashSet::new();
    for row in rows {
        let id = row.surface_id();
        if !seen.insert(id.as_str()) {
            return Err(InventoryValidationError::new(
                InventoryValidationErrorKind::DuplicateSurface,
                id,
                "Inventory contains the same seed or fixture surface more than once.",
            ));
        }
        validate_reason(row)?;
        validate_source_path(row)?;
        validate_workload_claim(row)?;
        validate_decision(row)?;
    }
    Ok(())
}

fn validate_reason(row: &SeedInventoryRow) -> Result<(), InventoryValidationError> {
    if row.classification().human_reason().is_empty() {
        return Err(InventoryValidationError::new(
            InventoryValidationErrorKind::MissingHumanReason,
            row.surface_id(),
            "Inventory decisions must include a human-readable reason.",
        ));
    }
    Ok(())
}

fn validate_source_path(row: &SeedInventoryRow) -> Result<(), InventoryValidationError> {
    if row.source_path().is_empty() {
        return Err(InventoryValidationError::new(
            InventoryValidationErrorKind::MissingSourcePath,
            row.surface_id(),
            "Inventory decisions must name the source path they classify.",
        ));
    }
    Ok(())
}

fn validate_decision(row: &SeedInventoryRow) -> Result<(), InventoryValidationError> {
    if matches!(row.decision(), InventoryDecision::ElevateToWorkloadPlatform)
        && matches!(
            row.classification().scope(),
            SurfaceScope::UnitSupportOnly | SurfaceScope::LegacyMigrationOnly
        )
    {
        return Err(InventoryValidationError::new(
            InventoryValidationErrorKind::UnitFixtureMarkedForElevation,
            row.surface_id(),
            "A unit-only or legacy fixture cannot be elevated as a workload source.",
        ));
    }
    if matches!(
        row.classification().scope(),
        SurfaceScope::WorkloadCandidate
    ) && !matches!(row.decision(), InventoryDecision::ElevateToWorkloadPlatform)
    {
        return Err(InventoryValidationError::new(
            InventoryValidationErrorKind::WorkloadCandidateWithoutElevationDecision,
            row.surface_id(),
            "A workload candidate must be explicitly marked for workload-platform elevation.",
        ));
    }
    Ok(())
}

fn validate_workload_claim(row: &SeedInventoryRow) -> Result<(), InventoryValidationError> {
    if !row.is_workload_candidate() {
        return Ok(());
    }
    let has_real_authority = matches!(
        row.classification().authority(),
        SurfaceAuthority::QueryBackedTopology | SurfaceAuthority::QueryBackedSpatialContract
    ) && !matches!(
        row.classification().topology_posture(),
        TopologyPosture::BypassesTopologyTruth
    );
    if !has_real_authority {
        return Err(InventoryValidationError::new(
            InventoryValidationErrorKind::UnownedEndToEndClaim,
            row.surface_id(),
            "A workload candidate must be backed by Query/topology authority, not a local fixture.",
        ));
    }
    if !matches!(
        row.classification().receipt_posture(),
        ReceiptPosture::ProductionOwned
    ) {
        return Err(InventoryValidationError::new(
            InventoryValidationErrorKind::WorkloadCandidateWithoutProductionReceipt,
            row.surface_id(),
            "A workload candidate must carry production-owned receipts.",
        ));
    }
    Ok(())
}
