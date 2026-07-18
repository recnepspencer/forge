mod artifact_store;
mod bounded_process;
mod cargo_configuration;
mod cargo_environment;
mod declaration;
mod diagnostics;
mod environment_lock;
mod evidence;
#[cfg(test)]
mod tests;
mod toolchain;

use std::path::Path;

pub use declaration::{
    ExpectedCompilerDenial, UiFixtureDeclaration, UiFixtureIdentity, UiProofEnvironment,
    UiProofSuiteDeclaration,
};
pub use diagnostics::CheckedCompilerDiagnostic;
pub use evidence::{
    UiCargoConfigurationIdentity, UiCompilerResourcePosture, UiCompilerToolIdentity,
    UiCompilerToolchainIdentity, UiFixtureRunEvidence, UiProofRunEvidence, UiProofRunFailure,
};

/// An outer proof controller may bind this to an attempt-scoped directory so
/// compiler-boundary evidence is handed directly into the owning proof run.
/// The harness rejects roots outside the Store workspace's `.store-proof`
/// evidence tree.
pub const UI_EVIDENCE_ROOT_ENV: &str = "WORTH_STORE_UI_EVIDENCE_ROOT";

/// Executes one declared compiler-boundary suite in its canonical cache-sharing
/// Cargo environment and persists checked diagnostic evidence.
pub fn run_ui_proof_suite(
    workspace_root: &Path,
    declaration: &UiProofSuiteDeclaration,
) -> Result<UiProofRunEvidence, UiProofRunFailure> {
    cargo_environment::run(workspace_root, declaration)
}

/// Concise declaration path for the common case where one source directory and
/// one dependency environment own a set of semantic compile denials.
pub fn run_cargo_ui_fixture_suite(
    workspace_root: &Path,
    suite_identity: &str,
    dependency_manifest: String,
    feature_identity: &str,
    profile_identity: &str,
    source_root: &Path,
    fixtures: &[(&str, &[&str])],
) -> Result<UiProofRunEvidence, UiProofRunFailure> {
    let fixtures = fixtures
        .iter()
        .map(|(name, fragments)| {
            UiFixtureDeclaration::new(
                name.trim_end_matches(".rs"),
                source_root.join(name),
                ExpectedCompilerDenial::semantic_fragments(fragments.iter().copied())
                    .map_err(UiProofRunFailure::InvalidDeclaration)?,
            )
            .map_err(UiProofRunFailure::InvalidDeclaration)
        })
        .collect::<Result<Vec<_>, _>>()?;
    let environment =
        UiProofEnvironment::cargo(dependency_manifest, feature_identity, profile_identity)
            .map_err(UiProofRunFailure::InvalidDeclaration)?;
    let declaration = UiProofSuiteDeclaration::new(suite_identity, environment, fixtures)
        .map_err(UiProofRunFailure::InvalidDeclaration)?;
    run_ui_proof_suite(workspace_root, &declaration)
}

pub fn cargo_dependency_manifest(
    path_dependencies: &[(&str, &Path, &[&str])],
    registry_dependencies: &[(&str, &str)],
) -> String {
    let mut manifest = String::from("[dependencies]\n");
    for (name, path, features) in path_dependencies {
        let path = path.display().to_string().replace('\\', "/");
        if features.is_empty() {
            manifest.push_str(&format!("{name} = {{ path = \"{path}\" }}\n"));
        } else {
            let features = features
                .iter()
                .map(|feature| format!("\"{feature}\""))
                .collect::<Vec<_>>()
                .join(", ");
            manifest.push_str(&format!(
                "{name} = {{ path = \"{path}\", features = [{features}] }}\n"
            ));
        }
    }
    for (name, version) in registry_dependencies {
        manifest.push_str(&format!("{name} = \"{version}\"\n"));
    }
    manifest
}
