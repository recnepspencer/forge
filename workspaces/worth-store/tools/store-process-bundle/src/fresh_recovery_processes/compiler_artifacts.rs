use std::path::{Path, PathBuf};

use serde_json::Value;

use super::metadata_graph::Metadata;
use super::targets::TargetSpec;

#[derive(Clone, Debug)]
pub struct CompilerArtifactRecord {
    package_id: String,
    package: String,
    target: String,
    kinds: Box<[String]>,
    features: Box<[String]>,
    executable: Option<PathBuf>,
    fresh: Option<bool>,
}

impl CompilerArtifactRecord {
    pub fn package_id(&self) -> &str {
        &self.package_id
    }

    pub fn package(&self) -> &str {
        &self.package
    }

    pub fn target(&self) -> &str {
        &self.target
    }

    pub fn kinds(&self) -> &[String] {
        &self.kinds
    }

    pub fn features(&self) -> &[String] {
        &self.features
    }

    pub fn executable(&self) -> Option<&Path> {
        self.executable.as_deref()
    }

    pub fn fresh(&self) -> Option<bool> {
        self.fresh
    }
}

#[derive(Clone, Debug)]
pub struct CompilerTranscript {
    raw_stdout: String,
    records: Box<[CompilerArtifactRecord]>,
}

impl CompilerTranscript {
    pub fn raw_stdout(&self) -> &str {
        &self.raw_stdout
    }

    pub fn records(&self) -> &[CompilerArtifactRecord] {
        &self.records
    }

    #[cfg(test)]
    pub(crate) fn test_empty() -> Self {
        Self {
            raw_stdout: String::new(),
            records: Box::new([]),
        }
    }
}

pub(crate) fn parse<R>(
    stdout: Vec<u8>,
    metadata: &Metadata,
    target: &TargetSpec<R>,
) -> Result<(CompilerTranscript, PathBuf), String> {
    let raw_stdout = String::from_utf8(stdout)
        .map_err(|error| format!("Cargo emitted non-UTF-8 JSON: {error}"))?;
    let mut records = Vec::new();
    let mut executable = None;
    let expected_package_id = metadata.package_id(target)?;
    for line in raw_stdout.lines().filter(|line| !line.trim().is_empty()) {
        let message: Value = serde_json::from_str(line)
            .map_err(|error| format!("decode Cargo compiler artifact: {error}; line={line}"))?;
        if message.get("reason").and_then(Value::as_str) != Some("compiler-artifact") {
            continue;
        }
        let package_id = required_string(&message, "package_id")?.to_owned();
        let Some(package) = metadata.package_name(&package_id) else {
            continue;
        };
        let target_value = message
            .get("target")
            .ok_or_else(|| format!("Cargo artifact omitted target for {package_id}"))?;
        let target_name = required_string(target_value, "name")?.to_owned();
        let kinds = target_value
            .get("kind")
            .and_then(Value::as_array)
            .ok_or_else(|| format!("Cargo artifact omitted target kind for {package}::{target_name}"))?
            .iter()
            .map(|kind| {
                kind.as_str()
                    .map(str::to_owned)
                    .ok_or_else(|| format!("Cargo artifact returned non-string target kind for {package}::{target_name}"))
            })
            .collect::<Result<Vec<_>, _>>()?;
        let features = message
            .get("features")
            .and_then(Value::as_array)
            .ok_or_else(|| format!("Cargo artifact omitted features for {package}::{target_name}"))?
            .iter()
            .map(|feature| {
                feature.as_str().map(str::to_owned).ok_or_else(|| {
                    format!(
                        "Cargo artifact returned non-string feature for {package}::{target_name}"
                    )
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        let path = message
            .get("executable")
            .and_then(Value::as_str)
            .map(PathBuf::from);
        let record = CompilerArtifactRecord {
            package_id: package_id.clone(),
            package: package.to_owned(),
            target: target_name.clone(),
            kinds: kinds.into_boxed_slice(),
            features: features.into_boxed_slice(),
            executable: path.clone(),
            fresh: message.get("fresh").and_then(Value::as_bool),
        };
        if package_id == expected_package_id
            && target_name == target.binary
            && record.kinds.iter().any(|kind| kind == "bin")
        {
            let path =
                path.ok_or_else(|| format!("Cargo omitted executable for {}", target.binary))?;
            if executable.replace(path).is_some() {
                return Err(format!(
                    "Cargo emitted duplicate executable for {}",
                    target.binary
                ));
            }
        }
        records.push(record);
    }
    let executable =
        executable.ok_or_else(|| format!("Cargo emitted no executable for {}", target.binary))?;
    if !executable.is_file() {
        return Err(format!("Cargo executable is not a file: {executable:?}"));
    }
    Ok((
        CompilerTranscript {
            raw_stdout,
            records: records.into_boxed_slice(),
        },
        executable,
    ))
}

fn required_string<'a>(value: &'a Value, field: &str) -> Result<&'a str, String> {
    value
        .get(field)
        .and_then(Value::as_str)
        .ok_or_else(|| format!("Cargo compiler artifact omitted string field {field}"))
}
