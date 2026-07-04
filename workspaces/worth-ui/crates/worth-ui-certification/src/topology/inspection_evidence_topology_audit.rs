use std::fs;
use std::path::Path;

const EXPECTED_FAMILY_RECORD_FILES: [(&str, &str); 5] = [
    (
        "declaration",
        "crates/worth-ui-runtime/src/declaration/inspection/declaration_evidence_record.rs",
    ),
    (
        "admission",
        "crates/worth-ui-runtime/src/admission/inspection/admission_evidence_record.rs",
    ),
    (
        "graph",
        "crates/worth-ui-runtime/src/graph/inspection/graph_evidence_record.rs",
    ),
    (
        "aspect",
        "crates/worth-ui-runtime/src/graph/inspection/aspect_evidence_record.rs",
    ),
    (
        "obligation",
        "crates/worth-ui-runtime/src/obligations/inspection/evidence_record.rs",
    ),
];

const FORBIDDEN_GENERIC_FALLBACKS: [&str; 5] = [
    "serde_json::Value",
    "HashMap<",
    "BTreeMap<",
    "dyn Debug",
    "debug_blob",
];

pub fn audit_evidence_family_storage_homes(workspace_root: &Path) -> Vec<String> {
    let mut violations = Vec::new();

    for (family, relative_path) in EXPECTED_FAMILY_RECORD_FILES {
        let path = workspace_root.join(relative_path);
        if !path.exists() {
            violations.push(format!(
                "{} is missing; {family} evidence lacks an explicit owner-local record home",
                path.display()
            ));
            continue;
        }

        let text = fs::read_to_string(&path).expect("evidence record file should decode");
        if !text.contains("UiEvidenceRef") {
            violations.push(format!(
                "{} does not bind {family} evidence to a typed UiEvidenceRef surface",
                path.display()
            ));
        }
        for forbidden in FORBIDDEN_GENERIC_FALLBACKS {
            if text.contains(forbidden) {
                violations.push(format!(
                    "{} falls back to forbidden generic evidence storage `{forbidden}`",
                    path.display()
                ));
            }
        }
    }

    violations.sort();
    violations.dedup();
    violations
}

pub fn audit_public_inspection_facades_do_not_export_family_local_records(
    workspace_root: &Path,
) -> Vec<String> {
    let mut violations = Vec::new();

    for relative_path in [
        "crates/worth-ui/src/facade/inspection.rs",
        "crates/worth-ui-runtime/src/facade/mod.rs",
    ] {
        let path = workspace_root.join(relative_path);
        let text = fs::read_to_string(&path).expect("inspection facade file should decode");
        for forbidden in [
            "EvidenceRecord",
            "declaration::inspection",
            "admission::inspection",
        ] {
            if text.contains(forbidden) {
                violations.push(format!(
                    "{} exports family-local inspection topology via `{forbidden}`",
                    path.display()
                ));
            }
        }
    }

    violations.sort();
    violations.dedup();
    violations
}

pub fn audit_inspection_crate_does_not_export_runtime_owned_evidence_surface(
    workspace_root: &Path,
) -> Vec<String> {
    let path = workspace_root.join("crates/worth-ui-inspection/src/lib.rs");
    let text = fs::read_to_string(&path).expect("inspection lib should decode");
    let mut violations = Vec::new();

    for forbidden in [
        "UiEvidenceIdentity",
        "UiEvidenceRef",
        "UiEvidenceHandle",
        "UiEvidenceSlice",
        "UiEvidenceMaterializedDetail",
        "UiInspectionObligationEvidenceReceipt",
    ] {
        if contains_identifier(&text, forbidden) {
            violations.push(format!(
                "{} still exports runtime-owned evidence authority surface `{forbidden}`",
                path.display()
            ));
        }
    }

    violations.sort();
    violations.dedup();
    violations
}

fn contains_identifier(text: &str, identifier: &str) -> bool {
    text.split(|ch: char| !(ch.is_ascii_alphanumeric() || ch == '_'))
        .any(|token| token == identifier)
}
