use std::collections::HashSet;

use super::types::{
    WorthGraphAuthorityAction, WorthGraphAuthorityDeletionLedgerRow,
    WorthGraphAuthorityDeletionTarget, WorthGraphAuthorityDiscoveryRecord,
    WorthGraphAuthorityInventoryRow, WorthGraphAuthorityRootFamily, WorthGraphAuthoritySourceScope,
    WorthLowerAuthorityPromotionCase, WorthLowerAuthorityPromotionGuardPlan,
};
use super::{
    gate_report_types::{WorthGraphAuthorityGateCounters, WorthGraphAuthorityGateReport},
    touched_graph_certification::{
        validate_touched_graph_authority_inventory, validate_touched_graph_deletion_ledger,
    },
    touched_graph_facade_audit::WorthTouchedGraphOrdinaryPublicFacadeExport,
    touched_graph_static_authority::WorthTouchedGraphStaticAuthorityEntry,
    WorthTouchedGraphAuthorityDeletionLedgerRow, WorthTouchedGraphAuthorityInventoryCategory,
    WorthTouchedGraphAuthorityInventoryRow,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthGraphAuthorityGateViolation {
    PrimitiveConstructionBirthExecutionNotCovered,
    DuplicateInventorySourceId(&'static str),
    EmptyInventoryField(&'static str),
    UnclassifiedAuditedSource(String),
    AuditedSourceSetManifestDrift(&'static str),
    DeletionLedgerSourceOutsideInventory(&'static str),
    DeletionTargetKeptWithoutDistinctProof(&'static str),
    ResidueWithoutOwnerCapOrRemovalTrigger(&'static str),
    QueryGapWithoutNamedBlocker(&'static str),
    DuplicateTouchedGraphInventorySourceId(&'static str),
    EmptyTouchedGraphInventoryField(&'static str),
    EmptyTouchedGraphDeletionField(&'static str),
    EmptyTouchedGraphStaticAuthorityField(&'static str),
    MissingTouchedGraphInventoryCategory(WorthTouchedGraphAuthorityInventoryCategory),
    MissingTouchedGraphStaticAuthorityInventoryRow(&'static str),
    TouchedGraphResidueWithoutCap(&'static str),
    TouchedGraphResiduePublicFacadePostureMismatch(&'static str),
    TouchedGraphDeletionSourceOutsideInventory(&'static str),
    TouchedGraphDeletionStillOrdinaryPublicFacade(&'static str),
    TouchedGraphDeletionStillExportedByFacade(&'static str),
    MissingRootDiscoveryRecord(WorthGraphAuthorityRootFamily),
    MissingLowerAuthorityPromotionGuard(WorthLowerAuthorityPromotionCase),
}

pub(crate) fn certify_worth_graph_authority_gate(
    inventory: Vec<WorthGraphAuthorityInventoryRow>,
    deletion_ledger: Vec<WorthGraphAuthorityDeletionLedgerRow>,
    touched_graph_inventory: Vec<WorthTouchedGraphAuthorityInventoryRow>,
    touched_graph_deletion_ledger: Vec<WorthTouchedGraphAuthorityDeletionLedgerRow>,
    touched_graph_static_authority_entries: Vec<WorthTouchedGraphStaticAuthorityEntry>,
    touched_graph_ordinary_public_facade_exports: Vec<WorthTouchedGraphOrdinaryPublicFacadeExport>,
    discovery_records: Vec<WorthGraphAuthorityDiscoveryRecord>,
    lower_authority_guard_plan: Vec<WorthLowerAuthorityPromotionGuardPlan>,
    audited_source_paths: &[String],
) -> Result<WorthGraphAuthorityGateReport, WorthGraphAuthorityGateViolation> {
    validate_inventory(&inventory, audited_source_paths)?;
    validate_deletion_ledger(&deletion_ledger, &inventory)?;
    validate_touched_graph_authority_inventory(
        &touched_graph_inventory,
        &touched_graph_static_authority_entries,
        &touched_graph_ordinary_public_facade_exports,
    )?;
    validate_touched_graph_deletion_ledger(
        &touched_graph_deletion_ledger,
        &touched_graph_inventory,
        &touched_graph_ordinary_public_facade_exports,
    )?;
    validate_root_discovery_records(&discovery_records)?;
    validate_lower_authority_guard_plan(&lower_authority_guard_plan)?;

    let graph_selection_counters = worth_graph_obligation_selection_counter_totals();
    let counters = WorthGraphAuthorityGateCounters {
        inventory_rows: inventory.len(),
        deletion_ledger_rows: deletion_ledger.len(),
        touched_graph_inventory_rows: touched_graph_inventory.len(),
        touched_graph_deletion_ledger_rows: touched_graph_deletion_ledger.len(),
        discovery_records: discovery_records.len(),
        lower_authority_guard_plans: lower_authority_guard_plan.len(),
        audited_sources: audited_source_paths.len(),
        graph_obligation_attempted_bucket_lookups: graph_selection_counters
            .attempted_bucket_lookups,
        graph_obligation_selected_rows: graph_selection_counters.selected_rows,
        graph_obligation_denied_rows: graph_selection_counters.denied_rows,
        graph_obligation_residue_rows: graph_selection_counters.residue_rows,
        graph_obligation_registration_full_scans: graph_selection_counters.registration_full_scans,
    };

    Ok(WorthGraphAuthorityGateReport {
        inventory,
        deletion_ledger,
        touched_graph_inventory,
        touched_graph_deletion_ledger,
        discovery_records,
        lower_authority_guard_plan,
        counters,
    })
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct WorthGraphObligationSelectionCounterTotals {
    attempted_bucket_lookups: usize,
    selected_rows: usize,
    denied_rows: usize,
    residue_rows: usize,
    registration_full_scans: usize,
}

fn worth_graph_obligation_selection_counter_totals() -> WorthGraphObligationSelectionCounterTotals {
    let selector_rows = crate::construction::graph_obligation_adoption::primitive_construction_graph_obligation_selector_precision_matrix()
        .expect("worth graph authority gate requires graph-obligation selector counters");
    let residue_rows = crate::construction::graph_obligation_adoption::primitive_construction_graph_obligation_residue_manifest()
        .expect("worth graph authority gate requires residue manifest counters")
        .rows()
        .len();

    WorthGraphObligationSelectionCounterTotals {
        attempted_bucket_lookups: selector_rows
            .iter()
            .map(|row| row.attempted_bucket_lookup_count())
            .sum(),
        selected_rows: selector_rows.iter().map(|row| row.selected_count()).sum(),
        denied_rows: selector_rows.iter().map(|row| row.denied_row_count()).sum(),
        residue_rows,
        registration_full_scans: selector_rows
            .iter()
            .map(|row| row.registration_full_scan_count())
            .sum(),
    }
}

fn validate_inventory(
    inventory: &[WorthGraphAuthorityInventoryRow],
    audited_source_paths: &[String],
) -> Result<(), WorthGraphAuthorityGateViolation> {
    let mut source_ids = HashSet::new();
    for row in inventory {
        if !source_ids.insert(row.source_id) {
            return Err(
                WorthGraphAuthorityGateViolation::DuplicateInventorySourceId(row.source_id),
            );
        }
        if row.action == WorthGraphAuthorityAction::QueryGap
            && row.replacement_or_blocker.is_empty()
        {
            return Err(
                WorthGraphAuthorityGateViolation::QueryGapWithoutNamedBlocker(row.source_id),
            );
        }
        if row.source_id.is_empty()
            || row.source_path.is_empty()
            || row.authority_claim.is_empty()
            || row.replacement_or_blocker.is_empty()
            || row.qa_evidence.is_empty()
        {
            return Err(WorthGraphAuthorityGateViolation::EmptyInventoryField(
                row.source_id,
            ));
        }
    }

    let mut covered_sources = HashSet::new();
    for row in inventory {
        match row.source_scope {
            WorthGraphAuthoritySourceScope::ExactSource => {
                if audited_source_paths
                    .iter()
                    .any(|audited_source_path| audited_source_path == row.source_path)
                {
                    covered_sources.insert(row.source_path.to_string());
                }
            }
            WorthGraphAuthoritySourceScope::AuditedSourceSet {
                expected_sources,
                manifest_digest,
            } => {
                let covered_by_set =
                    audited_sources_under_root(audited_source_paths, row.source_path);
                if covered_by_set.len() != expected_sources
                    || source_manifest_digest(&covered_by_set) != manifest_digest
                {
                    return Err(
                        WorthGraphAuthorityGateViolation::AuditedSourceSetManifestDrift(
                            row.source_id,
                        ),
                    );
                }
                covered_sources.extend(covered_by_set);
            }
        }
    }

    for audited_source_path in audited_source_paths {
        if !covered_sources.contains(audited_source_path) {
            return Err(WorthGraphAuthorityGateViolation::UnclassifiedAuditedSource(
                audited_source_path.clone(),
            ));
        }
    }

    Ok(())
}

fn validate_deletion_ledger(
    deletion_ledger: &[WorthGraphAuthorityDeletionLedgerRow],
    inventory: &[WorthGraphAuthorityInventoryRow],
) -> Result<(), WorthGraphAuthorityGateViolation> {
    for row in deletion_ledger {
        if row.target_id.is_empty()
            || row.source_path.is_empty()
            || row.replacement_or_blocker.is_empty()
            || row.qa_evidence.is_empty()
        {
            return Err(WorthGraphAuthorityGateViolation::EmptyInventoryField(
                row.target_id,
            ));
        }
        if row.deletion_target != WorthGraphAuthorityDeletionTarget::None
            && row.action == WorthGraphAuthorityAction::Keep
            && row.distinct_authority_proof.is_empty()
        {
            return Err(
                WorthGraphAuthorityGateViolation::DeletionTargetKeptWithoutDistinctProof(
                    row.target_id,
                ),
            );
        }
        if row.action == WorthGraphAuthorityAction::Residue
            && (row.residue_owner.is_empty()
                || row.residue_cap.is_empty()
                || row.introduced_phase.is_empty()
                || row.removal_trigger.is_empty())
        {
            return Err(
                WorthGraphAuthorityGateViolation::ResidueWithoutOwnerCapOrRemovalTrigger(
                    row.target_id,
                ),
            );
        }
        if row.action == WorthGraphAuthorityAction::QueryGap
            && row.replacement_or_blocker.is_empty()
        {
            return Err(
                WorthGraphAuthorityGateViolation::QueryGapWithoutNamedBlocker(row.target_id),
            );
        }
        if !inventory
            .iter()
            .any(|inventory_row| inventory_row_covers_ledger_source(inventory_row, row.source_path))
            && !row.source_path.starts_with("_docs/")
            && !row
                .source_path
                .contains("certification/public_facade_contracts")
        {
            return Err(
                WorthGraphAuthorityGateViolation::DeletionLedgerSourceOutsideInventory(
                    row.target_id,
                ),
            );
        }
    }
    Ok(())
}

fn validate_root_discovery_records(
    discovery_records: &[WorthGraphAuthorityDiscoveryRecord],
) -> Result<(), WorthGraphAuthorityGateViolation> {
    for family in WorthGraphAuthorityRootFamily::ALL {
        if !discovery_records.iter().any(|record| {
            record.root_family == family
                && !record.root_surface.is_empty()
                && !record.intentional_break.is_empty()
                && !record.downstream_compile_failures.is_empty()
                && !record.final_enforced_api.is_empty()
                && !record.qa_evidence.is_empty()
        }) {
            return Err(WorthGraphAuthorityGateViolation::MissingRootDiscoveryRecord(family));
        }
    }
    Ok(())
}

fn validate_lower_authority_guard_plan(
    guard_plan: &[WorthLowerAuthorityPromotionGuardPlan],
) -> Result<(), WorthGraphAuthorityGateViolation> {
    for promotion_case in WorthLowerAuthorityPromotionCase::ALL {
        if !guard_plan.iter().any(|plan| {
            plan.promotion_case == promotion_case
                && !plan.lower_authority_surface.is_empty()
                && !plan.required_authority_type.is_empty()
                && !plan.planned_compile_fail_path.is_empty()
                && !plan.enforcement_stage.is_empty()
                && !plan.qa_evidence.is_empty()
        }) {
            return Err(
                WorthGraphAuthorityGateViolation::MissingLowerAuthorityPromotionGuard(
                    promotion_case,
                ),
            );
        }
    }
    Ok(())
}

fn audited_sources_under_root(audited_source_paths: &[String], source_root: &str) -> Vec<String> {
    let mut covered_sources = audited_source_paths
        .iter()
        .filter(|audited_source_path| path_is_under_root(audited_source_path, source_root))
        .cloned()
        .collect::<Vec<_>>();
    covered_sources.sort();
    covered_sources
}

fn inventory_row_covers_ledger_source(
    inventory_row: &WorthGraphAuthorityInventoryRow,
    ledger_source_path: &str,
) -> bool {
    match inventory_row.source_scope {
        WorthGraphAuthoritySourceScope::ExactSource => {
            inventory_row.source_path == ledger_source_path
        }
        WorthGraphAuthoritySourceScope::AuditedSourceSet { .. } => {
            path_is_under_root(ledger_source_path, inventory_row.source_path)
        }
    }
}

fn path_is_under_root(path: &str, root: &str) -> bool {
    path == root
        || path
            .strip_prefix(root)
            .is_some_and(|suffix| suffix.starts_with('/'))
}

fn source_manifest_digest(source_paths: &[String]) -> u64 {
    let mut digest = 0xcbf29ce484222325_u64;
    for source_path in source_paths {
        for byte in source_path.as_bytes() {
            digest ^= u64::from(*byte);
            digest = digest.wrapping_mul(0x100000001b3);
        }
        digest ^= u64::from(b'\n');
        digest = digest.wrapping_mul(0x100000001b3);
    }
    digest
}
