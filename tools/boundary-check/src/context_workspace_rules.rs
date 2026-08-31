use crate::config::ContextWorkspaceConfig;
use crate::diagnostics::{Diagnostic, DiagnosticCode};
use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

pub(crate) fn validate_context_workspaces(
    root: &Path,
    workspaces: &[ContextWorkspaceConfig],
) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    for workspace in workspaces {
        diagnostics.extend(validate_context_workspace(root, workspace));
    }
    diagnostics
}

fn validate_context_workspace(root: &Path, workspace: &ContextWorkspaceConfig) -> Vec<Diagnostic> {
    let workspace_root = root.join(&workspace.path);
    let members = match workspace_members(&workspace_root.join("Cargo.toml")) {
        Ok(members) => members,
        Err(error) => return vec![contract_diagnostic(&workspace.path, error)],
    };
    let discovered = match discover_crates(&workspace_root.join("crates")) {
        Ok(discovered) => discovered,
        Err(error) => return vec![contract_diagnostic(&workspace.path, error)],
    };
    let mut diagnostics = validate_package_names(workspace, &discovered);
    diagnostics.extend(discovered.iter().filter_map(|(crate_path, _)| {
        (!members
            .iter()
            .any(|member| member == "crates/*" || member == crate_path))
        .then(|| {
            contract_diagnostic(
                crate_path,
                "context workspace crate is not admitted by workspace.members".into(),
            )
        })
    }));
    diagnostics
}

fn discover_crates(crates_root: &Path) -> Result<Vec<(String, std::path::PathBuf)>, String> {
    let entries = fs::read_dir(crates_root)
        .map_err(|error| format!("read context workspace crates failed: {error}"))?;
    let mut discovered = BTreeSet::new();
    for entry in entries {
        let entry = entry
            .map_err(|error| format!("read context workspace crate entry failed: {error}"))?;
        let manifest = entry.path().join("Cargo.toml");
        if manifest.is_file() {
            discovered.insert((
                format!("crates/{}", entry.file_name().to_string_lossy()),
                manifest,
            ));
        }
    }
    Ok(discovered.into_iter().collect())
}

fn validate_package_names(
    workspace: &ContextWorkspaceConfig,
    discovered: &[(String, std::path::PathBuf)],
) -> Vec<Diagnostic> {
    discovered
        .iter()
        .filter_map(|(relative, manifest)| match package_name(manifest) {
            Ok(package)
                if package == workspace.package_prefix
                    || package.starts_with(&format!("{}-", workspace.package_prefix)) => None,
            Ok(package) => Some(Diagnostic::new(
                DiagnosticCode::Bc1001IllegalCrateName,
                relative.clone(),
                format!("context workspace package {package} is outside {}", workspace.package_prefix),
            )),
            Err(error) => Some(contract_diagnostic(relative, error)),
        })
        .collect()
}

fn contract_diagnostic(path: &str, message: String) -> Diagnostic {
    Diagnostic::new(
        DiagnosticCode::Bc5002SubworkspaceContractViolation,
        path.to_owned(),
        message,
    )
}

fn workspace_members(path: &Path) -> Result<Vec<String>, String> {
    let text =
        fs::read_to_string(path).map_err(|error| format!("read {}: {error}", path.display()))?;
    let value: toml::Value =
        toml::from_str(&text).map_err(|error| format!("parse {}: {error}", path.display()))?;
    value
        .get("workspace")
        .and_then(|workspace| workspace.get("members"))
        .and_then(toml::Value::as_array)
        .map(|members| {
            members
                .iter()
                .filter_map(toml::Value::as_str)
                .map(str::to_owned)
                .collect()
        })
        .ok_or_else(|| format!("{} omits workspace.members", path.display()))
}

fn package_name(path: &Path) -> Result<String, String> {
    let text =
        fs::read_to_string(path).map_err(|error| format!("read {}: {error}", path.display()))?;
    let value: toml::Value =
        toml::from_str(&text).map_err(|error| format!("parse {}: {error}", path.display()))?;
    value
        .get("package")
        .and_then(|package| package.get("name"))
        .and_then(toml::Value::as_str)
        .map(str::to_owned)
        .ok_or_else(|| format!("{} omits package.name", path.display()))
}
