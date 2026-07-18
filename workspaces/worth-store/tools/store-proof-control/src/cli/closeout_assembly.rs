use std::path::{Component, Path, PathBuf};

use serde::Deserialize;

use crate::artifact_lifecycle::{BuildArtifactCleanupPlan, BuildArtifactInventory};
use crate::closeout::{
    execute_mutation_matrix, C2TestArchitectureReadiness, StableProofCommand,
    TestArchitectureCloseoutBundle, TestArchitectureCloseoutInputs,
};
use crate::evidence::{read_json, write_immutable_json};
use crate::selection::observe_repository_identity;

use super::closeout_commands::{
    artifact_reference, build_iteration, build_preservation, closeout_root, input_path,
    read_manifest_input,
};

pub(super) fn assemble(workspace_root: &Path, manifest_path: &Path) -> Result<(), String> {
    let manifest_path = input_path(workspace_root, manifest_path)?;
    let manifest: CloseoutAssemblyManifest = read_json(&manifest_path)?;
    manifest.validate()?;

    let (inventory, preservation, proof_inventory_path, _) = build_preservation(workspace_root)?;
    let mutation_sensitivity =
        execute_mutation_matrix(workspace_root, &inventory, preservation.evidence_identity())?;
    let developer_iteration = build_iteration(
        workspace_root,
        Path::new(&manifest.developer_iteration_manifest),
    )?;
    let ci_root = input_directory(workspace_root, Path::new(&manifest.ci_evidence_root))?;
    let ci_evidence = crate::ci::read_partition_evidence(&ci_root)?;
    let ci = crate::ci::CiCertificationAggregate::certify(&inventory, &ci_evidence)
        .map_err(render_missing_ci)?;
    let artifact_inventory: BuildArtifactInventory =
        read_manifest_input(workspace_root, &manifest.artifact_inventory)?;
    let artifact_cleanup_plan: BuildArtifactCleanupPlan =
        read_manifest_input(workspace_root, &manifest.artifact_cleanup_plan)?;
    let repository =
        observe_repository_identity(workspace_root).map_err(|error| error.to_string())?;

    let bundle = TestArchitectureCloseoutBundle::certify(TestArchitectureCloseoutInputs {
        repository,
        proof_inventory: artifact_reference(
            workspace_root,
            &proof_inventory_path.to_string_lossy(),
        )?,
        owner_build_closures: artifact_reference(
            workspace_root,
            "test-control/owner-build-closures.json",
        )?,
        scenario_suite_inventory: artifact_reference(
            workspace_root,
            "test-control/consolidated-suite-inventory.json",
        )?,
        preservation,
        mutation_sensitivity,
        developer_iteration,
        ci,
        artifact_inventory,
        artifact_cleanup_plan,
        stable_commands: manifest.stable_commands,
    })?;
    let readiness = C2TestArchitectureReadiness::issue(&bundle)?;
    publish_outputs(workspace_root, &bundle, &readiness)
}

fn publish_outputs(
    workspace_root: &Path,
    bundle: &TestArchitectureCloseoutBundle,
    readiness: &C2TestArchitectureReadiness,
) -> Result<(), String> {
    let bundle_path = closeout_root(workspace_root)
        .join("bundles")
        .join(format!("{}.json", bundle.evidence_identity()));
    let readiness_path = closeout_root(workspace_root)
        .join("c2-readiness")
        .join(format!("{}.json", readiness.readiness_identity()));
    write_immutable_json(&bundle_path, bundle)?;
    write_immutable_json(&readiness_path, readiness)?;
    println!("C1 closeout bundle: {}", bundle_path.display());
    println!("sealed C2 readiness: {}", readiness_path.display());
    Ok(())
}

fn input_directory(workspace_root: &Path, path: &Path) -> Result<PathBuf, String> {
    if path
        .components()
        .any(|part| matches!(part, Component::ParentDir | Component::CurDir))
    {
        return Err(format!(
            "closeout directory contains path traversal: {}",
            path.display()
        ));
    }
    let path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        workspace_root.join(path)
    };
    let metadata = std::fs::symlink_metadata(&path)
        .map_err(|error| format!("could not inspect {}: {error}", path.display()))?;
    if !metadata.is_dir() || metadata.file_type().is_symlink() {
        return Err(format!(
            "closeout CI input is not a real directory: {}",
            path.display()
        ));
    }
    path.canonicalize()
        .map_err(|error| format!("could not resolve {}: {error}", path.display()))
}

fn render_missing_ci(missing: Vec<crate::ci::MissingCiProofPartition>) -> String {
    missing
        .into_iter()
        .map(|lane| {
            format!(
                "{}/{}: {}",
                lane.partition, lane.operating_system, lane.reason
            )
        })
        .collect::<Vec<_>>()
        .join("\n  - ")
}

#[derive(Debug, Deserialize)]
struct CloseoutAssemblyManifest {
    schema_version: u32,
    developer_iteration_manifest: String,
    ci_evidence_root: String,
    artifact_inventory: String,
    artifact_cleanup_plan: String,
    stable_commands: Vec<StableProofCommand>,
}

impl CloseoutAssemblyManifest {
    fn validate(&self) -> Result<(), String> {
        if self.schema_version != 2
            || self.developer_iteration_manifest.trim().is_empty()
            || self.ci_evidence_root.trim().is_empty()
            || self.artifact_inventory.trim().is_empty()
            || self.artifact_cleanup_plan.trim().is_empty()
        {
            return Err(
                "closeout assembly manifest must use schema 2 and name every observed input"
                    .to_owned(),
            );
        }
        Ok(())
    }
}
