use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use topology::touched_graph_conflict::WorthTopologyTouchedGraphConflictSourceFirewallRegion;
use worth_spatial::touched_graph_conflict::WorthSpatialTouchedGraphConflictSourceFirewallRegion;

use super::declared_surface::{
    declared_private_identifier, declared_visible_identifier, ImplContext,
};
use super::forbidden_surface::WorthTouchedGraphConflictForbiddenSurface;
use super::private_surface_registry::phase_twelve_private_surface_is_owned;
use super::report::{
    WorthTouchedGraphConflictSourceFirewallRegionReport,
    WorthTouchedGraphConflictSourceFirewallReport,
    WorthTouchedGraphConflictSourceFirewallViolation,
};
use super::semantic_source_registry::phase_fifteen_semantic_source_coverages;
use crate::workload_composition::{
    current_conflict_batch_admission_inventory, ConflictBatchAdmissionInventory,
    ConflictBatchAdmissionInventoryError,
};

const KERNEL_REGION_LABEL: &str = "kernel_workload_composition";
const KERNEL_ROOT_IDENTITY: &str = "worth-kernel:workload-composition";

#[derive(Default)]
struct SourceFirewallScanResult {
    covered_forbidden_surfaces: BTreeSet<WorthTouchedGraphConflictForbiddenSurface>,
    violations: Vec<WorthTouchedGraphConflictSourceFirewallViolation>,
}

pub fn current_worth_touched_graph_conflict_source_firewall_report(
) -> Result<WorthTouchedGraphConflictSourceFirewallReport, ConflictBatchAdmissionInventoryError> {
    static CACHE: OnceLock<WorthTouchedGraphConflictSourceFirewallReport> = OnceLock::new();
    if let Some(cached) = CACHE.get() {
        return Ok(cached.clone());
    }
    let inventory = current_conflict_batch_admission_inventory()?;
    let report = scan_roots(
        [
            (
                KERNEL_REGION_LABEL,
                KERNEL_ROOT_IDENTITY,
                PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/workload_composition"),
            ),
            (
                WorthTopologyTouchedGraphConflictSourceFirewallRegion::region_label(),
                WorthTopologyTouchedGraphConflictSourceFirewallRegion::root_identity(),
                WorthTopologyTouchedGraphConflictSourceFirewallRegion::scan_root(),
            ),
            (
                WorthSpatialTouchedGraphConflictSourceFirewallRegion::region_label(),
                WorthSpatialTouchedGraphConflictSourceFirewallRegion::root_identity(),
                WorthSpatialTouchedGraphConflictSourceFirewallRegion::scan_root(),
            ),
        ],
        &inventory,
    )?;
    let _ = CACHE.set(report.clone());
    Ok(report)
}

pub(crate) fn scan_worth_touched_graph_conflict_source_firewall_region_for_tests(
    region_label: &str,
    root_identity: &str,
    root: &Path,
) -> Result<WorthTouchedGraphConflictSourceFirewallReport, ConflictBatchAdmissionInventoryError> {
    let inventory = current_conflict_batch_admission_inventory()?;
    scan_roots(
        [(region_label, root_identity, root.to_path_buf())],
        &inventory,
    )
}

fn scan_roots<'a, const N: usize>(
    roots: [(&'a str, &'a str, PathBuf); N],
    inventory: &ConflictBatchAdmissionInventory,
) -> Result<WorthTouchedGraphConflictSourceFirewallReport, ConflictBatchAdmissionInventoryError> {
    let mut region_reports = Vec::new();
    let mut violations = Vec::new();
    for (label, root_identity, absolute_root) in roots {
        let mut scanned_source_count = 0;
        let scan_result = scan_dir(&absolute_root, &mut scanned_source_count, inventory)?;
        let covered_forbidden_surfaces =
            inventory_covered_forbidden_surfaces(inventory, root_identity)
                .into_iter()
                .chain(scan_result.covered_forbidden_surfaces.into_iter())
                .collect::<BTreeSet<_>>();
        let forbidden_surfaces = scan_result
            .violations
            .iter()
            .map(WorthTouchedGraphConflictSourceFirewallViolation::forbidden_surface)
            .collect::<BTreeSet<_>>();
        violations.extend(scan_result.violations.iter().cloned().map(|violation| {
            WorthTouchedGraphConflictSourceFirewallViolation::new(
                label,
                violation.source_path(),
                violation.surface_name(),
                violation.forbidden_surface(),
            )
        }));
        region_reports.push(WorthTouchedGraphConflictSourceFirewallRegionReport::new(
            label,
            root_identity,
            scanned_source_count,
            covered_forbidden_surfaces,
            forbidden_surfaces,
            scan_result.violations.len(),
        ));
    }
    Ok(WorthTouchedGraphConflictSourceFirewallReport::new(
        region_reports,
        violations,
    ))
}

fn scan_dir(
    dir: &Path,
    scanned_source_count: &mut usize,
    inventory: &ConflictBatchAdmissionInventory,
) -> Result<SourceFirewallScanResult, ConflictBatchAdmissionInventoryError> {
    let mut result = SourceFirewallScanResult::default();
    let entries = fs::read_dir(dir).map_err(|error| {
        ConflictBatchAdmissionInventoryError::SourceFirewallViolation(format!(
            "cannot scan {}: {error}",
            dir.display()
        ))
    })?;
    for entry in entries {
        let path = entry
            .map_err(|error| {
                ConflictBatchAdmissionInventoryError::SourceFirewallViolation(error.to_string())
            })?
            .path();
        if path.is_dir() {
            let nested = scan_dir(&path, scanned_source_count, inventory)?;
            result
                .covered_forbidden_surfaces
                .extend(nested.covered_forbidden_surfaces);
            result.violations.extend(nested.violations);
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            *scanned_source_count += 1;
            if is_ignored_closeout_path(&path) {
                continue;
            }
            let file_result = scan_file(&path, inventory)?;
            result
                .covered_forbidden_surfaces
                .extend(file_result.covered_forbidden_surfaces);
            result.violations.extend(file_result.violations);
        }
    }
    Ok(result)
}

fn scan_file(
    path: &Path,
    inventory: &ConflictBatchAdmissionInventory,
) -> Result<SourceFirewallScanResult, ConflictBatchAdmissionInventoryError> {
    let text = fs::read_to_string(path).map_err(|error| {
        ConflictBatchAdmissionInventoryError::SourceFirewallViolation(format!(
            "cannot read {}: {error}",
            path.display()
        ))
    })?;
    let normalized_path = path.to_string_lossy().replace('\\', "/");
    let coverages = phase_fifteen_semantic_source_coverages()
        .iter()
        .copied()
        .filter(|coverage| coverage.matches_path(&normalized_path))
        .collect::<Vec<_>>();
    if coverages.is_empty() {
        return Ok(SourceFirewallScanResult::default());
    }
    let mut result = SourceFirewallScanResult::default();
    result.covered_forbidden_surfaces.extend(
        coverages
            .iter()
            .map(|coverage| coverage.forbidden_surface()),
    );
    let mut impl_context = ImplContext::default();
    for line in text.lines() {
        impl_context.observe_opening(line);
        let visible_identifier = declared_visible_identifier(line, impl_context.current_owner());
        let private_identifier = declared_private_identifier(line, impl_context.current_owner());
        if let Some(identifier) = visible_identifier {
            for coverage in &coverages {
                if coverage_contains_allowed_surface(
                    coverage,
                    inventory,
                    &normalized_path,
                    coverage.forbidden_surface(),
                    &identifier,
                    true,
                ) || inventory_contains_owned_surface(
                    inventory,
                    &normalized_path,
                    &identifier,
                    true,
                ) {
                    continue;
                }
                result
                    .violations
                    .push(WorthTouchedGraphConflictSourceFirewallViolation::new(
                        "scanned_region",
                        normalized_path.clone(),
                        identifier.clone(),
                        coverage.forbidden_surface(),
                    ));
            }
        }
        let Some(identifier) = private_identifier else {
            impl_context.observe_closing(line);
            continue;
        };
        let explicitly_allowed_private_surface = coverages.iter().any(|coverage| {
            coverage_contains_allowed_surface(
                coverage,
                inventory,
                &normalized_path,
                coverage.forbidden_surface(),
                &identifier,
                true,
            )
        });
        if explicitly_allowed_private_surface
            || phase_twelve_private_surface_is_owned(&normalized_path, &identifier)
            || inventory_contains_owned_surface(inventory, &normalized_path, &identifier, true)
        {
            impl_context.observe_closing(line);
            continue;
        }
        for coverage in &coverages {
            result
                .violations
                .push(WorthTouchedGraphConflictSourceFirewallViolation::new(
                    "scanned_region",
                    normalized_path.clone(),
                    identifier.clone(),
                    coverage.forbidden_surface(),
                ));
        }
        impl_context.observe_closing(line);
    }
    Ok(result)
}

fn coverage_contains_allowed_surface(
    coverage: &super::semantic_source_registry::SemanticSourceCoverage,
    inventory: &ConflictBatchAdmissionInventory,
    source_path: &str,
    forbidden_surface: WorthTouchedGraphConflictForbiddenSurface,
    identifier: &str,
    allow_owned_type_members: bool,
) -> bool {
    let normalized_source_path = source_path.replace('\\', "/");
    let explicitly_owned_path = coverage.matches_path(&normalized_source_path)
        || path_matches_any_owned_file(&normalized_source_path, coverage.explicit_owned_paths());
    if explicitly_owned_path
        && coverage.explicit_allowed_surfaces().iter().any(|allowed| {
            allowed_surface_matches_identifier(allowed, identifier, allow_owned_type_members)
        })
    {
        return true;
    }
    inventory.rows().iter().any(|row| {
        let covered = WorthTouchedGraphConflictForbiddenSurface::from_surface_identity(
            row.surface_identity(),
        );
        covered == Some(forbidden_surface)
            && path_matches_inventory_source(&normalized_source_path, row.source_path())
            && allowed_surface_matches_identifier(
                row.surface_name(),
                identifier,
                allow_owned_type_members,
            )
    })
}

fn path_matches_any_owned_file(source_path: &str, owned_paths: &[&str]) -> bool {
    owned_paths
        .iter()
        .any(|owned_path| path_matches_inventory_source(source_path, owned_path))
}

fn inventory_contains_owned_surface(
    inventory: &ConflictBatchAdmissionInventory,
    source_path: &str,
    identifier: &str,
    allow_owned_type_members: bool,
) -> bool {
    let normalized_source_path = source_path.replace('\\', "/");
    inventory.rows().iter().any(|row| {
        path_matches_inventory_source(&normalized_source_path, row.source_path())
            && allowed_surface_matches_identifier(
                row.surface_name(),
                identifier,
                allow_owned_type_members,
            )
    })
}

fn allowed_surface_matches_identifier(
    allowed_surface: &str,
    identifier: &str,
    allow_owned_type_members: bool,
) -> bool {
    if allowed_surface == identifier {
        return true;
    }
    if allow_owned_type_members && identifier.starts_with(&format!("{allowed_surface}::")) {
        return true;
    }
    let Some((type_name, _)) = allowed_surface.split_once("::") else {
        return false;
    };
    identifier == type_name
        || (allow_owned_type_members && identifier.starts_with(&format!("{type_name}::")))
}

fn path_matches_inventory_source(scanned_source_path: &str, inventory_source_path: &str) -> bool {
    let normalized_inventory_path = inventory_source_path.replace('\\', "/");
    scanned_source_path == normalized_inventory_path
        || scanned_source_path.ends_with(&normalized_inventory_path)
}

fn inventory_covered_forbidden_surfaces(
    inventory: &ConflictBatchAdmissionInventory,
    root_identity: &str,
) -> BTreeSet<WorthTouchedGraphConflictForbiddenSurface> {
    inventory
        .rows()
        .iter()
        .filter(|row| root_identity_matches_source_path(root_identity, row.source_path()))
        .filter_map(|row| {
            WorthTouchedGraphConflictForbiddenSurface::from_surface_identity(row.surface_identity())
        })
        .collect()
}

fn root_identity_matches_source_path(root_identity: &str, source_path: &str) -> bool {
    match root_identity {
        KERNEL_ROOT_IDENTITY => {
            source_path.contains("crates/worth-kernel/src/workload_composition/")
        }
        "worth-topo:touched-graph-conflict" => source_path.contains("crates/worth-topo/src/"),
        "worth-spatial:touched-graph-conflict" => source_path.contains("crates/worth-spatial/src/"),
        _ => false,
    }
}

fn is_ignored_closeout_path(path: &Path) -> bool {
    let normalized = path.to_string_lossy().replace('\\', "/");
    normalized.contains("/conflict_batch_admission_inventory/")
        || normalized.contains("/compile_fail/")
        || normalized.contains("/public_closeout_seed_support/")
        || normalized.contains("/test_support/")
        || normalized.contains("/tests/")
        || normalized.ends_with("_tests.rs")
        || normalized.ends_with("/tests.rs")
}
