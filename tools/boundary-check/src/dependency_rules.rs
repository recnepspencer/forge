use crate::config::{BandRuleConfig, ReplaySurfaceConfig, RuleContracts};
use crate::diagnostics::{Diagnostic, DiagnosticCode};
use crate::manifest_types::Road1Package;
use crate::naming::parse_crate_name;
use std::collections::BTreeMap;

pub(crate) fn validate_dependency_rules(
    packages: &[Road1Package],
    contracts: &RuleContracts,
) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    let band_rules = band_rule_map(&contracts.band_rules);

    for package in packages {
        let Ok(source_name) = parse_crate_name(&package.name) else {
            continue;
        };

        for dependency in &package.dependencies {
            if dependency == "worth-query"
                && !contracts
                    .query_host_bands
                    .iter()
                    .any(|band| band == &source_name.band)
            {
                diagnostics.push(Diagnostic::new(
                    DiagnosticCode::Bc3001QueryImportOutsideEntry,
                    &package.name,
                    "only entry crates may depend on worth-query",
                ));
            }

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
