use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use super::classification::PlannerOwnedRoutingDisplacedLane as Lane;
use super::error::PlannerOwnedRoutingInventoryError;
use super::row::PlannerOwnedRoutingInventoryRow;

struct DisplacedLaneSurfaceAuditSpec {
    lane: Lane,
    root_mod_path: &'static str,
}

const DISPLACED_LANE_SURFACE_AUDIT_SPECS: &[DisplacedLaneSurfaceAuditSpec] = &[
    DisplacedLaneSurfaceAuditSpec {
        lane: Lane::KernelPublicCloseout,
        root_mod_path: "crates/worth-kernel/src/workload_composition/public_closeout/mod.rs",
    },
    DisplacedLaneSurfaceAuditSpec {
        lane: Lane::KernelSourceFirewall,
        root_mod_path: "crates/worth-kernel/src/workload_composition/source_firewall/mod.rs",
    },
    DisplacedLaneSurfaceAuditSpec {
        lane: Lane::TopoDiagnosticSurfaces,
        root_mod_path: "crates/worth-topo/src/projection/diagnostic_surfaces/mod.rs",
    },
    DisplacedLaneSurfaceAuditSpec {
        lane: Lane::TopoQueryBackedConsumerCutover,
        root_mod_path: "crates/worth-topo/src/projection/query_backed_consumer_cutover/mod.rs",
    },
    DisplacedLaneSurfaceAuditSpec {
        lane: Lane::SpatialEvidenceLookupPublicCloseout,
        root_mod_path:
            "crates/worth-spatial/src/workload_platform/evidence_lookup_public_closeout/mod.rs",
    },
];

const PUBLIC_ITEM_PREFIXES: &[&str] = &[
    "pub struct ",
    "pub enum ",
    "pub fn ",
    "pub type ",
    "pub trait ",
    "pub const ",
];

pub(super) fn ensure_inventory_matches_live_sources(
    rows: &[PlannerOwnedRoutingInventoryRow],
) -> Result<(), PlannerOwnedRoutingInventoryError> {
    ensure_row_tokens_present(rows)?;
    ensure_displaced_lane_surface_coverage(rows)?;
    Ok(())
}

fn ensure_row_tokens_present(
    rows: &[PlannerOwnedRoutingInventoryRow],
) -> Result<(), PlannerOwnedRoutingInventoryError> {
    for row in rows {
        let contents = load_source(row.source_path())?;
        if !contents.contains(row.scan_token()) {
            return Err(PlannerOwnedRoutingInventoryError::MissingSourceToken {
                source_path: row.source_path(),
                token: row.scan_token(),
            });
        }
    }
    Ok(())
}

fn ensure_displaced_lane_surface_coverage(
    rows: &[PlannerOwnedRoutingInventoryRow],
) -> Result<(), PlannerOwnedRoutingInventoryError> {
    for spec in DISPLACED_LANE_SURFACE_AUDIT_SPECS {
        let covered_surfaces = load_displaced_lane_covered_surfaces(spec.root_mod_path)?;
        for surface in covered_surfaces {
            if !rows.iter().any(|row| {
                row.displaced_lane() == spec.lane && row.surface_name() == surface.as_str()
            }) {
                return Err(
                    PlannerOwnedRoutingInventoryError::MissingInventoryRowForCoveredSurface {
                        lane: spec.lane,
                        token: Box::leak(surface.into_boxed_str()),
                    },
                );
            }
        }
    }
    Ok(())
}

fn load_displaced_lane_covered_surfaces(
    root_mod_path: &'static str,
) -> Result<Vec<String>, PlannerOwnedRoutingInventoryError> {
    let mut covered_surfaces = BTreeSet::new();
    covered_surfaces.extend(load_public_exports(root_mod_path)?);
    covered_surfaces.extend(load_public_item_tokens(root_mod_path)?);

    let mut visited_modules = BTreeSet::new();
    for child_module_path in load_crate_visible_child_module_paths(root_mod_path)? {
        collect_crate_visible_module_surfaces(
            &child_module_path,
            &mut visited_modules,
            &mut covered_surfaces,
        )?;
    }

    Ok(covered_surfaces.into_iter().collect())
}

fn collect_crate_visible_module_surfaces(
    source_path: &str,
    visited_modules: &mut BTreeSet<String>,
    covered_surfaces: &mut BTreeSet<String>,
) -> Result<(), PlannerOwnedRoutingInventoryError> {
    if !visited_modules.insert(source_path.to_string()) {
        return Ok(());
    }

    covered_surfaces.extend(load_public_exports(source_path)?);
    covered_surfaces.extend(load_public_item_tokens(source_path)?);

    for child_module_path in load_crate_visible_child_module_paths(source_path)? {
        collect_crate_visible_module_surfaces(
            &child_module_path,
            visited_modules,
            covered_surfaces,
        )?;
    }

    Ok(())
}

fn load_public_exports(
    source_path: &str,
) -> Result<Vec<String>, PlannerOwnedRoutingInventoryError> {
    let contents = load_source(source_path)?;
    let mut exports = Vec::new();
    let mut statement = String::new();

    for line in contents.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with("#[") {
            continue;
        }

        statement.push_str(trimmed);
        statement.push(' ');

        if !trimmed.ends_with(';') {
            continue;
        }

        if statement.starts_with("pub use ") && !statement.starts_with("pub(crate) use ") {
            exports.extend(parse_pub_use_statement(source_path, &statement)?);
        }

        statement.clear();
    }

    Ok(exports)
}

fn load_public_item_tokens(
    source_path: &str,
) -> Result<Vec<String>, PlannerOwnedRoutingInventoryError> {
    let contents = load_source(source_path)?;
    let mut tokens = Vec::new();

    for line in contents.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with("#[") {
            continue;
        }

        if let Some(token) = parse_public_item_token(trimmed) {
            tokens.push(token);
        }
    }

    Ok(tokens)
}

fn load_crate_visible_child_module_paths(
    source_path: &str,
) -> Result<Vec<String>, PlannerOwnedRoutingInventoryError> {
    let contents = load_source(source_path)?;
    let mut child_module_paths = Vec::new();

    for line in contents.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with("#[") {
            continue;
        }

        if let Some(module_name) = parse_crate_visible_child_module_name(trimmed) {
            child_module_paths.push(resolve_child_module_path(source_path, module_name)?);
        }
    }

    Ok(child_module_paths)
}

fn parse_public_item_token(line: &str) -> Option<String> {
    for prefix in PUBLIC_ITEM_PREFIXES {
        if let Some(remainder) = line.strip_prefix(prefix) {
            let token = remainder
                .chars()
                .take_while(|ch| ch.is_ascii_alphanumeric() || *ch == '_')
                .collect::<String>();
            if !token.is_empty() {
                return Some(token);
            }
        }
    }

    None
}

fn parse_crate_visible_child_module_name(line: &str) -> Option<&str> {
    let remainder = line
        .strip_prefix("pub(crate) mod ")
        .or_else(|| line.strip_prefix("pub mod "))?;
    let module_name = remainder.strip_suffix(';')?.trim();
    if module_name.is_empty() {
        return None;
    }

    Some(module_name)
}

fn parse_pub_use_statement(
    source_path: &str,
    statement: &str,
) -> Result<Vec<String>, PlannerOwnedRoutingInventoryError> {
    let statement = statement.trim();
    let statement = statement
        .strip_prefix("pub use ")
        .and_then(|value| value.strip_suffix(';'))
        .ok_or_else(|| PlannerOwnedRoutingInventoryError::ExportParseFailure {
            source_path: leak_str(source_path),
            statement: statement.to_string(),
        })?;

    let mut exports = Vec::new();
    if let Some((_, tail)) = statement.rsplit_once("::{") {
        let inner = tail.strip_suffix('}').ok_or_else(|| {
            PlannerOwnedRoutingInventoryError::ExportParseFailure {
                source_path: leak_str(source_path),
                statement: statement.to_string(),
            }
        })?;
        for item in inner.split(',') {
            let export = item.trim();
            if export.is_empty() {
                continue;
            }
            exports.push(export_name(source_path, export)?);
        }
        return Ok(exports);
    }

    exports.push(export_name(source_path, statement)?);
    Ok(exports)
}

fn export_name(
    source_path: &str,
    export: &str,
) -> Result<String, PlannerOwnedRoutingInventoryError> {
    let export = export.trim();
    if export == "*" {
        return Err(PlannerOwnedRoutingInventoryError::ExportParseFailure {
            source_path: leak_str(source_path),
            statement: export.to_string(),
        });
    }

    let export = match export.rsplit_once(" as ") {
        Some((_, alias)) => alias.trim(),
        None => export.rsplit("::").next().unwrap_or(export).trim(),
    };

    if export.is_empty() {
        return Err(PlannerOwnedRoutingInventoryError::ExportParseFailure {
            source_path: leak_str(source_path),
            statement: export.to_string(),
        });
    }

    Ok(export.to_string())
}

fn resolve_child_module_path(
    source_path: &str,
    module_name: &str,
) -> Result<String, PlannerOwnedRoutingInventoryError> {
    let source_path = Path::new(source_path);
    let module_dir = if source_path.file_name().and_then(|name| name.to_str()) == Some("mod.rs") {
        source_path.parent().unwrap_or(source_path).to_path_buf()
    } else {
        source_path
            .parent()
            .unwrap_or(source_path)
            .join(source_path.file_stem().unwrap_or_default())
    };

    let direct_child = module_dir.join(format!("{module_name}.rs"));
    if workspace_root().join(&direct_child).is_file() {
        return Ok(path_to_string(&direct_child));
    }

    let nested_child = module_dir.join(module_name).join("mod.rs");
    if workspace_root().join(&nested_child).is_file() {
        return Ok(path_to_string(&nested_child));
    }

    Err(PlannerOwnedRoutingInventoryError::SourceReadFailure {
        source_path: leak_str(source_path.to_string_lossy().as_ref()),
        reason: format!("failed to resolve crate-visible child module `{module_name}`"),
    })
}

fn path_to_string(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

fn load_source(source_path: &str) -> Result<String, PlannerOwnedRoutingInventoryError> {
    let absolute = workspace_root().join(source_path);
    fs::read_to_string(&absolute).map_err(|error| {
        PlannerOwnedRoutingInventoryError::SourceReadFailure {
            source_path: leak_str(source_path),
            reason: error.to_string(),
        }
    })
}

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..")
}

fn leak_str(value: &str) -> &'static str {
    Box::leak(value.to_string().into_boxed_str())
}

#[cfg(test)]
pub(super) fn displaced_lane_covered_surfaces(
    lane: Lane,
) -> Result<Vec<String>, PlannerOwnedRoutingInventoryError> {
    let spec = DISPLACED_LANE_SURFACE_AUDIT_SPECS
        .iter()
        .find(|spec| spec.lane == lane)
        .ok_or(PlannerOwnedRoutingInventoryError::MissingDisplacedLanePath(
            lane.path(),
        ))?;
    load_displaced_lane_covered_surfaces(spec.root_mod_path)
}
