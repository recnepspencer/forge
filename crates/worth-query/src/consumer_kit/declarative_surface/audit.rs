use std::collections::{BTreeMap, BTreeSet};

use super::registry::worth_query_declarative_surface_rows;
use super::source::{
    WorthQueryDeclarativeSurfaceAudit, WorthQueryDeclarativeSurfaceFinding,
    WorthQueryDeclarativeSurfaceFindingKind, WorthQueryDeclarativeSurfaceSource,
    WorthQueryDeclarativeSurfaceSourceSite,
};
use super::surface_syntax::public_phase_surface_sites;

pub fn current_declarative_surface_audit() -> WorthQueryDeclarativeSurfaceAudit {
    audit_declarative_surface_sources(&current_sources())
}

pub fn audit_declarative_surface_sources(
    sources: &[WorthQueryDeclarativeSurfaceSource],
) -> WorthQueryDeclarativeSurfaceAudit {
    let rows = worth_query_declarative_surface_rows();
    let mut observed =
        BTreeMap::<(String, String), Vec<WorthQueryDeclarativeSurfaceSourceSite>>::new();

    let mut findings = Vec::new();
    for source in sources {
        match public_phase_surface_sites(source.path(), source.text()) {
            Ok(sites) => {
                for site in sites {
                    observed
                        .entry((source.path().to_string(), site.function_name().to_string()))
                        .or_default()
                        .push(site);
                }
            }
            Err(error) => findings.push(WorthQueryDeclarativeSurfaceFinding::new(
                WorthQueryDeclarativeSurfaceFindingKind::InvalidRustSource,
                WorthQueryDeclarativeSurfaceSourceSite::new(
                    source.path(),
                    error.span().start().line,
                    "<invalid-rust-source>",
                ),
            )),
        }
    }
    for ((path, function_name), sites) in &observed {
        let matching_rows = rows
            .iter()
            .filter(|row| row.source_path() == path && row.function_name() == function_name)
            .collect::<Vec<_>>();
        for site in sites {
            if !matching_rows
                .iter()
                .any(|row| row_matches_site(row, sites.len(), site))
            {
                findings.push(WorthQueryDeclarativeSurfaceFinding::new(
                    WorthQueryDeclarativeSurfaceFindingKind::UnclassifiedPublicPhaseSurface,
                    site.clone(),
                ));
            }
        }
        let mut seen_owners = BTreeSet::new();
        for site in sites {
            if !seen_owners.insert(site.owner()) {
                findings.push(WorthQueryDeclarativeSurfaceFinding::new(
                    WorthQueryDeclarativeSurfaceFindingKind::DuplicatePublicPhaseSurface,
                    site.clone(),
                ));
            }
        }
    }

    for row in rows {
        let sites = observed.get(&(
            row.source_path().to_string(),
            row.function_name().to_string(),
        ));
        if !sites.is_some_and(|sites| {
            sites
                .iter()
                .any(|site| row_matches_site(row, sites.len(), site))
        }) {
            let site = match row.owner() {
                Some(owner) => WorthQueryDeclarativeSurfaceSourceSite::method(
                    row.source_path(),
                    0,
                    owner,
                    row.function_name(),
                ),
                None => WorthQueryDeclarativeSurfaceSourceSite::new(
                    row.source_path(),
                    0,
                    row.function_name(),
                ),
            };
            findings.push(WorthQueryDeclarativeSurfaceFinding::new(
                WorthQueryDeclarativeSurfaceFindingKind::MissingRegisteredSurface,
                site,
            ));
        }
    }

    findings.sort_by(|left, right| left.site().cmp(right.site()));
    WorthQueryDeclarativeSurfaceAudit::new(
        observed.values().map(Vec::len).sum(),
        observed
            .values()
            .flat_map(|sites| sites.iter().map(move |site| (sites.len(), site)))
            .filter(|(site_count, site)| {
                rows.iter().any(|row| {
                    row.source_path() == site.path()
                        && row.function_name() == site.function_name()
                        && row_matches_site(row, *site_count, site)
                })
            })
            .count(),
        findings,
    )
}

fn row_matches_site(
    row: &super::WorthQueryDeclarativeSurfaceRow,
    same_named_site_count: usize,
    site: &WorthQueryDeclarativeSurfaceSourceSite,
) -> bool {
    match row.owner() {
        Some(owner) => site.owner() == Some(owner),
        None => site.owner().is_none() || same_named_site_count == 1,
    }
}

#[rustfmt::skip]
fn current_sources() -> Vec<WorthQueryDeclarativeSurfaceSource> {
    vec![
        WorthQueryDeclarativeSurfaceSource::new(
            "src/ordinary/count/mod.rs",
            include_str!("../../ordinary/count/mod.rs"),
        ),
        WorthQueryDeclarativeSurfaceSource::new(
            "src/ordinary/count/declaration.rs",
            include_str!("../../ordinary/count/declaration.rs"),
        ),
        WorthQueryDeclarativeSurfaceSource::new(
            "src/ordinary/count/execution.rs",
            include_str!("../../ordinary/count/execution.rs"),
        ),
        WorthQueryDeclarativeSurfaceSource::new(
            "src/ordinary/count/request.rs",
            include_str!("../../ordinary/count/request.rs"),
        ),
        WorthQueryDeclarativeSurfaceSource::new(
            "src/ordinary/live/mod.rs",
            include_str!("../../ordinary/live/mod.rs"),
        ),
        WorthQueryDeclarativeSurfaceSource::new(
            "src/ordinary/live/declaration.rs",
            include_str!("../../ordinary/live/declaration.rs"),
        ),
        WorthQueryDeclarativeSurfaceSource::new(
            "src/ordinary/live/disposal.rs",
            include_str!("../../ordinary/live/disposal.rs"),
        ),
        WorthQueryDeclarativeSurfaceSource::new(
            "src/ordinary/live/execution.rs",
            include_str!("../../ordinary/live/execution.rs"),
        ),
        WorthQueryDeclarativeSurfaceSource::new(
            "src/ordinary/live/request.rs",
            include_str!("../../ordinary/live/request.rs"),
        ),
        WorthQueryDeclarativeSurfaceSource::new(
            "src/ordinary/live/continuation/outcome.rs",
            include_str!("../../ordinary/live/continuation/outcome.rs"),
        ),
        WorthQueryDeclarativeSurfaceSource::new(
            "src/ordinary/read/mod.rs",
            include_str!("../../ordinary/read/mod.rs"),
        ),
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
            "src/ordinary/read/context/mod.rs",
            include_str!("../../ordinary/read/context/mod.rs"),
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
        WorthQueryDeclarativeSurfaceSource::new(
            "src/application/domain_handle/admitted_handle/declaration_entry/binding/context.rs",
            include_str!(
                "../../application/domain_handle/admitted_handle/declaration_entry/binding/context.rs"
            ),
        ),
        WorthQueryDeclarativeSurfaceSource::new(
            "src/application/domain_handle/admitted_handle/declaration_entry/binding/target.rs",
            include_str!(
                "../../application/domain_handle/admitted_handle/declaration_entry/binding/target.rs"
            ),
        ),
        WorthQueryDeclarativeSurfaceSource::new(
            "src/application/domain_handle/admitted_handle/declaration_entry/progression.rs",
            include_str!(
                "../../application/domain_handle/admitted_handle/declaration_entry/progression.rs"
            ),
        ),
        WorthQueryDeclarativeSurfaceSource::new(
            "src/application/domain_handle/admitted_handle/declaration_entry/seam.rs",
            include_str!(
                "../../application/domain_handle/admitted_handle/declaration_entry/seam.rs"
            ),
        ),
        WorthQueryDeclarativeSurfaceSource::new(
            "src/historical/planner.rs",
            include_str!("../../historical/planner.rs"),
        ),
        WorthQueryDeclarativeSurfaceSource::new(
            "src/correspondence/resolution.rs",
            include_str!("../../correspondence/resolution.rs"),
        ),
        WorthQueryDeclarativeSurfaceSource::new(
            "src/preview/scoped.rs",
            include_str!("../../preview/scoped.rs"),
        ),
        WorthQueryDeclarativeSurfaceSource::new(
            "src/workflow/foundation.rs",
            include_str!("../../workflow/foundation.rs"),
        ),
        WorthQueryDeclarativeSurfaceSource::new(
            "src/workflow/lowering/merge.rs",
            include_str!("../../workflow/lowering/merge.rs"),
        ),
        WorthQueryDeclarativeSurfaceSource::new(
            "src/workflow/lowering/mutation.rs",
            include_str!("../../workflow/lowering/mutation.rs"),
        ),
        WorthQueryDeclarativeSurfaceSource::new(
            "src/workflow/lowering/writeback.rs",
            include_str!("../../workflow/lowering/writeback.rs"),
        ),
        WorthQueryDeclarativeSurfaceSource::new(
            "src/workflow/inspection/operations.rs",
            include_str!("../../workflow/inspection/operations.rs"),
        ),
        WorthQueryDeclarativeSurfaceSource::new("src/authoring/domain_operation/declaration.rs", include_str!("../../authoring/domain_operation/declaration.rs")),
        WorthQueryDeclarativeSurfaceSource::new("src/authoring/request/guided_path.rs", include_str!("../../authoring/request/guided_path.rs")),
        WorthQueryDeclarativeSurfaceSource::new("src/binding/runtime.rs", include_str!("../../binding/runtime.rs")),
        WorthQueryDeclarativeSurfaceSource::new("src/canonicalization/pipeline.rs", include_str!("../../canonicalization/pipeline.rs")),
        WorthQueryDeclarativeSurfaceSource::new("src/execution/preflight.rs", include_str!("../../execution/preflight.rs")),
        WorthQueryDeclarativeSurfaceSource::new("src/live/mod.rs", include_str!("../../live/mod.rs")),
        WorthQueryDeclarativeSurfaceSource::new("src/live/region_scoped.rs", include_str!("../../live/region_scoped.rs")),
        WorthQueryDeclarativeSurfaceSource::new("src/planning/mod.rs", include_str!("../../planning/mod.rs")),
        WorthQueryDeclarativeSurfaceSource::new("src/policy_basis/admission.rs", include_str!("../../policy_basis/admission.rs")),
        WorthQueryDeclarativeSurfaceSource::new("src/policy_delivery/shape.rs", include_str!("../../policy_delivery/shape.rs")),
        WorthQueryDeclarativeSurfaceSource::new("src/policy_live/admission.rs", include_str!("../../policy_live/admission.rs")),
        WorthQueryDeclarativeSurfaceSource::new("src/policy_plan/branch.rs", include_str!("../../policy_plan/branch.rs")),
        WorthQueryDeclarativeSurfaceSource::new("src/policy_plan/current.rs", include_str!("../../policy_plan/current.rs")),
        WorthQueryDeclarativeSurfaceSource::new("src/policy_plan/diff.rs", include_str!("../../policy_plan/diff.rs")),
        WorthQueryDeclarativeSurfaceSource::new("src/policy_plan/historical.rs", include_str!("../../policy_plan/historical.rs")),
        WorthQueryDeclarativeSurfaceSource::new("src/policy_plan/optimizer.rs", include_str!("../../policy_plan/optimizer.rs")),
        WorthQueryDeclarativeSurfaceSource::new("src/preview/mod.rs", include_str!("../../preview/mod.rs")),
        WorthQueryDeclarativeSurfaceSource::new("src/validation/pipeline.rs", include_str!("../../validation/pipeline.rs")),
        WorthQueryDeclarativeSurfaceSource::new("src/facade/exports_application.rs", include_str!("../../facade/exports_application.rs")),
        WorthQueryDeclarativeSurfaceSource::new("src/facade/exports_certification.rs", include_str!("../../facade/exports_certification.rs")),
        WorthQueryDeclarativeSurfaceSource::new("src/facade/exports_foundation.rs", include_str!("../../facade/exports_foundation.rs")),
        WorthQueryDeclarativeSurfaceSource::new("src/facade/exports_live_capability.rs", include_str!("../../facade/exports_live_capability.rs")),
        WorthQueryDeclarativeSurfaceSource::new("src/facade/exports_policy.rs", include_str!("../../facade/exports_policy.rs")),
        WorthQueryDeclarativeSurfaceSource::new("src/facade/exports_read.rs", include_str!("../../facade/exports_read.rs")),
        WorthQueryDeclarativeSurfaceSource::new("src/facade/exports_runtime.rs", include_str!("../../facade/exports_runtime.rs")),
        WorthQueryDeclarativeSurfaceSource::new("src/facade/exports_runtime_capabilities.rs", include_str!("../../facade/exports_runtime_capabilities.rs")),
        WorthQueryDeclarativeSurfaceSource::new("src/facade/exports_runtime_core.rs", include_str!("../../facade/exports_runtime_core.rs")),
        WorthQueryDeclarativeSurfaceSource::new("src/facade/exports_runtime_phase_nine.rs", include_str!("../../facade/exports_runtime_phase_nine.rs")),
        WorthQueryDeclarativeSurfaceSource::new("src/facade/exports_runtime_products.rs", include_str!("../../facade/exports_runtime_products.rs")),
        WorthQueryDeclarativeSurfaceSource::new("src/lib.rs", include_str!("../../lib.rs")),
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
        "src/application/domain_handle/admitted_handle/declaration_entry/binding/context.rs",
        "src/application/domain_handle/admitted_handle/declaration_entry/binding/target.rs",
        "src/application/domain_handle/admitted_handle/declaration_entry/progression.rs",
        "src/application/domain_handle/admitted_handle/declaration_entry/seam.rs",
        "src/historical/planner.rs",
        "src/correspondence/resolution.rs",
        "src/preview/scoped.rs",
        "src/workflow/foundation.rs",
        "src/workflow/lowering/merge.rs",
        "src/workflow/lowering/mutation.rs",
        "src/workflow/lowering/writeback.rs",
        "src/workflow/inspection/operations.rs",
        "src/authoring/domain_operation/declaration.rs",
        "src/authoring/request/guided_path.rs",
        "src/binding/runtime.rs",
        "src/canonicalization/pipeline.rs",
        "src/execution/preflight.rs",
        "src/live/mod.rs",
        "src/live/region_scoped.rs",
        "src/planning/mod.rs",
        "src/policy_basis/admission.rs",
        "src/policy_delivery/shape.rs",
        "src/policy_live/admission.rs",
        "src/policy_plan/branch.rs",
        "src/policy_plan/current.rs",
        "src/policy_plan/diff.rs",
        "src/policy_plan/historical.rs",
        "src/policy_plan/optimizer.rs",
        "src/preview/mod.rs",
        "src/validation/pipeline.rs",
        "src/facade/exports_application.rs",
        "src/facade/exports_certification.rs",
        "src/facade/exports_foundation.rs",
        "src/facade/exports_live_capability.rs",
        "src/facade/exports_policy.rs",
        "src/facade/exports_read.rs",
        "src/facade/exports_runtime.rs",
        "src/facade/exports_runtime_capabilities.rs",
        "src/facade/exports_runtime_core.rs",
        "src/facade/exports_runtime_phase_nine.rs",
        "src/facade/exports_runtime_products.rs",
        "src/lib.rs",
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
