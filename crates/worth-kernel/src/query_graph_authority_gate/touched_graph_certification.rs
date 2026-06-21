use std::collections::HashSet;

use super::touched_graph_facade_audit::WorthTouchedGraphOrdinaryPublicFacadeExport;
use super::touched_graph_inventory::{
    CAPPED_RESIDUE_PUBLIC_FACADE, NO_ORDINARY_PUBLIC_FACADE, SEALED_FROM_ORDINARY_FACADE,
};
use super::touched_graph_static_authority::WorthTouchedGraphStaticAuthorityEntry;
use super::touched_graph_types::{
    WorthTouchedGraphAuthorityDeletionLedgerRow, WorthTouchedGraphAuthorityDisposition,
    WorthTouchedGraphAuthorityInventoryCategory, WorthTouchedGraphAuthorityInventoryRow,
};
use super::WorthGraphAuthorityGateViolation;

pub(crate) fn validate_touched_graph_authority_inventory(
    inventory: &[WorthTouchedGraphAuthorityInventoryRow],
    static_authority_entries: &[WorthTouchedGraphStaticAuthorityEntry],
    ordinary_public_facade_exports: &[WorthTouchedGraphOrdinaryPublicFacadeExport],
) -> Result<(), WorthGraphAuthorityGateViolation> {
    let mut source_ids = HashSet::new();
    for row in inventory {
        if !source_ids.insert(row.source_id()) {
            return Err(
                WorthGraphAuthorityGateViolation::DuplicateTouchedGraphInventorySourceId(
                    row.source_id(),
                ),
            );
        }
        if row.source_id().is_empty()
            || row.source_path().is_empty()
            || row.owner().is_empty()
            || row.current_authority_source().is_empty()
            || row.touched_graph_replacement().is_empty()
            || row.removal_trigger().is_empty()
            || row.ordinary_public_facade().is_empty()
            || row.qa_evidence().is_empty()
        {
            return Err(
                WorthGraphAuthorityGateViolation::EmptyTouchedGraphInventoryField(row.source_id()),
            );
        }
        validate_inventory_disposition(row, ordinary_public_facade_exports)?;
    }

    for category in WorthTouchedGraphAuthorityInventoryCategory::ALL {
        if !inventory.iter().any(|row| row.category() == *category) {
            return Err(
                WorthGraphAuthorityGateViolation::MissingTouchedGraphInventoryCategory(*category),
            );
        }
    }

    validate_static_authority_manifest(inventory, static_authority_entries)?;

    Ok(())
}

pub(crate) fn validate_touched_graph_deletion_ledger(
    deletion_ledger: &[WorthTouchedGraphAuthorityDeletionLedgerRow],
    inventory: &[WorthTouchedGraphAuthorityInventoryRow],
    ordinary_public_facade_exports: &[WorthTouchedGraphOrdinaryPublicFacadeExport],
) -> Result<(), WorthGraphAuthorityGateViolation> {
    for row in deletion_ledger {
        if row.target_id().is_empty()
            || row.source_id().is_empty()
            || row.source_path().is_empty()
            || row.owner().is_empty()
            || row.former_public_surface().is_empty()
            || row.enforced_outcome().is_empty()
            || row.touched_graph_replacement().is_empty()
            || row.removal_trigger().is_empty()
            || row.ordinary_public_facade().is_empty()
            || row.qa_evidence().is_empty()
        {
            return Err(
                WorthGraphAuthorityGateViolation::EmptyTouchedGraphDeletionField(row.target_id()),
            );
        }
        validate_deletion_disposition(row, ordinary_public_facade_exports)?;
        if !inventory
            .iter()
            .any(|inventory_row| inventory_row.source_id() == row.source_id())
        {
            return Err(
                WorthGraphAuthorityGateViolation::TouchedGraphDeletionSourceOutsideInventory(
                    row.target_id(),
                ),
            );
        }
    }

    Ok(())
}

fn validate_static_authority_manifest(
    inventory: &[WorthTouchedGraphAuthorityInventoryRow],
    static_authority_entries: &[WorthTouchedGraphStaticAuthorityEntry],
) -> Result<(), WorthGraphAuthorityGateViolation> {
    for entry in static_authority_entries {
        if entry.source_id().is_empty()
            || entry.source_path().is_empty()
            || entry.registry_name().is_empty()
            || entry.authority_surface().is_empty()
        {
            return Err(
                WorthGraphAuthorityGateViolation::EmptyTouchedGraphStaticAuthorityField(
                    entry.source_id(),
                ),
            );
        }
        if !inventory
            .iter()
            .any(|row| row.source_id() == entry.source_id() && row.category() == entry.category())
        {
            return Err(
                WorthGraphAuthorityGateViolation::MissingTouchedGraphStaticAuthorityInventoryRow(
                    entry.source_id(),
                ),
            );
        }
    }

    Ok(())
}

fn validate_inventory_disposition(
    row: &WorthTouchedGraphAuthorityInventoryRow,
    ordinary_public_facade_exports: &[WorthTouchedGraphOrdinaryPublicFacadeExport],
) -> Result<(), WorthGraphAuthorityGateViolation> {
    match row.disposition() {
        WorthTouchedGraphAuthorityDisposition::Delete
        | WorthTouchedGraphAuthorityDisposition::Collapse
        | WorthTouchedGraphAuthorityDisposition::CertificationOnly
        | WorthTouchedGraphAuthorityDisposition::QueryGap => Ok(()),
        WorthTouchedGraphAuthorityDisposition::Residue => {
            if row.residue_cap().is_empty() || row.residue_cap() == "not-residue" {
                Err(
                    WorthGraphAuthorityGateViolation::TouchedGraphResidueWithoutCap(
                        row.source_id(),
                    ),
                )
            } else if ordinary_public_facade_exports
                .iter()
                .any(|facade_export| facade_export_matches_inventory_row(row, facade_export))
                && row.ordinary_public_facade() != CAPPED_RESIDUE_PUBLIC_FACADE
            {
                Err(
                    WorthGraphAuthorityGateViolation::TouchedGraphResiduePublicFacadePostureMismatch(
                        row.source_id(),
                    ),
                )
            } else {
                Ok(())
            }
        }
    }
}

fn validate_deletion_disposition(
    row: &WorthTouchedGraphAuthorityDeletionLedgerRow,
    ordinary_public_facade_exports: &[WorthTouchedGraphOrdinaryPublicFacadeExport],
) -> Result<(), WorthGraphAuthorityGateViolation> {
    match row.disposition() {
        WorthTouchedGraphAuthorityDisposition::Delete
        | WorthTouchedGraphAuthorityDisposition::Collapse => {
            if row.ordinary_public_facade() != SEALED_FROM_ORDINARY_FACADE
                && row.ordinary_public_facade() != NO_ORDINARY_PUBLIC_FACADE
            {
                Err(
                    WorthGraphAuthorityGateViolation::TouchedGraphDeletionStillOrdinaryPublicFacade(
                        row.target_id(),
                    ),
                )
            } else if ordinary_public_facade_exports
                .iter()
                .any(|facade_export| facade_export_matches_deletion_target(row, facade_export))
            {
                Err(
                    WorthGraphAuthorityGateViolation::TouchedGraphDeletionStillExportedByFacade(
                        row.target_id(),
                    ),
                )
            } else {
                Ok(())
            }
        }
        WorthTouchedGraphAuthorityDisposition::CertificationOnly
        | WorthTouchedGraphAuthorityDisposition::Residue
        | WorthTouchedGraphAuthorityDisposition::QueryGap => Ok(()),
    }
}

fn facade_export_matches_deletion_target(
    row: &WorthTouchedGraphAuthorityDeletionLedgerRow,
    facade_export: &WorthTouchedGraphOrdinaryPublicFacadeExport,
) -> bool {
    facade_export.source_path() == row.source_path()
        || facade_export.authority_source_path() == row.source_path()
        || facade_export
            .authority_source_path()
            .strip_prefix(row.source_path())
            .is_some_and(|suffix| suffix.starts_with('/'))
        || facade_export.exported_surface() == row.former_public_surface()
}

fn facade_export_matches_inventory_row(
    row: &WorthTouchedGraphAuthorityInventoryRow,
    facade_export: &WorthTouchedGraphOrdinaryPublicFacadeExport,
) -> bool {
    facade_export.source_path() == row.source_path()
        || facade_export.authority_source_path() == row.source_path()
        || facade_export
            .authority_source_path()
            .strip_prefix(row.source_path())
            .is_some_and(|suffix| suffix.starts_with('/'))
}
