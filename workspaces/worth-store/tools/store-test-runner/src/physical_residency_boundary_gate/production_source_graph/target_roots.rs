use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde::Deserialize;

use super::{canonical, macro_provenance::external_declarative_source_expansion, PendingSource};

#[derive(Deserialize)]
struct Metadata {
    packages: Vec<Package>,
}

#[derive(Deserialize)]
struct Package {
    manifest_path: PathBuf,
    name: String,
    source: Option<String>,
    targets: Vec<Target>,
    version: String,
}

#[derive(Deserialize)]
struct Target {
    kind: Vec<String>,
    src_path: PathBuf,
}

const REVIEWED_PROC_MACROS: &[(&str, &str, &str)] = &[
    ("rustversion", "1.0.22", REGISTRY),
    ("serde_derive", "1.0.228", REGISTRY),
    ("tracing-attributes", "0.1.31", REGISTRY),
    ("wasm-bindgen-macro", "0.2.126", REGISTRY),
    ("windows-implement", "0.60.2", REGISTRY),
    ("windows-interface", "0.59.3", REGISTRY),
];
const REVIEWED_DECLARATIVE_MACROS: &[(&str, &str, &str)] = &[
    ("bitflags", "2.13.0", REGISTRY),
    ("cpufeatures", "0.2.17", REGISTRY),
    ("fastrand", "2.4.1", REGISTRY),
    ("getrandom", "0.4.3", REGISTRY),
    ("libc", "0.2.186", REGISTRY),
    ("memchr", "2.8.2", REGISTRY),
    ("rayon", "1.12.0", REGISTRY),
    ("serde", "1.0.228", REGISTRY),
    ("serde_core", "1.0.228", REGISTRY),
    ("syn", "2.0.118", REGISTRY),
    ("sysinfo", "0.38.4", REGISTRY),
    ("target-triple", "1.0.1", REGISTRY),
    ("windows-sys", "0.59.0", REGISTRY),
];
const REGISTRY: &str = "registry+https://github.com/rust-lang/crates.io-index";

pub(super) fn production_target_roots(workspace: &Path) -> Result<Vec<PendingSource>, String> {
    let output = Command::new("cargo")
        .args(["metadata", "--format-version", "1", "--manifest-path"])
        .arg(workspace.join("Cargo.toml"))
        .output()
        .map_err(|error| format!("cannot run Cargo metadata: {error}"))?;
    if !output.status.success() {
        return Err(format!(
            "Cargo metadata failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    let metadata: Metadata = serde_json::from_slice(&output.stdout)
        .map_err(|error| format!("invalid Cargo metadata: {error}"))?;
    reject_unreviewed_proc_macros(&metadata.packages)?;
    reject_external_source_expansions(workspace, &metadata.packages)?;
    target_roots(workspace, metadata.packages)
}

fn target_roots(workspace: &Path, packages: Vec<Package>) -> Result<Vec<PendingSource>, String> {
    let workspace = canonical(workspace)?;
    let mut targets = Vec::new();
    for package in packages {
        let manifest = canonical(&package.manifest_path)?;
        if !manifest.starts_with(&workspace) {
            continue;
        }
        for target in package.targets {
            if production_target(&target.kind) {
                targets.push(target_root(target)?);
            }
        }
    }
    targets.sort();
    targets.dedup();
    Ok(targets)
}

fn target_root(target: Target) -> Result<PendingSource, String> {
    let source = canonical(&target.src_path)?;
    let module_dir = source
        .parent()
        .unwrap_or_else(|| Path::new(""))
        .to_path_buf();
    Ok(PendingSource {
        source,
        path_attr_dir: module_dir.clone(),
        module_dir,
    })
}

fn reject_unreviewed_proc_macros(packages: &[Package]) -> Result<(), String> {
    for package in packages {
        if !package
            .targets
            .iter()
            .any(|target| target.kind.iter().any(|kind| kind == "proc-macro"))
        {
            continue;
        }
        let identity = (
            package.name.as_str(),
            package.version.as_str(),
            package.source.as_deref().unwrap_or("local"),
        );
        if !REVIEWED_PROC_MACROS.contains(&identity) {
            return Err(format!(
                "unreviewed procedural macro cannot prove constructor expansion: {} {} {}",
                identity.0, identity.1, identity.2
            ));
        }
    }
    Ok(())
}

fn reject_external_source_expansions(workspace: &Path, packages: &[Package]) -> Result<(), String> {
    let workspace = canonical(workspace)?;
    let mut denials = Vec::new();
    for package in packages {
        let manifest = canonical(&package.manifest_path)?;
        let identity = (
            package.name.as_str(),
            package.version.as_str(),
            package.source.as_deref().unwrap_or("local"),
        );
        if manifest.starts_with(&workspace)
            || REVIEWED_DECLARATIVE_MACROS.contains(&identity)
            || package
                .targets
                .iter()
                .any(|target| target.kind.iter().any(|kind| kind == "proc-macro"))
        {
            continue;
        }
        let root = manifest.parent().unwrap_or_else(|| Path::new(""));
        let mut pending = Vec::new();
        let mut visited = BTreeSet::new();
        collect_rust_sources(root, &mut pending)?;
        while let Some(source) = pending.pop() {
            let source = canonical(&source)?;
            if !visited.insert(source.clone()) {
                continue;
            }
            let text = std::fs::read_to_string(&source)
                .map_err(|error| format!("cannot read {}: {error}", source.display()))?;
            if !text.contains("include") && !text.contains("macro_rules") {
                continue;
            }
            let evidence = external_declarative_source_expansion(&text)?;
            if let Some(expansion) = evidence.denial {
                denials.push(format!(
                    "{} {} in {}",
                    package.name,
                    expansion,
                    source.display()
                ));
            }
            let source_dir = source.parent().unwrap_or_else(|| Path::new(""));
            for included in evidence.literal_includes {
                pending.push(source_dir.join(included));
            }
        }
    }
    if denials.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "external declarative macro source cannot prove expansion: {}",
            denials.join("; ")
        ))
    }
}

fn collect_rust_sources(root: &Path, sources: &mut Vec<PathBuf>) -> Result<(), String> {
    for entry in std::fs::read_dir(root)
        .map_err(|error| format!("cannot list {}: {error}", root.display()))?
    {
        let entry = entry.map_err(|error| format!("cannot inspect {}: {error}", root.display()))?;
        let path = entry.path();
        if path.is_dir() {
            let name = path.file_name().and_then(|name| name.to_str());
            if !matches!(name, Some("target" | ".git")) {
                collect_rust_sources(&path, sources)?;
            }
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            sources.push(path);
        }
    }
    Ok(())
}

fn production_target(kind: &[String]) -> bool {
    kind.iter().any(|kind| {
        matches!(
            kind.as_str(),
            "lib" | "rlib" | "dylib" | "cdylib" | "staticlib" | "proc-macro" | "bin"
        )
    })
}
