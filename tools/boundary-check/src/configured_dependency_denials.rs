use std::path::Path;

use crate::cargo_graph::cargo_metadata;
use crate::config::DependencyDenialConfig;
use crate::diagnostics::{Diagnostic, DiagnosticCode};

pub(crate) fn validate_configured_dependency_denials(
    root: &Path,
    rules: &[DependencyDenialConfig],
) -> Result<Vec<Diagnostic>, String> {
    let mut diagnostics = Vec::new();
    for rule in rules {
        let manifest = root.join(&rule.workspace_manifest);
        let metadata = cargo_metadata(root, &manifest)?;
        for package in metadata.packages {
            diagnostics.extend(diagnostics_for_package(
                &package.name,
                &package.manifest_path,
                package
                    .dependencies
                    .iter()
                    .map(|dependency| dependency.name.as_str()),
                rule,
            ));
        }
    }
    Ok(diagnostics)
}

fn diagnostics_for_package<'a>(
    package_name: &str,
    manifest_path: &str,
    dependencies: impl IntoIterator<Item = &'a str>,
    rule: &DependencyDenialConfig,
) -> Vec<Diagnostic> {
    if !rule.sources.iter().any(|source| source == package_name) {
        return Vec::new();
    }
    dependencies
        .into_iter()
        .filter(|dependency| {
            rule.forbidden_targets
                .iter()
                .any(|target| target == dependency)
        })
        .map(|dependency| {
            Diagnostic::new(
                DiagnosticCode::Bc2001BandDependencyViolation,
                manifest_path,
                format!(
                    "{package_name} must not depend on {dependency}: {}",
                    rule.guidance
                ),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::diagnostics_for_package;
    use crate::config::DependencyDenialConfig;

    #[test]
    fn adding_a_forbidden_lower_to_operations_edge_emits_the_exact_boundary_diagnostic() {
        let rule = DependencyDenialConfig {
            workspace_manifest: "workspaces/worth-store/Cargo.toml".into(),
            sources: vec!["worth-store-physical-backend".into()],
            forbidden_targets: vec!["worth-store-operations".into()],
            guidance: "lower Store owners cannot depend on Operations".into(),
        };
        let diagnostics = diagnostics_for_package(
            "worth-store-physical-backend",
            "crates/worth-store-physical-backend/Cargo.toml",
            ["sha2", "worth-store-operations"],
            &rule,
        );
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(
            diagnostics[0].code().as_str(),
            "BC2001_BAND_DEPENDENCY_VIOLATION"
        );
        assert!(diagnostics[0]
            .message()
            .contains("worth-store-physical-backend must not depend on worth-store-operations"));
    }
}
