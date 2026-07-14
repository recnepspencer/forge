use std::fs;
use std::path::{Path, PathBuf};

const FORBIDDEN_GENERIC_GROWTH_FALLBACKS: [&str; 7] = [
    "serde_json::Value",
    "HashMap<",
    "BTreeMap<",
    "dyn Debug",
    "debug_blob",
    "Generic(",
    "Other(",
];

pub fn audit_inspection_materialized_detail_growth_posture(workspace_root: &Path) -> Vec<String> {
    let path = workspace_root
        .join("crates/worth-ui-runtime/src/evidence/shared/evidence_materialized_detail.rs");
    let text = fs::read_to_string(&path).expect("materialized detail surface should decode");
    let mut violations = Vec::new();

    if !text.contains("#[non_exhaustive]") {
        violations.push(format!(
            "{} must stay #[non_exhaustive] so future evidence families extend one substrate instead of forcing public exhaustiveness churn",
            path.display()
        ));
    }

    if !text.contains("pub enum UiEvidenceMaterializedDetail") {
        violations.push(format!(
            "{} must define the typed materialized-detail growth surface explicitly",
            path.display()
        ));
    }

    for forbidden in FORBIDDEN_GENERIC_GROWTH_FALLBACKS {
        if text.contains(forbidden) {
            violations.push(format!(
                "{} widens future evidence growth into forbidden generic fallback `{forbidden}`",
                path.display()
            ));
        }
    }

    violations.sort();
    violations.dedup();
    violations
}

pub fn audit_dummy_future_family_extension_home(workspace_root: &Path) -> Vec<String> {
    let crates_root = workspace_root.join("crates");
    let mut violations = Vec::new();
    let mut hits = Vec::new();

    collect_dummy_future_family_paths(&crates_root, &mut hits);
    if hits.is_empty() {
        violations.push(format!(
            "{} is missing a dummy future family extension proof surface",
            crates_root.display()
        ));
        return violations;
    }

    let allowed_prefixes = [
        workspace_root.join("crates/worth-ui-inspection/src/receipt/evidence/dummy_future_family"),
        workspace_root.join("crates/worth-ui-runtime/src/evidence/dummy_future_family"),
    ];

    for hit in hits {
        if !allowed_prefixes
            .iter()
            .any(|prefix| hit.starts_with(prefix))
        {
            violations.push(format!(
                "{} places `dummy_future_family` outside the one certified inspection evidence substrate home",
                hit.display()
            ));
        }
        if hit.to_string_lossy().contains("facade") || hit.to_string_lossy().contains("debug") {
            violations.push(format!(
                "{} grows `dummy_future_family` through a forbidden facade/debug substrate instead of the evidence substrate",
                hit.display()
            ));
        }
    }

    violations.sort();
    violations.dedup();
    violations
}

fn collect_dummy_future_family_paths(root: &Path, output: &mut Vec<PathBuf>) {
    if !root.exists() {
        return;
    }
    for entry in fs::read_dir(root).expect("source directory should read") {
        let entry = entry.expect("directory entry should read");
        let path = entry.path();
        if path.is_dir() {
            if path
                .file_name()
                .is_some_and(|name| name.to_string_lossy() == "dummy_future_family")
            {
                output.push(path.clone());
            }
            collect_dummy_future_family_paths(&path, output);
        } else if path
            .file_stem()
            .is_some_and(|stem| stem.to_string_lossy() == "dummy_future_family")
        {
            output.push(path);
        }
    }
}
