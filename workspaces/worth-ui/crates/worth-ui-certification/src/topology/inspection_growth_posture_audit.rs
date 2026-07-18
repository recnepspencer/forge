use super::workspace_source_inventory::WorkspaceSourceInventory;

const FORBIDDEN_GENERIC_GROWTH_FALLBACKS: [&str; 7] = [
    "serde_json::Value",
    "HashMap<",
    "BTreeMap<",
    "dyn Debug",
    "debug_blob",
    "Generic(",
    "Other(",
];

pub fn audit_inspection_materialized_detail_growth_posture(
    inventory: &WorkspaceSourceInventory,
) -> Vec<String> {
    let path = inventory.absolute_path(
        "crates/worth-ui-runtime/src/evidence/shared/evidence_materialized_detail.rs",
    );
    let text = inventory.text(&path);
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

pub fn audit_dummy_future_family_extension_home(
    inventory: &WorkspaceSourceInventory,
) -> Vec<String> {
    let crates_root = inventory.absolute_path("crates");
    let mut violations = Vec::new();
    let hits = inventory
        .entries_under("crates")
        .filter(|path| {
            path.file_name()
                .is_some_and(|name| name == "dummy_future_family")
                || path
                    .file_stem()
                    .is_some_and(|stem| stem == "dummy_future_family")
        })
        .map(|path| inventory.absolute_path(path))
        .collect::<Vec<_>>();

    if hits.is_empty() {
        violations.push(format!(
            "{} is missing a dummy future family extension proof surface",
            crates_root.display()
        ));
        return violations;
    }

    let allowed_prefixes = [
        inventory
            .absolute_path("crates/worth-ui-inspection/src/receipt/evidence/dummy_future_family"),
        inventory.absolute_path("crates/worth-ui-runtime/src/evidence/dummy_future_family"),
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
