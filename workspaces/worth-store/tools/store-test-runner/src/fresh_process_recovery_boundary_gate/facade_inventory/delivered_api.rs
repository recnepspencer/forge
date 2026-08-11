use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::{Item, Token, Visibility};

use super::super::repository_root;
pub(super) use exactness::assert_exact_inventory;

mod cfg_reachability;
mod exactness;
mod export_resolution;
mod external_resolution;
mod facade_exports;
mod module_paths;
mod namespace_exports;
mod pre_c8_surface;
mod private_imports;
mod source_layout;
#[cfg(test)]
mod tests;

use facade_exports::{facade_exports, module_exports};
use namespace_exports::ExportCollection;
use source_layout::ModuleGraph;

#[derive(Clone, Debug)]
struct ExportedSurface {
    prefix: Vec<String>,
    source_name: String,
    export_name: String,
    direct: bool,
    glob: bool,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct CanonicalType {
    path: PathBuf,
    module: Vec<String>,
    inline: Vec<(String, usize)>,
    name: String,
}

struct FacadeFamily {
    facade: &'static str,
    source_root: &'static str,
    owner_prefix: &'static str,
    preserve_underscores: bool,
}

const FAMILIES: &[FacadeFamily] = &[
    FacadeFamily {
        facade: "workspaces/worth-store/crates/worth-store-recovery-runtime/src/lib.rs",
        source_root: "workspaces/worth-store/crates/worth-store-recovery-runtime/src",
        owner_prefix: "",
        preserve_underscores: true,
    },
    FacadeFamily {
        facade: "workspaces/worth-store/crates/worth-store-physical-backend/src/facade.rs",
        source_root: "workspaces/worth-store/crates/worth-store-physical-backend/src",
        owner_prefix: "worth-store-physical-backend/",
        preserve_underscores: false,
    },
    FacadeFamily {
        facade: "workspaces/worth-store/crates/worth-store/src/physical_runtime/mod.rs",
        source_root: "workspaces/worth-store/crates/worth-store/src/physical_runtime",
        owner_prefix: "worth-store/",
        preserve_underscores: false,
    },
    FacadeFamily {
        facade: "workspaces/worth-store/crates/worth-store-physical-format/src/lib.rs",
        source_root: "workspaces/worth-store/crates/worth-store-physical-format/src",
        owner_prefix: "worth-store-physical-format/",
        preserve_underscores: false,
    },
    FacadeFamily {
        facade: "workspaces/worth-store/crates/worth-store-wal/src/lib.rs",
        source_root: "workspaces/worth-store/crates/worth-store-wal/src",
        owner_prefix: "worth-store-wal/",
        preserve_underscores: false,
    },
];

pub(super) fn delivered_facades() -> Result<BTreeSet<(String, String)>, String> {
    let root = repository_root();
    let mut reachable = BTreeSet::new();
    for family in FAMILIES {
        reachable.extend(derive_family_at(&root, family)?);
    }
    let baseline = pre_c8_surface::pre_c8_surfaces()?;
    let missing_baseline = baseline.difference(&reachable).collect::<Vec<_>>();
    if !missing_baseline.is_empty() {
        return Err(format!(
            "pre-C.8 supporting facade surfaces disappeared: {missing_baseline:?}"
        ));
    }
    Ok(reachable.difference(&baseline).cloned().collect())
}

fn derive_family_at(
    root: &Path,
    family: &FacadeFamily,
) -> Result<BTreeSet<(String, String)>, String> {
    let facade = root.join(family.facade);
    let source_root = root.join(family.source_root);
    let graph = ModuleGraph::build(&source_root)?;
    facade_exports::validate_macro_exports(&graph)?;
    let facade_module = graph.module_for_file(&facade).ok_or_else(|| {
        format!(
            "facade {} is not reachable from the production module graph",
            facade.display()
        )
    })?;
    let mut delivered = BTreeSet::new();
    let mut exported_types = BTreeMap::<CanonicalType, BTreeSet<String>>::new();
    {
        let mut collection = ExportCollection {
            root,
            source_root: &source_root,
            family,
            graph: &graph,
            exported_types: &mut exported_types,
            delivered: &mut delivered,
        };
        collection.collect(facade_module, &[], facade_exports(&facade)?)?;
        collection.collect_public_namespaces()?;
    }
    collect_associated_surfaces(
        &graph,
        &source_root,
        family,
        &exported_types,
        &mut delivered,
    )?;
    Ok(delivered)
}

fn record_declaration(
    export_name: String,
    declaration: export_resolution::ResolvedDeclaration,
    source_root: &Path,
    family: &FacadeFamily,
    exported_types: &mut BTreeMap<CanonicalType, BTreeSet<String>>,
    delivered: &mut BTreeSet<(String, String)>,
) -> Result<(), String> {
    let owner = source_owner(&declaration.path, source_root, family)?;
    if declaration.is_type {
        exported_types
            .entry(CanonicalType {
                path: declaration.path,
                module: declaration.module,
                inline: declaration.inline,
                name: declaration.name,
            })
            .or_default()
            .insert(export_name.clone());
    }
    delivered.insert((export_name, owner));
    Ok(())
}

fn collect_associated_surfaces(
    graph: &ModuleGraph,
    source_root: &Path,
    family: &FacadeFamily,
    exported_types: &BTreeMap<CanonicalType, BTreeSet<String>>,
    delivered: &mut BTreeSet<(String, String)>,
) -> Result<(), String> {
    for module in graph.modules() {
        let owner = source_owner(&module.path, source_root, family)?;
        let items = graph.items(module)?;
        let counter_accessors = recognized_counter_accessor_macros(&items);
        for item in items {
            match item {
                Item::Impl(item) if item.trait_.is_none() && production_attrs(&item.attrs) => {
                    let type_identities = export_resolution::resolve_impl_type(
                        module,
                        graph,
                        source_root,
                        &item,
                        exported_types.keys(),
                    )?;
                    if type_identities.is_empty() {
                        continue;
                    }
                    let export_names = type_identities
                        .iter()
                        .filter_map(|identity| exported_types.get(identity))
                        .flatten()
                        .collect::<BTreeSet<_>>();
                    for associated in item.items {
                        if let syn::ImplItem::Macro(item) = &associated {
                            if !production_attrs(&item.attrs) {
                                continue;
                            }
                            if let Some(names) =
                                expanded_counter_accessors(item, &counter_accessors)?
                            {
                                for name in names {
                                    for export_name in &export_names {
                                        delivered.insert((
                                            format!("{export_name}::{name}"),
                                            owner.clone(),
                                        ));
                                    }
                                }
                                continue;
                            }
                        }
                        let (visibility, name, attrs) = match associated {
                            syn::ImplItem::Const(item) => {
                                (item.vis, item.ident.to_string(), item.attrs)
                            }
                            syn::ImplItem::Fn(item) => {
                                (item.vis, item.sig.ident.to_string(), item.attrs)
                            }
                            syn::ImplItem::Type(item) => {
                                (item.vis, item.ident.to_string(), item.attrs)
                            }
                            syn::ImplItem::Macro(item) if production_attrs(&item.attrs) => {
                                return Err(format!(
                                    "unsupported macro invocation in exported type {}; public expansion is not provable",
                                    export_names
                                        .iter()
                                        .next()
                                        .expect("exported type has a facade name")
                                ));
                            }
                            _ => continue,
                        };
                        if production_public(&visibility, &attrs) {
                            for export_name in &export_names {
                                delivered.insert((format!("{export_name}::{name}"), owner.clone()));
                            }
                        }
                    }
                }
                Item::Trait(item) if production_public(&item.vis, &item.attrs) => {
                    let identity = CanonicalType {
                        path: module.path.clone(),
                        module: module.logical.clone(),
                        inline: module.inline.clone(),
                        name: item.ident.to_string(),
                    };
                    let Some(export_names) = exported_types.get(&identity) else {
                        continue;
                    };
                    for associated in item.items {
                        let (name, attrs) = match associated {
                            syn::TraitItem::Const(item) => (item.ident.to_string(), item.attrs),
                            syn::TraitItem::Fn(item) => (item.sig.ident.to_string(), item.attrs),
                            syn::TraitItem::Type(item) => (item.ident.to_string(), item.attrs),
                            syn::TraitItem::Macro(item) if production_attrs(&item.attrs) => {
                                return Err(format!(
                                    "unsupported macro invocation in exported trait {}; public expansion is not provable",
                                    export_names
                                        .iter()
                                        .next()
                                        .expect("exported trait has a facade name")
                                ));
                            }
                            _ => continue,
                        };
                        if production_attrs(&attrs) {
                            for export_name in export_names {
                                delivered.insert((format!("{export_name}::{name}"), owner.clone()));
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
    Ok(())
}

fn recognized_counter_accessor_macros(items: &[Item]) -> BTreeSet<String> {
    items
        .iter()
        .filter_map(|item| {
            let Item::Macro(item) = item else {
                return None;
            };
            let name = item.ident.as_ref()?;
            if !item.mac.path.is_ident("macro_rules") || !production_attrs(&item.attrs) {
                return None;
            }
            let normalized = item
                .mac
                .tokens
                .to_string()
                .split_whitespace()
                .collect::<String>();
            (normalized == "($($name:ident),+$(,)?)=>{$(pubconstfn$name(self)->u64{self.$name})+};")
                .then(|| name.to_string())
        })
        .collect()
}

fn expanded_counter_accessors(
    invocation: &syn::ImplItemMacro,
    recognized: &BTreeSet<String>,
) -> Result<Option<Vec<String>>, String> {
    if invocation.mac.path.leading_colon.is_some() || invocation.mac.path.segments.len() != 1 {
        return Ok(None);
    }
    let name = invocation
        .mac
        .path
        .segments
        .first()
        .expect("one unqualified macro path segment");
    if !recognized.contains(&name.ident.to_string()) {
        return Ok(None);
    }
    let parser = Punctuated::<syn::Ident, Token![,]>::parse_terminated;
    parser
        .parse2(invocation.mac.tokens.clone())
        .map(|identifiers| {
            Some(
                identifiers
                    .into_iter()
                    .map(|ident| ident.to_string())
                    .collect(),
            )
        })
        .map_err(|error| format!("recognized counter accessor invocation is malformed: {error}"))
}

fn source_owner(path: &Path, source_root: &Path, family: &FacadeFamily) -> Result<String, String> {
    let mut relative = relative_source(path, source_root)?;
    relative = relative.strip_suffix(".rs").unwrap_or(&relative).to_owned();
    relative = relative
        .strip_suffix("/mod")
        .unwrap_or(&relative)
        .to_owned();
    if family.preserve_underscores {
        relative = relative.replace("authority_binding", "authority-binding");
    } else {
        relative = relative.replace('_', "-");
    }
    Ok(format!("{}{relative}", family.owner_prefix))
}

fn relative_source(path: &Path, root: &Path) -> Result<String, String> {
    path.strip_prefix(root)
        .map_err(|_| format!("{} is not under {}", path.display(), root.display()))
        .map(|relative| relative.to_string_lossy().replace('\\', "/"))
}

fn parse_source(path: &Path) -> Result<syn::File, String> {
    let source = std::fs::read_to_string(path)
        .map_err(|error| format!("cannot read {}: {error}", path.display()))?;
    syn::parse_file(&source).map_err(|error| format!("invalid {}: {error}", path.display()))
}

fn production_public(visibility: &Visibility, attrs: &[syn::Attribute]) -> bool {
    matches!(visibility, Visibility::Public(_)) && production_attrs(attrs)
}

fn production_attrs(attrs: &[syn::Attribute]) -> bool {
    cfg_reachability::can_reach_production(attrs)
}

fn macro_public_enum(item: &syn::ItemMacro) -> Option<String> {
    let tokens = item.mac.tokens.to_string();
    let words = tokens.split_whitespace().collect::<Vec<_>>();
    words
        .windows(3)
        .find(|window| window[0] == "pub" && window[1] == "enum")
        .map(|window| {
            window[2]
                .trim_matches(|character: char| !character.is_alphanumeric() && character != '_')
                .to_owned()
        })
}
