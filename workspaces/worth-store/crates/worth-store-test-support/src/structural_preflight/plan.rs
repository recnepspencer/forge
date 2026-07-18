use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::io::Read;
use std::path::{Path, PathBuf};

use super::StructuralPredicate;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum StructuralPreflightProfile {
    DeveloperSmoke,
    Ui,
    Complete,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StructuralPreflightRequest {
    pub profile: StructuralPreflightProfile,
    pub predicates: Vec<StructuralPredicate>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PreflightInputScope {
    pub scope_identity: String,
    pub source_paths: Vec<String>,
    pub included_extensions: Vec<String>,
    pub input_identity: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StructuralToolDeclaration {
    pub tool_identity: String,
    pub provenance: String,
    pub program: String,
    pub resolved_program_path: String,
    pub program_sha256: String,
    pub program_version_identity: String,
    pub arguments: Vec<String>,
    pub supporting_tools: Vec<StructuralSupportingToolIdentity>,
    pub environment: Vec<StructuralToolEnvironmentBinding>,
    pub removed_environment: Vec<String>,
    pub source_scope_identity: String,
    pub timeout_millis: u64,
    pub resource_posture: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StructuralSupportingToolIdentity {
    pub purpose: String,
    pub resolved_program_path: String,
    pub program_sha256: String,
    pub program_version_identity: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StructuralToolEnvironmentBinding {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StructuralPredicatePlan {
    pub predicate: StructuralPredicate,
    pub input_scopes: Vec<PreflightInputScope>,
    pub tool: Option<StructuralToolDeclaration>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StructuralPreflightPlan {
    pub schema_version: u32,
    pub request: StructuralPreflightRequest,
    pub predicates: Vec<StructuralPredicatePlan>,
    pub plan_identity: String,
}

impl StructuralPreflightRequest {
    pub fn new(
        profile: StructuralPreflightProfile,
        mut predicates: Vec<StructuralPredicate>,
    ) -> Result<Self, String> {
        predicates.sort();
        predicates.dedup();
        if predicates.is_empty() {
            return Err("structural preflight requires at least one predicate".to_owned());
        }
        Ok(Self {
            profile,
            predicates,
        })
    }
}

impl StructuralToolDeclaration {
    pub fn workspace_owned(
        tool_identity: impl Into<String>,
        program: impl Into<String>,
        program_version_identity: impl Into<String>,
        arguments: Vec<String>,
        source_scope_identity: impl Into<String>,
        timeout_millis: u64,
        resource_posture: impl Into<String>,
    ) -> Result<Self, String> {
        let program = program.into();
        let resolved_program = resolve_program(&program)?;
        let declaration = Self {
            tool_identity: tool_identity.into(),
            provenance: "forge-workspace-owned-source".to_owned(),
            program,
            resolved_program_path: normalized_path(&resolved_program),
            program_sha256: file_digest(&resolved_program)?,
            program_version_identity: program_version_identity.into(),
            arguments,
            supporting_tools: Vec::new(),
            environment: Vec::new(),
            removed_environment: Vec::new(),
            source_scope_identity: source_scope_identity.into(),
            timeout_millis,
            resource_posture: resource_posture.into(),
        };
        if declaration.tool_identity.trim().is_empty()
            || declaration.program.trim().is_empty()
            || declaration.program_version_identity.trim().is_empty()
            || declaration.source_scope_identity.trim().is_empty()
            || declaration.timeout_millis == 0
            || declaration.resource_posture.trim().is_empty()
        {
            return Err("structural tool declaration is incomplete".to_owned());
        }
        Ok(declaration)
    }

    pub fn with_supporting_tools(
        mut self,
        mut tools: Vec<StructuralSupportingToolIdentity>,
    ) -> Result<Self, String> {
        tools.sort_by(|left, right| left.purpose.cmp(&right.purpose));
        if tools
            .windows(2)
            .any(|pair| pair[0].purpose == pair[1].purpose)
            || tools.iter().any(|tool| {
                tool.purpose.trim().is_empty()
                    || tool.resolved_program_path.trim().is_empty()
                    || !is_sha256(&tool.program_sha256)
                    || tool.program_version_identity.trim().is_empty()
            })
        {
            return Err("structural supporting-tool declaration is invalid".to_owned());
        }
        self.supporting_tools = tools;
        Ok(self)
    }

    pub fn with_environment(
        mut self,
        mut environment: Vec<StructuralToolEnvironmentBinding>,
        mut removed_environment: Vec<String>,
    ) -> Result<Self, String> {
        environment.sort_by(|left, right| left.name.cmp(&right.name));
        removed_environment.sort();
        removed_environment.dedup();
        let names = environment
            .iter()
            .map(|binding| binding.name.as_str())
            .collect::<std::collections::BTreeSet<_>>();
        if names.len() != environment.len()
            || environment
                .iter()
                .any(|binding| !valid_environment_name(&binding.name) || binding.value.is_empty())
            || removed_environment
                .iter()
                .any(|name| !valid_environment_name(name) || names.contains(name.as_str()))
        {
            return Err("structural tool environment declaration is invalid".to_owned());
        }
        self.environment = environment;
        self.removed_environment = removed_environment;
        Ok(self)
    }
}

fn valid_environment_name(name: &str) -> bool {
    !name.is_empty()
        && name
            .bytes()
            .all(|byte| byte.is_ascii_uppercase() || byte.is_ascii_digit() || byte == b'_')
}

fn is_sha256(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

fn resolve_program(program: &str) -> Result<PathBuf, String> {
    let path = Path::new(program);
    if path.is_absolute() || path.components().count() > 1 {
        return std::fs::canonicalize(path)
            .map_err(|error| format!("could not resolve structural tool {program}: {error}"));
    }
    let extensions = executable_extensions();
    for root in std::env::split_paths(&std::env::var_os("PATH").unwrap_or_default()) {
        for extension in &extensions {
            let candidate = root.join(format!("{program}{extension}"));
            if candidate.is_file() {
                return std::fs::canonicalize(&candidate).map_err(|error| {
                    format!(
                        "could not resolve structural tool {}: {error}",
                        candidate.display()
                    )
                });
            }
        }
    }
    Err(format!(
        "structural tool executable {program} is not on PATH"
    ))
}

fn executable_extensions() -> Vec<String> {
    if cfg!(windows) {
        let mut extensions = std::env::var("PATHEXT")
            .unwrap_or_else(|_| ".COM;.EXE;.BAT;.CMD".to_owned())
            .split(';')
            .map(str::to_ascii_lowercase)
            .collect::<Vec<_>>();
        extensions.insert(0, String::new());
        extensions
    } else {
        vec![String::new()]
    }
}

fn file_digest(path: &Path) -> Result<String, String> {
    let mut file = std::fs::File::open(path)
        .map_err(|error| format!("could not read structural tool {}: {error}", path.display()))?;
    let mut digest = Sha256::new();
    let mut buffer = [0_u8; 64 * 1024];
    loop {
        let read = file.read(&mut buffer).map_err(|error| {
            format!("could not read structural tool {}: {error}", path.display())
        })?;
        if read == 0 {
            break;
        }
        digest.update(&buffer[..read]);
    }
    Ok(format!("{:x}", digest.finalize()))
}

fn normalized_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}
