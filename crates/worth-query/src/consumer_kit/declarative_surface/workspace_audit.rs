use super::{
    audit_declarative_surface_sources, WorthQueryDeclarativeSurfaceAudit,
    WorthQueryDeclarativeSurfaceSource,
};

pub(super) fn workspace_declarative_surface_audit() -> WorthQueryDeclarativeSurfaceAudit {
    let manifest_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut sources = Vec::new();
    collect_rust_sources(
        manifest_root,
        &manifest_root.join("src/ordinary"),
        &mut sources,
    );
    for relative_path in DECLARATIVE_MECHANISM_SOURCES {
        let absolute_path = manifest_root.join(relative_path);
        sources.push(WorthQueryDeclarativeSurfaceSource::new(
            *relative_path,
            std::fs::read_to_string(&absolute_path).unwrap_or_else(|error| {
                panic!("failed to read {}: {error}", absolute_path.display())
            }),
        ));
    }
    audit_declarative_surface_sources(&sources)
}

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

const DECLARATIVE_MECHANISM_SOURCES: &[&str] = &[
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
    "src/facade/exports_aggregate.rs",
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
];
