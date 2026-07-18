use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use super::super::cargo_surface::TestTargetIdentity;
use super::super::TestCaseIdentity;

pub(super) fn assigned_target<'a>(
    package: &str,
    source_path: &str,
    targets: &'a [TestTargetIdentity],
    target_files: &BTreeMap<String, BTreeSet<String>>,
) -> Option<&'a TestTargetIdentity> {
    targets
        .iter()
        .filter(|target| {
            !target.kinds.iter().any(|kind| kind == "doc")
                && target_files
                    .get(&target.identity)
                    .is_some_and(|files| files.contains(source_path))
        })
        .max_by_key(|target| target.source_path.len())
        .or_else(|| {
            source_path.contains("/src/").then(|| {
                targets.iter().find(|target| {
                    target.package == package && target.kinds.iter().any(|kind| kind == "lib")
                })
            })?
        })
}

pub(super) fn responsibility_for(package_root: &Path, source: &Path, case_name: &str) -> String {
    let relative = source.strip_prefix(package_root).unwrap_or(source);
    let components: Vec<_> = relative
        .components()
        .map(|component| component.as_os_str().to_string_lossy().to_string())
        .collect();
    if let Some(index) = components.iter().position(|part| part == "scenarios") {
        if components.len() > index + 2 {
            return format!("{}/{}", components[index + 1], components[index + 2]);
        }
    }
    for boundary in ["compile_fail", "ui"] {
        if let Some(index) = components.iter().position(|part| part == boundary) {
            return format!(
                "compiler/{}",
                components[index + 1..].join("/").trim_end_matches(".rs")
            );
        }
    }
    format!(
        "{}/{}",
        components.join("/").trim_end_matches(".rs"),
        case_name
    )
}

pub(super) fn stable_identity(
    package: &str,
    responsibility: &str,
    source: &Path,
    case_name: &str,
) -> TestCaseIdentity {
    let source_stem = source
        .file_stem()
        .and_then(|value| value.to_str())
        .unwrap_or("unknown");
    TestCaseIdentity {
        stable_id: format!("{package}::{responsibility}::{source_stem}::{case_name}"),
        package: package.to_owned(),
        responsibility: responsibility.to_owned(),
        case_name: case_name.to_owned(),
    }
}

pub(super) fn explicit_stable_identity(
    source: &str,
    case_name: &str,
) -> Result<Option<TestCaseIdentity>, String> {
    let prefix = format!("// store-proof-identity[{case_name}]: ");
    let declarations: Vec<_> = source
        .lines()
        .filter_map(|line| line.trim().strip_prefix(&prefix))
        .collect();
    let [stable_id] = declarations.as_slice() else {
        return if declarations.is_empty() {
            Ok(None)
        } else {
            Err(format!(
                "test {case_name} has multiple stable proof identity declarations"
            ))
        };
    };
    let parts: Vec<_> = stable_id.split("::").collect();
    if parts.len() != 4 || parts[3] != case_name {
        return Err(format!(
            "test {case_name} has malformed stable proof identity {stable_id}"
        ));
    }
    Ok(Some(TestCaseIdentity {
        stable_id: (*stable_id).to_owned(),
        package: parts[0].to_owned(),
        responsibility: parts[1].to_owned(),
        case_name: parts[3].to_owned(),
    }))
}
