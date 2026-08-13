use std::path::Path;

use super::source_layout::SourceModule;

pub(super) fn module_candidates(
    from_module: &SourceModule,
    source_root: &Path,
    prefix: &[String],
) -> Vec<Vec<String>> {
    let current = from_module.logical.clone();
    let mut suffix = prefix;
    let mut candidates = Vec::new();
    let mut base = current.clone();

    if suffix.first().is_some_and(|part| part == "crate") {
        base.clear();
        suffix = &suffix[1..];
        if names_source_root(source_root, suffix.first()) && source_root.join("mod.rs").is_file() {
            suffix = &suffix[1..];
        }
    } else if suffix.first().is_some_and(|part| part == "self") {
        suffix = &suffix[1..];
    } else {
        while suffix.first().is_some_and(|part| part == "super") {
            base.pop();
            suffix = &suffix[1..];
        }
    }

    let mut relative = base;
    relative.extend(suffix.iter().cloned());
    candidates.push(relative);
    if !prefix.is_empty()
        && !matches!(prefix[0].as_str(), "crate" | "self" | "super")
        && !current.is_empty()
    {
        candidates.push(prefix.to_vec());
    }
    candidates.dedup();
    candidates
}

fn names_source_root(source_root: &Path, component: Option<&String>) -> bool {
    component.is_some_and(|part| {
        source_root
            .file_name()
            .is_some_and(|root| root.to_string_lossy() == part.as_str())
    })
}
