use std::path::Path;

use crate::artifact_lifecycle::{
    BuildArtifactCleanupOutcome, BuildArtifactCleanupPlan, BuildArtifactCleanupReceipt,
    BuildArtifactInventory, BuildArtifactRetentionPolicy,
};
use crate::evidence::{read_json, write_immutable_json};
use crate::execution::ExecutedProofRun;

pub(super) fn inspect_artifacts(
    workspace_root: &Path,
    target_root: &Path,
    protected_run_path: Option<&Path>,
) -> Result<(), String> {
    let protected_run = protected_run_path
        .map(read_json::<ExecutedProofRun>)
        .transpose()?;
    let inventory =
        BuildArtifactInventory::inspect(workspace_root, target_root, protected_run.as_ref())?;
    let path = inventory.output_path();
    write_immutable_json(&path, &inventory)?;
    println!("artifact target root: {}", inventory.target_root());
    println!(
        "artifact inventory: files={} directories={} logical-bytes={}",
        inventory.file_count(),
        inventory.directory_count(),
        inventory.logical_bytes()
    );
    println!("artifact inventory evidence: {}", path.display());
    Ok(())
}

pub(super) fn plan_artifact_cleanup(inventory_path: &Path, policy: &str) -> Result<(), String> {
    let inventory: BuildArtifactInventory = read_json(inventory_path)?;
    let policy = match policy {
        "bounded-local" => BuildArtifactRetentionPolicy::bounded_local()?,
        _ => return Err(format!("unknown artifact retention policy: {policy}")),
    };
    let plan = BuildArtifactCleanupPlan::lower(&inventory, policy)?;
    print_cleanup_plan(&plan);
    let path = plan.output_path();
    write_immutable_json(&path, &plan)?;
    println!("artifact cleanup plan evidence: {}", path.display());
    println!("no artifact was deleted; execute requires the saved plan path");
    Ok(())
}

pub(super) fn execute_artifact_cleanup(plan_path: &Path) -> Result<(), String> {
    let plan: BuildArtifactCleanupPlan = read_json(plan_path)?;
    print_cleanup_plan(&plan);
    let receipt = BuildArtifactCleanupReceipt::execute(&plan)?;
    println!("artifact cleanup outcome: {:?}", receipt.outcome());
    println!(
        "artifact cleanup receipt: {}",
        receipt
            .output_path(Path::new(plan.workspace_root()))
            .display()
    );
    if matches!(receipt.outcome(), BuildArtifactCleanupOutcome::Completed) {
        Ok(())
    } else {
        Err(format!(
            "artifact cleanup was partially applied; receipt={}",
            receipt.receipt_identity()
        ))
    }
}

fn print_cleanup_plan(plan: &BuildArtifactCleanupPlan) {
    println!("artifact cleanup target root: {}", plan.target_root());
    println!(
        "artifact cleanup selection: files={} directories={} logical-bytes={}",
        plan.selected_file_count(),
        plan.selected_directory_count(),
        plan.selected_logical_bytes()
    );
    for target in plan.targets() {
        println!(
            "  - {:?} {:?} {}",
            target.class(),
            target.kind(),
            target.absolute_path()
        );
    }
    println!(
        "protected artifact identities: {}",
        plan.protected_artifacts().len()
    );
}
