//! Seal public re-exports whose target lives outside the local module graph.
//!
//! Cross-crate `pub use` is ordinary public API. Every externally reachable
//! re-export must resolve to a path dependency and seal the full callable surface
//! opened by that re-export (module trees, type-owned methods, nested uses,
//! macros, opaque attributes) through the shared surface closure — not a single
//! declaration node. Dependency module graphs are parsed once per crate root.

use super::callable_surface::collect_surface_violations;
use super::crate_modules::{
    is_public_visibility, module_path_display, parse_crate_modules, GovernedCrate, ModuleGraph,
    ModuleNode,
};
use super::external_use_target::{expand_use_targets, ExpandedUse};
use super::forbidden_aliases::collect_forbidden_aliases;
use super::path_dependencies::path_dependency_roots;
use super::public_reachability::{
    item_name, module_is_public_chain, reachability_from_seeds, Reachability, ReachabilitySeeds,
    ReachableItemKey,
};
use crate::diagnostics::{Diagnostic, DiagnosticCode};
use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;
use syn::Item;

const SEALING_LAW: &str = "Authority sealing law: governed public surfaces must demand concrete \
platform authority/capability types. Generic bounds over `AuthorityMarker`, `CapabilityMarker`, \
`AuthorityProves`, or `ProofSetAuthorizedBy` are forbidden. Use `AuthorityWitness<ConcreteAuthority>`, \
`CapabilityWitness<ConcreteCapability>`, or `Proof<Fact, ConcreteAuthority>`; keep the concrete \
marker value-gated and mint it only in the owning crate's ceremony.";

const EXTERNAL_FENCE: &str = "Public re-export of an external (or otherwise non-local) item is \
part of this crate's ordinary public surface. The sealing inventory must prove the target \
callable surface is concrete; unresolved, non-path, or uninspectable external re-exports fail closed.";

struct DependencySurface {
    graph: ModuleGraph,
    aliases: super::forbidden_aliases::AliasInventory,
}

pub(super) fn enforce_external_public_reexports(
    governed: &GovernedCrate,
    graph: &ModuleGraph,
    reachability: &Reachability,
) -> Vec<Diagnostic> {
    let path_deps = match path_dependency_roots(&governed.crate_root) {
        Ok(deps) => deps,
        Err(error) => {
            return vec![Diagnostic::new(
                DiagnosticCode::Bc7001AuthoritySealing,
                format!("{}::{}", governed.package, governed.relative_crate_root),
                format!("{SEALING_LAW} {EXTERNAL_FENCE} Failed to read path dependencies: {error}"),
            )];
        }
    };

    let mut dep_cache: BTreeMap<PathBuf, Result<DependencySurface, String>> = BTreeMap::new();
    let mut diagnostics = Vec::new();
    for (module_path, node) in &graph.modules {
        if !module_contributes_public_surface(graph, module_path, reachability) {
            continue;
        }
        for item in &node.items {
            let Item::Use(item_use) = item else { continue };
            if !is_public_visibility(&item_use.vis) {
                continue;
            }
            for expanded in expand_use_targets(module_path, &item_use.tree) {
                if local_target_exists(graph, &expanded.target_module, &expanded.target_name) {
                    continue;
                }
                let mut nested_local = module_path.to_vec();
                nested_local.extend(expanded.target_module.iter().cloned());
                if local_target_exists(graph, &nested_local, &expanded.target_name) {
                    continue;
                }
                diagnostics.extend(seal_external_target(
                    governed,
                    node,
                    module_path,
                    &expanded,
                    &path_deps,
                    &mut dep_cache,
                ));
            }
        }
    }
    diagnostics.sort_by(Diagnostic::compare_subject_message);
    diagnostics.dedup_by(|a, b| a.has_same_subject_message(b));
    diagnostics
}

fn local_target_exists(graph: &ModuleGraph, target_module: &[String], target_name: &str) -> bool {
    let Some(node) = graph.modules.get(target_module) else {
        return false;
    };
    if target_name == "*" {
        return true;
    }
    node.items.iter().any(|item| match item {
        Item::Mod(item_mod) => item_mod.ident == target_name,
        other => item_name(other).as_deref() == Some(target_name),
    })
}

fn seal_external_target(
    governed: &GovernedCrate,
    use_site: &ModuleNode,
    use_module: &[String],
    expanded: &ExpandedUse,
    path_deps: &BTreeMap<String, PathBuf>,
    dep_cache: &mut BTreeMap<PathBuf, Result<DependencySurface, String>>,
) -> Vec<Diagnostic> {
    let subject = reexport_subject(governed, use_site, use_module, expanded);
    if expanded.target_module.is_empty() {
        return vec![unresolved_diagnostic(
            &subject,
            "external re-export path is empty after expansion",
        )];
    }

    let crate_ident = &expanded.target_module[0];
    let Some(dep_root) = path_deps.get(crate_ident) else {
        return vec![unresolved_diagnostic(
            &subject,
            &format!(
                "public re-export target `{crate_ident}` is not a path dependency of {}; \
cannot prove the re-exported signature is concrete",
                governed.package
            ),
        )];
    };

    let surface = match load_dependency_surface(dep_root, crate_ident, dep_cache) {
        Ok(surface) => surface,
        Err(error) => {
            return vec![unresolved_diagnostic(&subject, &error)];
        }
    };

    let module_in_dep: Vec<String> = expanded.target_module[1..].to_vec();
    let seeds = match build_external_seeds(
        &surface.graph,
        &module_in_dep,
        &expanded.target_name,
        crate_ident,
    ) {
        Ok(seeds) => seeds,
        Err(detail) => return vec![unresolved_diagnostic(&subject, &detail)],
    };

    let mut reachability = reachability_from_seeds(&surface.graph, seeds);
    // Alias inventory is crate-global for the dependency (parent-visible renames).
    reachability.forbidden_aliases = surface.aliases.clone();

    collect_surface_violations(&surface.graph, &reachability)
        .into_iter()
        .map(|violation| hit_diagnostic(&subject, &violation.hit, &violation.key.item_name))
        .collect()
}

fn load_dependency_surface<'a>(
    dep_root: &PathBuf,
    crate_ident: &str,
    dep_cache: &'a mut BTreeMap<PathBuf, Result<DependencySurface, String>>,
) -> Result<&'a DependencySurface, String> {
    if !dep_cache.contains_key(dep_root) {
        let external = GovernedCrate {
            package: crate_ident.replace('_', "-"),
            crate_root: dep_root.clone(),
            relative_crate_root: dep_root.display().to_string(),
        };
        let loaded = parse_crate_modules(&external).and_then(|graph| {
            let aliases = collect_forbidden_aliases(&graph, dep_root)?;
            Ok(DependencySurface { graph, aliases })
        });
        dep_cache.insert(dep_root.clone(), loaded);
    }
    match dep_cache.get(dep_root) {
        Some(Ok(surface)) => Ok(surface),
        Some(Err(error)) => Err(format!(
            "failed to parse path dependency `{crate_ident}`: {error}"
        )),
        None => Err(format!(
            "failed to parse path dependency `{crate_ident}`: cache miss"
        )),
    }
}

fn build_external_seeds(
    graph: &ModuleGraph,
    module_in_dep: &[String],
    target_name: &str,
    crate_ident: &str,
) -> Result<ReachabilitySeeds, String> {
    if target_name == "*" {
        // Glob: open the target module as a public root.
        if !graph.modules.contains_key(module_in_dep) {
            return Err(format!(
                "glob re-export from `{crate_ident}::{}` could not resolve the target module",
                if module_in_dep.is_empty() {
                    "crate".to_owned()
                } else {
                    module_in_dep.join("::")
                }
            ));
        }
        let mut modules = BTreeSet::new();
        modules.insert(module_in_dep.to_vec());
        return Ok(ReachabilitySeeds {
            modules,
            items: BTreeSet::new(),
        });
    }

    let item = find_external_item(graph, module_in_dep, target_name).ok_or_else(|| {
        format!(
            "path dependency `{crate_ident}` has no public item `{target_name}` at module `{}`",
            if module_in_dep.is_empty() {
                "crate".to_owned()
            } else {
                module_in_dep.join("::")
            }
        )
    })?;

    match item {
        Item::Mod(item_mod) => {
            let mut child = module_in_dep.to_vec();
            child.push(item_mod.ident.to_string());
            let mut modules = BTreeSet::new();
            modules.insert(child);
            Ok(ReachabilitySeeds {
                modules,
                items: BTreeSet::new(),
            })
        }
        _ => {
            let mut items = BTreeSet::new();
            items.insert(ReachableItemKey {
                module_path: module_in_dep.to_vec(),
                item_name: target_name.to_owned(),
            });
            Ok(ReachabilitySeeds {
                modules: BTreeSet::new(),
                items,
            })
        }
    }
}

fn find_external_item<'a>(
    graph: &'a ModuleGraph,
    module_path: &[String],
    target_name: &str,
) -> Option<&'a Item> {
    let node = graph.modules.get(module_path)?;
    node.items.iter().find(|item| match item {
        Item::Mod(item_mod) => item_mod.ident == target_name,
        other => item_name(other).as_deref() == Some(target_name),
    })
}

fn module_contributes_public_surface(
    graph: &ModuleGraph,
    path: &[String],
    reachability: &Reachability,
) -> bool {
    if path.is_empty() {
        return true;
    }
    if reachability
        .items
        .iter()
        .any(|key| key.module_path.starts_with(path) || key.module_path == path)
    {
        return true;
    }
    if reachability.public_modules.contains(path) {
        return true;
    }
    module_is_public_chain(graph, path)
}

fn reexport_subject(
    governed: &GovernedCrate,
    use_site: &ModuleNode,
    use_module: &[String],
    expanded: &ExpandedUse,
) -> String {
    let module_display = if use_module.is_empty() {
        "crate".to_owned()
    } else {
        module_path_display(use_module)
    };
    let target = if expanded.target_module.is_empty() {
        expanded.export_name.clone()
    } else {
        format!(
            "{}::{}",
            expanded.target_module.join("::"),
            expanded.export_name
        )
    };
    format!(
        "{}::{}::{module_display}::pub_use({target})",
        governed.package, use_site.relative_source
    )
}

fn unresolved_diagnostic(subject: &str, detail: &str) -> Diagnostic {
    Diagnostic::new(
        DiagnosticCode::Bc7001AuthoritySealing,
        subject,
        format!("{SEALING_LAW} {EXTERNAL_FENCE} {detail}"),
    )
}

fn hit_diagnostic(
    subject: &str,
    hit: &super::authority_sealing_surface::SurfaceHit,
    item_name: &str,
) -> Diagnostic {
    use super::authority_sealing_surface::SurfaceHit;
    let detail = match hit {
        SurfaceHit::ForbiddenBound { trait_spelling } => format!(
            "Offending trait bound on re-exported target `{item_name}`: `{trait_spelling}`. \
Concrete pattern: `AuthorityWitness<ConcreteAuthority>`, `CapabilityWitness<ConcreteCapability>`, \
or `Proof<Fact, ConcreteAuthority>`."
        ),
        SurfaceHit::OpaqueMacroExpansion { macro_path } => format!(
            "Re-exported target `{item_name}` has opaque macro/attribute expansion: `{macro_path}`."
        ),
        SurfaceHit::PublicExternCrate { crate_ident } => {
            format!("Re-exported target exposes pub extern crate `{crate_ident}`.")
        }
        SurfaceHit::MintableAuthority {
            marker_type,
            reason,
        } => format!(
            "Re-exported target admits mintable marker `{marker_type}` (reason: {reason}). \
Seal with a private field and crate-local ceremony mint."
        ),
    };
    Diagnostic::new(
        DiagnosticCode::Bc7001AuthoritySealing,
        subject,
        format!("{SEALING_LAW} {EXTERNAL_FENCE} {detail}"),
    )
}
