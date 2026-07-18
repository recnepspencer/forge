use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;

use serde::Serialize;
use sha2::{Digest, Sha256};

use super::bounded_process;
use super::diagnostics::{checked_diagnostics, validate_denial};
use super::toolchain;
use super::{
    UiCompilerToolchainIdentity, UiFixtureIdentity, UiFixtureRunEvidence, UiProofRunEvidence,
    UiProofRunFailure, UiProofSuiteDeclaration, UI_EVIDENCE_ROOT_ENV,
};

struct FixtureEnvironment<'a> {
    workspace_root: &'a Path,
    environment_root: &'a Path,
    target_root: &'a Path,
    manifest_path: &'a Path,
    environment_identity: &'a str,
    suite_identity: &'a str,
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
    let toolchain = toolchain::observe(&workspace_root)?;
    let environment_identity = environment_identity(&workspace_root, declaration, &toolchain)?;
    let evidence_root = admitted_evidence_root(&workspace_root)?;
    let environment_root = workspace_root
        .join(".store-proof/ui/environments")
        .join(&environment_identity);
    let target_root = workspace_root
        .join(".store-proof/cache/ui")
        .join(&environment_identity);
    fs::create_dir_all(environment_root.join("src/bin"))
        .and_then(|()| fs::create_dir_all(&target_root))
        .map_err(|error| UiProofRunFailure::EnvironmentObservation(error.to_string()))?;
    let manifest_path = environment_root.join("Cargo.toml");
    let manifest = environment_manifest(&environment_identity, declaration);
    let manifest_created = write_immutable_file(&manifest_path, manifest.as_bytes())?;

    let mut fixtures = Vec::new();
    let environment = FixtureEnvironment {
        workspace_root: &workspace_root,
        environment_root: &environment_root,
        target_root: &target_root,
        manifest_path: &manifest_path,
        environment_identity: &environment_identity,
        suite_identity: declaration.suite_identity(),
        toolchain: &toolchain,
    };
    for fixture in declaration.fixtures() {
        fixtures.push(run_fixture(&environment, fixture)?);
    }
    let mut evidence = UiProofRunEvidence {
        schema_version: 1,
        suite_identity: declaration.suite_identity().to_owned(),
        environment_identity,
        toolchain,
        environment_manifest_path: normalized_path(&workspace_root, &manifest_path),
        shared_target_root: normalized_path(&workspace_root, &target_root),
        environment_manifest_created: manifest_created,
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
    persist_suite_evidence(&suite_path, &evidence)?;
    Ok(evidence)
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
    write_immutable_file(&fixture_path, &source)?;
    let before = artifact_count(environment.target_root);
    let mut command = Command::new(&environment.toolchain.cargo.executable_path);
    command
        .args(["check", "--offline", "--message-format=json", "--bin"])
        .arg(&bin_name)
        .arg("--manifest-path")
        .arg(environment.manifest_path)
        .env("CARGO_TARGET_DIR", environment.target_root)
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
    let evidence_path = admitted_evidence_root(environment.workspace_root)?
        .join("checked-diagnostics")
        .join(environment.environment_identity)
        .join(format!("{attempt_identity}.json"));
    evidence.evidence_path = normalized_path(environment.workspace_root, &evidence_path);
    persist_evidence(&evidence_path, &evidence)?;
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

fn admitted_evidence_root(workspace_root: &Path) -> Result<PathBuf, UiProofRunFailure> {
    let default = workspace_root.join(".store-proof/evidence/ui");
    let Some(declared) = std::env::var_os(UI_EVIDENCE_ROOT_ENV).map(PathBuf::from) else {
        return Ok(default);
    };
    let declared = if declared.is_absolute() {
        declared
    } else {
        workspace_root.join(declared)
    };
    let admitted = workspace_root.join(".store-proof/evidence");
    if !declared.starts_with(&admitted) {
        return Err(UiProofRunFailure::EnvironmentObservation(format!(
            "{UI_EVIDENCE_ROOT_ENV} must remain under {}",
            admitted.display()
        )));
    }
    Ok(declared)
}

fn environment_identity(
    workspace_root: &Path,
    declaration: &UiProofSuiteDeclaration,
    toolchain: &UiCompilerToolchainIdentity,
) -> Result<String, UiProofRunFailure> {
    let lock = fs::read(workspace_root.join("Cargo.lock"))
        .map_err(|error| UiProofRunFailure::EnvironmentObservation(error.to_string()))?;
    digest_serialized(&(
        declaration.environment().dependency_manifest(),
        declaration.environment().feature_identity(),
        declaration.environment().profile_identity(),
        toolchain,
        digest_bytes(&lock),
    ))
}

fn environment_manifest(identity: &str, declaration: &UiProofSuiteDeclaration) -> String {
    format!(
        "[package]\nname = \"store_ui_{}\"\nversion = \"0.0.0\"\nedition = \"2021\"\n\n[workspace]\n\n{}",
        &identity[..16],
        declaration.environment().dependency_manifest()
    )
}

fn write_immutable_file(path: &Path, bytes: &[u8]) -> Result<bool, UiProofRunFailure> {
    let parent = path.parent().ok_or_else(|| {
        UiProofRunFailure::EnvironmentObservation(format!("{} has no parent", path.display()))
    })?;
    fs::create_dir_all(parent)
        .map_err(|error| UiProofRunFailure::EnvironmentObservation(error.to_string()))?;
    match fs::OpenOptions::new().write(true).create_new(true).open(path) {
        Ok(mut file) => {
            file.write_all(bytes)
                .and_then(|()| file.sync_all())
                .map_err(|error| UiProofRunFailure::EnvironmentObservation(error.to_string()))?;
            Ok(true)
        }
        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
            let existing = fs::read(path)
                .map_err(|error| UiProofRunFailure::EnvironmentObservation(error.to_string()))?;
            if existing == bytes {
                Ok(false)
            } else {
                Err(UiProofRunFailure::EnvironmentObservation(format!(
                    "immutable UI environment collision at {}",
                    path.display()
                )))
            }
        }
        Err(error) => Err(UiProofRunFailure::EnvironmentObservation(error.to_string())),
    }
}

fn persist_evidence(path: &Path, evidence: &UiFixtureRunEvidence) -> Result<(), UiProofRunFailure> {
    let parent = path.parent().ok_or_else(|| {
        UiProofRunFailure::EvidenceWrite(format!("{} has no parent", path.display()))
    })?;
    fs::create_dir_all(parent)
        .map_err(|error| UiProofRunFailure::EvidenceWrite(error.to_string()))?;
    let mut encoded = serde_json::to_vec_pretty(evidence)
        .map_err(|error| UiProofRunFailure::EvidenceWrite(error.to_string()))?;
    encoded.push(b'\n');
    persist_encoded(path, &encoded, "checked diagnostic")
}

fn persist_suite_evidence(
    path: &Path,
    evidence: &UiProofRunEvidence,
) -> Result<(), UiProofRunFailure> {
    let parent = path.parent().ok_or_else(|| {
        UiProofRunFailure::EvidenceWrite(format!("{} has no parent", path.display()))
    })?;
    fs::create_dir_all(parent)
        .map_err(|error| UiProofRunFailure::EvidenceWrite(error.to_string()))?;
    let mut encoded = serde_json::to_vec_pretty(evidence)
        .map_err(|error| UiProofRunFailure::EvidenceWrite(error.to_string()))?;
    encoded.push(b'\n');
    match fs::OpenOptions::new().write(true).create_new(true).open(path) {
        Ok(mut file) => file
            .write_all(&encoded)
            .and_then(|()| file.sync_all())
            .map_err(|error| UiProofRunFailure::EvidenceWrite(error.to_string())),
        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
            let existing = fs::read(path)
                .map_err(|error| UiProofRunFailure::EvidenceWrite(error.to_string()))?;
            if existing == encoded {
                Ok(())
            } else {
                Err(UiProofRunFailure::EvidenceWrite(format!(
                    "UI run evidence identity collision at {}",
                    path.display()
                )))
            }
        }
        Err(error) => Err(UiProofRunFailure::EvidenceWrite(error.to_string())),
    }
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
    format!(
        "{}_{}",
        readable.trim_matches('_'),
        &identity[..12]
    )
}

fn persist_encoded(
    path: &Path,
    encoded: &[u8],
    evidence_kind: &str,
) -> Result<(), UiProofRunFailure> {
    match fs::OpenOptions::new().write(true).create_new(true).open(path) {
        Ok(mut file) => file
            .write_all(encoded)
            .and_then(|()| file.sync_all())
            .map_err(|error| UiProofRunFailure::EvidenceWrite(error.to_string())),
        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
            let existing = fs::read(path)
                .map_err(|error| UiProofRunFailure::EvidenceWrite(error.to_string()))?;
            if existing == encoded {
                Ok(())
            } else {
                Err(UiProofRunFailure::EvidenceWrite(format!(
                    "{evidence_kind} identity collision at {}",
                    path.display()
                )))
            }
        }
        Err(error) => Err(UiProofRunFailure::EvidenceWrite(error.to_string())),
    }
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
        if value.get("reason").and_then(serde_json::Value::as_str)
            != Some("compiler-artifact")
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

fn digest_serialized(value: &impl Serialize) -> Result<String, UiProofRunFailure> {
    serde_json::to_vec(value)
        .map(|bytes| digest_bytes(&bytes))
        .map_err(|error| UiProofRunFailure::InvalidDeclaration(error.to_string()))
}

fn digest_bytes(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
