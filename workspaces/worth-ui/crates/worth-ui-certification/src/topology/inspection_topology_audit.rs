use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use super::workspace_source_inventory::WorkspaceSourceInventory;
use syn::{Item, UseTree, Visibility};

const FORBIDDEN_PUBLIC_MODULE_NAMES: [&str; 7] = [
    "internal", "common", "helpers", "utils", "data", "manager", "debug",
];

pub fn audit_inspection_public_module_names(inventory: &WorkspaceSourceInventory) -> Vec<String> {
    let mut violations = Vec::new();

    for declaration in collect_public_module_declarations(inventory) {
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

pub fn audit_inspection_public_module_role_purity(
    inventory: &WorkspaceSourceInventory,
) -> Vec<String> {
    let inspection_root = inventory.absolute_path("crates/worth-ui-inspection/src");
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
                "UiInspectionMeasurementEvidenceViewInput".to_string(),
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
        let actual_names = collect_public_export_names(inventory, &path);
        if actual_names != expected_names {
            violations.push(format!(
                "{} exports {:?}; expected {:?} for its single public responsibility",
                path.display(),
                actual_names,
                expected_names
            ));
        }
    }

    for declaration in collect_public_module_declarations(inventory) {
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

pub fn audit_inspection_future_artifact_seed_topology(
    inventory: &WorkspaceSourceInventory,
) -> Vec<String> {
    let inspection_root = inventory.absolute_path("crates/worth-ui-inspection/src");
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
        if inventory.source(module_path).is_none() {
            violations.push(format!(
                "{} is missing; future {module_name} inspection artifacts lack an honest internal home",
                module_path.display()
            ));
        }
    }

    let parsed = parse_rust_file(inventory, &receipt_mod);
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
        if inventory.source(module_path).is_none() {
            violations.push(format!(
                "{} is missing; future {module_name} evidence lacks one obvious typed substrate home",
                module_path.display()
            ));
        }
    }

    let parsed_evidence = parse_rust_file(inventory, &evidence_mod);
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

fn collect_public_export_names(
    inventory: &WorkspaceSourceInventory,
    path: &Path,
) -> BTreeSet<String> {
    let parsed = parse_rust_file(inventory, path);
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

fn collect_public_module_declarations(
    inventory: &WorkspaceSourceInventory,
) -> Vec<PublicModuleDeclaration> {
    let mut declarations = Vec::new();

    for source in inventory.rust_files_under("crates/worth-ui-inspection/src") {
        let path = source.absolute_path();
        let parsed = parse_rust_file(inventory, path);
        for item in parsed.items {
            if let Item::Mod(item_mod) = item {
                if matches!(item_mod.vis, Visibility::Public(_)) {
                    declarations.push(PublicModuleDeclaration {
                        declaring_file: path.to_path_buf(),
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

fn parse_rust_file(inventory: &WorkspaceSourceInventory, path: &Path) -> syn::File {
    let text = inventory.text(path);
    syn::parse_file(text).unwrap_or_else(|error| {
        panic!("{} should parse as Rust source: {error}", path.display());
    })
}
