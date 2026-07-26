use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use crate::topology::WorkspaceSourceInventory;

use super::{certification_surfaces, ledger};

const FACADE_ROOT: &str = "crates/worth-ui/src";
const RUNTIME_ROOT: &str = "crates/worth-ui-runtime/src";

pub(super) fn audit(
    inventory: &WorkspaceSourceInventory,
    document: &toml::Value,
) -> Result<(), String> {
    validate_header(document)?;
    audit_crate_boundaries(inventory, ledger::tables(document, "crate_boundary")?)?;
    audit_surfaces(inventory, ledger::tables(document, "surface")?)?;
    certification_surfaces::audit(
        inventory,
        ledger::tables(document, "certification_surface")?,
    )?;
    audit_lifecycle(document)?;
    audit_transitions(document)?;
    audit_future_insertions(document)?;
    audit_subsystems(inventory, ledger::tables(document, "subsystem")?)
}

fn validate_header(document: &toml::Value) -> Result<(), String> {
    if ledger::text(document, "schema")? != "worth-ui.milestone-3.10.1.facade-runtime.v1" {
        return Err("facade-runtime ledger schema should be v1".to_owned());
    }
    let product_entry = ledger::text(document, "canonical_product_entry")?;
    if !product_entry.contains("execute_mounted_frame") {
        return Err("canonical product entry should terminate at execute_mounted_frame".to_owned());
    }
    Ok(())
}

fn audit_surfaces(
    inventory: &WorkspaceSourceInventory,
    rows: &[toml::Value],
) -> Result<(), String> {
    let facade_root = Path::new(FACADE_ROOT).join("facade");
    let actual = inventory
        .rust_files_under(&facade_root)
        .filter_map(|source| {
            source
                .relative_path()
                .strip_prefix(FACADE_ROOT)
                .ok()
                .map(normalize)
        })
        .collect::<BTreeSet<_>>();
    let declared = rows
        .iter()
        .filter(|row| {
            ledger::text(row, "disposition")
                .map(|disposition| disposition != "removed")
                .unwrap_or(true)
        })
        .map(|row| ledger::text(row, "file").map(str::to_owned))
        .collect::<Result<BTreeSet<_>, _>>()?;
    if actual != declared {
        return Err(format!(
            "product facade files differ from the audience ledger; actual={actual:?}, declared={declared:?}"
        ));
    }

    for row in rows {
        let file = ledger::text(row, "file")?;
        for field in [
            "audience",
            "responsibility",
            "disposition",
            "fingerprint",
            "forbidden_shortcut",
        ] {
            ledger::text(row, field)?;
        }
        require_transition_exit(row, file)?;
        if ledger::text(row, "disposition")? == "removed" {
            if inventory.contains(Path::new(FACADE_ROOT).join(file)) {
                return Err(format!("removed product facade `{file}` still exists"));
            }
            continue;
        }
        let source = inventory.text(Path::new(FACADE_ROOT).join(file));
        reject_certification_export(file, source)?;
        let observed = ledger::fingerprint(source);
        let expected = ledger::text(row, "fingerprint")?;
        if observed != expected {
            return Err(format!(
                "`{file}` public allowlist fingerprint changed: {observed} != {expected}"
            ));
        }
        let syntax = syn::parse_file(source)
            .map_err(|error| format!("`{file}` should parse for public-surface audit: {error}"))?;
        if !syntax.items.iter().any(is_public_item) {
            return Err(format!("`{file}` should classify at least one public item"));
        }
    }
    Ok(())
}

pub(super) fn reject_certification_export(file: &str, source: &str) -> Result<(), String> {
    if source.contains("certification_support")
        || source.contains("certification_construction")
        || source.contains("worth_ui_certification")
    {
        return Err(format!("`{file}` exports certification-only authority"));
    }
    Ok(())
}

fn audit_crate_boundaries(
    inventory: &WorkspaceSourceInventory,
    rows: &[toml::Value],
) -> Result<(), String> {
    if rows.len() != 2 {
        return Err("crate-boundary inventory should classify DSL and runtime".to_owned());
    }
    let mut edges = BTreeMap::new();
    for row in rows {
        let crate_name = ledger::text(row, "crate")?;
        ledger::text(row, "authority")?;
        ledger::text(row, "forbidden_direction")?;
        let manifest = inventory.text(ledger::text(row, "manifest")?);
        let document = manifest
            .parse::<toml::Value>()
            .map_err(|error| format!("`{crate_name}` manifest should parse: {error}"))?;
        let dependencies = document
            .get("dependencies")
            .and_then(toml::Value::as_table)
            .into_iter()
            .flatten()
            .map(|(name, _)| name.as_str())
            .filter(|name| ["worth-ui-dsl", "worth-ui-runtime"].contains(name))
            .filter(|name| *name != crate_name)
            .map(str::to_owned)
            .collect::<BTreeSet<_>>();
        let allowed = ledger::strings(row, "allowed_peer_dependencies")?
            .into_iter()
            .map(str::to_owned)
            .collect::<BTreeSet<_>>();
        if dependencies != allowed {
            return Err(format!(
                "`{crate_name}` peer dependencies changed: actual={dependencies:?}, ledger={allowed:?}"
            ));
        }
        edges.insert(crate_name.to_owned(), dependencies);
    }
    reject_bidirectional_edges(&edges)
}

pub(super) fn reject_bidirectional_edges(
    edges: &BTreeMap<String, BTreeSet<String>>,
) -> Result<(), String> {
    for (source, targets) in edges {
        for target in targets {
            if edges
                .get(target)
                .is_some_and(|reverse| reverse.contains(source))
            {
                return Err(format!(
                    "crate boundary is bidirectional: `{source}` <-> `{target}`"
                ));
            }
        }
    }
    Ok(())
}

fn is_public_item(item: &syn::Item) -> bool {
    match item {
        syn::Item::Const(item) => matches!(item.vis, syn::Visibility::Public(_)),
        syn::Item::Enum(item) => matches!(item.vis, syn::Visibility::Public(_)),
        syn::Item::Fn(item) => matches!(item.vis, syn::Visibility::Public(_)),
        syn::Item::Mod(item) => matches!(item.vis, syn::Visibility::Public(_)),
        syn::Item::Static(item) => matches!(item.vis, syn::Visibility::Public(_)),
        syn::Item::Struct(item) => matches!(item.vis, syn::Visibility::Public(_)),
        syn::Item::Trait(item) => matches!(item.vis, syn::Visibility::Public(_)),
        syn::Item::TraitAlias(item) => matches!(item.vis, syn::Visibility::Public(_)),
        syn::Item::Type(item) => matches!(item.vis, syn::Visibility::Public(_)),
        syn::Item::Union(item) => matches!(item.vis, syn::Visibility::Public(_)),
        syn::Item::Use(item) => matches!(item.vis, syn::Visibility::Public(_)),
        _ => false,
    }
}

fn audit_lifecycle(document: &toml::Value) -> Result<(), String> {
    let rows = ledger::tables(document, "lifecycle_entry")?;
    let symbols = rows
        .iter()
        .map(|row| ledger::text(row, "symbol"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    for required in [
        "WorthUiApplicationBuilder",
        "WorthUiApp::launch",
        "WorthUiActiveApplicationSession::execute_mounted_frame",
        "WorthUiFrameworkTurnCertificationExt::execute_framework_turn",
    ] {
        if !symbols.contains(required) {
            return Err(format!("lifecycle inventory is missing `{required}`"));
        }
    }
    for row in rows {
        for field in [
            "current_route",
            "audience",
            "authority_in",
            "authority_out",
            "disposition",
        ] {
            ledger::text(row, field)?;
        }
        require_transition_exit(row, ledger::text(row, "symbol")?)?;
    }
    Ok(())
}

fn audit_transitions(document: &toml::Value) -> Result<(), String> {
    let rows = ledger::tables(document, "predecessor_route")?;
    if rows.len() < 4 {
        return Err(
            "predecessor deletion list should cover all known compatibility routes".to_owned(),
        );
    }
    for row in rows {
        ledger::text(row, "route")?;
        ledger::text(row, "reason")?;
        let exit = ledger::integer(row, "exit_phase")?;
        if !(2..=6).contains(&exit) {
            return Err(format!(
                "predecessor exit phase {exit} should be within migration"
            ));
        }
    }
    Ok(())
}

fn audit_future_insertions(document: &toml::Value) -> Result<(), String> {
    let rows = ledger::tables(document, "future_insertion")?;
    if rows.len() < 4 {
        return Err(
            "future insertion map should cover syntax, transport, runtime, and inspection"
                .to_owned(),
        );
    }
    for row in rows {
        for field in ["change", "owner", "insertion", "forbidden_location"] {
            ledger::text(row, field)?;
        }
    }
    Ok(())
}

fn audit_subsystems(
    inventory: &WorkspaceSourceInventory,
    rows: &[toml::Value],
) -> Result<(), String> {
    let declared_names = rows
        .iter()
        .map(|row| ledger::text(row, "name").map(str::to_owned))
        .collect::<Result<BTreeSet<_>, _>>()?;
    let actual_names = inventory
        .direct_entries_under(RUNTIME_ROOT)
        .filter(|path| inventory.absolute_path(path).is_dir())
        .filter_map(Path::file_name)
        .filter_map(|name| name.to_str())
        .filter(|name| declared_names.contains(*name))
        .map(str::to_owned)
        .collect::<BTreeSet<_>>();
    if actual_names != declared_names {
        return Err(format!(
            "runtime subsystem ledger does not map its named directories: actual={actual_names:?}, declared={declared_names:?}"
        ));
    }

    for row in rows {
        let name = ledger::text(row, "name")?;
        for field in ["owner", "target", "disposition"] {
            ledger::text(row, field)?;
        }
        require_transition_exit(row, name)?;
        let expected = ledger::strings(row, "allowed_dependencies")?
            .into_iter()
            .map(str::to_owned)
            .collect::<BTreeSet<_>>();
        let actual = subsystem_dependencies(inventory, name);
        if actual != expected {
            return Err(format!(
                "runtime subsystem `{name}` edges changed: actual={actual:?}, ledger={expected:?}"
            ));
        }
    }
    Ok(())
}

fn subsystem_dependencies(
    inventory: &WorkspaceSourceInventory,
    subsystem: &str,
) -> BTreeSet<String> {
    let root = Path::new(RUNTIME_ROOT).join(subsystem);
    let mut dependencies = BTreeSet::new();
    for source in inventory.rust_files_under(root) {
        if normalize(source.relative_path())
            .split('/')
            .any(|component| component == "tests")
        {
            continue;
        }
        scan_crate_paths(source.text(), &mut dependencies);
    }
    dependencies.remove(subsystem);
    dependencies
}

fn scan_crate_paths(source: &str, dependencies: &mut BTreeSet<String>) {
    for suffix in source.split("crate::").skip(1) {
        let dependency = suffix
            .chars()
            .take_while(|character| character.is_ascii_alphanumeric() || *character == '_')
            .collect::<String>();
        if !dependency.is_empty() {
            dependencies.insert(dependency);
        }
    }
}

fn require_transition_exit(row: &toml::Value, label: &str) -> Result<(), String> {
    if ["move", "split", "remove", "removed"].contains(&ledger::text(row, "disposition")?) {
        let phase = ledger::integer(row, "exit_phase")?;
        if !(2..=6).contains(&phase) {
            return Err(format!(
                "`{label}` transition exit phase {phase} is invalid"
            ));
        }
    }
    Ok(())
}

fn normalize(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}
