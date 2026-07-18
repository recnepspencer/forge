use crate::classification::{
    validate_inventory_build_graph_policy, BuildGraphPolicyViolation, DependencyBoundaryDenial,
    FeatureSemanticAuthorityDenial,
};
use crate::discovery::discover_workspace;
use worth_store_test_support::structural_preflight::DependencyBoundaryPredicate;

use super::scratch_workspace::ScratchCargoWorkspace;

#[test]
fn transitive_default_feature_authority_leak_names_the_manifest_edge() {
    let workspace = ScratchCargoWorkspace::new("feature-leak");
    workspace.write(
        "Cargo.toml",
        "[workspace]\nmembers = [\"app\", \"substrate\"]\nresolver = \"2\"\n",
    );
    workspace.write(
        "app/Cargo.toml",
        "[package]\nname = \"fixture-app\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\nfixture-substrate = { path = \"../substrate\" }\n",
    );
    workspace.write("app/src/lib.rs", "pub fn app() {}\n");
    workspace.write(
        "substrate/Cargo.toml",
        "[package]\nname = \"fixture-substrate\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[features]\ndefault = [\"certification-test-authority\"]\ncertification-test-authority = []\n",
    );
    workspace.write("substrate/src/lib.rs", "pub fn substrate() {}\n");
    workspace.write(
        "test-control/feature-semantic-authority.json",
        "{\"schema_version\":1,\"declarations\":[{\"package\":\"fixture-substrate\",\"feature\":\"certification-test-authority\",\"authority\":\"test_authority\"}]}\n",
    );

    let discovered = discover_workspace(workspace.root(), false).unwrap();
    let denials = validate_inventory_build_graph_policy(discovered.inventory()).unwrap_err();
    let violation = denials
        .iter()
        .find_map(|denial| match denial {
            BuildGraphPolicyViolation::DependencyBoundary(violation)
                if violation.denial
                    == DependencyBoundaryDenial::ResolvedProductionFeatureClosure =>
            {
                Some(violation)
            }
            _ => None,
        })
        .unwrap();
    assert_eq!(
        violation.predicate,
        DependencyBoundaryPredicate::ForbiddenFeatureEdge {
            source_package: "fixture-app".to_owned(),
            feature: "certification-test-authority".to_owned(),
            forbidden_dependency: "fixture-substrate".to_owned(),
        }
    );
    assert_eq!(violation.dependency_kind, "normal");
}

#[test]
fn new_workspace_feature_requires_explicit_semantic_classification() {
    let workspace = ScratchCargoWorkspace::new("unclassified-feature");
    workspace.write(
        "Cargo.toml",
        "[workspace]\nmembers = [\"owner\"]\nresolver = \"2\"\n",
    );
    workspace.write(
        "owner/Cargo.toml",
        "[package]\nname = \"fixture-owner\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[features]\nopaque-fixture-surface = []\n",
    );
    workspace.write("owner/src/lib.rs", "pub fn owner() {}\n");

    let discovered = discover_workspace(workspace.root(), false).unwrap();
    let denials = validate_inventory_build_graph_policy(discovered.inventory()).unwrap_err();
    assert!(denials.iter().any(|denial| matches!(
        denial,
        BuildGraphPolicyViolation::FeatureSemanticAuthority(violation)
            if violation.denial == FeatureSemanticAuthorityDenial::MissingDeclaration
    )));
    assert!(denials[0].to_string().contains("fixture-owner"));
    assert!(denials[0].to_string().contains("opaque-fixture-surface"));
}

#[test]
fn feature_semantic_authority_schema_is_enforced() {
    let workspace = ScratchCargoWorkspace::new("feature-authority-schema");
    workspace.write(
        "Cargo.toml",
        "[workspace]\nmembers = [\"owner\"]\nresolver = \"2\"\n",
    );
    workspace.write(
        "owner/Cargo.toml",
        "[package]\nname = \"fixture-owner\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
    );
    workspace.write("owner/src/lib.rs", "pub fn owner() {}\n");
    workspace.write(
        "test-control/feature-semantic-authority.json",
        "{\"schema_version\":99,\"declarations\":[]}\n",
    );
    let discovered = discover_workspace(workspace.root(), false).unwrap();
    let denials = validate_inventory_build_graph_policy(discovered.inventory()).unwrap_err();
    assert!(denials.iter().any(|denial| matches!(
        denial,
        BuildGraphPolicyViolation::FeatureSemanticAuthority(violation)
            if violation.denial == FeatureSemanticAuthorityDenial::UnsupportedSchema
    )));
    assert!(denials[0].to_string().contains("schema 99"));
}
