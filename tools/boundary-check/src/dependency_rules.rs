use crate::config::{
    BandRuleConfig, LawSubstrateConfig, QueryAudienceContract, ReplaySurfaceConfig, RuleContracts,
};
use crate::diagnostics::{Diagnostic, DiagnosticCode};
use crate::manifest_types::Road1Package;
use crate::naming::parse_crate_name;
use crate::source_rules::{illegal_law_substrate_edge, is_legal_law_substrate_edge};
use std::collections::BTreeMap;

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
