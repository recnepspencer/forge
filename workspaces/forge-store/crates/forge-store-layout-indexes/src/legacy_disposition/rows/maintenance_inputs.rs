//! Historical maintenance declarations retained only as inputs to the maintenance owner.

use super::super::{LegacyAccessPathBypass as Bypass, LegacySurfaceInventoryRow};
use super::row::owner_input as input;

pub(super) const ROWS: &[LegacySurfaceInventoryRow] = &[
    input("MaintenanceDeclaration", Bypass::Lowering),
    input("AdmittedMaintenanceDeclaration", Bypass::Lowering),
    input(
        "AuthoritativeReclaimMaintenanceDeclaration",
        Bypass::Lowering,
    ),
    input("CompactionMaintenanceDeclaration", Bypass::Lowering),
    input(
        "DerivedFamilyRebuildMaintenanceDeclaration",
        Bypass::Lowering,
    ),
    input("MaintenanceAuditMaintenanceDeclaration", Bypass::Lowering),
    input("MaintenanceDeclarationClass", Bypass::Lowering),
    input("MaintenanceDeclarationId", Bypass::Lowering),
    input("RebuildMaintenanceDeclaration", Bypass::Lowering),
    input("ReclaimMaintenanceDeclaration", Bypass::Lowering),
    input(
        "ReplicationPreparationMaintenanceDeclaration",
        Bypass::Lowering,
    ),
    input("RetentionMaintenanceDeclaration", Bypass::Lowering),
    input("SnapshotRefreshMaintenanceDeclaration", Bypass::Lowering),
    input("TierMoveMaintenanceDeclaration", Bypass::Lowering),
    input("TierPlacementMaintenanceDeclaration", Bypass::Lowering),
];
