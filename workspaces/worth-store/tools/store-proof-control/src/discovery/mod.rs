mod artifact_footprint;
mod baseline_capture;
mod cargo_diagnostics;
mod cargo_surface;
mod executable_listing;
mod libtest_listing;
mod owner_build_closure;
mod owner_fixture_policy;
mod rustdoc_listing;
mod source_tests;
mod workflow_commands;

use std::path::Path;

pub use artifact_footprint::{observe_artifact_footprint, ObservedArtifactFootprint};
pub use baseline_capture::BaselineCaptureStatus;
use cargo_surface::discover_cargo_surface;
pub use cargo_surface::{
    DependencyEdge, ObservedBuildGraph, ObservedFeatureGraph, PackageSurface, TestTargetIdentity,
};
pub use executable_listing::{
    observe_executable_listing, validate_executable_listing, CurrentExecutableListing,
};
pub use owner_build_closure::{
    generate_owner_build_closures, validate_owner_build_closures, OwnerBuildClosure,
    OwnerTestBoundary, TestSupportAuthorityClass,
};
pub use owner_fixture_policy::OwnerFixtureDependency;
use serde::{Deserialize, Serialize};
pub use source_tests::{CaseKind, TestCaseIdentity, TestCaseSurface};
pub use workflow_commands::WorkflowProofCommand;

use crate::classification::FeatureSemanticAuthority;
use crate::evidence::read_json;
use crate::DiscoveredTestSurface;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestSurfaceInventory {
    pub schema_version: u32,
    pub workspace_root: String,
    pub target_root: String,
    pub packages: Vec<PackageSurface>,
    pub targets: Vec<TestTargetIdentity>,
    pub cases: Vec<TestCaseSurface>,
    pub build_graph: ObservedBuildGraph,
    #[serde(default)]
    pub workflow_commands: Vec<WorkflowProofCommand>,
    pub historical_artifacts: ObservedArtifactFootprint,
    #[serde(default)]
    pub feature_semantic_authority: FeatureSemanticAuthority,
}

pub type PreCleanupProofInventory = TestSurfaceInventory;

pub fn discover_workspace(
    workspace_root: &Path,
    observe_artifacts: bool,
) -> Result<DiscoveredTestSurface, String> {
    let cargo_surface = discover_cargo_surface(workspace_root)?;
    let cases = source_tests::discover_test_cases(&cargo_surface)?;
    let historical_artifacts = if observe_artifacts {
        observe_artifact_footprint(Path::new(&cargo_surface.target_root))?
    } else {
        ObservedArtifactFootprint::not_observed(&cargo_surface.target_root)
    };
    let workflow_commands = workflow_commands::discover_workflow_commands(workspace_root)?;
    let feature_semantic_authority =
        read_json(&workspace_root.join("test-control/feature-semantic-authority.json"))?;
    let inventory = TestSurfaceInventory {
        schema_version: 2,
        workspace_root: cargo_surface.workspace_root,
        target_root: cargo_surface.target_root,
        packages: cargo_surface.packages,
        targets: cargo_surface.targets,
        cases,
        build_graph: cargo_surface.build_graph,
        workflow_commands,
        historical_artifacts,
        feature_semantic_authority,
    };
    Ok(DiscoveredTestSurface::from_repository(inventory))
}
