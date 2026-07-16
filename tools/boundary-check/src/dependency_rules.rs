use crate::config::{
    BandRuleConfig, LawSubstrateConfig, QueryAudienceContract, ReplaySurfaceConfig, RuleContracts,
};
use crate::diagnostics::{Diagnostic, DiagnosticCode};
use crate::manifest_types::Road1Package;
use crate::naming::parse_crate_name;
use crate::source_rules::{illegal_law_substrate_edge, is_legal_law_substrate_edge};
use std::collections::BTreeMap;
use std::path::Path;

pub(crate) fn validate_dependency_rules(
    packages: &[Road1Package],
    contracts: &RuleContracts,
    law_substrates: &[LawSubstrateConfig],
) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    let band_rules = band_rule_map(&contracts.band_rules);

    for package in packages {
        let Ok(source_name) = parse_crate_name(&package.name) else {
            continue;
        };

        for dependency in &package.dependencies {
            if let Some(surface_label) = replay_surface_label(dependency, contracts) {
                if source_name.band != "cert" {
                    diagnostics.push(Diagnostic::new(
                        DiagnosticCode::Bc4001OrdinaryReplayImport,
                        &package.name,
                        format!(
                            "only cert crates may depend on replay surface family {surface_label}; found dependency {dependency}"
                        ),
                    ));
                }
            }

            // Query engine/audience package edges are owned by query_audience::rules.
            // They are framework packages, not in-tree band members.
            if is_query_framework_package(dependency, &contracts.query_audience) {
                continue;
            }

            // Configured law substrates (worth-proof) are legal only for listed tiers/bands.
            // Known substrate packages that miss admission fail closed — they must not fall
            // through the out-of-grammar `parse_crate_name` skip (worth-proof is outside the
            // {tier}-{band}-{domain} birth grammar).
            if is_legal_law_substrate_edge(
                dependency,
                &source_name.tier,
                &source_name.band,
                law_substrates,
            ) {
                continue;
            }
            if let Some(message) = illegal_law_substrate_edge(
                dependency,
                &source_name.tier,
                &source_name.band,
                law_substrates,
            ) {
                diagnostics.push(Diagnostic::new(
                    DiagnosticCode::Bc7002LawSubstrateConfig,
                    &package.name,
                    format!("dependency {dependency}: {message}"),
                ));
                continue;
            }

            let Ok(target_name) = parse_crate_name(dependency) else {
                continue;
            };

            if source_name.tier == "worth" && target_name.tier == "worthy" {
                diagnostics.push(Diagnostic::new(
                    DiagnosticCode::Bc2002WorthToWorthyInversion,
                    &package.name,
                    format!("platform crate may not depend on CAD-tier crate {dependency}"),
                ));
            }

            if target_name.band == "cert" && source_name.band != "cert" {
                diagnostics.push(Diagnostic::new(
                    DiagnosticCode::Bc2001BandDependencyViolation,
                    &package.name,
                    format!("only cert crates may depend on cert crate {dependency}"),
                ));
            }

            if source_name.band == "cert" {
                continue;
            }

            if let Some(allowed_target_bands) = band_rules.get(&source_name.band) {
                if !allowed_target_bands.contains(&target_name.band) {
                    diagnostics.push(Diagnostic::new(
                        DiagnosticCode::Bc2001BandDependencyViolation,
                        &package.name,
                        format!(
                            "{} band may not depend on {} band crate {}; allowed in-tree target bands: {}",
                            source_name.band,
                            target_name.band,
                            dependency,
                            render_allowed_bands(allowed_target_bands),
                        ),
                    ));
                }
            }
        }
    }

    diagnostics
}

pub(crate) fn validate_worth_ui_query_edge(
    root: &Path,
    contract: &crate::config::WorthUiQueryEdgeContract,
) -> Result<Vec<Diagnostic>, String> {
    let crates_root = root.join(&contract.workspace).join("crates");
    let entries = std::fs::read_dir(&crates_root).map_err(|error| {
        format!("failed to read {}: {error}", crates_root.display())
    })?;
    let mut diagnostics = Vec::new();
    for entry in entries {
        let entry = entry.map_err(|error| format!("failed to read crate entry: {error}"))?;
        let manifest_path = entry.path().join("Cargo.toml");
        if !manifest_path.is_file() {
            continue;
        }
        let text = std::fs::read_to_string(&manifest_path)
            .map_err(|error| format!("failed to read {}: {error}", manifest_path.display()))?;
        let manifest: toml::Value = toml::from_str(&text)
            .map_err(|error| format!("failed to parse {}: {error}", manifest_path.display()))?;
        let package = manifest
            .get("package")
            .and_then(|value| value.get("name"))
            .and_then(toml::Value::as_str)
            .unwrap_or("unknown");
        let has_engine_dependency = manifest
            .get("dependencies")
            .and_then(toml::Value::as_table)
            .is_some_and(|dependencies| dependencies.contains_key(&contract.engine_package));
        if has_engine_dependency
            && !contract
                .allowed_production_consumers
                .iter()
                .any(|allowed| allowed == package)
        {
            diagnostics.push(Diagnostic::new(
                DiagnosticCode::Bc2001BandDependencyViolation,
                package,
                format!(
                    "direct production dependency on `{}` is denied by the Worth UI Query edge: {}",
                    contract.engine_package, contract.guidance
                ),
            ));
        }
        if !contract
            .allowed_production_consumers
            .iter()
            .any(|allowed| allowed == package)
        {
            diagnostics.extend(validate_crate_query_source_edge(
                &entry.path().join("src"),
                package,
                contract,
            )?);
        }
    }
    Ok(diagnostics)
}

fn validate_crate_query_source_edge(
    source_root: &Path,
    package: &str,
    contract: &crate::config::WorthUiQueryEdgeContract,
) -> Result<Vec<Diagnostic>, String> {
    if !source_root.is_dir() {
        return Ok(Vec::new());
    }
    let mut pending = vec![source_root.to_path_buf()];
    let mut diagnostics = Vec::new();
    while let Some(directory) = pending.pop() {
        for entry in std::fs::read_dir(&directory)
            .map_err(|error| format!("failed to read {}: {error}", directory.display()))?
        {
            let entry = entry.map_err(|error| format!("failed to read source entry: {error}"))?;
            let path = entry.path();
            if path.is_dir() {
                pending.push(path);
                continue;
            }
            if path.extension().and_then(std::ffi::OsStr::to_str) != Some("rs") {
                continue;
            }
            let text = std::fs::read_to_string(&path)
                .map_err(|error| format!("failed to read {}: {error}", path.display()))?;
            let syntax = syn::parse_file(&text)
                .map_err(|error| format!("failed to parse {}: {error}", path.display()))?;
            let mut visitor = WorthQueryPathVisitor::default();
            syn::visit::Visit::visit_file(&mut visitor, &syntax);
            if visitor.found {
                diagnostics.push(Diagnostic::new(
                    DiagnosticCode::Bc2001BandDependencyViolation,
                    package,
                    format!(
                        "raw `worth_query` production source edge in `{}` is denied: {}",
                        path.display(), contract.guidance
                    ),
                ));
            }
        }
    }
    Ok(diagnostics)
}

#[derive(Default)]
struct WorthQueryPathVisitor {
    found: bool,
}

impl<'ast> syn::visit::Visit<'ast> for WorthQueryPathVisitor {
    fn visit_path(&mut self, path: &'ast syn::Path) {
        if path
            .segments
            .first()
            .is_some_and(|segment| segment.ident == "worth_query")
        {
            self.found = true;
        }
        syn::visit::visit_path(self, path);
    }

    fn visit_item_use(&mut self, item: &'ast syn::ItemUse) {
        if use_tree_starts_with_worth_query(&item.tree) {
            self.found = true;
        }
        syn::visit::visit_item_use(self, item);
    }
}

fn use_tree_starts_with_worth_query(tree: &syn::UseTree) -> bool {
    matches!(tree, syn::UseTree::Path(path) if path.ident == "worth_query")
}

#[cfg(test)]
mod worth_ui_query_edge_tests {
    use super::*;

    fn contract() -> crate::config::WorthUiQueryEdgeContract {
        crate::config::WorthUiQueryEdgeContract {
            workspace: "worth-ui".to_owned(),
            engine_package: "worth-query".to_owned(),
            allowed_production_consumers: vec!["worth-ui-query-binding".to_owned()],
            guidance: "consume binding-owned artifacts".to_owned(),
        }
    }

    fn fixture_root(label: &str) -> std::path::PathBuf {
        std::env::temp_dir().join(format!(
            "worth-ui-query-edge-{label}-{}",
            std::process::id()
        ))
    }

    fn write_crate(root: &Path, package: &str, dependency: bool, source: &str) {
        let crate_root = root.join("worth-ui").join("crates").join(package);
        std::fs::create_dir_all(crate_root.join("src")).expect("fixture source directory");
        let dependency = dependency
            .then_some("worth-query = { path = \"../../worth-query\" }")
            .unwrap_or_default();
        std::fs::write(
            crate_root.join("Cargo.toml"),
            format!("[package]\nname = \"{package}\"\nversion = \"0.1.0\"\n[dependencies]\n{dependency}\n"),
        )
        .expect("fixture manifest");
        std::fs::write(crate_root.join("src/lib.rs"), source).expect("fixture source");
    }

    #[test]
    fn binding_crate_is_the_only_admitted_production_query_edge() {
        let root = fixture_root("allowed");
        write_crate(
            &root,
            "worth-ui-query-binding",
            true,
            "pub fn binding_edge() {}",
        );
        assert!(validate_worth_ui_query_edge(&root, &contract())
            .expect("edge validation")
            .is_empty());
        std::fs::remove_dir_all(root).expect("fixture cleanup");
    }

    #[test]
    fn direct_runtime_dependency_reports_the_admitted_path() {
        let root = fixture_root("runtime-dependency");
        write_crate(&root, "worth-ui-runtime", true, "pub fn runtime() {}");
        let diagnostics = validate_worth_ui_query_edge(&root, &contract()).expect("edge validation");
        assert_eq!(diagnostics.len(), 1);
        assert!(diagnostics[0]
            .message()
            .contains("consume binding-owned artifacts"));
        std::fs::remove_dir_all(root).expect("fixture cleanup");
    }

    #[test]
    fn raw_facade_reexport_reports_the_admitted_path() {
        let root = fixture_root("facade-reexport");
        write_crate(
            &root,
            "worth-ui",
            false,
            "pub use worth_query::facade::read::*;",
        );
        let diagnostics = validate_worth_ui_query_edge(&root, &contract()).expect("edge validation");
        assert_eq!(diagnostics.len(), 1);
        assert!(diagnostics[0].message().contains("raw `worth_query`"));
        std::fs::remove_dir_all(root).expect("fixture cleanup");
    }
}

fn is_query_framework_package(dependency: &str, contract: &QueryAudienceContract) -> bool {
    dependency == contract.engine_package
        || contract
            .audiences
            .iter()
            .any(|audience| audience.package == dependency)
}

fn replay_surface_label(dependency: &str, contracts: &RuleContracts) -> Option<String> {
    contracts
        .replay_surfaces
        .iter()
        .find(|surface| replay_surface_matches(dependency, surface))
        .map(|surface| surface.label.clone())
}

fn replay_surface_matches(dependency: &str, surface: &ReplaySurfaceConfig) -> bool {
    if surface
        .package_prefixes
        .iter()
        .any(|prefix| dependency.starts_with(prefix))
    {
        return true;
    }

    let Ok(parsed) = parse_crate_name(dependency) else {
        return false;
    };
    parsed.band == "cert"
        && surface
            .cert_domains
            .iter()
            .any(|domain| parsed.domain == *domain)
}

fn band_rule_map(band_rules: &[BandRuleConfig]) -> BTreeMap<String, Vec<String>> {
    band_rules
        .iter()
        .map(|rule| (rule.source_band.clone(), rule.allowed_target_bands.clone()))
        .collect()
}

fn render_allowed_bands(allowed_target_bands: &[String]) -> String {
    if allowed_target_bands.is_empty() {
        return "none".to_owned();
    }
    allowed_target_bands.join(", ")
}
