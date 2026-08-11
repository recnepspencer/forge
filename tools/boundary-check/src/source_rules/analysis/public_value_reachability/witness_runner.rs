//! Compile and, on the host, execute one downstream witness consumer per target world.

use super::super::bounded_process;
use super::super::crate_modules::GovernedCrate;
use super::super::production_world::ProductionWorld;
use crate::config::{PublicValueWitness, PublicValueWitnessPosture};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

static SESSION_ID: AtomicU64 = AtomicU64::new(0);
pub(super) fn run(
    root: &Path,
    governed: &GovernedCrate,
    witness_source: &Path,
    witnesses: &[&PublicValueWitness],
    world: &ProductionWorld,
    host_timeout_ms: u64,
    compilation_limits: bounded_process::Limits,
) -> Result<(), String> {
    let session = WitnessSession::prepare(root, governed, witness_source, witnesses, world)?;
    let result = session.execute(
        world,
        Duration::from_millis(host_timeout_ms),
        compilation_limits,
    );
    session.cleanup();
    result
}

struct WitnessSession {
    cargo_working_directory: PathBuf,
    root: PathBuf,
    manifest: PathBuf,
    completion: CompletionExpectation,
    target_dir: PathBuf,
    ephemeral_target: bool,
}

struct CompletionExpectation {
    nonce: String,
    witness_count: usize,
}

impl WitnessSession {
    fn prepare(
        root: &Path,
        governed: &GovernedCrate,
        witness_source: &Path,
        witnesses: &[&PublicValueWitness],
        world: &ProductionWorld,
    ) -> Result<Self, String> {
        let id = SESSION_ID.fetch_add(1, Ordering::Relaxed);
        let directory = std::env::temp_dir().join(format!(
            "boundary-public-value-{}-{id}-{}",
            std::process::id(),
            sanitize(&world.target)
        ));
        let source_dir = directory.join("src");
        fs::create_dir_all(&source_dir).map_err(|error| {
            format!(
                "create public-value witness consumer {}: {error}",
                directory.display()
            )
        })?;
        let manifest = directory.join("Cargo.toml");
        fs::write(
            &manifest,
            manifest_source(&governed.crate_root, &governed.package, world),
        )
        .map_err(|error| {
            format!(
                "write public-value witness manifest {}: {error}",
                manifest.display()
            )
        })?;
        let completion = CompletionExpectation::mint(witnesses.len())?;
        let main = source_dir.join("main.rs");
        fs::write(&main, consumer_source(witness_source, witnesses)).map_err(|error| {
            format!(
                "write public-value witness consumer {}: {error}",
                main.display()
            )
        })?;
        let ordinary_target = root.join("tools/boundary-check/target/public-value-witnesses");
        let ephemeral_target = root.as_os_str().to_string_lossy().len() > 90;
        let target_dir = if ephemeral_target {
            std::env::temp_dir().join(format!("bc-pvw-{}-{id}", std::process::id()))
        } else {
            ordinary_target
        };
        Ok(Self {
            cargo_working_directory: governed.crate_root.clone(),
            completion,
            root: directory,
            manifest,
            target_dir,
            ephemeral_target,
        })
    }

    fn execute(
        &self,
        world: &ProductionWorld,
        timeout: Duration,
        compilation_limits: bounded_process::Limits,
    ) -> Result<(), String> {
        let mut command = Command::new(cargo());
        if world.is_host {
            command.arg("build");
        } else {
            command.arg("check").args(["--target", &world.target]);
        }
        command
            .current_dir(&self.cargo_working_directory)
            .args(["--profile", &world.profile])
            .arg("--quiet")
            .arg("--manifest-path")
            .arg(&self.manifest)
            .env("CARGO_TARGET_DIR", &self.target_dir);
        let build = bounded_process::run(
            &mut command,
            None,
            compilation_limits,
            "public-value witness compilation",
        )?;
        if !build.status.success() {
            return Err(failed_output(world, "compilation", &build));
        }
        if !world.is_host {
            return Ok(());
        }
        let mut command = Command::new(self.host_executable(world));
        let runtime = bounded_process::run(
            &mut command,
            Some(self.completion.nonce.as_bytes()),
            bounded_process::Limits::new(timeout, compilation_limits.max_output_bytes()),
            "public-value witness runtime",
        )?;
        if !runtime.status.success() {
            return Err(failed_output(world, "runtime", &runtime));
        }
        self.completion.verify(&runtime.stdout)
    }

    fn cleanup(self) {
        let _ = fs::remove_dir_all(self.root);
        if self.ephemeral_target {
            let _ = fs::remove_dir_all(self.target_dir);
        }
    }

    fn host_executable(&self, world: &ProductionWorld) -> PathBuf {
        self.target_dir
            .join(world.artifact_directory())
            .join(format!(
                "boundary-public-value-witness{}",
                std::env::consts::EXE_SUFFIX
            ))
    }
}

fn failed_output(world: &ProductionWorld, phase: &str, output: &Output) -> String {
    format!(
        "public-value witness {phase} failed for target `{}`:\n{}{}",
        world.target,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

impl CompletionExpectation {
    fn mint(witness_count: usize) -> Result<Self, String> {
        let mut bytes = [0_u8; 32];
        getrandom::fill(&mut bytes)
            .map_err(|error| format!("mint public-value witness completion nonce: {error}"))?;
        let nonce = bytes.iter().map(|byte| format!("{byte:02x}")).collect();
        Ok(Self {
            nonce,
            witness_count,
        })
    }

    fn verify(&self, stdout: &[u8]) -> Result<(), String> {
        let expected = format!("{}\t{}", self.nonce, self.witness_count);
        let actual = String::from_utf8_lossy(stdout);
        if actual.trim_end_matches(['\r', '\n']) == expected {
            return Ok(());
        }
        Err(format!(
            "public-value witness consumer did not emit its exact completion receipt for {} rows",
            self.witness_count
        ))
    }
}

fn manifest_source(crate_root: &Path, package: &str, world: &ProductionWorld) -> String {
    let crate_root = crate_root.to_string_lossy();
    let crate_root = crate_root.strip_prefix(r"\\?\").unwrap_or(&crate_root);
    let crate_root = crate_root.replace('\\', "/");
    let features = world
        .features
        .iter()
        .map(|feature| format!("\"{feature}\""))
        .collect::<Vec<_>>()
        .join(", ");
    format!(
        r#"[package]
name = "boundary-public-value-witness"
version = "0.0.0"
edition = "2021"

[dependencies]
worth-proof = {{ package = "{package}", path = "{crate_root}", default-features = {}, features = [{}] }}

[workspace]
{}
"#,
        world.default_features, features, world.workspace_profile_source
    )
}

fn consumer_source(witness_source: &Path, witnesses: &[&PublicValueWitness]) -> String {
    let source = witness_source.to_string_lossy();
    let source = source.strip_prefix(r"\\?\").unwrap_or(&source);
    let source = source.replace('\\', "/");
    let mut consumer = format!("#![forbid(unsafe_code)]\n\n#[path = r#\"{source}\"#]\nmod witnesses;\n\nfn main() {{\n    let mut completion_nonce = String::new();\n    std::io::Read::read_to_string(\n        &mut std::io::stdin().lock(),\n        &mut completion_nonce,\n    )\n    .expect(\"read parent completion nonce\");\n    let mut completed = 0_usize;\n");
    for (index, witness) in witnesses.iter().enumerate() {
        let function = witness
            .function
            .split("::")
            .map(sanitize_identifier)
            .collect::<Vec<_>>()
            .join("::");
        match witness.posture {
            PublicValueWitnessPosture::Value => {
                consumer.push_str(&format!(
                    "    let _value_{index}: {} = witnesses::{function}();\n    completed += 1;\n",
                    witness.public_type_path
                ));
            }
            PublicValueWitnessPosture::Callback => {
                consumer.push_str(&format!(
                    "    let delivered_{index} = std::cell::Cell::new(false);\n    witnesses::{function}(|_: {}| delivered_{index}.set(true));\n    assert!(delivered_{index}.get(), \"callback witness `{}` did not deliver its value\");\n    completed += 1;\n",
                    witness.public_type_path,
                    witness.definition_path
                ));
            }
        }
    }
    consumer.push_str("    println!(\"{completion_nonce}\\t{completed}\");\n}\n");
    consumer
}

fn sanitize_identifier(value: &str) -> &str {
    if !value.is_empty()
        && value
            .bytes()
            .all(|byte| byte == b'_' || byte.is_ascii_alphanumeric())
        && !value.as_bytes()[0].is_ascii_digit()
    {
        value
    } else {
        "INVALID_WITNESS_FUNCTION"
    }
}

fn sanitize(value: &str) -> String {
    value
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() {
                character
            } else {
                '-'
            }
        })
        .collect()
}

fn cargo() -> std::ffi::OsString {
    std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into())
}

#[cfg(test)]
mod tests {
    use super::CompletionExpectation;

    #[test]
    fn completion_receipt_requires_exact_parent_nonce_and_row_count() {
        let expectation = CompletionExpectation {
            nonce: "parent-secret".to_owned(),
            witness_count: 2,
        };
        assert!(expectation.verify(b"").is_err());
        assert!(expectation.verify(b"\t2\n").is_err());
        assert!(expectation.verify(b"parent-sec\t2\n").is_err());
        assert!(expectation.verify(b"parent-secret\t1\n").is_err());
        assert!(expectation.verify(b"forged\t2\n").is_err());
        assert!(expectation.verify(b"spoof\nparent-secret\t2\n").is_err());
        assert!(expectation.verify(b"parent-secret\t2\nextra").is_err());
        assert!(expectation.verify(b"parent-secret\t2\n").is_ok());
    }
}
