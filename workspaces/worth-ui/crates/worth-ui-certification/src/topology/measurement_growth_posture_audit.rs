use std::path::Path;

use super::workspace_source_inventory::WorkspaceSourceInventory;

const FORBIDDEN_GENERIC_GROWTH_FALLBACKS: [&str; 6] = [
    "serde_json::Value",
    "HashMap<",
    "BTreeMap<",
    "Generic(",
    "Other(",
    "debug_blob",
];

pub fn audit_measurement_future_growth_posture(
    inventory: &WorkspaceSourceInventory,
) -> Vec<String> {
    audit_measurement_basis_artifact_growth_posture(inventory)
}

pub fn audit_measurement_basis_artifact_growth_posture(
    inventory: &WorkspaceSourceInventory,
) -> Vec<String> {
    let basis_path =
        inventory.absolute_path("crates/worth-ui-runtime/src/evidence/measurement/basis/admit.rs");
    let lineage_path = inventory
        .absolute_path("crates/worth-ui-runtime/src/evidence/measurement/dependency/lineage.rs");
    let inspection_path = inventory.absolute_path(
        "crates/worth-ui-inspection/src/receipt/measurement/inspection_measurement_evidence_receipt.rs",
    );
    let basis_text = inventory.text(&basis_path);
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

    if let Some(lineage_text) = inventory.source(&lineage_path).map(|source| source.text()) {
        if !lineage_text.contains("pub enum UiMeasurementDependencyLineageKind") {
            violations.push(format!(
                "{} must keep dependency lineage kinds as a closed typed axis",
                lineage_path.display()
            ));
        }
        push_generic_fallback_violations(&mut violations, &lineage_path, lineage_text);
    }

    if let Some(inspection_text) = inventory
        .source(&inspection_path)
        .map(|source| source.text())
    {
        if !inspection_text.contains("#[non_exhaustive]") {
            violations.push(format!(
                "{} must stay #[non_exhaustive] so future measurement inspection families extend one typed substrate",
                inspection_path.display()
            ));
        }
        push_generic_fallback_violations(&mut violations, &inspection_path, inspection_text);
    }

    push_generic_fallback_violations(&mut violations, &basis_path, basis_text);

    violations.sort();
    violations.dedup();
    violations
}

pub fn audit_measurement_future_family_extension_home(
    inventory: &WorkspaceSourceInventory,
) -> Vec<String> {
    let crates_root = inventory.absolute_path("crates");
    let hits = inventory
        .entries_under("crates")
        .filter(|path| {
            !path
                .components()
                .any(|component| component.as_os_str() == "tests")
        })
        .filter(|path| {
            path.file_name()
                .is_some_and(|name| name == "dummy_measurement_family")
                || path
                    .file_stem()
                    .is_some_and(|stem| stem == "dummy_measurement_family")
        })
        .map(|path| inventory.absolute_path(path))
        .collect::<Vec<_>>();
    let mut violations = Vec::new();

    if hits.is_empty() {
        violations.push(format!(
            "{} is missing a dummy measurement future-family proof surface",
            crates_root.display()
        ));
        return violations;
    }

    let allowed_prefixes = [
        inventory.absolute_path("crates/worth-ui-runtime/src/evidence/dummy_measurement_family"),
        inventory.absolute_path(
            "crates/worth-ui-inspection/src/receipt/measurement/dummy_measurement_family",
        ),
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
