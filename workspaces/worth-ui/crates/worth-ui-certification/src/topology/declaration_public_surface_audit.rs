use std::fs;
use std::path::Path;

use syn::{File, Item, ItemUse, UseTree, Visibility};

use super::public_surface_audit::collect_public_names;

const RUNTIME_FACADE_ROOT: &str = "crates/worth-ui-runtime/src/facade/mod.rs";
const RUNTIME_DECLARATION_FACADE: &str = "crates/worth-ui-runtime/src/facade/declaration.rs";
const PRODUCT_DECLARATION_FACADE: &str = "crates/worth-ui/src/facade/declaration.rs";
const CURATED_DECLARATION_PUBLIC_NAMES: &[&str] = &[
    "UiAspectContract",
    "UiAspectContractAdmissionDenial",
    "UiAspectCoverageEntry",
    "UiAspectCoverageReport",
    "UiAspectFamily",
    "UiAspectName",
    "UiAspectSemanticSlice",
    "UiConsumedAspectContract",
    "UiDeclarationArtifact",
    "UiDeclarationArtifactDigest",
    "UiDeclarationAspectDigest",
    "UiDeclarationCloseoutGuarantee",
    "UiDeclarationCloseoutNonGoal",
    "UiDeclarationCloseoutReport",
    "UiDeclarationClosedSemanticLane",
    "UiDeclarationContainmentIntent",
    "UiDeclarationDigestProjection",
    "UiDeclarationEquivalenceContract",
    "UiDeclarationFamily",
    "UiDeclarationFamilyAdmissionDenial",
    "UiDeclarationFamilyCatalog",
    "UiDeclarationFamilyDigest",
    "UiDeclarationFamilyKind",
    "UiDeclarationGraphHandoff",
    "UiDeclarationGraphHandoffDenial",
    "UiDeclarationIdentity",
    "UiDeclarationIdentityDigest",
    "UiDeclarationOrderingGuarantee",
    "UiDeclarationPostureDigest",
    "UiDeclarationProvenance",
    "UiDeclarationRepetitionPosture",
    "UiDeclarationSlotParticipationIntent",
    "UiDeclarationStructuralDigest",
    "UiDeclarationStructuralRole",
    "UiDeclarationStructuralSemantics",
    "UiDeclarationStructuralSemanticsAdmissionDenial",
    "UiDeclarationSupportDigest",
    "UiDeclarationSupportMilestoneExpectation",
    "UiDeclarationSupportRow",
    "UiDeclarationSupportRowSchemaKind",
    "UiDeclarationSupportSnapshot",
    "UiDeclarationSupportSnapshotAdmissionDenial",
    "UiDeclarationUnsupportedPosture",
    "UiDeclaredHostCapabilityPosture",
    "UiDeclaredMeasurementPolicyPosture",
    "UiDeclaredPostureAdmissionDenial",
    "UiDeclaredPostureApplicability",
    "UiDeclaredPostureContract",
    "UiDeclaredPostureLane",
    "UiDeclaredPostureLaneKind",
    "UiDeclaredQueryBindingPosture",
    "UiDeclaredServiceUsagePosture",
    "UiDeclaredTouchMeaningPosture",
    "UiPublishedAspectContract",
    "WorthUiHostCapability",
];

pub fn audit_runtime_declaration_surface_routes_through_curated_submodule(
    workspace_root: &Path,
) -> Vec<String> {
    let root_path = workspace_root.join(RUNTIME_FACADE_ROOT);
    let root_public_names = collect_public_names(&root_path);
    let mut violations = Vec::new();

    for name in root_public_names {
        if looks_like_declaration_surface(&name) {
            violations.push(format!(
                "{} publicly exposes `{name}` from the runtime facade root instead of routing declaration authority through `facade::declaration`",
                root_path.display()
            ));
        }
    }

    if !parse_rust_file(&root_path).items.iter().any(|item| {
        matches!(
            item,
            Item::Mod(item_mod)
                if matches!(item_mod.vis, Visibility::Public(_)) && item_mod.ident == "declaration"
        )
    }) {
        violations.push(format!(
            "{} must publish one dedicated `declaration` facade submodule",
            root_path.display()
        ));
    }

    violations.sort();
    violations.dedup();
    violations
}

pub fn audit_declaration_facades_are_curated_and_glob_free(workspace_root: &Path) -> Vec<String> {
    let runtime_path = workspace_root.join(RUNTIME_DECLARATION_FACADE);
    let product_path = workspace_root.join(PRODUCT_DECLARATION_FACADE);
    let runtime_names = collect_public_names(&runtime_path);
    let product_names = collect_public_names(&product_path);
    let mut violations = Vec::new();

    if runtime_names != product_names {
        violations.push(format!(
            "{} must mirror the curated runtime declaration facade exactly; product names: {:?}, runtime names: {:?}",
            product_path.display(),
            product_names,
            runtime_names
        ));
    }

    let curated_names = curated_name_set();
    if runtime_names != curated_names {
        violations.push(format!(
            "{} must expose exactly the curated declaration capability set; observed: {:?}, expected: {:?}",
            runtime_path.display(),
            runtime_names,
            curated_names
        ));
    }

    if let Some(reason) = first_invalid_public_use(
        &runtime_path,
        &[&["crate", "declaration"], &["worth_ui_host_contract"]],
    ) {
        violations.push(format!("{} {reason}", runtime_path.display()));
    }
    if let Some(reason) = first_invalid_public_use(
        &product_path,
        &[&["worth_ui_runtime", "facade", "declaration"]],
    ) {
        violations.push(format!("{} {reason}", product_path.display()));
    }

    violations.sort();
    violations.dedup();
    violations
}

fn first_invalid_public_use(path: &Path, allowed_prefixes: &[&[&str]]) -> Option<String> {
    let parsed = parse_rust_file(path);

    for item in parsed.items {
        match item {
            Item::Use(item_use) if matches!(item_use.vis, Visibility::Public(_)) => {
                if contains_glob_use(&item_use.tree) {
                    return Some(
                        "must enumerate curated declaration exports explicitly instead of using glob re-exports"
                            .to_string(),
                    );
                }

                let prefixes = public_use_prefixes(&item_use);
                if prefixes.iter().any(|prefix| {
                    !allowed_prefixes.iter().any(|expected_prefix| {
                        prefix.len() >= expected_prefix.len()
                            && expected_prefix
                                .iter()
                                .zip(prefix.iter())
                                .all(|(expected, actual)| expected == actual)
                    })
                }) {
                    return Some(format!(
                        "must route public declaration exports only through one of: {}",
                        allowed_prefixes
                            .iter()
                            .map(|prefix| prefix.join("::"))
                            .collect::<Vec<_>>()
                            .join(", ")
                    ));
                }
            }
            Item::Mod(item_mod) if matches!(item_mod.vis, Visibility::Public(_)) => {
                return Some(format!(
                    "must not publish nested public modules such as `{}` from the declaration facade",
                    item_mod.ident
                ));
            }
            _ => {}
        }
    }

    None
}

fn public_use_prefixes(item_use: &ItemUse) -> Vec<Vec<String>> {
    let mut prefixes = Vec::new();
    collect_use_prefixes(&item_use.tree, Vec::new(), &mut prefixes);
    prefixes
}

fn collect_use_prefixes(tree: &UseTree, prefix: Vec<String>, output: &mut Vec<Vec<String>>) {
    match tree {
        UseTree::Path(path) => {
            let mut next = prefix;
            next.push(path.ident.to_string());
            collect_use_prefixes(&path.tree, next, output);
        }
        UseTree::Group(group) => {
            for item in &group.items {
                collect_use_prefixes(item, prefix.clone(), output);
            }
        }
        UseTree::Name(_) | UseTree::Rename(_) => output.push(prefix),
        UseTree::Glob(_) => output.push(prefix),
    }
}

fn contains_glob_use(tree: &UseTree) -> bool {
    match tree {
        UseTree::Glob(_) => true,
        UseTree::Path(path) => contains_glob_use(&path.tree),
        UseTree::Group(group) => group.items.iter().any(contains_glob_use),
        UseTree::Name(_) | UseTree::Rename(_) => false,
    }
}

fn looks_like_declaration_surface(name: &str) -> bool {
    name.starts_with("UiDeclaration")
        || name.starts_with("UiDeclared")
        || name.starts_with("UiAspect")
        || name.starts_with("UiStructuralDeclaration")
        || name.starts_with("UiControlDeclarationFamily")
        || name.starts_with("UiPage")
        || name.starts_with("UiRegion")
        || name.starts_with("UiMosaic")
        || name.starts_with("UiQueryBindingDeclarationFamily")
        || name.starts_with("UiDiagnosticSurfaceDeclarationFamily")
        || name.starts_with("UiIntentDeclarationFamily")
        || name.starts_with("UiLocalCompositionDeclarationFamily")
        || name.starts_with("UiPublishedAspectContract")
        || name.starts_with("UiConsumedAspectContract")
}

fn parse_rust_file(path: &Path) -> File {
    let text = fs::read_to_string(path).expect("source file should decode");
    syn::parse_file(&text).unwrap_or_else(|error| {
        panic!("{} should parse as Rust source: {error}", path.display());
    })
}

fn curated_name_set() -> std::collections::BTreeSet<String> {
    CURATED_DECLARATION_PUBLIC_NAMES
        .iter()
        .map(|name| (*name).to_owned())
        .collect()
}
