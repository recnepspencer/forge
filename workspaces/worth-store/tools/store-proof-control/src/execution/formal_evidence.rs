use std::collections::BTreeMap;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::evidence::sha256_file;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FormalToolEvidenceReference {
    pub tool: String,
    pub version: String,
    pub jar_sha256: String,
    pub toolchain_sha256: String,
    pub protocol_count: usize,
    pub receipt_sha256: String,
    pub receipt_path: String,
}

#[derive(Deserialize)]
struct FormalToolReceipt {
    schema_version: u32,
    tool: String,
    version: String,
    jar_sha256: String,
    toolchain_sha256: String,
    main_class: String,
    model: String,
    configuration: String,
    java_executable: String,
    protocol_count: usize,
    verdict: String,
}

pub(crate) fn collect(
    workspace_root: &Path,
    receipt_path: &Path,
    evidence_required: bool,
) -> Result<Option<FormalToolEvidenceReference>, String> {
    if !receipt_path.exists() {
        return if evidence_required {
            Err("formal tool unit passed without a structured TLC receipt".to_owned())
        } else {
            Ok(None)
        };
    }
    let bytes = std::fs::read(receipt_path)
        .map_err(|error| format!("could not read {}: {error}", receipt_path.display()))?;
    let receipt: FormalToolReceipt = serde_json::from_slice(&bytes)
        .map_err(|error| format!("could not decode {}: {error}", receipt_path.display()))?;
    let toolchain_path =
        workspace_root.join("crates/worth-store-formal-models/formal-toolchain.toml");
    let declared = declared_toolchain(&toolchain_path)?;
    let observed_protocol_count =
        protocol_count(&workspace_root.join("crates/worth-store-formal-models/src/protocols"))?;
    let mut denials = Vec::new();
    if receipt.schema_version != 1 {
        denials.push(format!(
            "formal receipt schema differs: expected 1, observed {}",
            receipt.schema_version
        ));
    }
    require_equal(&mut denials, "tool", &receipt.tool, "tlc");
    require_equal(
        &mut denials,
        "version",
        &receipt.version,
        &declared["version"],
    );
    require_equal(
        &mut denials,
        "jar sha256",
        &receipt.jar_sha256,
        &declared["sha256"],
    );
    require_equal(
        &mut denials,
        "main class",
        &receipt.main_class,
        &declared["main_class"],
    );
    require_equal(&mut denials, "model", &receipt.model, &declared["model"]);
    require_equal(
        &mut denials,
        "configuration",
        &receipt.configuration,
        &declared["configuration"],
    );
    require_equal(&mut denials, "verdict", &receipt.verdict, "passed");
    let toolchain_sha256 = sha256_file(&toolchain_path)?;
    require_equal(
        &mut denials,
        "toolchain sha256",
        &receipt.toolchain_sha256,
        &toolchain_sha256,
    );
    if receipt.java_executable.trim().is_empty() {
        denials.push("formal receipt omits the Java executable identity".to_owned());
    }
    if receipt.protocol_count != observed_protocol_count || receipt.protocol_count == 0 {
        denials.push(format!(
            "formal receipt protocol count differs: receipt={} repository={observed_protocol_count}",
            receipt.protocol_count
        ));
    }
    if !denials.is_empty() {
        return Err(format!(
            "formal tool receipt is invalid:\n  - {}",
            denials.join("\n  - ")
        ));
    }
    Ok(Some(FormalToolEvidenceReference {
        tool: receipt.tool,
        version: receipt.version,
        jar_sha256: receipt.jar_sha256,
        toolchain_sha256: receipt.toolchain_sha256,
        protocol_count: receipt.protocol_count,
        receipt_sha256: crate::evidence::sha256_bytes(&bytes),
        receipt_path: receipt_path
            .strip_prefix(workspace_root)
            .unwrap_or(receipt_path)
            .to_string_lossy()
            .replace('\\', "/"),
    }))
}

fn declared_toolchain(path: &Path) -> Result<BTreeMap<String, String>, String> {
    let source = std::fs::read_to_string(path)
        .map_err(|error| format!("could not read {}: {error}", path.display()))?;
    let declared: BTreeMap<_, _> = source
        .lines()
        .filter_map(|line| {
            let (name, value) = line.split_once('=')?;
            Some((
                name.trim().to_owned(),
                value.trim().trim_matches('"').to_owned(),
            ))
        })
        .collect();
    for required in ["version", "sha256", "main_class", "model", "configuration"] {
        if !declared.contains_key(required) {
            return Err(format!(
                "formal toolchain declaration omits {required}: {}",
                path.display()
            ));
        }
    }
    Ok(declared)
}

fn protocol_count(root: &Path) -> Result<usize, String> {
    let mut count = 0;
    let mut pending = vec![root.to_path_buf()];
    while let Some(directory) = pending.pop() {
        for entry in std::fs::read_dir(&directory)
            .map_err(|error| format!("could not inspect {}: {error}", directory.display()))?
        {
            let entry = entry.map_err(|error| error.to_string())?;
            let file_type = entry.file_type().map_err(|error| error.to_string())?;
            if file_type.is_dir() {
                pending.push(entry.path());
            } else if file_type.is_file()
                && entry.path().extension().and_then(|value| value.to_str()) == Some("tla")
            {
                count += 1;
            }
        }
    }
    Ok(count)
}

fn require_equal(denials: &mut Vec<String>, field: &str, actual: &str, expected: &str) {
    if actual != expected {
        denials.push(format!(
            "formal receipt {field} differs: expected {expected:?}, observed {actual:?}"
        ));
    }
}
