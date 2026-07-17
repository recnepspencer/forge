use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use syn::{Item, UseTree, Visibility};

const FORBIDDEN_PUBLIC_MODULE_NAMES: [&str; 7] = [
    "internal", "common", "helpers", "utils", "data", "manager", "debug",
];

pub fn audit_inspection_public_module_names(workspace_root: &Path) -> Vec<String> {
    let inspection_root = workspace_root.join("crates/worth-ui-inspection/src");
    let mut violations = Vec::new();

    for declaration in collect_public_module_declarations(&inspection_root) {
        if FORBIDDEN_PUBLIC_MODULE_NAMES.contains(&declaration.module_name.as_str()) {
            violations.push(format!(
                "{} exposes forbidden public module `{}`",
                declaration.declaring_file.display(),
                declaration.module_name
            ));
        }
    }

    violations.sort();
    violations.dedup();
    violations
}

pub fn audit_inspection_public_module_role_purity(workspace_root: &Path) -> Vec<String> {
    let inspection_root = workspace_root.join("crates/worth-ui-inspection/src");
    let expected_exports = [
        (
            "facade/mod.rs",
            BTreeSet::from([
                "RUNTIME_INSPECTION_SCOPE_INVENTORY".to_string(),
                "UiInspectionScopeInventory".to_string(),
            ]),
        ),
        (
            "query/mod.rs",
            BTreeSet::from([
                "UiEvidenceBudget".to_string(),
                "UiEvidenceLinkKind".to_string(),
                "UiEvidenceRichness".to_string(),
                "UiAllocationPlanningQuestion".to_string(),
                "UiInspectionAspectRelevanceDetail".to_string(),
                "UiInspectionEvidenceSource".to_string(),
                "UiInspectionObligationRelevanceDetail".to_string(),
                "UiInspectionQuery".to_string(),
                "UiInspectionRelevance".to_string(),
                "UiInspectionRelevanceAdmission".to_string(),
                "UiInspectionRelevanceOutcome".to_string(),
                "UiInspectionTargetClass".to_string(),
                "UiRelevanceFamily".to_string(),
                "UiRelevanceFilter".to_string(),
            ]),
        ),
        (
            "target/mod.rs",
            BTreeSet::from([
                "UiAuthoredSourceProvenanceRef".to_string(),
                "UiInspectionAspectName".to_string(),
                "UiInspectionDeclarationIdentity".to_string(),
                "UiInspectionTarget".to_string(),
                "UiSourceArtifactGeneration".to_string(),
                "UiSourceArtifactIdentity".to_string(),
            ]),
        ),
        (
            "scope/mod.rs",
            BTreeSet::from(["UiInspectionScope".to_string()]),
        ),
        (
            "receipt/mod.rs",
            BTreeSet::from([
                "UiInspectionAiHarnessLane".to_string(),
                "UiInspectionClosedSemanticLane".to_string(),
                "UiInspectionClosureReport".to_string(),
                "UiInspectionCloseoutGuarantee".to_string(),
                "UiInspectionCloseoutNonGoal".to_string(),
                "UiInspectionCloseoutReport".to_string(),
                "UiInspectionCostLane".to_string(),
                "UiInspectionCostReceipt".to_string(),
                "UiInspectionDerivedIndexLane".to_string(),
                "UiInspectionMeasurementBasisInput".to_string(),
                "UiInspectionMeasurementBasisPosture".to_string(),
                "UiInspectionMeasurementBasisSource".to_string(),
                "UiInspectionMeasurementChildIntrinsicSource".to_string(),
                "UiInspectionMeasurementDenialPosture".to_string(),
                "UiInspectionMeasurementDependencyLineageEntry".to_string(),
                "UiInspectionMeasurementDependencyLineageKind".to_string(),
                "UiInspectionMeasurementEvidenceCategory".to_string(),
                "UiInspectionMeasurementEvidenceSlot".to_string(),
                "UiInspectionMeasurementEvidenceView".to_string(),
                "UiInspectionMeasurementFailureSource".to_string(),
                "UiInspectionMeasurementGenerationCompatibility".to_string(),
                "UiInspectionMeasurementNeighborhoodClassHint".to_string(),
                "UiInspectionMeasurementOwnershipPosture".to_string(),
                "UiInspectionMeasurementQueryFactFamily".to_string(),
                "UiInspectionMeasurementQueryUnsupportedReason".to_string(),
                "UiInspectionQueryWorldCompatibilityFailure".to_string(),
                "UiInspectionRefLifecycleLane".to_string(),
                "UiInspectionScopeSupportRow".to_string(),
                "UiInspectionSliceLane".to_string(),
                "UiInspectionSupportReport".to_string(),
            ]),
        ),
        (
            "posture/mod.rs",
            BTreeSet::from([
                "UiInspectionAdmissionPosture".to_string(),
                "UiInspectionDeferredPosture".to_string(),
                "UiInspectionDiagnosticOnlyPosture".to_string(),
                "UiInspectionMilestoneExpectation".to_string(),
                "UiInspectionPosture".to_string(),
                "UiInspectionSupportPosture".to_string(),
                "UiInspectionSupportReason".to_string(),
                "UiInspectionSupportStatus".to_string(),
                "UiInspectionSupportWorld".to_string(),
                "UiInspectionUnsupportedPosture".to_string(),
                "UiInspectionWrongWorldPosture".to_string(),
            ]),
        ),
    ];
    let mut violations = Vec::new();

    for (relative_path, expected_names) in expected_exports {
        let path = inspection_root.join(relative_path);
        let actual_names = collect_public_export_names(&path);
        if actual_names != expected_names {
            violations.push(format!(
                "{} exports {:?}; expected {:?} for its single public responsibility",
                path.display(),
                actual_names,
                expected_names
            ));
        }
    }

    for declaration in collect_public_module_declarations(&inspection_root) {
        violations.push(format!(
            "{} introduces public child module `{}`; inspection topology must stay on the curated root re-export surface instead of growing nested public module trees by default",
            declaration.declaring_file.display(),
            declaration.module_name
        ));
    }

    violations.sort();
    violations.dedup();
    violations
}

pub fn audit_inspection_future_artifact_seed_topology(workspace_root: &Path) -> Vec<String> {
    let inspection_root = workspace_root.join("crates/worth-ui-inspection/src");
    let receipt_mod = inspection_root.join("receipt/mod.rs");
    let expected_seed_modules = [
        ("evidence", inspection_root.join("receipt/evidence/mod.rs")),
        ("replay", inspection_root.join("receipt/replay/mod.rs")),
        ("snapshot", inspection_root.join("receipt/snapshot/mod.rs")),
    ];
    let evidence_mod = inspection_root.join("receipt/evidence/mod.rs");
    let expected_evidence_seed_modules = [
        (
            "measurement",
            inspection_root.join("receipt/evidence/measurement/mod.rs"),
        ),
        (
            "mounting",
            inspection_root.join("receipt/evidence/mounting/mod.rs"),
        ),
        (
            "inspector",
            inspection_root.join("receipt/evidence/inspector/mod.rs"),
        ),
    ];
    let mut violations = Vec::new();

    for (module_name, module_path) in &expected_seed_modules {
        if !module_path.exists() {
            violations.push(format!(
                "{} is missing; future {module_name} inspection artifacts lack an honest internal home",
                module_path.display()
            ));
        }
    }

    let parsed = parse_rust_file(&receipt_mod);
    for (module_name, _) in &expected_seed_modules {
        let has_private_module = parsed.items.iter().any(|item| match item {
            Item::Mod(item_mod) => {
                item_mod.ident == *module_name && !matches!(item_mod.vis, Visibility::Public(_))
            }
            _ => false,
        });
        if !has_private_module {
            violations.push(format!(
                "{} must declare a private `{module_name}` child module as the future {module_name} inspection landing zone",
                receipt_mod.display()
            ));
        }
    }

    for (module_name, module_path) in &expected_evidence_seed_modules {
        if !module_path.exists() {
            violations.push(format!(
                "{} is missing; future {module_name} evidence lacks one obvious typed substrate home",
                module_path.display()
            ));
        }
    }

    let parsed_evidence = parse_rust_file(&evidence_mod);
    for (module_name, _) in &expected_evidence_seed_modules {
        let has_private_module = parsed_evidence.items.iter().any(|item| match item {
            Item::Mod(item_mod) => {
                item_mod.ident == *module_name && !matches!(item_mod.vis, Visibility::Public(_))
            }
            _ => false,
        });
        if !has_private_module {
            violations.push(format!(
                "{} must declare a private `{module_name}` child module as the future {module_name} evidence landing zone",
                evidence_mod.display()
            ));
        }
    }

    violations.sort();
    violations.dedup();
    violations
}

fn collect_public_export_names(path: &Path) -> BTreeSet<String> {
    let parsed = parse_rust_file(path);
    let mut names = BTreeSet::new();

    for item in parsed.items {
        match item {
            Item::Use(item_use) if matches!(item_use.vis, Visibility::Public(_)) => {
                collect_public_use_names(&item_use.tree, &mut names);
            }
            Item::Struct(item_struct) if matches!(item_struct.vis, Visibility::Public(_)) => {
                names.insert(item_struct.ident.to_string());
            }
            Item::Enum(item_enum) if matches!(item_enum.vis, Visibility::Public(_)) => {
                names.insert(item_enum.ident.to_string());
            }
            Item::Fn(item_fn) if matches!(item_fn.vis, Visibility::Public(_)) => {
                names.insert(item_fn.sig.ident.to_string());
            }
            Item::Const(item_const) if matches!(item_const.vis, Visibility::Public(_)) => {
                names.insert(item_const.ident.to_string());
            }
            _ => {}
        }
    }

    names
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct PublicModuleDeclaration {
    declaring_file: PathBuf,
    module_name: String,
}

fn collect_public_module_declarations(inspection_root: &Path) -> Vec<PublicModuleDeclaration> {
    let mut declarations = Vec::new();

    for path in collect_rust_files(inspection_root) {
        let parsed = parse_rust_file(&path);
        for item in parsed.items {
            if let Item::Mod(item_mod) = item {
                if matches!(item_mod.vis, Visibility::Public(_)) {
                    declarations.push(PublicModuleDeclaration {
                        declaring_file: path.clone(),
                        module_name: item_mod.ident.to_string(),
                    });
                }
            }
        }
    }

    declarations.sort_by(|left, right| {
        left.declaring_file
            .cmp(&right.declaring_file)
            .then(left.module_name.cmp(&right.module_name))
    });
    declarations.dedup();
    declarations
}

fn collect_rust_files(root: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    collect_rust_files_into(root, &mut files);
    files.sort();
    files
}

fn collect_rust_files_into(root: &Path, files: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(root).expect("source directory should read") {
        let entry = entry.expect("directory entry should read");
        let path = entry.path();
        if path.is_dir() {
            collect_rust_files_into(&path, files);
        } else if path.extension().is_some_and(|ext| ext == "rs") {
            files.push(path);
        }
    }
}

fn collect_public_use_names(tree: &UseTree, output: &mut BTreeSet<String>) {
    match tree {
        UseTree::Name(name) => {
            output.insert(name.ident.to_string());
        }
        UseTree::Rename(rename) => {
            output.insert(rename.rename.to_string());
        }
        UseTree::Group(group) => {
            for item in &group.items {
                collect_public_use_names(item, output);
            }
        }
        UseTree::Path(path) => collect_public_use_names(&path.tree, output),
        UseTree::Glob(_) => {}
    }
}

fn parse_rust_file(path: &Path) -> syn::File {
    let text = fs::read_to_string(path).expect("source file should decode");
    syn::parse_file(&text).unwrap_or_else(|error| {
        panic!("{} should parse as Rust source: {error}", path.display());
    })
}
