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

fn current_sources() -> [WorthQueryDeclarativeSurfaceSource; 7] {
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
            "src/ordinary/read/request.rs",
            include_str!("../../ordinary/read/request.rs"),
        ),
        WorthQueryDeclarativeSurfaceSource::new(
            "src/ordinary/read/context/declaration.rs",
            include_str!("../../ordinary/read/context/declaration.rs"),
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

#[cfg(test)]
pub(super) fn workspace_declarative_surface_audit() -> WorthQueryDeclarativeSurfaceAudit {
    let manifest_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut sources = Vec::new();
    collect_rust_sources(
        manifest_root,
        &manifest_root.join("src/ordinary"),
        &mut sources,
    );
    for relative_path in [
        "src/runtime/workspace_queries.rs",
        "src/application/domain_handle/admitted_handle/declaration_entry/orchestration.rs",
        "src/application/domain_handle/admitted_handle/declaration_entry/products.rs",
    ] {
        let absolute_path = manifest_root.join(relative_path);
        sources.push(WorthQueryDeclarativeSurfaceSource::new(
            relative_path,
            std::fs::read_to_string(&absolute_path).unwrap_or_else(|error| {
                panic!("failed to read {}: {error}", absolute_path.display())
            }),
        ));
    }
    audit_declarative_surface_sources(&sources)
}

#[cfg(test)]
fn collect_rust_sources(
    manifest_root: &std::path::Path,
    directory: &std::path::Path,
    sources: &mut Vec<WorthQueryDeclarativeSurfaceSource>,
) {
    let mut entries = std::fs::read_dir(directory)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", directory.display()))
        .map(|entry| {
            entry
                .expect("source directory entry should be readable")
                .path()
        })
        .collect::<Vec<_>>();
    entries.sort();
    for path in entries {
        if path.is_dir() {
            collect_rust_sources(manifest_root, &path, sources);
        } else if path.extension().and_then(std::ffi::OsStr::to_str) == Some("rs") {
            let relative_path = path
                .strip_prefix(manifest_root)
                .expect("ordinary source must remain below the crate root")
                .to_string_lossy()
                .replace('\\', "/");
            sources.push(WorthQueryDeclarativeSurfaceSource::new(
                relative_path,
                std::fs::read_to_string(&path)
                    .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display())),
            ));
        }
    }
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
    let line = line.trim_start();
    let declaration = [
        "pub fn ",
        "pub async fn ",
        "pub const fn ",
        "pub unsafe fn ",
    ]
    .into_iter()
    .find_map(|prefix| line.strip_prefix(prefix))?;
    let name_end = declaration.find(['<', '(']).unwrap_or(declaration.len());
    Some(&declaration[..name_end])
}

fn is_phase_surface(function_name: &str) -> bool {
    function_name == "declare"
        || function_name == "run"
        || function_name == "using"
        || function_name == "current"
        || function_name == "under_policy_tenant"
        || function_name == "with_relationship_proofs"
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
