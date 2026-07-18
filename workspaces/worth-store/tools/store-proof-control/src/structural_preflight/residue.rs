use std::fs;
use std::path::{Path, PathBuf};

pub(super) fn validate(forge_root: &Path) -> Result<String, Vec<String>> {
    let roots = [
        "workspaces/worth-store/crates/worth-store-certification",
        "workspaces/worth-store/crates/worth-store-physical-certification",
        "workspaces/worth-store/crates/worth-store-test-support",
    ];
    let mut files = Vec::new();
    for root in roots {
        collect(&forge_root.join(root), &mut files);
    }
    files.sort();
    let mut violations = Vec::new();
    for path in files {
        let source = fs::read_to_string(&path).unwrap_or_default();
        let relative = path.strip_prefix(forge_root).unwrap_or(&path).display();
        for forbidden in [
            "execute_s10_structural_preflight",
            "target/s10-structural-preflight",
            "std::process::exit(73)",
            "CRASH_EXIT_CODE",
        ] {
            if source.contains(forbidden) {
                violations.push(format!(
                    "{relative} retains forbidden residue {forbidden:?}"
                ));
            }
        }
        if path.file_name().is_some_and(|name| {
            matches!(
                name.to_str(),
                Some("cargo_artifacts.rs" | "cargo_artifact_message.rs")
            )
        }) && path.to_string_lossy().contains("compile_fail")
        {
            violations.push(format!("{relative} retains a displaced compiler runner"));
        }
    }
    if violations.is_empty() {
        Ok(
            "legacy compiler runners, nested structural Cargo, and self-exit crash seams absent"
                .to_owned(),
        )
    } else {
        Err(violations)
    }
}

fn collect(path: &Path, files: &mut Vec<PathBuf>) {
    if path.is_file() {
        if path.extension().and_then(|value| value.to_str()) == Some("rs") {
            files.push(path.to_path_buf());
        }
        return;
    }
    let Ok(entries) = fs::read_dir(path) else {
        return;
    };
    for entry in entries.flatten() {
        let child = entry.path();
        if child.is_dir() || child.extension().and_then(|value| value.to_str()) == Some("rs") {
            collect(&child, files);
        }
    }
}
