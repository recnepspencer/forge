use crate::domain_capabilities::identity::compose_compile_fail_boundary_digest;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryDomainCapabilityCompileFailBoundary {
    label: &'static str,
    path: &'static str,
}

impl WorthQueryDomainCapabilityCompileFailBoundary {
    pub(crate) const fn new(label: &'static str, path: &'static str) -> Self {
        Self { label, path }
    }

    pub fn label(&self) -> &'static str {
        self.label
    }

    pub fn path(&self) -> &'static str {
        self.path
    }
}

const COMPILE_FAIL_BOUNDARIES: [WorthQueryDomainCapabilityCompileFailBoundary; 41] = [
    WorthQueryDomainCapabilityCompileFailBoundary::new(
        "checked_outcome_constructor_private",
        "tests/ui/domain_capabilities/dx_boundaries/domain_capability_checked_outcome_constructor_private.rs",
    ),
    WorthQueryDomainCapabilityCompileFailBoundary::new(
        "support_requires_because",
        "tests/ui/domain_capabilities/dx_boundaries/domain_capability_support_requires_because.rs",
    ),
    WorthQueryDomainCapabilityCompileFailBoundary::new(
        "preview_inspection_requires_because",
        "tests/ui/domain_capabilities/dx_boundaries/domain_capability_preview_inspection_requires_because.rs",
    ),
    WorthQueryDomainCapabilityCompileFailBoundary::new(
        "lower_runtime_support_requires_because",
        "tests/ui/domain_capabilities/dx_boundaries/domain_capability_lower_runtime_support_requires_because.rs",
    ),
    WorthQueryDomainCapabilityCompileFailBoundary::new(
        "lower_runtime_explanation_requires_because",
        "tests/ui/domain_capabilities/dx_boundaries/domain_capability_lower_runtime_explanation_requires_because.rs",
    ),
    WorthQueryDomainCapabilityCompileFailBoundary::new(
        "aftermath_requires_because",
        "tests/ui/domain_capabilities/dx_boundaries/domain_capability_aftermath_requires_because.rs",
    ),
    WorthQueryDomainCapabilityCompileFailBoundary::new(
        "admission_requires_because",
        "tests/ui/domain_capabilities/dx_boundaries/domain_capability_admission_requires_because.rs",
    ),
    WorthQueryDomainCapabilityCompileFailBoundary::new(
        "continuity_requires_because",
        "tests/ui/domain_capabilities/dx_boundaries/domain_capability_continuity_requires_because.rs",
    ),
    WorthQueryDomainCapabilityCompileFailBoundary::new(
        "invariant_registration_requires_because",
        "tests/ui/domain_capabilities/dx_boundaries/domain_capability_invariant_registration_requires_because.rs",
    ),
    WorthQueryDomainCapabilityCompileFailBoundary::new(
        "workflow_requires_because",
        "tests/ui/domain_capabilities/dx_boundaries/domain_capability_workflow_requires_because.rs",
    ),
    WorthQueryDomainCapabilityCompileFailBoundary::new(
        "workflow_draft_has_no_workflow_declaration",
        "tests/ui/domain_capabilities/dx_boundaries/domain_capability_workflow_draft_has_no_workflow_declaration.rs",
    ),
    WorthQueryDomainCapabilityCompileFailBoundary::new(
        "workflow_rejects_raw_preview_identity",
        "tests/ui/domain_capabilities/dx_boundaries/domain_capability_workflow_rejects_raw_preview_identity.rs",
    ),
    WorthQueryDomainCapabilityCompileFailBoundary::new(
        "aftermath_draft_has_no_review",
        "tests/ui/domain_capabilities/dx_boundaries/domain_capability_aftermath_draft_has_no_review.rs",
    ),
    WorthQueryDomainCapabilityCompileFailBoundary::new(
        "lower_runtime_explanation_draft_has_no_review",
        "tests/ui/domain_capabilities/dx_boundaries/domain_capability_lower_runtime_explanation_draft_has_no_review.rs",
    ),
    WorthQueryDomainCapabilityCompileFailBoundary::new(
        "external_lower_runtime_boundary_source_impl_forbidden",
        "tests/ui/domain_capabilities/dx_boundaries/external_lower_runtime_boundary_source_impl_forbidden.rs",
    ),
    WorthQueryDomainCapabilityCompileFailBoundary::new(
        "lower_runtime_boundary_source_rejects_binding_target",
        "tests/ui/domain_capabilities/dx_boundaries/lower_runtime_boundary_source_rejects_binding_target.rs",
    ),
    WorthQueryDomainCapabilityCompileFailBoundary::new(
        "lower_runtime_boundary_source_rejects_raw_string",
        "tests/ui/domain_capabilities/dx_boundaries/lower_runtime_boundary_source_rejects_raw_string.rs",
    ),
    WorthQueryDomainCapabilityCompileFailBoundary::new(
        "certification_surface_row_constructor_private",
        "tests/ui/domain_capabilities/certification/domain_capability_certification_surface_row_constructor_private.rs",
    ),
    WorthQueryDomainCapabilityCompileFailBoundary::new(
        "certification_inventory_constructor_private",
        "tests/ui/domain_capabilities/certification/domain_capability_certification_inventory_constructor_private.rs",
    ),
    WorthQueryDomainCapabilityCompileFailBoundary::new(
        "certification_surface_constructor_private",
        "tests/ui/domain_capabilities/certification/domain_capability_certification_surface_constructor_private.rs",
    ),
    WorthQueryDomainCapabilityCompileFailBoundary::new(
        "admission_runtime_decision_requires_admitted_plan_target",
        "tests/ui/domain_capabilities/boundaries/materializers/domain_capability_admission_runtime_decision_requires_admitted_plan_target.rs",
    ),
    WorthQueryDomainCapabilityCompileFailBoundary::new(
        "admission_support_traceability_requires_admitted_plan_target",
        "tests/ui/domain_capabilities/boundaries/materializers/domain_capability_admission_support_traceability_requires_admitted_plan_target.rs",
    ),
    WorthQueryDomainCapabilityCompileFailBoundary::new(
        "admitted_preview_workflow_foundation_requires_workflow_target",
        "tests/ui/domain_capabilities/boundaries/materializers/domain_capability_admitted_preview_workflow_foundation_requires_workflow_target.rs",
    ),
    WorthQueryDomainCapabilityCompileFailBoundary::new(
        "continuity_correspondence_requires_admitted_plan_target",
        "tests/ui/domain_capabilities/boundaries/materializers/domain_capability_continuity_correspondence_requires_admitted_plan_target.rs",
    ),
    WorthQueryDomainCapabilityCompileFailBoundary::new(
        "preview_workflow_artifact_requires_workflow_target",
        "tests/ui/domain_capabilities/boundaries/materializers/domain_capability_preview_workflow_artifact_requires_workflow_target.rs",
    ),
    WorthQueryDomainCapabilityCompileFailBoundary::new(
        "support_traceability_report_requires_admitted_plan_target",
        "tests/ui/domain_capabilities/boundaries/materializers/domain_capability_support_traceability_report_requires_admitted_plan_target.rs",
    ),
    WorthQueryDomainCapabilityCompileFailBoundary::new(
        "workflow_declaration_requires_workflow_target",
        "tests/ui/domain_capabilities/boundaries/materializers/domain_capability_workflow_declaration_requires_workflow_target.rs",
    ),
    WorthQueryDomainCapabilityCompileFailBoundary::new(
        "workflow_inspection_requires_workflow_target",
        "tests/ui/domain_capabilities/boundaries/materializers/domain_capability_workflow_inspection_requires_workflow_target.rs",
    ),
    WorthQueryDomainCapabilityCompileFailBoundary::new(
        "workflow_lowering_requires_workflow_target",
        "tests/ui/domain_capabilities/boundaries/materializers/domain_capability_workflow_lowering_requires_workflow_target.rs",
    ),
    WorthQueryDomainCapabilityCompileFailBoundary::new(
        "cannot_construct_admitted_directly",
        "tests/ui/domain_capabilities/boundaries/progression/domain_capability_cannot_construct_admitted_directly.rs",
    ),
    WorthQueryDomainCapabilityCompileFailBoundary::new(
        "cannot_construct_eligible_directly",
        "tests/ui/domain_capabilities/boundaries/progression/domain_capability_cannot_construct_eligible_directly.rs",
    ),
    WorthQueryDomainCapabilityCompileFailBoundary::new(
        "cannot_construct_materialization_ready_directly",
        "tests/ui/domain_capabilities/boundaries/progression/domain_capability_cannot_construct_materialization_ready_directly.rs",
    ),
    WorthQueryDomainCapabilityCompileFailBoundary::new(
        "requested_cannot_admit",
        "tests/ui/domain_capabilities/boundaries/progression/domain_capability_requested_cannot_admit.rs",
    ),
    WorthQueryDomainCapabilityCompileFailBoundary::new(
        "requested_cannot_prepare_materialization",
        "tests/ui/domain_capabilities/boundaries/progression/domain_capability_requested_cannot_prepare_materialization.rs",
    ),
    WorthQueryDomainCapabilityCompileFailBoundary::new(
        "declaration_entry_route_target_cannot_satisfy_contribution_target",
        "tests/ui/domain_capabilities/boundaries/runtime_targets/declaration_entry_route_target_cannot_satisfy_contribution_target.rs",
    ),
    WorthQueryDomainCapabilityCompileFailBoundary::new(
        "explanation_artifact_requires_lower_runtime_target",
        "tests/ui/domain_capabilities/boundaries/runtime_targets/domain_capability_explanation_artifact_requires_lower_runtime_target.rs",
    ),
    WorthQueryDomainCapabilityCompileFailBoundary::new(
        "declaration_support_artifact_requires_declaration_target",
        "tests/ui/domain_capabilities/boundaries/runtime_targets/declaration_bound/domain_capability_declaration_support_artifact_requires_declaration_target.rs",
    ),
    WorthQueryDomainCapabilityCompileFailBoundary::new(
        "invariant_registration_artifact_requires_declaration_target",
        "tests/ui/domain_capabilities/boundaries/runtime_targets/declaration_bound/domain_capability_invariant_registration_artifact_requires_declaration_target.rs",
    ),
    WorthQueryDomainCapabilityCompileFailBoundary::new(
        "graph_capability_row_requires_lower_runtime_target",
        "tests/ui/domain_capabilities/boundaries/runtime_targets/lower_runtime_bound/domain_capability_graph_capability_row_requires_lower_runtime_target.rs",
    ),
    WorthQueryDomainCapabilityCompileFailBoundary::new(
        "graph_invariant_denial_requires_lower_runtime_target",
        "tests/ui/domain_capabilities/boundaries/runtime_targets/lower_runtime_bound/domain_capability_graph_invariant_denial_requires_lower_runtime_target.rs",
    ),
    WorthQueryDomainCapabilityCompileFailBoundary::new(
        "lower_runtime_support_artifact_requires_lower_runtime_target",
        "tests/ui/domain_capabilities/boundaries/runtime_targets/lower_runtime_bound/domain_capability_lower_runtime_support_artifact_requires_lower_runtime_target.rs",
    ),
];

pub fn worth_query_domain_capability_compile_fail_boundaries(
) -> &'static [WorthQueryDomainCapabilityCompileFailBoundary] {
    &COMPILE_FAIL_BOUNDARIES
}

pub fn worth_query_domain_capability_compile_fail_boundary_digest() -> String {
    compose_compile_fail_boundary_digest(worth_query_domain_capability_compile_fail_boundaries())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;
    use std::fs;
    use std::path::Path;

    fn manifest_paths_under(relative_dir: &str) -> BTreeSet<String> {
        collect_rs_paths(
            &Path::new(env!("CARGO_MANIFEST_DIR")).join(relative_dir),
            Path::new(env!("CARGO_MANIFEST_DIR")),
        )
    }

    fn collect_rs_paths(root: &Path, manifest_root: &Path) -> BTreeSet<String> {
        let mut results = BTreeSet::new();

        for entry in fs::read_dir(root).expect("certification boundary directory should exist") {
            let entry = entry.expect("directory entry should be readable");
            let path = entry.path();

            if path.is_dir() {
                results.extend(collect_rs_paths(&path, manifest_root));
                continue;
            }

            if path.extension().and_then(|ext| ext.to_str()) != Some("rs") {
                continue;
            }

            results.insert(
                path.strip_prefix(manifest_root)
                    .expect("boundary path should live under crate manifest dir")
                    .to_string_lossy()
                    .replace('\\', "/"),
            );
        }

        results
    }

    #[test]
    fn compile_fail_boundary_manifest_is_duplicate_free_and_nonempty() {
        let rows = worth_query_domain_capability_compile_fail_boundaries();
        let labels = rows.iter().map(|row| row.label()).collect::<Vec<_>>();
        let paths = rows.iter().map(|row| row.path()).collect::<Vec<_>>();

        assert_eq!(rows.len(), 41);
        assert_eq!(
            labels.len(),
            labels.iter().copied().collect::<BTreeSet<_>>().len()
        );
        assert_eq!(
            paths.len(),
            paths.iter().copied().collect::<BTreeSet<_>>().len()
        );
        assert!(rows
            .iter()
            .any(|row| row.path().contains("/certification/")));
        assert!(rows
            .iter()
            .any(|row| row.path().contains("/boundaries/progression/")));
        assert!(rows
            .iter()
            .any(|row| row.path().contains("/boundaries/materializers/")));
        assert!(rows
            .iter()
            .any(|row| row.path().contains("/boundaries/runtime_targets/")));
    }

    #[test]
    fn compile_fail_boundary_manifest_matches_checked_in_boundary_suite() {
        let expected = worth_query_domain_capability_compile_fail_boundaries()
            .iter()
            .map(|row| row.path())
            .collect::<BTreeSet<_>>();
        let actual = manifest_paths_under("tests/ui/domain_capabilities/dx_boundaries")
            .into_iter()
            .chain(manifest_paths_under(
                "tests/ui/domain_capabilities/certification",
            ))
            .chain(manifest_paths_under(
                "tests/ui/domain_capabilities/boundaries",
            ))
            .collect::<BTreeSet<_>>();

        assert_eq!(
            expected,
            actual.iter().map(String::as_str).collect::<BTreeSet<_>>()
        );
    }
}
