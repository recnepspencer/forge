use crate::discovery::{discover_workspace, generate_owner_build_closures};

use super::scratch_workspace::ScratchCargoWorkspace;

#[test]
fn renamed_optional_dependency_features_are_resolved_by_manifest_alias() {
    let workspace = ScratchCargoWorkspace::new("renamed-optional-dependency");
    workspace.write(
        "Cargo.toml",
        "[workspace]\nmembers = [\"owner\", \"support\"]\nresolver = \"2\"\n",
    );
    workspace.write(
        "owner/Cargo.toml",
        "[package]\nname = \"fixture-owner\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[features]\ndefault = [\"dep:fixture_alias\", \"fixture_alias/narrow-surface\"]\n\n[dependencies]\nfixture_alias = { package = \"fixture-support\", path = \"../support\", optional = true }\n",
    );
    workspace.write("owner/src/lib.rs", "pub fn owner() {}\n");
    workspace.write(
        "support/Cargo.toml",
        "[package]\nname = \"fixture-support\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[features]\nnarrow-surface = []\n",
    );
    workspace.write("support/src/lib.rs", "pub fn support() {}\n");
    workspace.write(
        "test-control/feature-semantic-authority.json",
        "{\"schema_version\":1,\"declarations\":[{\"package\":\"fixture-support\",\"feature\":\"narrow-surface\",\"authority\":\"production\"}]}\n",
    );

    let discovered = discover_workspace(workspace.root(), false).unwrap();
    let closures = generate_owner_build_closures(discovered.inventory());
    let owner = closures
        .iter()
        .find(|closure| closure.boundary.owner_package == "fixture-owner")
        .unwrap();
    assert!(owner
        .compiled_workspace_packages
        .contains("fixture-support"));
    assert!(owner
        .activated_features
        .get("fixture-support")
        .is_some_and(|features| features.contains("narrow-surface")));
}
