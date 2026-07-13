use std::collections::{BTreeMap, BTreeSet};

use super::registry::worth_query_declarative_surface_rows;
use super::source::{
    WorthQueryDeclarativeSurfaceAudit, WorthQueryDeclarativeSurfaceFinding,
    WorthQueryDeclarativeSurfaceFindingKind, WorthQueryDeclarativeSurfaceSource,
    WorthQueryDeclarativeSurfaceSourceSite,
};

pub fn current_declarative_surface_audit() -> WorthQueryDeclarativeSurfaceAudit {
    audit_declarative_surface_sources(&current_sources())
}

pub fn audit_declarative_surface_sources(
    sources: &[WorthQueryDeclarativeSurfaceSource],
) -> WorthQueryDeclarativeSurfaceAudit {
    let rows = worth_query_declarative_surface_rows();
    let registered = rows
        .iter()
        .map(|row| {
            (
                row.source_path().to_string(),
                row.function_name().to_string(),
            )
        })
        .collect::<BTreeSet<_>>();
    let mut observed =
        BTreeMap::<(String, String), Vec<WorthQueryDeclarativeSurfaceSourceSite>>::new();

    for source in sources {
        for site in phase_surface_sites(source) {
            observed
                .entry((source.path().to_string(), site.function_name().to_string()))
                .or_default()
                .push(site);
        }
    }

    let mut findings = Vec::new();
    for ((path, function_name), sites) in &observed {
        if !registered.contains(&(path.clone(), function_name.clone())) {
            findings.push(WorthQueryDeclarativeSurfaceFinding::new(
                WorthQueryDeclarativeSurfaceFindingKind::UnclassifiedPublicPhaseSurface,
                sites[0].clone(),
            ));
        }
        for duplicate in sites.iter().skip(1) {
            findings.push(WorthQueryDeclarativeSurfaceFinding::new(
                WorthQueryDeclarativeSurfaceFindingKind::DuplicatePublicPhaseSurface,
                duplicate.clone(),
            ));
        }
    }

    for row in rows {
        if !observed.contains_key(&(
            row.source_path().to_string(),
            row.function_name().to_string(),
        )) {
            findings.push(WorthQueryDeclarativeSurfaceFinding::new(
                WorthQueryDeclarativeSurfaceFindingKind::MissingRegisteredSurface,
                WorthQueryDeclarativeSurfaceSourceSite::new(
                    row.source_path(),
                    0,
                    row.function_name(),
                ),
            ));
        }
    }

    findings.sort_by(|left, right| left.site().cmp(right.site()));
    WorthQueryDeclarativeSurfaceAudit::new(
        observed.values().map(Vec::len).sum(),
        observed
            .keys()
            .filter(|key| registered.contains(key))
            .count(),
        findings,
    )
}

fn current_sources() -> [WorthQueryDeclarativeSurfaceSource; 5] {
    [
        WorthQueryDeclarativeSurfaceSource::new(
            "src/ordinary/read/declaration.rs",
            include_str!("../../ordinary/read/declaration.rs"),
        ),
        WorthQueryDeclarativeSurfaceSource::new(
            "src/ordinary/read/execution.rs",
            include_str!("../../ordinary/read/execution.rs"),
        ),
        WorthQueryDeclarativeSurfaceSource::new(
            "src/runtime/workspace_queries.rs",
            include_str!("../../runtime/workspace_queries.rs"),
        ),
        WorthQueryDeclarativeSurfaceSource::new(
            "src/application/domain_handle/admitted_handle/declaration_entry/orchestration.rs",
            include_str!(
                "../../application/domain_handle/admitted_handle/declaration_entry/orchestration.rs"
            ),
        ),
        WorthQueryDeclarativeSurfaceSource::new(
            "src/application/domain_handle/admitted_handle/declaration_entry/products.rs",
            include_str!(
                "../../application/domain_handle/admitted_handle/declaration_entry/products.rs"
            ),
        ),
    ]
}

fn phase_surface_sites(
    source: &WorthQueryDeclarativeSurfaceSource,
) -> Vec<WorthQueryDeclarativeSurfaceSourceSite> {
    source
        .text()
        .lines()
        .enumerate()
        .filter_map(|(index, line)| public_function_name(line).map(|name| (index + 1, name)))
        .filter(|(_, function_name)| is_phase_surface(function_name))
        .map(|(line, function_name)| {
            WorthQueryDeclarativeSurfaceSourceSite::new(source.path(), line, function_name)
        })
        .collect()
}

fn public_function_name(line: &str) -> Option<&str> {
    let declaration = line.trim_start().strip_prefix("pub fn ")?;
    let name_end = declaration.find(['<', '(']).unwrap_or(declaration.len());
    Some(&declaration[..name_end])
}

fn is_phase_surface(function_name: &str) -> bool {
    function_name == "declare"
        || function_name == "run"
        || function_name == "install_program"
        || function_name.starts_with("compose_")
        || function_name.starts_with("define_")
        || function_name.starts_with("execute_")
        || function_name.starts_with("explain_")
        || function_name.starts_with("admit_")
        || function_name.starts_with("plan_")
        || function_name.starts_with("inspect")
        || function_name.starts_with("orchestrate_")
}
