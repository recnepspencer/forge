use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use crate::topology::WorkspaceSourceInventory;

use super::ledger;

const PRODUCT_ROOT: &str = "crates/worth-ui/src";

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct ProductExport {
    qualified_symbol: String,
    source: String,
}

pub(super) fn audit(
    inventory: &WorkspaceSourceInventory,
    document: &toml::Value,
) -> Result<(), String> {
    validate_header(document)?;
    let journeys = audit_journeys(inventory, ledger::tables(document, "journey")?)?;
    let declared = declared_exports(ledger::tables(document, "export_group")?, &journeys)?;
    let actual = actual_exports(inventory)?;
    if actual != declared {
        return Err(format!(
            "Phase 5 product API differs from its exact manifest; {}",
            describe_difference(&actual, &declared)
        ));
    }
    audit_product_root(inventory)
}

fn validate_header(document: &toml::Value) -> Result<(), String> {
    if ledger::text(document, "schema")? != "worth-ui.milestone-3.10.1.product-api.v1" {
        return Err("Phase 5 product API manifest schema should be v1".to_owned());
    }
    Ok(())
}

fn audit_journeys(
    inventory: &WorkspaceSourceInventory,
    rows: &[toml::Value],
) -> Result<BTreeMap<String, String>, String> {
    let mut journeys = BTreeMap::new();
    for row in rows {
        let id = ledger::text(row, "id")?;
        let audience = ledger::text(row, "audience")?;
        let path = ledger::text(row, "path")?;
        if id.trim().is_empty() || audience.trim().is_empty() {
            return Err(
                "product caller journeys require non-empty identity and audience".to_owned(),
            );
        }
        if journeys
            .insert(id.to_owned(), audience.to_owned())
            .is_some()
        {
            return Err(format!("duplicate product caller journey `{id}`"));
        }
        if !inventory.contains(path) {
            return Err(format!(
                "product caller journey `{id}` path `{path}` is absent"
            ));
        }
        let source = inventory.text(path);
        if !source.contains("worth_ui::facade") {
            return Err(format!(
                "product caller journey `{id}` does not enter the product facade"
            ));
        }
    }
    Ok(journeys)
}

fn declared_exports(
    rows: &[toml::Value],
    journeys: &BTreeMap<String, String>,
) -> Result<BTreeSet<ProductExport>, String> {
    let mut declared = BTreeSet::new();
    let mut owners = BTreeMap::<String, String>::new();
    for row in rows {
        let audience = ledger::text(row, "audience")?;
        let source = ledger::text(row, "source")?;
        let caller = ledger::text(row, "caller")?;
        for field in ["stability", "authority"] {
            if ledger::text(row, field)?.trim().is_empty() {
                return Err(format!("`{audience}` export group has empty `{field}`"));
            }
        }
        if journeys.get(caller).map(String::as_str) != Some(audience) {
            return Err(format!(
                "`{audience}` export group caller `{caller}` is absent or belongs to another audience"
            ));
        }
        let expected_source = format!("facade/{audience}.rs");
        if source != expected_source {
            return Err(format!(
                "`{audience}` exports should be owned by `{expected_source}`, not `{source}`"
            ));
        }
        for symbol in ledger::strings(row, "symbols")? {
            if symbol.trim().is_empty() {
                return Err(format!("`{audience}` manifest contains an empty symbol"));
            }
            if let Some(prior) = owners.insert(symbol.to_owned(), audience.to_owned()) {
                return Err(format!(
                    "product symbol `{symbol}` has duplicate audience ownership: `{prior}` and `{audience}`"
                ));
            }
            reject_audience_authority(audience, symbol)?;
            let export = ProductExport {
                qualified_symbol: format!("{audience}::{symbol}"),
                source: source.to_owned(),
            };
            if !declared.insert(export) {
                return Err(format!("duplicate product export `{audience}::{symbol}`"));
            }
        }
    }
    Ok(declared)
}

fn reject_audience_authority(audience: &str, symbol: &str) -> Result<(), String> {
    if audience == "app"
        && [
            "ActiveCanvas",
            "ActiveFrameworkTurn",
            "ActiveOrdinaryFrame",
            "ActiveRealtime",
            "ActiveVirtualized",
            "AllocationReplan",
            "Certification",
            "FrameTarget",
            "HostAdapter",
            "HostContract",
            "HostMeasurement",
            "HostSessionPlan",
            "MountedAllocation",
            "MountedLaneProjection",
            "MountedPreview",
            "OrdinaryPlan",
            "PlanRegion",
            "PresentationAttempt",
            "PreparedApplicationAuthority",
            "PreparedMountedFrame",
            "ReloadLowering",
            "ResizePreview",
            "RuntimeLaunch",
            "VirtualizedPlan",
            "WorthUiRuntime",
        ]
        .iter()
        .any(|forbidden| symbol.contains(forbidden))
    {
        return Err(format!(
            "app audience exports host, certification, or mid-protocol authority `{symbol}`"
        ));
    }
    if audience == "inspection"
        && (symbol.starts_with("Frozen")
            || [
                "Assembly",
                "Builder",
                "MaterializationBoundary",
                "Mutable",
                "Planner",
                "Reconstruction",
                "Store",
                "Writer",
            ]
            .iter()
            .any(|forbidden| symbol.contains(forbidden)))
    {
        return Err(format!(
            "inspection audience exports storage, materialization, or reconstruction authority `{symbol}`"
        ));
    }
    Ok(())
}

fn actual_exports(inventory: &WorkspaceSourceInventory) -> Result<BTreeSet<ProductExport>, String> {
    let module_source = inventory.text(Path::new(PRODUCT_ROOT).join("facade/mod.rs"));
    let syntax = syn::parse_file(module_source)
        .map_err(|error| format!("product facade module should parse: {error}"))?;
    let mut exports = BTreeSet::new();
    for item in syntax.items {
        let syn::Item::Mod(module) = item else {
            if is_public_item(&item) {
                return Err(
                    "product facade root may publish only named audience modules".to_owned(),
                );
            }
            continue;
        };
        if !matches!(module.vis, syn::Visibility::Public(_)) {
            continue;
        }
        if module.ident == "prelude" {
            return Err("product facade must not publish a prelude".to_owned());
        }
        let audience = module.ident.to_string();
        let relative_source = format!("facade/{audience}.rs");
        let path = Path::new(PRODUCT_ROOT).join(&relative_source);
        if !inventory.contains(&path) {
            return Err(format!(
                "public product audience `{audience}` has no source file `{relative_source}`"
            ));
        }
        collect_file_exports(
            inventory.text(path),
            &audience,
            &relative_source,
            &mut exports,
        )?;
    }
    Ok(exports)
}

fn collect_file_exports(
    source: &str,
    audience: &str,
    relative_source: &str,
    exports: &mut BTreeSet<ProductExport>,
) -> Result<(), String> {
    let syntax = syn::parse_file(source)
        .map_err(|error| format!("`{relative_source}` should parse: {error}"))?;
    for item in syntax.items {
        match item {
            syn::Item::Use(item) if matches!(item.vis, syn::Visibility::Public(_)) => {
                let mut names = Vec::new();
                collect_use_names(&item.tree, &mut names)?;
                for name in names {
                    exports.insert(ProductExport {
                        qualified_symbol: format!("{audience}::{name}"),
                        source: relative_source.to_owned(),
                    });
                }
            }
            item if is_public_item(&item) => {
                let name = public_item_name(&item).ok_or_else(|| {
                    format!("`{relative_source}` contains an unsupported public item")
                })?;
                exports.insert(ProductExport {
                    qualified_symbol: format!("{audience}::{name}"),
                    source: relative_source.to_owned(),
                });
            }
            _ => {}
        }
    }
    Ok(())
}

fn collect_use_names(tree: &syn::UseTree, names: &mut Vec<String>) -> Result<(), String> {
    match tree {
        syn::UseTree::Path(path) => collect_use_names(&path.tree, names),
        syn::UseTree::Name(name) => {
            names.push(name.ident.to_string());
            Ok(())
        }
        syn::UseTree::Rename(rename) => {
            names.push(rename.rename.to_string());
            Ok(())
        }
        syn::UseTree::Group(group) => {
            for item in &group.items {
                collect_use_names(item, names)?;
            }
            Ok(())
        }
        syn::UseTree::Glob(_) => Err("product facades may not use wildcard exports".to_owned()),
    }
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

fn public_item_name(item: &syn::Item) -> Option<String> {
    match item {
        syn::Item::Const(item) => Some(item.ident.to_string()),
        syn::Item::Enum(item) => Some(item.ident.to_string()),
        syn::Item::Fn(item) => Some(item.sig.ident.to_string()),
        syn::Item::Mod(item) => Some(item.ident.to_string()),
        syn::Item::Static(item) => Some(item.ident.to_string()),
        syn::Item::Struct(item) => Some(item.ident.to_string()),
        syn::Item::Trait(item) => Some(item.ident.to_string()),
        syn::Item::TraitAlias(item) => Some(item.ident.to_string()),
        syn::Item::Type(item) => Some(item.ident.to_string()),
        syn::Item::Union(item) => Some(item.ident.to_string()),
        _ => None,
    }
}

fn describe_difference(
    actual: &BTreeSet<ProductExport>,
    declared: &BTreeSet<ProductExport>,
) -> String {
    let unmanifested = actual.difference(declared).take(8).collect::<Vec<_>>();
    let absent = declared.difference(actual).take(8).collect::<Vec<_>>();
    format!("unmanifested={unmanifested:?}, absent={absent:?}")
}

fn audit_product_root(inventory: &WorkspaceSourceInventory) -> Result<(), String> {
    let source = inventory.text(Path::new(PRODUCT_ROOT).join("lib.rs"));
    audit_product_root_source(source)
}

fn audit_product_root_source(source: &str) -> Result<(), String> {
    let syntax = syn::parse_file(source)
        .map_err(|error| format!("product crate root should parse: {error}"))?;
    for item in syntax.items {
        match item {
            syn::Item::Mod(module)
                if matches!(module.vis, syn::Visibility::Public(_)) && module.ident == "facade" => {
            }
            item if is_public_item(&item) => {
                return Err("product crate root may publish only `facade`".to_owned());
            }
            _ => {}
        }
    }
    Ok(())
}

#[cfg(test)]
#[path = "phase5_product_api_tests.rs"]
mod tests;
