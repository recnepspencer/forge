//! Query audience dependency law: engine is facade-only; facades are band-gated.
//!
//! Decisions are driven only by the configured `QueryAudienceContract`. This
//! module does not own general band direction or non-Query replay families.

use crate::config::{QueryAudienceContract, QueryAudienceFacadeConfig};
use crate::diagnostics::{Diagnostic, DiagnosticCode};
use crate::manifest_types::Road1Package;
use crate::naming::parse_crate_name;
use std::collections::BTreeMap;

pub(crate) fn validate_query_audience_rules(
    packages: &[Road1Package],
    contract: &QueryAudienceContract,
) -> Vec<Diagnostic> {
    let audiences_by_package = audience_index(contract);
    let mut diagnostics = Vec::new();

    for package in packages {
        if let Some(certification_package) = &contract.certification_package {
            if package
                .dependencies
                .iter()
                .any(|dependency| dependency == certification_package)
                && !contract
                    .certification_consumers
                    .iter()
                    .any(|owner| owner == &package.name)
            {
                diagnostics.push(Diagnostic::with_legal_home(
                    DiagnosticCode::Bc3002WrongQueryAudience,
                    &package.name,
                    format!(
                        "package may not depend on cold Query certification `{certification_package}`"
                    ),
                    format!(
                        "tools/boundary-check/config/road1.toml [rule_contracts.query_audience.certification_consumers]; configured owners: {}",
                        render_bands(&contract.certification_consumers)
                    ),
                ));
            }
        }
        let Ok(source_name) = parse_crate_name(&package.name) else {
            continue;
        };

        for dependency in &package.dependencies {
            if dependency == &contract.engine_package
                || contract
                    .internal_packages
                    .iter()
                    .any(|name| name == dependency)
            {
                diagnostics.push(deny_direct_engine(package, &source_name.band, contract));
                continue;
            }

            if let Some(audience) = audiences_by_package.get(dependency.as_str()) {
                if !audience
                    .allowed_bands
                    .iter()
                    .any(|band| band == &source_name.band)
                {
                    diagnostics.push(deny_wrong_audience(
                        package,
                        &source_name.band,
                        audience,
                        contract,
                    ));
                }
            }
        }
    }

    diagnostics
}

fn audience_index(contract: &QueryAudienceContract) -> BTreeMap<&str, &QueryAudienceFacadeConfig> {
    contract
        .audiences
        .iter()
        .map(|audience| (audience.package.as_str(), audience))
        .collect()
}

fn deny_direct_engine(
    package: &Road1Package,
    source_band: &str,
    contract: &QueryAudienceContract,
) -> Diagnostic {
    Diagnostic::with_legal_home(
        DiagnosticCode::Bc3001DirectQueryEngine,
        &package.name,
        render_direct_engine_message(source_band, contract),
        render_query_legal_home(source_band, contract),
    )
}

fn deny_wrong_audience(
    package: &Road1Package,
    source_band: &str,
    audience: &QueryAudienceFacadeConfig,
    contract: &QueryAudienceContract,
) -> Diagnostic {
    Diagnostic::with_legal_home(
        DiagnosticCode::Bc3002WrongQueryAudience,
        &package.name,
        format!(
            "{source_band} band may not depend on Query audience facade `{}` ({}); allowed bands: {}; {}",
            audience.package,
            audience.label,
            render_bands(&audience.allowed_bands),
            audience.guidance
        ),
        render_query_legal_home(source_band, contract),
    )
}

fn render_query_legal_home(source_band: &str, contract: &QueryAudienceContract) -> String {
    let facade_root = if contract.workspace == "." {
        "crates".to_owned()
    } else {
        format!("{}/crates", contract.workspace.trim_end_matches('/'))
    };
    let facades = contract
        .audiences
        .iter()
        .filter(|audience| {
            audience
                .allowed_bands
                .iter()
                .any(|band| band == source_band)
        })
        .map(|audience| format!("{facade_root}/{}/src/facade.rs", audience.package))
        .collect::<Vec<_>>();
    if facades.is_empty() {
        format!(
            "tools/boundary-check/config/road1.toml [rule_contracts.query_audience]: no Query audience is legal for `{source_band}`; remove the Query dependency"
        )
    } else {
        format!(
            "{}; consume Query only through the configured facade for `{source_band}`",
            facades.join(" or ")
        )
    }
}

fn render_direct_engine_message(source_band: &str, contract: &QueryAudienceContract) -> String {
    let legal_homes: Vec<String> = contract
        .audiences
        .iter()
        .filter(|audience| {
            audience
                .allowed_bands
                .iter()
                .any(|band| band == source_band)
        })
        .map(|audience| format!("`{}` ({})", audience.package, audience.guidance))
        .collect();

    if legal_homes.is_empty() {
        format!(
            "direct dependency on `{}` is denied; no Query audience facade is legal for the `{}` band; engine consumption is only through configured audience facades",
            contract.engine_package, source_band
        )
    } else {
        format!(
            "direct dependency on `{}` is denied; use {} instead",
            contract.engine_package,
            legal_homes.join(" or ")
        )
    }
}

fn render_bands(bands: &[String]) -> String {
    if bands.is_empty() {
        return "none".to_owned();
    }
    bands.join(", ")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn contract() -> QueryAudienceContract {
        QueryAudienceContract {
            workspace: "workspaces/worth-query".into(),
            engine_package: "worth-query".into(),
            certification_package: Some("worth-query-certification".into()),
            certification_authority_packages: vec![
                "worth-query-host".into(),
                "worth-query-replay".into(),
            ],
            certification_consumers: vec!["worth-cert-workflows".into()],
            internal_packages: vec![
                "worth-query-declaration".into(),
                "worth-query-installation".into(),
            ],
            facade_surfaces: Vec::new(),
            audiences: [
                ("worth-query-decl", &["entry", "cert"][..]),
                ("worth-query-host", &["entry", "cert"][..]),
                ("worth-query-replay", &["cert"][..]),
            ]
            .into_iter()
            .map(|(package, bands)| QueryAudienceFacadeConfig {
                package: package.into(),
                label: package.into(),
                allowed_bands: bands.iter().map(|band| (*band).into()).collect(),
                guidance: "configured guidance".into(),
                authority_packages: vec!["worth-query".into()],
            })
            .collect(),
        }
    }

    #[test]
    fn legal_home_names_every_facade_configured_for_the_source_band() {
        let contract = contract();
        assert_eq!(
            render_query_legal_home("schema", &contract),
            "tools/boundary-check/config/road1.toml [rule_contracts.query_audience]: no Query audience is legal for `schema`; remove the Query dependency"
        );
        let entry = render_query_legal_home("entry", &contract);
        assert!(entry.contains("workspaces/worth-query/crates/worth-query-decl/src/facade.rs"));
        assert!(entry.contains("workspaces/worth-query/crates/worth-query-host/src/facade.rs"));
        assert!(!entry.contains("worth-query-replay"));
        let cert = render_query_legal_home("cert", &contract);
        for facade in ["decl", "host", "replay"] {
            assert!(cert.contains(&format!(
                "workspaces/worth-query/crates/worth-query-{facade}/src/facade.rs"
            )));
        }
    }

    #[test]
    fn cold_certification_dependency_is_explicitly_owner_gated() {
        let contract = contract();
        let denied = Road1Package {
            name: "worth-cert-replay".into(),
            dependencies: vec!["worth-query-certification".into()],
            manifest_path: "Cargo.toml".into(),
        };
        assert!(validate_query_audience_rules(&[denied], &contract)
            .iter()
            .any(|diagnostic| diagnostic.code() == DiagnosticCode::Bc3002WrongQueryAudience));

        let admitted = Road1Package {
            name: "worth-cert-workflows".into(),
            dependencies: vec!["worth-query-certification".into()],
            manifest_path: "Cargo.toml".into(),
        };
        assert!(validate_query_audience_rules(&[admitted], &contract).is_empty());
    }

    #[test]
    fn replay_audience_is_denied_across_every_ordinary_consumer_band() {
        let contract = contract();
        for band in [
            "entry", "derived", "app", "ui", "resolver", "solver", "pack",
        ] {
            let package = Road1Package {
                name: format!("worth-{band}-phase-twenty-six"),
                dependencies: vec!["worth-query-replay".into()],
                manifest_path: "Cargo.toml".into(),
            };
            let diagnostics = validate_query_audience_rules(&[package], &contract);
            assert!(
                diagnostics
                    .iter()
                    .any(|diagnostic| diagnostic.code() == DiagnosticCode::Bc3002WrongQueryAudience),
                "{band} must not import replay"
            );
        }
        let cert = Road1Package {
            name: "worth-cert-workflows".into(),
            dependencies: vec!["worth-query-replay".into()],
            manifest_path: "Cargo.toml".into(),
        };
        assert!(validate_query_audience_rules(&[cert], &contract).is_empty());
    }
}
