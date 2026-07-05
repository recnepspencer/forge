use std::fs;
use std::path::{Path, PathBuf};

const FORBIDDEN_GENERIC_GROWTH_FALLBACKS: [&str; 6] = [
    "serde_json::Value",
    "HashMap<",
    "BTreeMap<",
    "Generic(",
    "Other(",
    "debug_blob",
];

pub fn audit_measurement_future_growth_posture(workspace_root: &Path) -> Vec<String> {
    let mut violations = audit_measurement_basis_artifact_growth_posture(workspace_root);
    violations.extend(audit_measurement_future_family_extension_home(
        workspace_root,
    ));
    violations.sort();
    violations.dedup();
    violations
}

pub fn audit_measurement_basis_artifact_growth_posture(workspace_root: &Path) -> Vec<String> {
    let basis_path =
        workspace_root.join("crates/worth-ui-runtime/src/evidence/measurement_basis.rs");
    let lineage_path = workspace_root
        .join("crates/worth-ui-runtime/src/evidence/measurement_dependency_lineage.rs");
    let inspection_path = workspace_root.join(
        "crates/worth-ui-inspection/src/receipt/measurement/inspection_measurement_evidence_receipt.rs",
    );
    let basis_text = fs::read_to_string(&basis_path).expect("measurement basis should decode");
    let mut violations = Vec::new();

    for required in [
        "generation_compatibility",
        "dependency_lineage",
        "dependency_map",
        "neighborhood_class_hint",
    ] {
        if !basis_text.contains(required) {
            violations.push(format!(
                "{} must retain explicit `{required}` storage so future work extends the measurement kernel instead of reopening a helper lane",
                basis_path.display()
            ));
        }
    }

    if let Ok(lineage_text) = fs::read_to_string(&lineage_path) {
        if !lineage_text.contains("pub enum UiMeasurementDependencyLineageKind") {
            violations.push(format!(
                "{} must keep dependency lineage kinds as a closed typed axis",
                lineage_path.display()
            ));
        }
        push_generic_fallback_violations(&mut violations, &lineage_path, &lineage_text);
    }

    if let Ok(inspection_text) = fs::read_to_string(&inspection_path) {
        if !inspection_text.contains("#[non_exhaustive]") {
            violations.push(format!(
                "{} must stay #[non_exhaustive] so future measurement inspection families extend one typed substrate",
                inspection_path.display()
            ));
        }
        push_generic_fallback_violations(&mut violations, &inspection_path, &inspection_text);
    }

    push_generic_fallback_violations(&mut violations, &basis_path, &basis_text);

    violations.sort();
    violations.dedup();
    violations
}

pub fn audit_measurement_future_family_extension_home(workspace_root: &Path) -> Vec<String> {
    let crates_root = workspace_root.join("crates");
    let mut hits = Vec::new();
    let mut violations = Vec::new();

    collect_dummy_measurement_family_paths(&crates_root, &mut hits);
    if hits.is_empty() {
        violations.push(format!(
            "{} is missing a dummy measurement future-family proof surface",
            crates_root.display()
        ));
        return violations;
    }

    let allowed_prefixes = [
        workspace_root.join("crates/worth-ui-runtime/src/evidence/dummy_measurement_family"),
        workspace_root
            .join("crates/worth-ui-inspection/src/receipt/measurement/dummy_measurement_family"),
    ];

    for hit in hits {
        if !allowed_prefixes
            .iter()
            .any(|prefix| hit.starts_with(prefix))
        {
            violations.push(format!(
                "{} places `dummy_measurement_family` outside the one certified measurement growth home",
                hit.display()
            ));
        }
        let text = hit.to_string_lossy();
        if text.contains("facade") || text.contains("debug") || text.contains("host") {
            violations.push(format!(
                "{} grows `dummy_measurement_family` through a forbidden facade/debug/host substrate",
                hit.display()
            ));
        }
    }

    violations.sort();
    violations.dedup();
    violations
}

fn collect_dummy_measurement_family_paths(root: &Path, output: &mut Vec<PathBuf>) {
    if !root.exists() {
        return;
    }

    for entry in fs::read_dir(root).expect("source directory should read") {
        let entry = entry.expect("directory entry should read");
        let path = entry.path();
        if path.is_dir() {
            if path.file_name().is_some_and(|name| name == "tests") {
                continue;
            }
            if path
                .file_name()
                .is_some_and(|name| name.to_string_lossy() == "dummy_measurement_family")
            {
                output.push(path.clone());
            }
            collect_dummy_measurement_family_paths(&path, output);
        } else if path
            .file_stem()
            .is_some_and(|stem| stem.to_string_lossy() == "dummy_measurement_family")
        {
            output.push(path);
        }
    }
}

fn push_generic_fallback_violations(violations: &mut Vec<String>, path: &Path, text: &str) {
    for forbidden in FORBIDDEN_GENERIC_GROWTH_FALLBACKS {
        if text.contains(forbidden) {
            violations.push(format!(
                "{} widens future measurement growth into forbidden generic fallback `{forbidden}`",
                path.display()
            ));
        }
    }
}
