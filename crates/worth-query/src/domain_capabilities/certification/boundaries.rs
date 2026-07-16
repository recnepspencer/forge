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

const COMPILE_FAIL_BOUNDARIES: [WorthQueryDomainCapabilityCompileFailBoundary; 42] = [
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
        "admitted_package_type_private",
        "tests/ui/installed_domain/boundaries/admitted_package_type_is_private.rs",
    ),
    WorthQueryDomainCapabilityCompileFailBoundary::new(
        "application_facade_executable_authority_not_public",
        "tests/ui/installed_domain/boundaries/application_facade_executable_authority_is_not_public.rs",
    ),
    WorthQueryDomainCapabilityCompileFailBoundary::new(
        "certification_digests_cannot_be_caller_authored",
        "tests/ui/installed_domain/boundaries/certification_digests_cannot_be_caller_authored.rs",
    ),
    WorthQueryDomainCapabilityCompileFailBoundary::new(
        "installed_contribution_target_cannot_be_restamped",
        "tests/ui/installed_domain/boundaries/installed_contribution_target_cannot_be_restamped.rs",
    ),
    WorthQueryDomainCapabilityCompileFailBoundary::new(
        "installed_operation_declaration_cannot_be_restamped",
        "tests/ui/installed_domain/boundaries/installed_operation_declaration_cannot_be_restamped.rs",
    ),
    WorthQueryDomainCapabilityCompileFailBoundary::new(
        "low_level_materializer_not_public",
        "tests/ui/installed_domain/boundaries/low_level_materializer_is_not_public.rs",
    ),
    WorthQueryDomainCapabilityCompileFailBoundary::new(
        "contribution_evaluation_not_public",
        "tests/ui/installed_domain/boundaries/contribution_evaluation_is_not_public.rs",
    ),
    WorthQueryDomainCapabilityCompileFailBoundary::new(
        "contribution_admission_not_public",
        "tests/ui/installed_domain/boundaries/contribution_admission_is_not_public.rs",
    ),
    WorthQueryDomainCapabilityCompileFailBoundary::new(
        "contribution_preparation_not_public",
        "tests/ui/installed_domain/boundaries/contribution_preparation_is_not_public.rs",
    ),
    WorthQueryDomainCapabilityCompileFailBoundary::new(
        "manual_operation_registry_not_public",
        "tests/ui/installed_domain/boundaries/manual_operation_registry_is_not_public.rs",
    ),
    WorthQueryDomainCapabilityCompileFailBoundary::new(
        "package_admission_not_consumer_callable",
        "tests/ui/installed_domain/boundaries/package_admission_is_not_consumer_callable.rs",
    ),
    WorthQueryDomainCapabilityCompileFailBoundary::new(
        "raw_domain_constructor_not_public",
        "tests/ui/installed_domain/boundaries/raw_domain_constructor_is_not_public.rs",
    ),
    WorthQueryDomainCapabilityCompileFailBoundary::new(
        "operating_context_digest_authoring_forbidden",
        "tests/ui/installed_domain/boundaries/operating_context_digest_authoring_is_forbidden.rs",
    ),
    WorthQueryDomainCapabilityCompileFailBoundary::new(
        "raw_operation_owner_constructor_private",
        "tests/ui/installed_domain/boundaries/raw_operation_owner_constructor_is_private.rs",
    ),
    WorthQueryDomainCapabilityCompileFailBoundary::new(
        "package_identity_constructor_private",
        "tests/ui/installed_domain/boundaries/package_identity_constructor_is_private.rs",
    ),
    WorthQueryDomainCapabilityCompileFailBoundary::new(
        "package_execution_callback_forbidden",
        "tests/ui/installed_domain/boundaries/package_execution_callback_is_forbidden.rs",
    ),
    WorthQueryDomainCapabilityCompileFailBoundary::new(
        "installation_generation_constructor_private",
        "tests/ui/installed_domain/boundaries/installation_generation_constructor_is_private.rs",
    ),
    WorthQueryDomainCapabilityCompileFailBoundary::new(
        "installation_receipt_constructor_private",
        "tests/ui/installed_domain/boundaries/installation_receipt_constructor_is_private.rs",
    ),
    WorthQueryDomainCapabilityCompileFailBoundary::new(
        "installed_handle_fields_private",
        "tests/ui/installed_domain/boundaries/installed_handle_fields_are_private.rs",
    ),
    WorthQueryDomainCapabilityCompileFailBoundary::new(
        "inspection_cannot_be_promoted_to_rebind_authority",
        "tests/ui/installed_domain/boundaries/inspection_cannot_be_promoted_to_rebind_authority.rs",
    ),
    WorthQueryDomainCapabilityCompileFailBoundary::new(
        "closed_installed_live_handle_cannot_be_revived",
        "tests/ui/installed_domain/boundaries/closed_installed_live_handle_cannot_be_revived.rs",
    ),
    WorthQueryDomainCapabilityCompileFailBoundary::new(
        "domain_capability_vocabulary_not_in_runtime_facade",
        "tests/ui/installed_domain/boundaries/domain_capability_vocabulary_is_not_in_runtime_facade.rs",
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

        assert_eq!(rows.len(), 42);
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
            .any(|row| row.path().contains("/installed_domain/boundaries/")));
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
            .chain(manifest_paths_under("tests/ui/installed_domain/boundaries"))
            .collect::<BTreeSet<_>>();

        assert_eq!(
            expected,
            actual.iter().map(String::as_str).collect::<BTreeSet<_>>()
        );
    }
}
