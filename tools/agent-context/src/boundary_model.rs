use serde::Deserialize;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use syn::{Item, ItemMod, ItemUse, UseTree};

#[derive(Deserialize)]
struct BoundaryConfig {
    machine_authority: MachineAuthorityConfig,
    rule_contracts: RuleContracts,
    routing_proof: RoutingProofConfig,
    seed_skeletons: Vec<SeedSkeletonConfig>,
    subworkspaces: Vec<SubworkspaceConfig>,
}

#[derive(Deserialize)]
struct MachineAuthorityConfig {
    canonical_config: String,
}

#[derive(Deserialize)]
struct RuleContracts {
    query_host_bands: Vec<String>,
    replay_surfaces: Vec<ReplaySurfaceConfig>,
    band_rules: Vec<BandRuleConfig>,
}

#[derive(Deserialize)]
struct ReplaySurfaceConfig {
    label: String,
    package_prefixes: Vec<String>,
    cert_domains: Vec<String>,
}

#[derive(Deserialize)]
struct BandRuleConfig {
    source_band: String,
    allowed_target_bands: Vec<String>,
}

#[derive(Deserialize)]
struct SeedSkeletonConfig {
    path: String,
}

#[derive(Deserialize)]
struct SubworkspaceConfig {
    path: String,
}

#[derive(Deserialize)]
struct RoutingProofConfig {
    exemplars: Vec<ExemplarRouteConfig>,
}

#[derive(Deserialize)]
struct ExemplarRouteConfig {
    package: String,
    specimen: String,
    deferred_routes: Vec<String>,
}

pub(crate) struct CrateOrientation {
    pub(crate) crate_name: String,
    pub(crate) relative_path: String,
    pub(crate) constitutional_class: String,
    pub(crate) domain: String,
    pub(crate) exemplar_role: String,
    pub(crate) deferred_routes: Vec<String>,
    pub(crate) allowed_target_bands: Vec<String>,
    pub(crate) facade_exports: Vec<String>,
    pub(crate) owned_modules: Vec<String>,
    pub(crate) machine_fences: Vec<String>,
    pub(crate) skeleton_fence: String,
    pub(crate) machine_constitution: String,
}

struct DiscoveredCrate {
    package: String,
    relative_path: String,
}

pub(crate) fn load_orientations(
    root: &Path,
    config_path: &Path,
) -> Result<Vec<CrateOrientation>, String> {
    let config_text =
        fs::read_to_string(config_path).map_err(|e| format!("read {}: {e}", config_path.display()))?;
    let config: BoundaryConfig =
        toml::from_str(&config_text).map_err(|e| format!("parse {}: {e}", config_path.display()))?;
    let route_proof = config
        .routing_proof
        .exemplars
        .into_iter()
        .map(|exemplar| (exemplar.package.clone(), exemplar))
        .collect::<BTreeMap<_, _>>();
    let band_rules = config
        .rule_contracts
        .band_rules
        .into_iter()
        .map(|rule| (rule.source_band, rule.allowed_target_bands))
        .collect::<BTreeMap<_, _>>();
    let replay_surface_summary = config
        .rule_contracts
        .replay_surfaces
        .iter()
        .map(|surface| {
            format!(
                "{} [{}; cert domains: {}]",
                surface.label,
                surface.package_prefixes.join(", "),
                surface.cert_domains.join(", ")
            )
        })
        .collect::<Vec<_>>()
        .join("; ");
    let seed_skeleton_paths = config
        .seed_skeletons
        .into_iter()
        .map(|skeleton| skeleton.path)
        .collect::<Vec<_>>();
    let discovered = discover_born_crates(root, &config.subworkspaces)?;

    discovered
        .into_iter()
        .map(|born| {
            let crate_root = root.join(&born.relative_path);
            let parsed = parse_crate_name(&born.package)?;
            let exemplar = route_proof.get(&born.package);
            let facade_exports = collect_facade_exports(&crate_root.join("src/facade.rs"))?;
            let owned_modules = collect_owned_modules(&crate_root.join("src"))?;
            ensure_facade_only_public_surface(&crate_root.join("src/lib.rs"))?;
            let mut machine_fences = Vec::new();
            if parsed.tier == "worth" {
                machine_fences.push("Must not depend on worthy-* crates.".to_owned());
            }
            if !config
                .rule_contracts
                .query_host_bands
                .iter()
                .any(|band| band == &parsed.band)
            {
                machine_fences.push("Must not depend on worth-query.".to_owned());
            }
            if parsed.band != "cert" {
                machine_fences.push(format!(
                    "Must not depend on replay surface families such as {replay_surface_summary}."
                ));
            }

            let exemplar_role = exemplar
                .map(|value| value.specimen.clone())
                .unwrap_or_else(|| "No exemplar route assigned yet.".to_owned());
            let deferred_routes = exemplar
                .map(|value| value.deferred_routes.clone())
                .unwrap_or_default();
            let skeleton_fence = if seed_skeleton_paths
                .iter()
                .any(|path| path == &born.relative_path)
            {
                "Seed skeleton is machine-fenced by boundary-check; undeclared files and mixed-class modules are denied."
                    .to_owned()
            } else {
                "No seed-specific skeleton allowlist is declared for this born crate; general Road 1 boundary law still applies."
                    .to_owned()
            };

            Ok(CrateOrientation {
                crate_name: born.package,
                relative_path: born.relative_path,
                constitutional_class: format!("{}/{}", parsed.tier, parsed.band),
                domain: parsed.domain,
                exemplar_role,
                deferred_routes,
                allowed_target_bands: band_rules.get(&parsed.band).cloned().unwrap_or_default(),
                facade_exports,
                owned_modules,
                machine_fences,
                skeleton_fence,
                machine_constitution: config.machine_authority.canonical_config.clone(),
            })
        })
        .collect()
}

fn discover_born_crates(
    root: &Path,
    subworkspaces: &[SubworkspaceConfig],
) -> Result<Vec<DiscoveredCrate>, String> {
    let mut discovered = Vec::new();
    for subworkspace in subworkspaces {
        let crates_root = root.join(&subworkspace.path).join("crates");
        if !crates_root.is_dir() {
            continue;
        }
        for entry in fs::read_dir(&crates_root)
            .map_err(|e| format!("read {}: {e}", crates_root.display()))?
        {
            let entry = entry.map_err(|e| format!("read {} entry: {e}", crates_root.display()))?;
            let crate_root = entry.path();
            let manifest_path = crate_root.join("Cargo.toml");
            if !crate_root.is_dir() || !manifest_path.is_file() {
                continue;
            }
            let relative_path = crate_root
                .strip_prefix(root)
                .map_err(|e| format!("strip root prefix from {}: {e}", crate_root.display()))?
                .to_string_lossy()
                .replace('\\', "/");
            let package = package_name_from_manifest(&manifest_path)?;
            discovered.push(DiscoveredCrate {
                package,
                relative_path,
            });
        }
    }
    discovered.sort_by(|left, right| left.package.cmp(&right.package));
    Ok(discovered)
}

fn package_name_from_manifest(path: &Path) -> Result<String, String> {
    let text = fs::read_to_string(path).map_err(|e| format!("read {}: {e}", path.display()))?;
    let value: toml::Value = toml::from_str(&text).map_err(|e| format!("parse {}: {e}", path.display()))?;
    value
        .get("package")
        .and_then(|package| package.get("name"))
        .and_then(|name| name.as_str())
        .map(str::to_owned)
        .ok_or_else(|| format!("{} missing package.name", path.display()))
}

struct ParsedCrateName {
    tier: String,
    band: String,
    domain: String,
}

fn parse_crate_name(raw: &str) -> Result<ParsedCrateName, String> {
    let parts = raw.split('-').collect::<Vec<_>>();
    if parts.len() < 3 {
        return Err(format!("{raw} does not parse as {{tier}}-{{band}}-{{domain}}"));
    }
    Ok(ParsedCrateName {
        tier: parts[0].to_owned(),
        band: parts[1].to_owned(),
        domain: parts[2..].join("-"),
    })
}

fn collect_facade_exports(path: &Path) -> Result<Vec<String>, String> {
    let text = fs::read_to_string(path).map_err(|e| format!("read {}: {e}", path.display()))?;
    let syntax = syn::parse_file(&text).map_err(|e| format!("parse {}: {e}", path.display()))?;
    let mut exports = Vec::new();
    for item in syntax.items {
        if let Item::Use(item_use) = item {
            collect_use_tree_exports(&item_use, &mut exports);
        }
    }
    exports.sort();
    Ok(exports)
}

fn collect_owned_modules(src_root: &Path) -> Result<Vec<String>, String> {
    let mut modules = Vec::new();
    for entry in fs::read_dir(src_root).map_err(|e| format!("read {}: {e}", src_root.display()))? {
        let entry = entry.map_err(|e| format!("read {} entry: {e}", src_root.display()))?;
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();
        if name == "lib.rs" || name == "facade.rs" {
            continue;
        }
        if path.is_dir() {
            modules.push(name);
        } else if path.extension().and_then(|ext| ext.to_str()) == Some("rs") {
            modules.push(
                path.file_stem()
                    .and_then(|stem| stem.to_str())
                    .ok_or_else(|| format!("invalid utf-8 module name in {}", path.display()))?
                    .to_owned(),
            );
        }
    }
    modules.sort();
    Ok(modules)
}

fn ensure_facade_only_public_surface(path: &Path) -> Result<(), String> {
    let text = fs::read_to_string(path).map_err(|e| format!("read {}: {e}", path.display()))?;
    let syntax = syn::parse_file(&text).map_err(|e| format!("parse {}: {e}", path.display()))?;
    let mut facade_exports = 0usize;
    for item in syntax.items {
        match item {
            Item::Mod(ItemMod {
                vis: syn::Visibility::Public(_),
                ident,
                content: None,
                ..
            }) if ident == "facade" => {
                facade_exports += 1;
            }
            Item::Mod(ItemMod {
                vis: syn::Visibility::Inherited,
                content: None,
                ..
            }) => {}
            Item::Use(item_use) if matches!(item_use.vis, syn::Visibility::Public(_)) => {
                return Err(format!(
                    "{} no longer exposes a facade-only public surface",
                    path.display()
                ));
            }
            _ => {
                return Err(format!(
                    "{} no longer exposes a facade-only public surface",
                    path.display()
                ));
            }
        }
    }
    if facade_exports == 1 {
        Ok(())
    } else {
        Err(format!(
            "{} no longer exposes a facade-only public surface",
            path.display()
        ))
    }
}

fn collect_use_tree_exports(item_use: &ItemUse, exports: &mut Vec<String>) {
    collect_use_tree_names(&item_use.tree, exports);
}

fn collect_use_tree_names(tree: &UseTree, exports: &mut Vec<String>) {
    match tree {
        UseTree::Path(path) => collect_use_tree_names(&path.tree, exports),
        UseTree::Name(name) => exports.push(name.ident.to_string()),
        UseTree::Rename(rename) => exports.push(rename.rename.to_string()),
        UseTree::Group(group) => {
            for item in &group.items {
                collect_use_tree_names(item, exports);
            }
        }
        UseTree::Glob(_) => exports.push("*".to_owned()),
    }
}
