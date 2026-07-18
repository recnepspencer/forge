use std::fs;
use std::path::Path;
use std::process::Command;
use std::time::Duration;

use serde::Serialize;
use sha2::{Digest, Sha256};

use super::diagnostics::{checked_diagnostics, validate_denial};
use super::toolchain;
use super::{artifact_store, bounded_process, environment_lock, environment_manifest};
use super::{
    UiCompilerToolchainIdentity, UiFixtureIdentity, UiFixtureRunEvidence, UiProofRunEvidence,
    UiProofRunFailure, UiProofSuiteDeclaration, UI_EXECUTION_IDENTITY_ENV,
};

struct FixtureEnvironment<'a> {
    workspace_root: &'a Path,
    environment_root: &'a Path,
    target_root: &'a Path,
    manifest_path: &'a Path,
    environment_identity: &'a str,
    suite_identity: &'a str,
    profile_identity: &'a str,
    toolchain: &'a UiCompilerToolchainIdentity,
}

pub(super) fn run(
    workspace_root: &Path,
    declaration: &UiProofSuiteDeclaration,
) -> Result<UiProofRunEvidence, UiProofRunFailure> {
    let workspace_root = workspace_root.canonicalize().map_err(|error| {
        UiProofRunFailure::EnvironmentObservation(format!(
            "canonicalize {}: {error}",
            workspace_root.display()
        ))
    })?;
    let workspace_root = cargo_compatible_path(&workspace_root);
    let toolchain = toolchain::observe(&workspace_root)?;
    let environment_manifest = environment_manifest::UiEnvironmentManifestContract::load(
        &workspace_root,
        declaration.environment().profile_identity(),
    )?;
    let environment_root_identity = environment_basis_identity(
        &workspace_root,
        declaration,
        &toolchain,
        &environment_manifest,
    )?;
    let evidence_root = artifact_store::admitted_evidence_root(&workspace_root)?;
    let environment_root = workspace_root
        .join(".store-proof/ui/environments")
        .join(&environment_root_identity);
    fs::create_dir_all(environment_root.join("src/bin"))
        .map_err(|error| UiProofRunFailure::EnvironmentObservation(error.to_string()))?;
    let manifest_path = environment_root.join("Cargo.toml");
    let manifest = environment_manifest.render(
        &environment_root_identity,
        declaration.environment().dependency_manifest(),
    )?;
    let manifest_created =
        artifact_store::write_immutable_file(&manifest_path, manifest.as_bytes())?;
    let lock = environment_lock::seal(
        &workspace_root,
        &environment_root,
        &manifest_path,
        &manifest,
        &toolchain,
    )?;
    let environment_identity = digest_serialized(&(
        "worth-store-ui-environment-v5",
        &environment_root_identity,
        &lock.sha256,
    ))?;
    let target_root = workspace_root
        .join("target/store-ui")
        .join(&environment_identity[..24]);
    fs::create_dir_all(&target_root)
        .map_err(|error| UiProofRunFailure::EnvironmentObservation(error.to_string()))?;
    artifact_store::write_immutable_file(
        &target_root.join(".environment-identity"),
        environment_identity.as_bytes(),
    )?;

    let mut fixtures = Vec::new();
    let environment = FixtureEnvironment {
        workspace_root: &workspace_root,
        environment_root: &environment_root,
        target_root: &target_root,
        manifest_path: &manifest_path,
        environment_identity: &environment_identity,
        suite_identity: declaration.suite_identity(),
        profile_identity: declaration.environment().profile_identity(),
        toolchain: &toolchain,
    };
    for fixture in declaration.fixtures() {
        fixtures.push(run_fixture(&environment, fixture)?);
    }
    toolchain::validate_unchanged(&workspace_root, &toolchain)?;
    let mut evidence = UiProofRunEvidence {
        schema_version: 2,
        suite_identity: declaration.suite_identity().to_owned(),
        execution_identity: execution_identity(declaration)?,
        environment_identity,
        environment_root_identity,
        profile_identity: declaration.environment().profile_identity().to_owned(),
        toolchain,
        environment_manifest_path: normalized_path(&workspace_root, &manifest_path),
        environment_lock_path: normalized_path(&workspace_root, &lock.path),
        environment_lock_sha256: lock.sha256,
        shared_target_root: normalized_path(&workspace_root, &target_root),
        environment_manifest_created: manifest_created,
        environment_lock_created: lock.created,
        fixtures,
        evidence_identity: String::new(),
    };
    evidence.evidence_identity = digest_serialized(&evidence)?;
    evidence
        .validate_integrity()
        .map_err(UiProofRunFailure::EvidenceWrite)?;
    let suite_path = evidence_root
        .join("runs")
        .join(format!("{}.json", evidence.evidence_identity));
    artifact_store::persist_suite_evidence(&suite_path, &evidence)?;
    Ok(evidence)
}

fn execution_identity(declaration: &UiProofSuiteDeclaration) -> Result<String, UiProofRunFailure> {
    match std::env::var(UI_EXECUTION_IDENTITY_ENV) {
        Ok(identity)
            if identity.len() == 64 && identity.bytes().all(|byte| byte.is_ascii_hexdigit()) =>
        {
            Ok(identity)
        }
        Ok(_) => Err(UiProofRunFailure::EnvironmentObservation(format!(
            "{UI_EXECUTION_IDENTITY_ENV} is not a SHA-256 identity"
        ))),
        Err(std::env::VarError::NotPresent) => digest_serialized(&(
            "worth-store-standalone-ui-execution-v1",
            declaration.suite_identity(),
        )),
        Err(error) => Err(UiProofRunFailure::EnvironmentObservation(format!(
            "could not read {UI_EXECUTION_IDENTITY_ENV}: {error}"
        ))),
    }
}

fn run_fixture(
    environment: &FixtureEnvironment<'_>,
    fixture: &super::UiFixtureDeclaration,
) -> Result<UiFixtureRunEvidence, UiProofRunFailure> {
    let source = fs::read(fixture.source_path()).map_err(|error| {
        UiProofRunFailure::FixtureRead(format!("{}: {error}", fixture.source_path().display()))
    })?;
    let source_digest = digest_bytes(&source);
    let expected_denial_identity = digest_serialized(fixture.expected_denial())?;
    let identity = UiFixtureIdentity {
        suite_identity: environment.suite_identity.to_owned(),
        case_identity: fixture.case_identity().to_owned(),
        source_path: normalized_path(environment.workspace_root, fixture.source_path()),
        source_digest: source_digest.clone(),
        environment_identity: environment.environment_identity.to_owned(),
        expected_denial_identity,
    };
    let bin_name = fixture_bin_name(
        environment.suite_identity,
        fixture.case_identity(),
        &source_digest,
    );
    let fixture_path = environment
        .environment_root
        .join("src/bin")
        .join(format!("{bin_name}.rs"));
    artifact_store::write_immutable_file(&fixture_path, &source)?;
    let before = artifact_count(environment.target_root);
    let mut command = Command::new(&environment.toolchain.cargo.executable_path);
    command
        .args([
            "check",
            "--offline",
            "--locked",
            "--message-format=json",
            "--profile",
            environment.profile_identity,
            "--bin",
        ])
        .arg(&bin_name)
        .arg("--manifest-path")
        .arg(environment.manifest_path)
        .env("CARGO_TARGET_DIR", environment.target_root)
        .env("RUSTC", &environment.toolchain.rustc.executable_path)
        .env_remove("RUSTC_WRAPPER")
        .env_remove("RUSTC_WORKSPACE_WRAPPER")
        .env_remove("RUSTC_BOOTSTRAP")
        .env_remove("RUSTFLAGS")
        .env_remove("CARGO_ENCODED_RUSTFLAGS")
        .current_dir(environment.workspace_root);
    let output = bounded_process::run(
        &mut command,
        Duration::from_millis(environment.toolchain.compile_timeout_millis),
        environment.toolchain.output_cap_bytes_per_stream,
    )
    .map_err(UiProofRunFailure::CompilerLaunch)?;
    if output.timed_out {
        return Err(UiProofRunFailure::CompilerTimedOut(
            fixture.case_identity().to_owned(),
        ));
    }
    if output.status.success() {
        return Err(UiProofRunFailure::UnexpectedCompilerSuccess(
            fixture.case_identity().to_owned(),
        ));
    }
    let root = environment.workspace_root.to_string_lossy();
    let diagnostics = checked_diagnostics(&output.stdout, &root);
    let dependency_artifacts = dependency_artifacts(&output.stdout, &bin_name);
    let stderr = String::from_utf8_lossy(&output.stderr)
        .replace(root.as_ref(), "<workspace>")
        .replace('\\', "/");
    validate_denial(fixture.expected_denial(), &diagnostics, &stderr).map_err(|reason| {
        UiProofRunFailure::WrongCompilerDenial {
            fixture: fixture.case_identity().to_owned(),
            reason,
            diagnostics: diagnostics.clone(),
        }
    })?;
    let mut evidence = UiFixtureRunEvidence {
        fixture: identity,
        cargo_process_id: output.process_id,
        cargo_exit_code: output.status.code(),
        cargo_stdout_sha256: digest_bytes(&output.stdout),
        cargo_stderr_sha256: digest_bytes(&output.stderr),
        dependency_artifacts_compiled: dependency_artifacts.compiled,
        dependency_artifacts_reused: dependency_artifacts.reused,
        target_artifact_count_before: before,
        target_artifact_count_after: artifact_count(environment.target_root),
        diagnostics,
        semantic_denial_matched: true,
        evidence_path: String::new(),
    };
    let attempt_identity = digest_serialized(&evidence)?;
    let evidence_path = artifact_store::admitted_evidence_root(environment.workspace_root)?
        .join("checked-diagnostics")
        .join(environment.environment_identity)
        .join(format!("{attempt_identity}.json"));
    evidence.evidence_path = normalized_path(environment.workspace_root, &evidence_path);
    artifact_store::persist_fixture_evidence(&evidence_path, &evidence)?;
    // The evidence file path is a projection, never compiler authority. The
    // immutable round trip still has to preserve the complete checked row.
    let checked: UiFixtureRunEvidence = serde_json::from_slice(
        &fs::read(&evidence_path)
            .map_err(|error| UiProofRunFailure::EvidenceWrite(error.to_string()))?,
    )
    .map_err(|error| UiProofRunFailure::EvidenceWrite(error.to_string()))?;
    if checked != evidence {
        return Err(UiProofRunFailure::EvidenceWrite(format!(
            "checked diagnostic round trip drifted at {}",
            evidence_path.display()
        )));
    }
    Ok(evidence)
}

fn environment_basis_identity(
    workspace_root: &Path,
    declaration: &UiProofSuiteDeclaration,
    toolchain: &UiCompilerToolchainIdentity,
    environment_manifest: &environment_manifest::UiEnvironmentManifestContract,
) -> Result<String, UiProofRunFailure> {
    let lock = digest_file(&workspace_root.join("Cargo.lock"))?;
    digest_serialized(&(
        declaration.environment().dependency_manifest(),
        declaration.environment().feature_identity(),
        declaration.environment().profile_identity(),
        environment_manifest,
        toolchain,
        lock,
    ))
}

fn fixture_bin_name(suite: &str, case: &str, source_digest: &str) -> String {
    let readable = format!("{suite}_{case}")
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() {
                character.to_ascii_lowercase()
            } else {
                '_'
            }
        })
        .collect::<String>();
    let identity = digest_bytes(format!("{readable}\0{source_digest}").as_bytes());
    format!("{}_{}", readable.trim_matches('_'), &identity[..12])
}

fn artifact_count(root: &Path) -> usize {
    let mut pending = vec![root.to_path_buf()];
    let mut count = 0;
    while let Some(path) = pending.pop() {
        let Ok(entries) = fs::read_dir(path) else {
            continue;
        };
        for entry in entries.flatten() {
            if entry.file_type().is_ok_and(|kind| kind.is_dir()) {
                pending.push(entry.path());
            } else {
                count += 1;
            }
        }
    }
    count
}

fn dependency_artifacts(stdout: &[u8], fixture_bin: &str) -> DependencyArtifactObservation {
    let mut observation = DependencyArtifactObservation::default();
    for line in stdout.split(|byte| *byte == b'\n') {
        let Ok(value) = serde_json::from_slice::<serde_json::Value>(line) else {
            continue;
        };
        if value.get("reason").and_then(serde_json::Value::as_str) != Some("compiler-artifact")
            || value
                .pointer("/target/name")
                .and_then(serde_json::Value::as_str)
                == Some(fixture_bin)
        {
            continue;
        }
        if value
            .get("fresh")
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
        {
            observation.reused += 1;
        } else {
            observation.compiled += 1;
        }
    }
    observation
}

#[derive(Default)]
struct DependencyArtifactObservation {
    compiled: usize,
    reused: usize,
}

fn normalized_path(workspace_root: &Path, path: &Path) -> String {
    path.strip_prefix(workspace_root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

#[cfg(windows)]
fn cargo_compatible_path(path: &Path) -> std::path::PathBuf {
    let value = path.to_string_lossy();
    if let Some(unc) = value.strip_prefix(r"\\?\UNC\") {
        return std::path::PathBuf::from(format!(r"\\{unc}"));
    }
    value
        .strip_prefix(r"\\?\")
        .map_or_else(|| path.to_path_buf(), std::path::PathBuf::from)
}

#[cfg(not(windows))]
fn cargo_compatible_path(path: &Path) -> std::path::PathBuf {
    path.to_path_buf()
}

fn digest_serialized(value: &impl Serialize) -> Result<String, UiProofRunFailure> {
    serde_json::to_vec(value)
        .map(|bytes| digest_bytes(&bytes))
        .map_err(|error| UiProofRunFailure::InvalidDeclaration(error.to_string()))
}

fn digest_bytes(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn digest_file(path: &Path) -> Result<String, UiProofRunFailure> {
    let mut file = fs::File::open(path)
        .map_err(|error| UiProofRunFailure::EnvironmentObservation(error.to_string()))?;
    let mut digest = Sha256::new();
    let mut buffer = [0_u8; 64 * 1024];
    loop {
        let read = std::io::Read::read(&mut file, &mut buffer)
            .map_err(|error| UiProofRunFailure::EnvironmentObservation(error.to_string()))?;
        if read == 0 {
            break;
        }
        digest.update(&buffer[..read]);
    }
    Ok(format!("{:x}", digest.finalize()))
}
