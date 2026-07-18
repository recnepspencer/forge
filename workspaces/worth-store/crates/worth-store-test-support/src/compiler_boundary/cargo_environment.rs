use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use serde::Serialize;
use sha2::{Digest, Sha256};

use super::diagnostics::{checked_diagnostics, validate_denial};
use super::{
    UiFixtureIdentity, UiFixtureRunEvidence, UiProofRunEvidence, UiProofRunFailure,
    UiProofSuiteDeclaration, UI_EVIDENCE_ROOT_ENV,
};

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
    let environment_identity = environment_identity(&workspace_root, declaration)?;
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
    let manifest_created = write_canonical_file(&manifest_path, manifest.as_bytes())?;

    let mut fixtures = Vec::new();
    for fixture in declaration.fixtures() {
        fixtures.push(run_fixture(
            &workspace_root,
            &environment_root,
            &target_root,
            &manifest_path,
            &environment_identity,
            declaration.suite_identity(),
            fixture,
        )?);
    }
    let mut evidence = UiProofRunEvidence {
        schema_version: 1,
        suite_identity: declaration.suite_identity().to_owned(),
        environment_identity,
        environment_manifest_path: normalized_path(&workspace_root, &manifest_path),
        shared_target_root: normalized_path(&workspace_root, &target_root),
        environment_manifest_created: manifest_created,
        fixtures,
        evidence_identity: String::new(),
    };
    evidence.evidence_identity = digest_serialized(&evidence)?;
    let suite_path = evidence_root
        .join("runs")
        .join(format!("{}.json", evidence.evidence_identity));
    persist_suite_evidence(&suite_path, &evidence)?;
    Ok(evidence)
}

#[allow(clippy::too_many_arguments)]
fn run_fixture(
    workspace_root: &Path,
    environment_root: &Path,
    target_root: &Path,
    manifest_path: &Path,
    environment_identity: &str,
    suite_identity: &str,
    fixture: &super::UiFixtureDeclaration,
) -> Result<UiFixtureRunEvidence, UiProofRunFailure> {
    let source = fs::read(fixture.source_path()).map_err(|error| {
        UiProofRunFailure::FixtureRead(format!("{}: {error}", fixture.source_path().display()))
    })?;
    let source_digest = digest_bytes(&source);
    let expected_denial_identity = digest_serialized(fixture.expected_denial())?;
    let identity = UiFixtureIdentity {
        suite_identity: suite_identity.to_owned(),
        case_identity: fixture.case_identity().to_owned(),
        source_path: normalized_path(workspace_root, fixture.source_path()),
        source_digest,
        environment_identity: environment_identity.to_owned(),
        expected_denial_identity,
    };
    let bin_name = fixture_bin_name(suite_identity, fixture.case_identity());
    let fixture_path = environment_root
        .join("src/bin")
        .join(format!("{bin_name}.rs"));
    write_canonical_file(&fixture_path, &source)?;
    let before = artifact_count(target_root);
    let mut child = Command::new(std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into()))
        .args(["check", "--offline", "--message-format=json", "--bin"])
        .arg(&bin_name)
        .arg("--manifest-path")
        .arg(manifest_path)
        .env("CARGO_TARGET_DIR", target_root)
        .current_dir(workspace_root)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|error| UiProofRunFailure::CompilerLaunch(error.to_string()))?;
    let cargo_process_id = child.id();
    let output = child
        .wait_with_output()
        .map_err(|error| UiProofRunFailure::CompilerLaunch(error.to_string()))?;
    if output.status.success() {
        return Err(UiProofRunFailure::UnexpectedCompilerSuccess(
            fixture.case_identity().to_owned(),
        ));
    }
    let root = workspace_root.to_string_lossy();
    let diagnostics = checked_diagnostics(&output.stdout, &root);
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
        cargo_process_id,
        target_artifact_count_before: before,
        target_artifact_count_after: artifact_count(target_root),
        diagnostics,
        semantic_denial_matched: true,
        evidence_path: String::new(),
    };
    let attempt_identity = digest_serialized(&evidence)?;
    let evidence_path = admitted_evidence_root(workspace_root)?
        .join("checked-diagnostics")
        .join(environment_identity)
        .join(format!("{attempt_identity}.json"));
    evidence.evidence_path = normalized_path(workspace_root, &evidence_path);
    persist_evidence(&evidence_path, &evidence)?;
    // The evidence file path is part of the projection, never part of compiler
    // authority. Re-read only to catch accidental serialization drift.
    let checked: UiFixtureRunEvidence = serde_json::from_slice(
        &fs::read(&evidence_path)
            .map_err(|error| UiProofRunFailure::EvidenceWrite(error.to_string()))?,
    )
    .map_err(|error| UiProofRunFailure::EvidenceWrite(error.to_string()))?;
    evidence.semantic_denial_matched = checked.semantic_denial_matched;
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
) -> Result<String, UiProofRunFailure> {
    let rustc = command_identity(workspace_root, "rustc", &["-Vv"])?;
    let lock = fs::read(workspace_root.join("Cargo.lock"))
        .map_err(|error| UiProofRunFailure::EnvironmentObservation(error.to_string()))?;
    digest_serialized(&(
        declaration.environment().dependency_manifest(),
        declaration.environment().feature_identity(),
        declaration.environment().profile_identity(),
        rustc,
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

fn command_identity(
    current_dir: &Path,
    program: &str,
    arguments: &[&str],
) -> Result<String, UiProofRunFailure> {
    let output = Command::new(program)
        .args(arguments)
        .current_dir(current_dir)
        .output()
        .map_err(|error| UiProofRunFailure::EnvironmentObservation(error.to_string()))?;
    if !output.status.success() {
        return Err(UiProofRunFailure::EnvironmentObservation(format!(
            "{program} identity failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )));
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_owned())
}

fn write_canonical_file(path: &Path, bytes: &[u8]) -> Result<bool, UiProofRunFailure> {
    if path.exists() {
        let existing = fs::read(path)
            .map_err(|error| UiProofRunFailure::EnvironmentObservation(error.to_string()))?;
        if existing == bytes {
            return Ok(false);
        }
    }
    let parent = path.parent().ok_or_else(|| {
        UiProofRunFailure::EnvironmentObservation(format!("{} has no parent", path.display()))
    })?;
    fs::create_dir_all(parent)
        .map_err(|error| UiProofRunFailure::EnvironmentObservation(error.to_string()))?;
    let temporary = parent.join(format!(
        ".{}.{}.tmp",
        path.file_name()
            .and_then(|value| value.to_str())
            .unwrap_or("ui"),
        std::process::id()
    ));
    fs::write(&temporary, bytes)
        .map_err(|error| UiProofRunFailure::EnvironmentObservation(error.to_string()))?;
    if path.exists() {
        fs::remove_file(path)
            .map_err(|error| UiProofRunFailure::EnvironmentObservation(error.to_string()))?;
    }
    fs::rename(&temporary, path)
        .map_err(|error| UiProofRunFailure::EnvironmentObservation(error.to_string()))?;
    Ok(true)
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
    if path.exists() {
        let existing =
            fs::read(path).map_err(|error| UiProofRunFailure::EvidenceWrite(error.to_string()))?;
        return if existing == encoded {
            Ok(())
        } else {
            Err(UiProofRunFailure::EvidenceWrite(format!(
                "checked diagnostic identity collision at {}",
                path.display()
            )))
        };
    }
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| UiProofRunFailure::EvidenceWrite(error.to_string()))?;
    file.write_all(&encoded)
        .map_err(|error| UiProofRunFailure::EvidenceWrite(error.to_string()))
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

fn fixture_bin_name(suite: &str, case: &str) -> String {
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
    format!(
        "{}_{}",
        readable.trim_matches('_'),
        &digest_bytes(readable.as_bytes())[..12]
    )
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
